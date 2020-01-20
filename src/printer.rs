use crate::stats::Metric;

pub struct Printer {
    pretty: bool,
}

impl Printer {
    pub fn new(pretty: bool) -> Self {
        Printer { pretty }
    }

    fn print_computer(&self, results: Vec<(Metric, f64)>) {
        let line = results
            .iter()
            .map(|result| result.1.to_string())
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", line);
    }

    fn print_pretty(&self, results: Vec<(Metric, f64)>) {
        let line = results
            .iter()
            .map(|result| format!("{:?}: {}", result.0, result.1))
            .collect::<Vec<String>>()
            .join(", ");
        print!("{}\r", line);
    }

    pub fn print(&self, results: Vec<(Metric, f64)>) {
        if self.pretty {
            self.print_pretty(results);
        } else {
            self.print_computer(results);
        }
    }
}
