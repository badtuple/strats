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
    /// Print results on a new line each time a new input is received.
    #[structopt(long)]
    verbose: bool,

    /// Update results on the commandline in a human friendly way.
    #[structopt(long)]
    human: bool,

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
    let printer = Printer::new(args.human, args.verbose, args.count, args.mean, args.sum);

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

        printer.maybe_print(&stats);
        buffer.clear();
    }

    printer.print(&stats);
}
