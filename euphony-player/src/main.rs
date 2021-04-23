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
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Terminal,
};

mod event;
mod progress;
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

    let input_path = args.input;

    let name = input_path.display().to_string();

    let (project, buffer, tracks) = project::Project::new(input_path)?;

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
        let playing = timeline.playing();
        let looping = timeline.looping();
        let clipped = timeline.clipped();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Length(5),
                        Constraint::Percentage(100),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let status = Block::default().title(vec![
                if playing {
                    Span::styled("Playing [Space]", Style::default().fg(Color::Green))
                } else {
                    Span::styled("Paused  [Space]", Style::default().fg(Color::Yellow))
                },
                Span::from(" | "),
                Span::styled(
                    "Loop [L]",
                    if looping {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
                Span::from(" | "),
                Span::styled(
                    "Clip [C]",
                    if clipped {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
            ]);

            f.render_widget(status, chunks[0]);

            let progress = progress::Progress::default()
                .block(Block::default().borders(Borders::ALL))
                .style(
                    Style::default()
                        .fg(if playing { Color::Green } else { Color::Yellow })
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .clipped(clipped)
                .cursor(timeline.cursor())
                .duration(timeline.duration())
                .clip_start(timeline.clip_start())
                .clip_end(timeline.clip_end());
            f.render_widget(progress, chunks[1]);

            let tracks_entries = tracks.tracks.load();
            let rows = tracks_entries.tracks().iter().map(|item| {
                let muted = Cell::from("M").style(if item.is_muted() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                });

                let solo = Cell::from("S").style(if item.is_solo() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                });

                let cells = vec![
                    Cell::from("["),
                    muted,
                    solo,
                    Cell::from("] "),
                    Cell::from(item.name()),
                ];
                Row::new(cells).height(1)
            });

            let t = Table::new(rows)
                .block(Block::default().borders(Borders::ALL).title(name.as_ref()))
                .column_spacing(0)
                .highlight_symbol("> ")
                .widths(&[
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Percentage(100),
                ]);
            f.render_stateful_widget(t, chunks[2], &mut tracks.state);
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
                    let duration = timeline.duration();
                    let cursor = timeline.cursor() + Duration::from_secs(1);
                    let cursor = cursor.min(duration);
                    updates.set(cursor);
                }
                Key::Home => {
                    updates.set(Duration::from_secs(0));
                }
                Key::End => {
                    updates.set(timeline.duration());
                }
                Key::Char('m') => {
                    tracks.mute();
                }
                Key::Char('M') => {
                    tracks.tracks.load().mute_all();
                }
                Key::Char('s') => {
                    tracks.solo();
                }
                Key::Char('S') => {
                    tracks.tracks.load().solo_all();
                }
                Key::Char('l') | Key::Char('L') => {
                    updates.looping(!timeline.looping());
                }
                Key::Char('q') | Key::Esc => {
                    break;
                }
                Key::Char('[') => {
                    updates.clip_start(Some(timeline.cursor()));
                }
                Key::Char('{') => {
                    updates.clip_start(None);
                }
                Key::Char(']') => {
                    // rewind to the start in clipped mode
                    if clipped && playing {
                        updates.set(timeline.clip_start().unwrap_or_default());
                    }
                    updates.clip_end(Some(timeline.cursor()));
                }
                Key::Char('}') => {
                    updates.clip_end(None);
                }
                Key::Char('c') | Key::Char('C') => {
                    updates.clipped(!timeline.clipped());
                }
                Key::Down => {
                    tracks.next();
                }
                Key::Up => {
                    tracks.previous();
                }
                _key => {
                    // dbg!(_key);
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
        let mut state = TableState::default();
        state.select(Some(0));
        Self { state, tracks }
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
