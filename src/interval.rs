#[derive(Copy, Clone, Debug)]
pub struct Interval {
    start: f64,
    end: f64,
}

impl Interval {
    pub fn new(start: f64, end: f64) -> Interval {
        Interval {
            start: f64::min(start, end),
            end: f64::max(start, end),
        }
    }

    pub fn new_from_intervals(a: Interval, b: Interval) -> Self {
        let min = f64::min(a.start(), b.start());
        let max = f64::max(a.end(), b.end());
        Interval {
            start: min,
            end: max,
        }
    }

    pub fn start(self) -> f64 {
        self.start
    }

    pub fn end(self) -> f64 {
        self.end
    }

    pub fn clamp(self, x: f64) -> f64 {
        if x < self.start {
            return self.start;
        }
        if x > self.end {
            return self.end;
        }
        return x;
    }

    pub fn expand(self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        return Interval {
            start: self.start - padding,
            end: self.end + padding,
        };
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            start: 0.0,
            end: 0.0,
        }
    }
}
