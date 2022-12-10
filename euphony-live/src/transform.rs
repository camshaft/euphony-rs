use crate::{
    connection::{Input, Output},
    event::{self, Event, MidiMessage},
    Result,
};
use core::ops::{Deref, DerefMut};
use std::collections::HashMap;

pub mod builder;
mod cell;
mod database;
mod filter;
mod instruction;
mod local;
mod logic;
mod math;
mod midi;
mod select;
mod table;
mod theory;

pub use cell::*;
pub use database::*;
pub use filter::*;
pub use instruction::*;
pub use local::*;
pub use logic::*;
pub use math::*;
pub use midi::*;
pub use select::*;
pub use table::*;
pub use theory::*;

#[derive(Clone, Debug)]
pub struct Config {
    pub(super) locals: usize,
    pub(super) instructions: Vec<Transform>,
    pub(super) once_instructions: Vec<Transform>,
    pub(super) cells: Vec<Event>,
    pub(super) databases: usize,
    pub(super) inputs: Vec<(String, builder::Input)>,
    pub(super) outputs: Vec<(String, builder::Output)>,
}

impl Config {
    pub fn init_offline(&self) -> State {
        self.do_init("", &Default::default(), &Default::default(), true)
            .unwrap()
    }

    pub fn init(
        &self,
        client: &str,
        input_overrides: &HashMap<String, String>,
        output_overrides: &HashMap<String, String>,
    ) -> Result<State> {
        self.do_init(client, input_overrides, output_overrides, false)
    }

    fn do_init(
        &self,
        client: &str,
        input_overrides: &HashMap<String, String>,
        output_overrides: &HashMap<String, String>,
        is_offline: bool,
    ) -> Result<State> {
        let locals = vec![Event::default(); self.locals];
        let instructions = self.instructions.clone();
        let databases = vec![Database::default(); self.databases];
        let cells = self
            .cells
            .iter()
            .cloned()
            .map(crate::cell::Cell::new)
            .collect();

        let (sender, receiver) = crate::connection::channel();

        let input = Input::new(receiver);

        let mut inputs = vec![];
        let mut outputs = vec![];

        for (name, conf) in self.inputs.iter() {
            // we don't need to do anything for input buses
            if conf.is_bus || is_offline {
                continue;
            }

            let output = Output::new(conf.id, sender.clone());

            inputs.push(if let Some(physical_name) = input_overrides.get(name) {
                Input::spawn(client, physical_name, false, output)?
            } else {
                Input::spawn(client, &format!("{} ({})", client, name), true, output)?
            });
        }

        for (name, conf) in self.outputs.iter() {
            if let Some(id) = conf.bus.or(if is_offline { Some(conf.id) } else { None }) {
                outputs.push(Output::new(id, sender.clone()));
                continue;
            }

            outputs.push(if let Some(physical_name) = output_overrides.get(name) {
                Output::open(client, physical_name, false)?
            } else {
                Output::open(client, &format!("{} ({})", client, name), true)?
            });
        }

        let mut state = State {
            locals,
            instructions,
            cells,
            databases,
            input,
            inputs,
            outputs,
        };

        let mut idx = 0;
        while idx < self.once_instructions.len() {
            let t = &self.once_instructions[idx];
            t.apply(
                &[],
                &mut idx,
                &mut state.locals,
                &state.cells,
                &mut state.outputs,
                &mut state.databases,
            );
        }

        Ok(state)
    }
}

#[derive(Debug)]
pub struct State {
    locals: Vec<Event>,
    instructions: Vec<Transform>,
    cells: Vec<crate::cell::Cell>,
    databases: Vec<Database>,
    input: Input,
    #[allow(dead_code)] // needed to ensure the inputs state alive
    inputs: Vec<crate::connection::Handle>,
    outputs: Vec<Output>,
}

impl State {
    pub fn apply(&mut self, args: &[Event]) {
        // TODO reset locals but not constants
        // for local in &mut self {
        //     *local = Event::undefined();
        // }

        let mut idx = 0;
        while idx < self.instructions.len() {
            let t = &self.instructions[idx];
            t.apply(
                args,
                &mut idx,
                &mut self.locals,
                &self.cells,
                &mut self.outputs,
                &mut self.databases,
            );
        }
    }

    pub async fn apply_inputs(&mut self) {
        let (id, event) = self.input.recv_idx().await;
        let id = (id as i64).into();
        self.apply(&[event, id]);
    }
}

impl Deref for State {
    type Target = [Event];

    fn deref(&self) -> &Self::Target {
        &self.locals
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.locals
    }
}
