use super::stream::Stream;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, ops::ControlFlow, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
    Frame, Terminal,
};
use tui_logger::{TuiLoggerWidget, TuiWidgetState};

mod timeline;
mod tracks;

struct App {
    stream: Stream,
    tracks: tracks::TracksTable,
    logger: TuiWidgetState,
}

impl App {
    fn new(stream: Stream) -> Self {
        Self {
            stream,
            tracks: tracks::TracksTable::new(),
            logger: TuiWidgetState::default(),
        }
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if !event::poll(Duration::from_millis(50))? {
                continue;
            }

            if self.on_event(event::read()?).is_break() {
                return Ok(());
            }
        }
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let stream = &self.stream;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Length(5),
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(f.size());

        let is_playing = stream.is_playing();
        let is_looping = stream.is_looping();
        let is_clipped = stream.is_clipped();

        {
            let status = Block::default().title(vec![
                if is_playing {
                    Span::styled("Playing [Space]", Style::default().fg(Color::Green))
                } else {
                    Span::styled("Paused  [Space]", Style::default().fg(Color::Yellow))
                },
                Span::from(" | "),
                Span::styled(
                    "Loop [l]",
                    if is_looping {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
                Span::from(" | "),
                Span::styled(
                    "Clip [c]",
                    if is_clipped {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
                Span::from(" | Volume [-] "),
                Span::from(format!("{:>3}", (stream.volume() * 100.0).round())),
                Span::from("% [+]"),
            ]);
            f.render_widget(status, chunks[0]);
        }

        {
            let progress = timeline::Timeline::default()
                .block(Block::default().borders(Borders::ALL))
                .style(
                    Style::default()
                        .fg(if is_playing {
                            Color::Green
                        } else {
                            Color::Yellow
                        })
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .clipped(is_clipped)
                .cursor(stream.playhead())
                .duration(stream.duration())
                .clip_start(stream.clip_start())
                .clip_end(stream.clip_end());
            f.render_widget(progress, chunks[1]);
        }

        self.tracks.render(f, chunks[2], stream);

        {
            let logger = TuiLoggerWidget::default()
                .state(&self.logger)
                .output_file(false)
                .output_level(None)
                .output_line(false)
                .output_target(false)
                .output_timestamp(None);
            f.render_widget(logger, chunks[3]);
        }
    }

    fn on_event(&mut self, event: Event) -> ControlFlow<()> {
        match event {
            Event::Key(event) => self.on_key(event),
            Event::Mouse(event) => self.on_mouse(event),
            _ => ControlFlow::Continue(()),
        }
    }

    fn on_key(&mut self, event: KeyEvent) -> ControlFlow<()> {
        match event.code {
            KeyCode::Char(' ') => {
                let _ = self.stream.play_toggle();
            }
            KeyCode::Char('q') | KeyCode::Esc => return ControlFlow::Break(()),
            KeyCode::Char('l') => self.stream.loop_toggle(),
            KeyCode::Char('c') => self.stream.clip_toggle(),
            KeyCode::Left => self.stream.seek_back(Duration::from_secs(1)),
            KeyCode::Right => self.stream.seek_forward(Duration::from_secs(1)),
            KeyCode::Up => self.tracks.previous(&self.stream),
            KeyCode::Down => self.tracks.next(&self.stream),
            KeyCode::Home => self.stream.seek_start(),
            KeyCode::End => self.stream.seek_end(),
            KeyCode::Char('[') => self.stream.clip_start_set(),
            KeyCode::Char('{') => self.stream.clip_start_clear(),
            KeyCode::Char(']') => self.stream.clip_end_set(),
            KeyCode::Char('}') => self.stream.clip_end_clear(),
            KeyCode::Char('-') => self.stream.volume_add(-0.05),
            KeyCode::Char('+') => self.stream.volume_add(0.05),
            KeyCode::Char('m') => self.tracks.mute(&self.stream),
            KeyCode::Char('s') => self.tracks.solo(&self.stream),
            _ => {}
        }

        ControlFlow::Continue(())
    }

    fn on_mouse(&mut self, _event: MouseEvent) -> ControlFlow<()> {
        // TODO
        ControlFlow::Continue(())
    }
}

pub fn start(stream: Stream) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(stream);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        log::error!("could not close tui: {}", err);
    }

    Ok(())
}
