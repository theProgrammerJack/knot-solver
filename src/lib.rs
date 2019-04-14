use bitvec::{BitVec, LittleEndian};
use std::{collections::HashSet, str::FromStr};

pub struct Knot {
    crossings: Vec<Crossing>,
    region_num: usize,
}

impl Knot {
    pub fn num_regions(&self) -> usize {
        self.region_num
    }

    pub fn num_crossings(&self) -> usize {
        self.crossings.len()
    }

    fn resolutions(&self) -> Vec<usize> {
        let r = (0u128..(2u128.pow(self.num_crossings() as u32)));
        r.map(|n| {
            let bits: BitVec<LittleEndian, _> = BitVec::from(&n.to_le_bytes()[..]);
            let mut counter = RegionCounter::new(self.num_regions());
            self.crossings
                .iter()
                .zip(bits.iter())
                .for_each(|(crossing, bit)| match crossing.orientation {
                    Orientation::Over => {
                        if bit {
                            counter.combine(crossing.left, crossing.right)
                        } else {
                            counter.combine(crossing.top, crossing.bottom)
                        }
                    }
                    Orientation::Under => {
                        if bit {
                            counter.combine(crossing.top, crossing.bottom)
                        } else {
                            counter.combine(crossing.left, crossing.right)
                        }
                    }
                });
            counter.current_count()
        })
        .collect()
    }
}

impl FromStr for Knot {
    type Err = KnotParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bad_chars: Vec<char> = s.chars().filter(|c| !c.is_ascii_alphabetic()).collect();
        if !bad_chars.is_empty() {
            Err(KnotParseError::InvalidCharacter(bad_chars))
        } else {
            let mut crossing_builders: Vec<CrossingBuilder> = s
                .chars()
                .map(|c| {
                    let orientation = if c.is_ascii_lowercase() {
                        Orientation::Over
                    } else {
                        Orientation::Under
                    };
                    let c = c.to_ascii_lowercase() as u8;
                    let column = c - 97; // 97 -> 'a' on the ascii table

                    CrossingBuilder::new(column, orientation)
                })
                .collect();
            // TODO: Verify no missing column indices

            let max_column = crossing_builders
                .iter()
                .max_by_key(|cb| cb.column)
                .unwrap()
                .column;

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

            Ok(Knot {
                crossings: crossing_builders
                    .into_iter()
                    .map(|c| c.build().unwrap())
                    .collect(),
                region_num: index,
            })
        }
    }
}

#[derive(Debug)]
pub enum KnotParseError {
    InvalidCharacter(Vec<char>),
}

#[derive(Debug)]
struct Crossing {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    column: u8,
    orientation: Orientation,
}

#[derive(Copy, Clone, Debug)]
enum Orientation {
    Over,
    Under,
}

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
}

impl CrossingBuilder {
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

pub struct RegionCounter {
    count: usize,
    regions: Vec<HashSet<usize>>,
}

impl RegionCounter {
    pub fn new(start: usize) -> Self {
        Self {
            count: start,
            regions: Vec::new(),
        }
    }

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

    mod region_counter {
        use crate::RegionCounter;

        #[test]
        fn basics() {
            let mut counter = RegionCounter::new(7);
            assert_eq!(7, counter.current_count());

            counter.combine(1, 2);
            assert_eq!(6, counter.current_count());

            counter.combine(1, 2);
            assert_eq!(6, counter.current_count());

            counter.combine(3, 4);
            assert_eq!(5, counter.current_count());

            counter.combine(4, 5);
            assert_eq!(4, counter.current_count());

            counter.combine(4, 5);
            assert_eq!(4, counter.current_count());

            counter.combine(4, 1);
            assert_eq!(3, counter.current_count());
        }
    }

    mod knot_parsing {
        #![allow(non_snake_case)]

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
    }

    mod resolving {
        use crate::Knot;
        use std::str::FromStr;

        #[test]
        fn basics() {
            let knot = Knot::from_str("abc").unwrap();

            println!("{:?}", knot.resolutions());
        }
    }
}
