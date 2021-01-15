use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Metric {
    Count(Count),
    Sum(Sum),
    Min(Min),
    Max(Max),
    CountValues(CountValues),
    Mean(Mean),
}

impl Metric {
    pub fn new_from_string(s: &str) -> Self {
        match s {
            "count" => Metric::Count(Count { count: 0 }),
            "count_values" => Metric::CountValues(CountValues {
                values: HashMap::new(),
            }),
            "mean" => Metric::Mean(Mean {
                sum: 0.0,
                count: 0.0,
            }),
            "sum" => Metric::Sum(Sum { sum: 0.0 }),
            "min" => Metric::Min(Min { min: None }),
            "max" => Metric::Max(Max { max: None }),
            _ => panic!("unrecognized metric {:?}", s),
        }
    }

    pub fn register(&mut self, line: String, x: Option<f64>) {
        use Metric::*;

        if let Some(x) = x {
            match self {
                Sum(m) => m.sum += x,
                Min(m) => {
                    match m.min {
                        Some(min) if x < min => m.min = Some(x),
                        None => m.min = Some(x),
                        _ => (),
                    };
                }
                Max(m) => match m.max {
                    Some(max) if x > max => m.max = Some(x),
                    None => m.max = Some(x),
                    _ => (),
                },
                Mean(m) => {
                    m.sum += x;
                    m.count += 1.0;
                }
                _ => (),
            }
        }

        match self {
            Count(m) => m.count += 1,
            CountValues(m) => *m.values.entry(line).or_insert(0) += 1,
            _ => (),
        }
    }

    pub fn name(&self) -> String {
        use Metric::*;
        match self {
            Count(_) => "count",
            Sum(_) => "sum",
            Min(_) => "min",
            Max(_) => "max",
            CountValues(_) => "count_values",
            Mean(_) => "mean",
        }
        .into()
    }

    // Results in string form so we can print it to the screen
    pub fn results(&self) -> Value {
        use Metric::*;
        match self {
            Count(m) => m.count.into(),
            Sum(m) => m.sum.into(),
            Min(m) => match m.min {
                Some(x) => x.into(),
                None => Value::Null,
            },
            Max(m) => match m.max {
                Some(x) => x.into(),
                None => Value::Null,
            },
            Mean(m) => (m.sum / m.count).into(),
            CountValues(m) => m.results().into(),
        }
    }

    // TODO: will probably have to register specific flags on individual metrics
}

#[derive(Debug)]
/// Report the number of entries received.
// State for the Count Command
pub struct Count {
    count: u64,
}

#[derive(Debug)]
/// Report the sum of the input.
// State for the Sum Command
pub struct Sum {
    sum: f64,
}

#[derive(Debug)]
/// Report the smallest entry.
pub struct Min {
    min: Option<f64>,
}

#[derive(Debug)]
/// Report the largest entry.
pub struct Max {
    max: Option<f64>,
}

#[derive(Debug)]
/// Report the arithmetic mean.
pub struct Mean {
    sum: f64,
    count: f64,
}

#[derive(Debug)]
/// Report the number of entries for each unique value.
pub struct CountValues {
    values: HashMap<String, u64>,
}

impl CountValues {
    fn results(&self) -> Value {
        use serde_json::map::Map;
        let mut map: Map<String, Value> = Map::new();
        for (k, v) in self.values.iter() {
            map.insert(k.clone(), (*v).into());
        }
        map.into()
    }
}
