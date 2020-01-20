#[derive(Debug)]
pub struct Stats {
    count: f64,
    sum: f64,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            count: 0.0,
            sum: 0.0,
        }
    }

    pub fn register(&mut self, x: f64) {
        self.count += 1.0;
        self.sum += x;
    }

    pub fn get_count(&self) -> f64 {
        self.count
    }

    pub fn get_sum(&self) -> f64 {
        self.sum
    }

    pub fn get_mean(&self) -> f64 {
        self.sum / self.count
    }
}
