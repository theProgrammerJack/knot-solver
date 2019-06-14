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

/// Computes polynomial representations of knots specified in braid notation.
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Computes the bracket polynomial of a given knot.
    #[structopt(name = "bracket")]
    Bracket {
        /// The braid representation of the knot.
        braid: String
    },

    /// Computes the beta polynomial of a given knot.
    #[structopt(name = "beta")]
    Beta {
        /// The braid representation of the knot.
        braid: String
    },

    /// Computes the jones polynomial of a given knot.
    #[structopt(name = "jones")]
    Jones {
        /// The braid representation of the knot.
        braid: String
    },

    /// Generates a csv file with all of the polynomials for all of the given knots.
    #[structopt(name = "csv")]
    Csv {
        /// Optional output file. The file will be printed to stdout if not specified.
        #[structopt(short = "o", long = "output")]
        output: Option<PathBuf>,

        /// The list of braids to compute for.
        braids: Vec<String>,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt.command {
        Command::Bracket { braid } => println!(
            "{}",
            Knot::from_str(braid.as_str()).expect("Invalid braid").bracket_polynomial()
        ),
        Command::Beta { braid } => println!(
            "{}",
            Knot::from_str(braid.as_str()).expect("Invalid braid").beta_polynomial()
        ),
        Command::Jones { braid } => println!(
            "{}",
            Knot::from_str(braid.as_str()).expect("Invalid braid").jones_polynomial()
        ),
        Command::Csv { output, braids } => run_csv(output, braids).expect("Invalid braid"),
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
