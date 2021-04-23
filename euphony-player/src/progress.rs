use core::time::Duration;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols,
    text::Span,
    widgets::{Block, Widget},
};

#[derive(Debug, Clone)]
pub struct Progress<'a> {
    block: Option<Block<'a>>,
    style: Style,
    cursor: Duration,
    duration: Duration,
    clipped: bool,
    clip_start: Option<Duration>,
    clip_end: Option<Duration>,
}

impl<'a> Default for Progress<'a> {
    fn default() -> Progress<'a> {
        Progress {
            block: None,
            clipped: false,
            cursor: Duration::from_secs(0),
            duration: Duration::from_secs(1),
            clip_start: None,
            clip_end: None,
            style: Style::default(),
        }
    }
}

impl<'a> Progress<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn cursor(mut self, value: Duration) -> Self {
        self.cursor = value;
        self
    }

    pub fn duration(mut self, value: Duration) -> Self {
        self.duration = value;
        self
    }

    pub fn clipped(mut self, value: bool) -> Self {
        self.clipped = value;
        self
    }

    pub fn clip_start(mut self, value: Option<Duration>) -> Self {
        self.clip_start = value;
        self
    }

    pub fn clip_end(mut self, value: Option<Duration>) -> Self {
        self.clip_end = value;
        self
    }

    fn position(&self, value: Duration, width: u16) -> u16 {
        let value = value.as_secs_f64() / self.duration.as_secs_f64();
        let value = f64::from(width) * value;
        value.round() as u16
    }
}

impl<'a> Widget for Progress<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let gauge_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        buf.set_style(gauge_area, self.style);
        if gauge_area.height < 1 {
            return;
        }

        let center = gauge_area.height / 2 + gauge_area.top();
        let width = gauge_area.width;

        let clip_start = self.clip_start.map(|start| self.position(start, width));
        let clip_end = self.clip_end.map(|start| self.position(start, width));

        let cursor = self.position(self.cursor, width);

        let label = Span::from(format!(
            "{} / {}",
            Timecode(self.cursor),
            Timecode(self.duration)
        ));

        for y in gauge_area.top()..gauge_area.bottom() {
            // Cursor
            buf.get_mut(cursor, y)
                .set_symbol(symbols::line::THICK_VERTICAL);

            if let Some(x) = clip_start {
                if y > center {
                    let mut symbol = symbols::line::DOUBLE_VERTICAL;
                    if y == gauge_area.bottom() - 1 {
                        symbol = symbols::line::DOUBLE_BOTTOM_LEFT;
                    }
                    buf.get_mut(x, y).set_symbol(symbol);
                }
            }

            if let Some(x) = clip_end {
                if y > center {
                    let mut symbol = symbols::line::DOUBLE_VERTICAL;
                    if y == gauge_area.bottom() - 1 {
                        symbol = symbols::line::DOUBLE_BOTTOM_RIGHT;
                    }
                    buf.get_mut(x, y).set_symbol(symbol);
                }
            }

            if y == center {
                let label_width = label.width() as u16;
                let middle = (gauge_area.width - label_width) / 2 + gauge_area.left();
                buf.set_span(middle, y, &label, gauge_area.right() - middle);
            }
        }
    }
}

struct Timecode(Duration);

use core::fmt;
impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs();
        let millis = self.0.subsec_millis();
        write!(f, "{:02}:{:02}.{:02}", secs / 60, secs % 60, millis / 10,)
    }
}
