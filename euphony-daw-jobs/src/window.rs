/// A worker's concurrency window, discovered from its answers.
///
/// It starts at one attempt and doubles each round of clean answers. When the
/// worker refuses an attempt for capacity, or loses or corrupts one, it
/// halves, and from then on grows by one attempt a round.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    limit: usize,
    threshold: Option<usize>,
    credit: usize,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            limit: 1,
            threshold: None,
            credit: 0,
        }
    }
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }

    /// The most attempts the worker holds at once now.
    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn on_success(&mut self) {
        match self.threshold {
            Some(threshold) if self.limit >= threshold => {
                self.credit += 1;
                if self.credit >= self.limit {
                    self.credit = 0;
                    self.limit += 1;
                }
            }
            _ => self.limit += 1,
        }
    }

    pub fn on_congestion(&mut self) {
        self.limit = (self.limit / 2).max(1);
        self.threshold = Some(self.limit);
        self.credit = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_each_clean_round_then_halves_and_grows_by_one() {
        let mut window = Window::new();
        assert_eq!(window.limit(), 1);
        for expected in [2, 4, 8] {
            for _ in 0..window.limit() {
                window.on_success();
            }
            assert_eq!(window.limit(), expected);
        }
        window.on_congestion();
        assert_eq!(window.limit(), 4);
        for _ in 0..4 {
            window.on_success();
        }
        assert_eq!(window.limit(), 5);
        for _ in 0..10 {
            window.on_congestion();
        }
        assert_eq!(window.limit(), 1);
    }
}
