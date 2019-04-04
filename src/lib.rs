use std::{collections::HashSet, str::FromStr};

struct Knot {
    crossings: Vec<Crossing>,
    region_num: usize,
}

impl FromStr for Knot {
    type Err = KnotParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!();

        let bad_chars: Vec<char> = s.chars().filter(|c| !c.is_ascii_alphabetic()).collect();
        if !bad_chars.is_empty() {
            Err(KnotParseError::InvalidCharacter(bad_chars))
        } else {
            let mut crossing_builders: Vec<CrossingBuilder> = s.chars().map(|c| {
                let c = c.to_ascii_lowercase() as u8;
                let column = c - 97; // 97 -> 'a' on the ascii table

                CrossingBuilder::new(column)
            }).collect();
            // TODO: Verify no missing column indices

            let max_column = crossing_builders.iter().max_by_key(|cb| cb.column).unwrap().column;


            let mut index = 2;
            for i in 0..crossing_builders.len() {
                let mut builder = crossing_builders[i];

                if builder.column == 0 {
                    builder.left = Some(0);
                } else if builder.column == max_column {
                    builder.right = Some(1);
                }

                if builder.bottom.is_none() {
                    builder.bottom = Some(index);
                    for j in 0..crossing_builders.len() {
                        let j = j + i + 1;
                        let j = j % crossing_builders.len();

                        let mut builder2 = crossing_builders[j];

                        if builder2.column == builder.column {
                            builder2.top = Some(index);
                            index += 1;
                            break;
                        } else if builder.column < builder2.column && builder2.column - builder.column == 1 {
                            builder2.left = Some(index);
                        } else if builder2.column < builder.column && builder.column - builder2.column == 1 {
                            builder2.right = Some(index);
                        }
                    }
                }
            }

            Ok(Knot {
                crossings: crossing_builders.into_iter().map(|c| c.build().unwrap()).collect(),
                region_num: index,
            })
        }
    }
}

enum KnotParseError {
    InvalidCharacter(Vec<char>),
}

struct Crossing {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    column: u8,
}

struct CrossingBuilder {
    top: Option<usize>,
    bottom: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    column: u8,
}

impl CrossingBuilder {
    fn build(self) -> Result<Crossing, ()> {
        Ok(Crossing {
            top: self.top.ok_or(())?,
            bottom: self.bottom.ok_or(())?,
            left: self.left.ok_or(())?,
            right: self.right.ok_or(())?,
            column: self.column,
        })
    }
}

impl CrossingBuilder {
    fn new(column: u8) -> Self {
        Self {
            top: None,
            bottom: None,
            left: None,
            right: None,
            column,
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
}
