#[derive(Debug)]
pub struct Stats {
    count: f64,
    sum: f64,
    min: f64,
    max: f64,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            count: 0.0,
            sum: 0.0,
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn register(&mut self, x: f64) {
        self.count += 1.0;
        self.sum += x;

        if x < self.min {
            self.min = x
        }

        if x > self.max {
            self.max = x
        }
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

    pub fn get_min(&self) -> f64 {
        self.min
    }

    pub fn get_max(&self) -> f64 {
        self.max
    }
}
