use std::io::stdin;
use std::io::BufRead;

mod metric;

mod strats;
use strats::Strats;

fn main() {
    let mut strats = Strats::from_args();

    let mut buffer = String::new();
    let stdin = stdin();
    let mut lock = stdin.lock();
    while lock
        .read_line(&mut buffer)
        .expect("unable to read from stdin")
        > 0
    {
        let val = buffer.trim().parse::<f64>().ok();

        for metric in strats.metrics.iter_mut() {
            metric.register(buffer.trim().into(), val);
        }

        buffer.clear();
    }

    let results = strats.get_results();
    println!("{}", results);
}
