use super::*;
use std::cell::RefCell;

thread_local! {
    static INSTRUCTIONS: RefCell<Builder> = RefCell::new(Builder::default());
}

fn borrow<F: FnOnce(&mut Builder) -> R, R>(f: F) -> R {
    INSTRUCTIONS.with(|b| f(&mut b.borrow_mut()))
}

#[derive(Clone, Debug, Default)]
struct Builder {
    once_instructions: Vec<Transform>,
    instructions: Vec<Transform>,
    in_once: bool,
    skip_stack: Vec<usize>,
    local: usize,
    cells: Vec<Event>,
    outputs: Vec<(String, Output)>,
    inputs: Vec<(String, Input)>,
    constants: HashMap<Event, Local>,
    tables: usize,
}

impl Builder {
    fn build(self) -> Config {
        let locals = self.local;
        let instructions = self.instructions;
        let databases = self.tables;
        let cells = self.cells;
        let outputs = self.outputs;
        let inputs = self.inputs;
        let once_instructions = self.once_instructions;

        Config {
            locals,
            instructions,
            once_instructions,
            cells,
            databases,
            outputs,
            inputs,
        }
    }

    fn swap_once(&mut self) {
        self.in_once = !self.in_once;
    }
}

fn emit<T: Into<Transform>>(transform: T) -> usize {
    borrow(|b| {
        let t = transform.into();

        if matches!(t, Transform::Constant { .. }) || b.in_once {
            b.once_instructions.push(t);
            return 0;
        }

        let idx = b.instructions.len();
        b.instructions.push(t);
        for s in &mut b.skip_stack {
            *s += 1;
        }
        idx
    })
}

pub fn constant<T: Into<Event>>(value: T) -> Local {
    let value = value.into();
    if let Some(l) = borrow(|b| b.constants.get(&value).copied()) {
        return l;
    }

    let destination = local();

    borrow(|b| b.constants.insert(value.clone(), destination));

    emit(Transform::Constant { value, destination });
    destination
}

macro_rules! ext {
    ($name:ident, $variant:ident, $ty:ty, $field:ident) => {
        pub fn $name<T: Into<$ty>>($field: T) -> Local {
            let l = local();
            emit(Transform::$variant {
                $field: $field.into(),
                destination: l,
            });
            l
        }
    };
}

ext!(math, Math, Math, math);
ext!(logic, Logic, Logic, logic);
ext!(midi, Midi, Midi, midi);

pub fn local() -> Local {
    borrow(|state| {
        let id = state.local;
        state.local += 1;
        Local { id }
    })
}

pub fn argument(index: usize) -> Local {
    let destination = local();

    emit(Transform::Argument { index, destination });

    destination
}

pub fn table() -> TableBuilder {
    borrow(|state| {
        let id = state.tables;
        state.tables += 1;
        TableBuilder { id }
    })
}

pub struct TableBuilder {
    id: usize,
}

impl TableBuilder {
    pub fn push<G: Into<Vec<Local>>>(&self, groups: G, value: Local) {
        emit(Transform::Table {
            id: self.id,
            table: Table::Push {
                source: value,
                groups: groups.into(),
            },
        });
    }

    pub fn group_len(&self, group: Local) -> Local {
        let destination = local();

        emit(Transform::Table {
            id: self.id,
            table: Table::GroupLen { group, destination },
        });

        destination
    }

    pub fn select_index(&self, group: Local, index: Local) -> Local {
        let destination = local();

        emit(Transform::Table {
            id: self.id,
            table: Table::SelectIndex {
                group,
                index,
                destination,
            },
        });

        destination
    }

    pub fn send_all<G: Into<Vec<Local>>>(&self, groups: G, output: Output) {
        emit(Transform::Table {
            id: self.id,
            table: Table::SendAll {
                groups: groups.into(),
                output: constant(output.id as i64),
            },
        });
    }
}

pub fn cell<I: Into<Event>>(init: I) -> Cell {
    borrow(|state| {
        let id = state.cells.len();
        state.cells.push(init.into());
        Cell { id }
    })
}

impl Cell {
    pub fn load(&self) -> Local {
        let l = local();
        emit(Transform::Load {
            cell: *self,
            destination: l,
        });
        l
    }

    pub fn store(&self, value: Local) {
        value.store(*self)
    }
}

pub fn output<N: Into<String>>(name: N) -> Output {
    borrow(|state| {
        let id = state.outputs.len();
        let v = Output { id, bus: None };
        state.outputs.push((name.into(), v));
        v
    })
}

pub fn input<N: Into<String>>(name: N) -> Input {
    borrow(|state| {
        let id = state.inputs.len();
        let v = Input { id, is_bus: false };
        state.inputs.push((name.into(), v));
        v
    })
}

pub fn bus() -> (Input, Output) {
    let i = input("");
    let o = output("");

    // remove the physical I/O
    borrow(|state| {
        state.inputs.last_mut().unwrap().1.is_bus = true;
        state.outputs.last_mut().unwrap().1.bus = Some(i.id);
    });

    (i, o)
}

pub fn cond<F: FnOnce() -> R, R>(condition: Local, f: F) -> R {
    // push the condition instruction
    let idx = emit(Transform::Condition {
        skip_count: 0,
        condition: condition.id,
    });

    // push a condition on to the stack
    borrow(|v| v.skip_stack.push(0));

    // execute the truthy condition
    let r = f();

    // get the final skip count
    borrow(|b| {
        let skip_count = b.skip_stack.pop().unwrap();
        b.instructions[idx] = Transform::Condition {
            skip_count,
            condition: condition.id,
        };
    });

    // return the closure value
    r
}

pub fn build<F: FnOnce()>(f: F) -> Config {
    f();
    borrow(|b| core::mem::take(b).build())
}

pub fn once<F: FnOnce() -> R, R>(f: F) -> R {
    borrow(|b| b.swap_once());
    let r = f();
    borrow(|b| b.swap_once());
    r
}

impl Local {
    pub fn assign_to(&self, destination: Local) {
        emit(Transform::Move {
            source: *self,
            destination,
        });
    }

    pub fn filter<T: Into<Filter>>(&self, filter: T) -> Local {
        let l = local();
        emit(Transform::Filter {
            source: *self,
            filter: filter.into(),
            destination: l,
        });
        l
    }

    pub fn select<T: Into<Select>>(&self, select: T) -> Local {
        let l = local();
        emit(Transform::Select {
            source: *self,
            select: select.into(),
            destination: l,
        });
        l
    }

    pub fn theory<T: Into<Theory>>(&self, theory: T) -> Local {
        let l = local();
        emit(Transform::Theory {
            source: *self,
            theory: theory.into(),
            destination: l,
        });
        l
    }

    pub fn inspect<S: Into<String>>(&self, label: S) -> Self {
        emit(Transform::Inspect {
            label: label.into(),
            source: *self,
        });
        *self
    }

    pub fn store(&self, cell: Cell) {
        emit(Transform::Store {
            source: *self,
            cell,
        });
    }

    pub fn not(&self) -> Local {
        logic(Logic::Not(vec![*self]))
    }

    pub fn or(&self, rhs: Local) -> Local {
        logic(Logic::Any(vec![*self, rhs]))
    }

    pub fn eq(&self, rhs: Local) -> Local {
        logic(Logic::Equal(vec![*self, rhs]))
    }

    pub fn add(&self, rhs: Local) -> Local {
        math(Math::Add(vec![*self, rhs]))
    }

    pub fn sub(&self, rhs: Local) -> Local {
        math(Math::Sub(vec![*self, rhs]))
    }

    pub fn div(&self, rhs: Local) -> Local {
        math(Math::Div(vec![*self, rhs]))
    }

    pub fn rem(&self, rhs: Local) -> Local {
        math(Math::Mod(vec![*self, rhs]))
    }

    // TODO
    // pub fn load()
    // pub fn store()
}

#[derive(Clone, Copy, Debug)]
pub struct Output {
    pub(super) id: usize,
    pub(super) bus: Option<usize>,
}

impl Output {
    pub fn send(&self, local: Local) {
        emit(Transform::Send {
            source: local,
            output: constant(self.id as i64),
        });
    }

    pub fn send_after(&self, local: Local, delay: Local) {
        emit(Transform::SendAfter {
            source: local,
            delay,
            output: constant(self.id as i64),
        });
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub(super) id: usize,
    pub(super) is_bus: bool,
}

impl Input {
    pub fn is_from(&self) -> Local {
        argument(1).eq(constant(self.id as i64))
    }
}
