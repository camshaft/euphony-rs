use crate::play::stream::Stream;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Cell, Row, Table, TableState},
    Frame,
};

pub struct TracksTable {
    state: TableState,
}

impl TracksTable {
    pub fn new() -> Self {
        let mut state = TableState::default();
        state.select(Some(0));
        Self { state }
    }

    pub fn mute(&self, stream: &Stream) {
        if let Some(idx) = self.state.selected() {
            stream.mute_toggle(idx);
        }
    }

    pub fn solo(&self, stream: &Stream) {
        if let Some(idx) = self.state.selected() {
            stream.solo_toggle(idx);
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect, stream: &Stream) {
        let mut rows = vec![];
        let tracks = stream.tracks();
        for track in tracks.iter() {
            let muted = Cell::from("M").style(if track.is_muted() {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            });

            let solo = Cell::from("S").style(if track.is_soloed() {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            });

            let cells = vec![
                Cell::from("["),
                muted,
                solo,
                Cell::from("] "),
                Cell::from(track.name()),
            ];
            let row = Row::new(cells).height(1);
            rows.push(row);
        }

        let t = Table::new(rows)
            .column_spacing(0)
            .highlight_symbol("> ")
            .widths(&[
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Percentage(100),
            ]);
        f.render_stateful_widget(t, rect, &mut self.state);
    }

    pub fn next(&mut self, stream: &Stream) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= stream.tracks().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, stream: &Stream) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    stream.tracks().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
