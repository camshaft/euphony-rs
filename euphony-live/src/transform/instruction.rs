use super::*;

#[derive(Clone, Debug)]
pub enum Transform {
    Noop,
    Constant {
        value: Event,
        destination: Local,
    },
    Logic {
        logic: Logic,
        destination: Local,
    },
    Math {
        math: Math,
        destination: Local,
    },
    Move {
        source: Local,
        destination: Local,
    },
    Argument {
        index: usize,
        destination: Local,
    },
    Load {
        cell: Cell,
        destination: Local,
    },
    Store {
        source: Local,
        cell: Cell,
    },
    Send {
        source: Local,
        output: Local,
    },
    SendAfter {
        source: Local,
        output: Local,
        delay: Local,
    },
    Filter {
        source: Local,
        filter: Filter,
        destination: Local,
    },
    Select {
        source: Local,
        select: Select,
        destination: Local,
    },
    Midi {
        midi: Midi,
        destination: Local,
    },
    Theory {
        source: Local,
        theory: Theory,
        destination: Local,
    },
    Inspect {
        label: String,
        source: Local,
    },
    Condition {
        skip_count: usize,
        condition: usize,
    },
    Table {
        id: usize,
        table: Table,
    },
}

impl Transform {
    pub fn apply(
        &self,
        args: &[Event],
        idx: &mut usize,
        locals: &mut [Event],
        cells: &[crate::cell::Cell],
        outputs: &mut [Output],
        databases: &mut [Database],
    ) {
        use Transform as T;
        match self {
            T::Noop {} => {}
            T::Constant { value, destination } => locals[destination.id] = value.clone(),
            T::Logic { logic, destination } => locals[destination.id] = logic.apply(locals),
            T::Math { math, destination } => locals[destination.id] = math.apply(locals),
            T::Move {
                source,
                destination,
            } => locals[destination.id] = locals[source.id].clone(),
            T::Argument { index, destination } => {
                locals[destination.id] = args.get(*index).cloned().unwrap_or_default();
            }
            T::Load { cell, destination } => locals[destination.id] = cells[cell.id].get(),
            T::Store { source, cell } => {
                cells[cell.id].set(locals[source.id].clone());
            }
            T::Send { source, output } => {
                if let Some(idx) = locals[output.id].as_number().map(|v| v.whole() as usize) {
                    if let Some(output) = outputs.get_mut(idx) {
                        let event = locals[source.id].clone();
                        output.send(event);
                    }
                }
            }
            T::SendAfter {
                source,
                output,
                delay,
            } => {
                if let Some(idx) = locals[output.id].as_number().map(|v| v.whole() as usize) {
                    if let Some(delay) = locals[delay.id].as_duration() {
                        if let Some(output) = outputs.get_mut(idx) {
                            let event = locals[source.id].clone();
                            output.send_delay(event, delay);
                        }
                    }
                }
            }
            T::Filter {
                source,
                filter,
                destination,
            } => {
                let event = &locals[source.id];
                let event = if filter.apply(event) {
                    event.clone()
                } else {
                    Event::undefined()
                };
                locals[destination.id] = event;
            }
            T::Select {
                source,
                select,
                destination,
            } => {
                let event = &locals[source.id];
                let event = select.apply(event, locals).unwrap_or_default();
                locals[destination.id] = event;
            }
            T::Midi { midi, destination } => {
                let event = midi.apply(locals).unwrap_or_default();
                locals[destination.id] = event;
            }
            T::Theory {
                source,
                theory,
                destination,
            } => {
                let event = &locals[source.id];
                let event = theory.apply(event, locals).unwrap_or_default();
                locals[destination.id] = event;
            }
            T::Inspect { label, source } => {
                let event = &locals[source.id];
                eprintln!("{}{:?}", label, event);
            }
            T::Condition {
                skip_count,
                condition,
            } => {
                if !&locals[*condition].is_truthy() {
                    *idx += *skip_count;
                }
            }
            T::Table { id, table } => {
                let db = &mut databases[*id];
                table.apply(db, locals, outputs);
            }
        }

        // increment the instruction pointer
        *idx += 1;
    }
}
