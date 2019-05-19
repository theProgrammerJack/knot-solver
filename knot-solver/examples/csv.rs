#[macro_use]
extern crate serde_derive;

use knot_solver::{Knot, KnotParseError};
use std::{env, error::Error, path::Path, process, str::FromStr};

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

fn run<P: AsRef<Path>>(path: P) -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_path(path)?;

    let knots = vec![
        "a", "A", "ab", "AB", "Ba", "abc", "abca", "bbac", "abcd", "bacd", "BaCd", "aaBBccDD", "aaa", "AAA"
    ];

    for record in knots
        .into_iter()
        .map(|braid| KnotRecord::with_braid(braid.to_string()).unwrap())
    {
        wtr.serialize(record)?;
    }

    wtr.flush()?;

    Ok(())
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        if let Err(err) = run(path) {
            println!("{}", err);
            process::exit(1);
        }
    } else {
        process::exit(1);
    }
}
