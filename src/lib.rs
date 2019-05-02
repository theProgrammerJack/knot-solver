use crate::polynomial::{Binomial, Polynomial, Term};
use bitvec::{BitVec, LittleEndian};
use std::{collections::HashSet, str::FromStr};

mod polynomial;

/// Represents a knot.
pub struct Knot {
    crossings: Vec<Crossing>,
    region_num: usize,
}

impl Knot {
    /// Returns the number of regions in the unresolved knot.
    pub fn num_regions(&self) -> usize {
        self.region_num
    }

    /// Returns the number of crossings in the knot.
    pub fn num_crossings(&self) -> usize {
        self.crossings.len()
    }

    /// Iterates over all possible resolutions of the knot, returning a `Vec<(usize, i16)>` containing the
    /// number of unknots in each and the difference between the number of 0 and infinity resolutions taken.
    pub fn resolutions(&self) -> Vec<(usize, i16)> {
        let r = 0u128..(2u128.pow(self.num_crossings() as u32));
        r.map(|n| {
            let mut diff: i16 = 0;
            let bits: BitVec<LittleEndian, _> = BitVec::from(&n.to_le_bytes()[..]);
            let mut counter = RegionCounter::new(self.num_regions());
            self.crossings
                .iter()
                .zip(bits.iter())
                .for_each(|(crossing, bit)| {
                    if bit {
                        diff += 1; // TODO: might need to switch
                        match crossing.orientation {
                            Orientation::Positive => counter.combine(crossing.left, crossing.right),
                            Orientation::Negative => counter.combine(crossing.top, crossing.bottom),
                        }
                    } else {
                        diff -= 1; // TODO: might need to switch
                        match crossing.orientation {
                            Orientation::Positive => counter.combine(crossing.top, crossing.bottom),
                            Orientation::Negative => counter.combine(crossing.left, crossing.right),
                        }
                    }
                });
            (counter.current_count() - 1, diff)
        })
        .collect()
    }

    pub fn bracket_polynomial(&self) -> Polynomial {
        self.resolutions()
            .iter()
            .map(|(c, d)| {
                Binomial(Term::new(-1, 2), Term::new(-1, -2)).expand(*c as isize - 1)
                    * Term::new(1, *d as isize)
            })
            .sum()
    }

    fn from_crossing_builders(mut crossing_builders: Vec<CrossingBuilder>) -> Self {
        let max_column = crossing_builders
            .iter()
            .max_by_key(|cb| cb.column)
            .unwrap()
            .column;

        let missing: Vec<u8> = (0..max_column)
            .filter(|x| !crossing_builders.iter().any(|cb| cb.column == *x))
            .collect();

        let mut index = 2;
        for i in 0..crossing_builders.len() {
            if crossing_builders[i].column == 0 {
                crossing_builders[i].left = Some(0);
            }
            // can't have an else here in case there is only one column.
            if crossing_builders[i].column == max_column {
                crossing_builders[i].right = Some(1);
            }

            if crossing_builders[i].bottom.is_none() {
                crossing_builders[i].bottom = Some(index);
                for j in 0..crossing_builders.len() {
                    let j = j + i + 1;
                    let j = j % crossing_builders.len();

                    if crossing_builders[j].column == crossing_builders[i].column {
                        crossing_builders[j].top = Some(index);
                        index += 1;
                        break;
                    } else if crossing_builders[i].column < crossing_builders[j].column
                        && crossing_builders[j].column - crossing_builders[i].column == 1
                    {
                        crossing_builders[j].left = Some(index);
                    } else if crossing_builders[j].column < crossing_builders[i].column
                        && crossing_builders[i].column - crossing_builders[j].column == 1
                    {
                        crossing_builders[j].right = Some(index);
                    }
                }
            }
        }

        for column in missing {
            crossing_builders
                .iter_mut()
                .filter(|cb| cb.column == column + 1)
                .for_each(|cb| cb.left = Some(index));
            if column != 0 {
                crossing_builders
                    .iter_mut()
                    .filter(|cb| cb.column == column - 1)
                    .for_each(|cb| cb.right = Some(index));
            }
            index += 1;
        }

        Knot {
            crossings: crossing_builders
                .into_iter()
                .map(|c| c.build().unwrap())
                .collect(),
            region_num: index,
        }
    }
}

impl FromStr for Knot {
    type Err = KnotParseError;

    /// Attempts to create a `Knot` from a input `str`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bad_chars: Vec<char> = s.chars().filter(|c| !c.is_ascii_alphabetic()).collect();
        if !bad_chars.is_empty() {
            Err(KnotParseError::InvalidCharacter(bad_chars))
        } else {
            let crossing_builders: Vec<CrossingBuilder> = s
                .chars()
                .map(|c| {
                    let orientation = if c.is_ascii_lowercase() {
                        Orientation::Positive
                    } else {
                        Orientation::Negative
                    };
                    let c = c.to_ascii_lowercase() as u8;
                    let column = c - 97; // 97 -> 'a' on the ascii table

                    CrossingBuilder::new(column, orientation)
                })
                .collect();

            Ok(Knot::from_crossing_builders(crossing_builders))
        }
    }
}

#[derive(Debug)]
pub enum KnotParseError {
    InvalidCharacter(Vec<char>),
}

/// Represents one crossing of two strands in a knot.
#[derive(Debug)]
struct Crossing {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    column: u8,
    orientation: Orientation,
}

/// The two possible orientations for a `Crossing`.
#[derive(Copy, Clone, Debug)]
enum Orientation {
    Positive,
    Negative,
}

/// A struct used to represent a `Crossing` still in the process of being parsed.
#[derive(Debug)]
struct CrossingBuilder {
    top: Option<usize>,
    bottom: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    column: u8,
    orientation: Orientation,
}

impl CrossingBuilder {
    /// Attempts to create a `Crossing`.
    fn build(self) -> Result<Crossing, ()> {
        Ok(Crossing {
            top: self.top.ok_or(())?,
            bottom: self.bottom.ok_or(())?,
            left: self.left.ok_or(())?,
            right: self.right.ok_or(())?,
            column: self.column,
            orientation: self.orientation,
        })
    }

    /// Creates a new `CrossingBuilder` with the given `column` number and `orientation`.
    fn new(column: u8, orientation: Orientation) -> Self {
        Self {
            top: None,
            bottom: None,
            left: None,
            right: None,
            column,
            orientation,
        }
    }
}

/// A utility for combining regions in an efficient manner.
pub struct RegionCounter {
    count: usize,
    regions: Vec<HashSet<usize>>,
}

impl RegionCounter {
    /// Creates a new `RegionCounter` with `start` as the initial number of regions.
    pub fn new(start: usize) -> Self {
        Self {
            count: start,
            regions: Vec::new(),
        }
    }

    /// Combines the two regions passed in, reducing the total count if necessary.
    pub fn combine(&mut self, first: usize, second: usize) {
        if first == second {
            return;
        }
        let mut temp: Vec<&mut HashSet<usize>> = self
            .regions
            .iter_mut()
            .filter(|r| r.contains(&first) || r.contains(&second))
            .collect();
        if temp.len() >= 2 {
            self.count -= 1;

            let new_set = temp
                .iter()
                .fold(HashSet::new(), |acc, e| acc.union(e).cloned().collect());
            self.regions
                .retain(|r| !(r.contains(&first) || r.contains(&second)));
            self.regions.push(new_set);
        } else if temp.is_empty() {
            self.count -= 1;

            let mut set = HashSet::new();
            set.insert(first);
            set.insert(second);
            self.regions.push(set);
        } else {
            let set = temp.pop().unwrap();
            if set.insert(first) || set.insert(second) {
                self.count -= 1;
            }
        }
    }

    /// Returns the current number of regions counted.
    fn current_count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn unknots(v: Vec<(usize, i16)>) -> Vec<usize> {
        v.iter().map(|r| r.0).collect()
    }

    mod region_counter {
        use crate::RegionCounter;

        #[test]
        fn basics() {
            let mut counter = RegionCounter::new(7);
            assert_eq!(7, counter.current_count());

            // Combining a region with itself at the beginning should have no effect.
            counter.combine(1, 1);
            assert_eq!(7, counter.current_count());
            counter.combine(2, 2);
            assert_eq!(7, counter.current_count());
            counter.combine(3, 3);
            assert_eq!(7, counter.current_count());

            // Combining two uncombined regions should decrement the count.
            counter.combine(1, 2);
            assert_eq!(6, counter.current_count());

            // Combining regions that have already been combined should have no effect
            counter.combine(1, 2);
            assert_eq!(6, counter.current_count());

            // Combining two uncombined regions should decrement the count.
            counter.combine(3, 4);
            assert_eq!(5, counter.current_count());

            counter.combine(4, 5);
            assert_eq!(4, counter.current_count());

            // Combining regions that have already been combined should have no effect
            counter.combine(4, 5);
            assert_eq!(4, counter.current_count());

            counter.combine(4, 1);
            assert_eq!(3, counter.current_count());

            counter.combine(5, 2);
            assert_eq!(3, counter.current_count());
        }
    }

    mod knot_parsing {
        #![allow(non_snake_case)]

        use super::unknots;
        use crate::Knot;
        use std::str::FromStr;

        #[test]
        fn basics() {
            let _knot = Knot::from_str("aaabacbadehfg").unwrap();

            let a = Knot::from_str("a").unwrap();
            assert_eq!(a.num_regions(), 3);

            let abb = Knot::from_str("abb").unwrap();
            assert_eq!(abb.num_regions(), 5);

            let abbaabb = Knot::from_str("abbaabb").unwrap();
            assert_eq!(abbaabb.num_regions(), 9);

            let abcB = Knot::from_str("abcB").unwrap();
            assert_eq!(abcB.num_regions(), 6);
        }

        #[test]
        fn missing_columns() {
            let ac = Knot::from_str("ac").unwrap();
            assert_eq!(ac.num_regions(), 5);

            let ace = Knot::from_str("ace").unwrap();
            assert_eq!(ace.num_regions(), 7);

            let acd = Knot::from_str("acd").unwrap();
            assert_eq!(acd.num_regions(), 6);

            let bc = Knot::from_str("bc").unwrap();
            assert_eq!(bc.num_regions(), 5);

            let b = Knot::from_str("b").unwrap();
            assert_eq!(b.num_regions(), 4);
        }
    }

    mod resolving {
        use super::unknots;
        use crate::Knot;
        use std::str::FromStr;

        #[test]
        fn basics() {
            let knot = Knot::from_str("abc").unwrap();

            let mut resolutions = unknots(knot.resolutions());
            //            println!("{:?}", knot.resolutions());
            resolutions.sort();
            assert_eq!(resolutions, vec![1, 2, 2, 2, 3, 3, 3, 4]);

            let knot = Knot::from_str("acb").unwrap();
            let mut resolutions = unknots(knot.resolutions());
            //            println!("{:?}", knot.resolutions());
            resolutions.sort();
            assert_eq!(resolutions, vec![1, 2, 2, 2, 3, 3, 3, 4]);

            let mut s = String::new();
            for _ in 0..10 {
                s.push('a');
            }
            let knot = Knot::from_str(&s).unwrap();
            let resolutions = unknots(knot.resolutions());
            //            println!("{:?}", knot.resolutions());
        }

        #[test]
        fn missing_columns() {
            let knot = Knot::from_str("b").unwrap();
            let mut resolutions = unknots(knot.resolutions());
            resolutions.sort();
            assert_eq!(resolutions, vec![2, 3]);

            let knot = Knot::from_str("bc").unwrap();
            let mut resolutions = unknots(knot.resolutions());
            resolutions.sort();
            assert_eq!(resolutions, vec![2, 3, 3, 4]);

            let knot = Knot::from_str("bd").unwrap();
            let mut resolutions = unknots(knot.resolutions());
            resolutions.sort();
            assert_eq!(resolutions, vec![3, 4, 4, 5]);
        }
    }

    mod polynomial_generation {
        use crate::Knot;
        use std::str::FromStr;

        #[test]
        fn basics() {
            println!("a: {}", Knot::from_str("a").unwrap().bracket_polynomial());
            println!("A: {}", Knot::from_str("A").unwrap().bracket_polynomial());
            println!("trefoil: {}", Knot::from_str("aaa").unwrap().bracket_polynomial());
        }
    }
}
