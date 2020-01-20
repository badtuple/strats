use std::io::stdin;
use std::io::BufRead;
use structopt::StructOpt;

mod stats;
use stats::Metric;
use stats::Stats;

mod printer;
use printer::Printer;

/// strats calculates stats on streams
#[derive(StructOpt, Debug)]
struct Cli {
    /// Incrementally update the output each time an input is received.
    /// [default: false]
    #[structopt(long)]
    incremental: bool,

    /// Print results in a human friendly way. [default: false]
    #[structopt(long)]
    pretty: bool,

    /// Report the number of entries received. [default: false]
    #[structopt(long)]
    count: bool,

    /// Report the arithmetic mean. [default: false]
    #[structopt(long)]
    mean: bool,

    /// Report the sum of the input. [default: false]
    #[structopt(long)]
    sum: bool,

    /// Report the smallest entry. [default: false]
    #[structopt(long)]
    min: bool,

    /// Report the largest entry. [default: false]
    #[structopt(long)]
    max: bool,
}

fn main() {
    let args = Cli::from_args();

    let mut stats = Stats::new();
    let printer = Printer::new(args.pretty);
    let metrics = Metric::from_args(args.count, args.mean, args.sum, args.min, args.max);

    let mut buffer = String::new();
    let stdin = stdin();
    let mut lock = stdin.lock();
    while lock
        .read_line(&mut buffer)
        .expect("unable to read from stdin")
        > 0
    {
        let val = buffer.trim().parse::<f64>();
        if let Ok(v) = val {
            stats.register(v);
        }

        if args.incremental {
            let results = stats.get_results(&metrics);
            printer.print(results);
        }

        buffer.clear();
    }

    let results = stats.get_results(&metrics);
    printer.print(results);
}
