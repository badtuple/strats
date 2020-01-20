use crate::Stats;

#[derive(Copy, Clone, Debug)]
enum StatType {
    Count,
    Mean,
    Sum,
}

pub struct Printer {
    human: bool,
    stats_requested: Vec<StatType>,
}

impl Printer {
    pub fn new(human: bool, count: bool, mean: bool, sum: bool) -> Self {
        let mut stats_requested = vec![];
        if count {
            stats_requested.push(StatType::Count)
        }

        if mean {
            stats_requested.push(StatType::Mean)
        }

        if sum {
            stats_requested.push(StatType::Sum)
        }

        Printer {
            human,
            stats_requested,
        }
    }

    fn print_computer(&self, results: Vec<(StatType, f64)>) {
        let line = results
            .iter()
            .map(|result| result.1.to_string())
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", line);
    }

    fn print_human(&self, results: Vec<(StatType, f64)>) {
        let line = results
            .iter()
            .map(|result| format!("{:?}: {}", result.0, result.1))
            .collect::<Vec<String>>()
            .join(", ");
        print!("{}\r", line);
    }

    pub fn print(&self, stats: &Stats) {
        let results = self
            .stats_requested
            .iter()
            .map(|t| {
                use StatType::*;
                let result = match t {
                    Count => stats.get_count(),
                    Mean => stats.get_mean(),
                    Sum => stats.get_sum(),
                };
                (*t, result)
            })
            .collect();

        if self.human {
            self.print_human(results);
        } else {
            self.print_computer(results);
        }
    }
}
