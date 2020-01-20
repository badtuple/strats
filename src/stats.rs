#[derive(Copy, Clone, Debug)]
pub enum Metric {
    Count,
    Mean,
    Sum,
    Min,
    Max,
}

impl Metric {
    pub fn from_args(count: bool, mean: bool, sum: bool, min: bool, max: bool) -> Vec<Metric> {
        let mut metrics = vec![];
        if count {
            metrics.push(Metric::Count)
        }
        if mean {
            metrics.push(Metric::Mean)
        }
        if sum {
            metrics.push(Metric::Sum)
        }
        if min {
            metrics.push(Metric::Min)
        }
        if max {
            metrics.push(Metric::Max)
        }

        metrics
    }
}

#[derive(Debug)]
pub struct Stats {
    count: f64,
    sum: f64,
    min: Option<f64>,
    max: Option<f64>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            count: 0.0,
            sum: 0.0,
            min: None,
            max: None,
        }
    }

    pub fn register(&mut self, x: f64) {
        self.count += 1.0;
        self.sum += x;

        match self.min {
            Some(min) if x < min => self.min = Some(x),
            None => self.min = Some(x),
            _ => (),
        };

        match self.max {
            Some(max) if x > max => self.max = Some(x),
            None => self.max = Some(x),
            _ => (),
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
        self.min.unwrap()
    }

    pub fn get_max(&self) -> f64 {
        self.max.unwrap()
    }

    pub fn get_results(&self, metrics: &Vec<Metric>) -> Vec<(Metric, f64)> {
        metrics
            .iter()
            .map(|t| {
                use Metric::*;
                let result = match t {
                    Count => self.get_count(),
                    Mean => self.get_mean(),
                    Sum => self.get_sum(),
                    Min => self.get_min(),
                    Max => self.get_max(),
                };
                (*t, result)
            })
            .collect()
    }
}
