use crate::metric::*;

use serde_json::{map::Map, Value};
use std::env;

#[derive(Debug)]
/// strats calculates stats on streams
pub struct Strats {
    pub global: GlobalOptions,
    pub metrics: Vec<Metric>,
}

#[derive(Debug)]
/// Options that effect the program globally
pub struct GlobalOptions {
    /// Incrementally update the output each time an input is received.
    /// [default: false]
    pub incremental: bool,

    /// Print results in a human friendly way. [default: false]
    pub pretty: bool,
}

impl Strats {
    pub fn from_args() -> Self {
        let mut options = Strats {
            global: GlobalOptions {
                incremental: false,
                pretty: false,
            },
            metrics: vec![],
        };

        let mut global_options = true;

        for (i, arg) in env::args().enumerate() {
            if i == 0 {
                // skip command name
                continue;
            }

            use ArgumentType::*;
            match argument_type(&*arg) {
                Subcommand => {
                    options.metrics.push(Metric::new_from_string(&*arg));
                    global_options = false;
                }
                Flag => {
                    if global_options {
                        match &*arg {
                            "--incremental" => options.global.incremental = true,
                            "--pretty" => options.global.pretty = true,
                            _ => panic!("{:?} is not a global option", arg),
                        };
                    } else {
                        panic!("metric does not take the {:?} flag", arg)
                    }
                }
                Invalid => panic!("unrecognized option {:?}", arg),
            };
        }

        options
    }

    pub fn get_results(&self) -> Value {
        let mut out: Map<String, Value> = Map::new();
        for m in &self.metrics {
            out.insert(m.name(), m.results());
        }

        return out.into();
    }
}

enum ArgumentType {
    Subcommand,
    Flag,
    Invalid,
}

fn argument_type(arg: &str) -> ArgumentType {
    // TODO: we need to validate if values are alphanumeric and stuff like that.

    if arg.len() < 3 {
        return ArgumentType::Invalid;
    }

    if &arg[0..2] == "--" {
        return ArgumentType::Flag;
    }

    return ArgumentType::Subcommand;
}
