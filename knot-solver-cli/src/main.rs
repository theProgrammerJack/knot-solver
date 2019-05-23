#[macro_use]
extern crate serde_derive;

use knot_solver::{Knot, KnotParseError};
use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
    str::FromStr,
    string::ToString,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "bracket")]
    Bracket { braid: String },

    #[structopt(name = "beta")]
    Beta { braid: String },

    #[structopt(name = "jones")]
    Jones { braid: String },

    #[structopt(name = "csv")]
    Csv {
        #[structopt(short = "o", long = "output")]
        output: Option<PathBuf>,
        braids: Vec<String>,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt.command {
        Command::Bracket { braid } => println!(
            "{}",
            Knot::from_str(braid.as_str()).unwrap().bracket_polynomial()
        ),
        Command::Beta { braid } => println!(
            "{}",
            Knot::from_str(braid.as_str()).unwrap().beta_polynomial()
        ),
        Command::Jones { braid } => println!(
            "{}",
            Knot::from_str(braid.as_str()).unwrap().jones_polynomial()
        ),
        Command::Csv { output, braids } => run_csv(output, braids).unwrap(),
    }
}

#[derive(Debug, Serialize)]
struct KnotRecord {
    braid: String,
    bracket: String,
    beta: String,
    jones: String,
}

impl KnotRecord {
    fn with_braid(braid: String) -> Result<Self, KnotParseError> {
        let knot = Knot::from_str(braid.as_str())?;
        Ok(KnotRecord {
            braid,
            bracket: knot.bracket_polynomial().to_string(),
            beta: knot.beta_polynomial().to_string(),
            jones: knot.jones_polynomial().to_string(),
        })
    }
}

fn run_csv<P: AsRef<Path>>(output_path: Option<P>, braids: Vec<String>) -> Result<(), Box<Error>> {
    if let Some(path) = output_path {
        let wtr = csv::Writer::from_path(path)?;
        write_csv(wtr, braids)
    } else {
        let wtr = csv::Writer::from_writer(io::stdout());
        write_csv(wtr, braids)
    }
}

fn write_csv<T: io::Write>(mut wtr: csv::Writer<T>, braids: Vec<String>) -> Result<(), Box<Error>> {
    for record in braids
        .into_iter()
        .map(|braid| KnotRecord::with_braid(braid.to_string()).unwrap())
    {
        wtr.serialize(record)?;
    }

    wtr.flush()?;

    Ok(())
}
