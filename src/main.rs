use std::io::stdin;
use std::io::BufRead;
use structopt::StructOpt;

mod stats;
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

    /// Count the number of entries received. [default: false]
    #[structopt(long)]
    count: bool,

    /// Calculate the arithmetic mean. [default: false]
    #[structopt(long)]
    mean: bool,

    /// Calculate the sum of the input. [default: false]
    #[structopt(long)]
    sum: bool,
}

fn main() {
    let args = Cli::from_args();

    let mut stats = Stats::new();
    let printer = Printer::new(args.pretty, args.count, args.mean, args.sum);

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
            printer.print(&stats);
        }

        buffer.clear();
    }

    printer.print(&stats);
}
