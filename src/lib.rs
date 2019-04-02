use std::{
    str::FromStr,
    collections::HashSet,
};

struct Knot {
    crossings: Vec<Crossing>,
}

impl FromStr for Knot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

struct Crossing {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    column: usize,
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
        let temp: Vec<&HashSet<usize>> = self.regions.iter().filter(|r| r.contains(&first) || r.contains(&second)).collect();
        if temp.len() >= 2 {
            self.count -= 1;

            let new_set = temp.iter().fold(HashSet::new(), |acc, e| acc.union(e).cloned().collect());
            self.regions.retain(|r| !(r.contains(&first) || r.contains(&second)));
            self.regions.push(new_set);
        } else if temp.is_empty() {
            self.count -= 1;

            let mut set = HashSet::new();
            set.insert(first);
            set.insert(second);
            self.regions.push(set);
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
        }


    }
}
