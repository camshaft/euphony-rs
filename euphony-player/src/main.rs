use anyhow::Result;
use core::time::Duration;
use rodio::OutputStream;
use std::{io, path::PathBuf};
use structopt::StructOpt;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Gauge, Row, Table, TableState},
    Terminal,
};

mod event;
mod project;
mod timeline;

#[derive(Debug, StructOpt)]
struct Args {
    /// Enable looping
    #[structopt(long = "loop", short)]
    looping: bool,
    /// Start the player stopped
    #[structopt(long, short)]
    stop: bool,
    /// Path to the input file
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let (project, buffer, tracks) = project::Project::new(args.input)?;

    let mut timeline = timeline::Timeline::new(buffer);

    if args.looping || args.stop {
        let mut update = timeline::Update::new();
        update.looping(args.looping).playing(!args.stop);
        timeline.update(update);
    }

    stream_handle.play_raw(timeline.playhead().clone())?;

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = event::Events::new();
    let mut tracks = TracksTable::new(tracks);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
                .split(f.size());

            let gauge = Gauge::default()
                .block(Block::default().title("Progress").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Green))
                .ratio(timeline.progress());
            f.render_widget(gauge, chunks[0]);

            let tracks_entries = tracks.tracks.load();
            let rows = tracks_entries.iter().map(|item| {
                let status = match (item.is_muted(), item.is_solo()) {
                    (true, true) => "MS",
                    (true, false) => "M",
                    (false, true) => "S",
                    (false, false) => "",
                };
                let cells = vec![Cell::from(item.name()), Cell::from(status)];
                Row::new(cells).height(1).bottom_margin(1)
            });
            let t = Table::new(rows)
                .block(Block::default().borders(Borders::ALL).title("Tracks"))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ")
                .widths(&[Constraint::Percentage(90), Constraint::Length(10)]);
            f.render_stateful_widget(t, chunks[1], &mut tracks.state);
        })?;

        if let event::Event::Input(input) = events.next()? {
            let mut updates = timeline::Update::new();
            match input {
                Key::Char(' ') => {
                    updates.playing(!timeline.playing());
                }
                Key::Left => {
                    updates.set(
                        timeline
                            .cursor()
                            .checked_sub(Duration::from_secs(1))
                            .unwrap_or_default(),
                    );
                }
                Key::Right => {
                    updates.set(timeline.cursor() + Duration::from_secs(1));
                }
                Key::Home => {
                    updates.set(Duration::from_secs(0));
                }
                Key::Char('m') => {
                    tracks.mute();
                }
                Key::Char('s') => {
                    tracks.solo();
                }
                Key::Char('l') => {
                    updates.looping(!timeline.looping());
                }
                Key::Char('q') | Key::Esc => {
                    break;
                }
                Key::Down => {
                    tracks.next();
                }
                Key::Up => {
                    tracks.previous();
                }
                _key => {
                    //dbg!(key);
                }
            }

            timeline.update(updates);
        }
    }

    drop(project);

    Ok(())
}

pub struct TracksTable {
    state: TableState,
    tracks: project::TracksHandle,
}

impl TracksTable {
    fn new(tracks: project::TracksHandle) -> Self {
        Self {
            state: TableState::default(),
            tracks,
        }
    }

    fn mute(&self) {
        if let Some(idx) = self.state.selected() {
            self.tracks.load().mute(idx);
        }
    }

    fn solo(&self) {
        if let Some(idx) = self.state.selected() {
            self.tracks.load().solo(idx);
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.tracks.load().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tracks.load().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
