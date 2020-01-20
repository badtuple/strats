use std::io::stdin;
use structopt::StructOpt;

/// strats calculates stats on streams
#[derive(StructOpt, Debug)]
struct Cli {
    /// Print results on a new line each time a new input is received.
    #[structopt(long)]
    immediate: bool,

    /// Update results on the commandline in a human friendly way.
    #[structopt(long)]
    interactive: bool,

    /// Count the number of entries received.
    #[structopt(long)]
    count: bool,

    /// Calculate the arithmetic mean.
    #[structopt(long)]
    mean: bool,

    /// Calculate the sum of the input.
    #[structopt(long)]
    sum: bool,
}

fn main() {
    let args = Cli::from_args();

    let mut stats = Stats::new();

    let mut buffer = String::new();
    while stdin()
        .read_line(&mut buffer)
        .expect("unable to read from stdin")
        > 0
    {
        let val = buffer.trim().parse::<f64>();
        match val {
            Ok(v) => stats.register(v),
            Err(e) => eprintln!("{}", e),
        }

        buffer.clear();
    }

    println!("{:?} {:?}", stats, stats.mean());
}

#[derive(Debug)]
struct Stats {
    count: f64,
    sum: f64,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            count: 0.0,
            sum: 0.0,
        }
    }

    fn register(&mut self, x: f64) {
        self.count += 1.0;
        self.sum += x;
    }

    fn mean(&self) -> f64 {
        self.sum / self.count
    }
}
