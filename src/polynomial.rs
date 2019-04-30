use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, AddAssign};

pub struct Polynomial(Vec<Term>);

impl Polynomial {
    fn from_vec(mut terms: Vec<Term>) -> Self {
        terms.sort_unstable_by(compare_term_exponent);
        Polynomial(terms)
    }

    fn iter(&self) -> impl Iterator + '_ {
        self.0.iter()
    }
}

impl IntoIterator for Polynomial {
    type Item = Term;
    type IntoIter = <Vec<Term> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Add<Term> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Term) -> Self::Output {
        let mut p = self;
        p += rhs;
        p
    }
}

impl AddAssign<Term> for Polynomial {
    fn add_assign(&mut self, rhs: Term) {
        match self.0.binary_search_by(|t| compare_term_exponent(t, &rhs)) {
            Ok(i) => {
                let old = self.0.remove(i);
                self.0.insert(
                    i,
                    Term {
                        // Need to manually construct term since adding terms returns a polynomial.
                        coefficient: old.coefficient + rhs.coefficient,
                        exponent: old.exponent,
                    },
                )
            }
            Err(i) => self.0.insert(i, rhs),
        }
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Self::Output {
        let mut p = self;
        for term in rhs {
            p += term;
        }
        p
    }
}

pub struct Binomial(Term, Term);

impl Binomial {
    fn expand(exp: i16) -> Polynomial {
        unimplemented!()
    }
}

#[derive(Eq)]
pub struct Term {
    coefficient: isize,
    exponent: isize,
}

fn compare_term_exponent(t1: &Term, t2: &Term) -> Ordering {
    t1.exponent.cmp(&t2.exponent)
}

impl Term {
    fn new(coefficient: isize, exponent: isize) -> Self {
        Term {
            coefficient,
            exponent,
        }
    }

    fn pow(&self, exponent: isize) -> Self {
        Term {
            coefficient: self.coefficient,
            exponent: self.exponent + exponent,
        }
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> Ordering {
        self.exponent
            .cmp(&other.exponent)
            .then_with(|| self.coefficient.cmp(&other.coefficient))
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Mul for Term {
    type Output = Term;

    fn mul(self, rhs: Self) -> Self::Output {
        Term {
            coefficient: self.coefficient * rhs.coefficient,
            exponent: self.exponent + rhs.exponent,
        }
    }
}

impl Add for Term {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        if self.exponent == rhs.exponent {
            Polynomial(vec![Term {
                coefficient: self.coefficient + rhs.coefficient,
                exponent: self.exponent,
            }])
        } else {
            Polynomial(vec![self, rhs])
        }
    }
}

fn binomial_coefficient(n: isize, mut k: isize) -> isize {
    let mut res = 1;

    if k > n - k {
        k = n - k;
    }

    for i in 0..k {
        res *= (n - i);
        res /= (i + 1);
    }

    res
}

struct BinomialIter {
    n: isize,
    k: isize,
}

impl BinomialIter {
    fn new(n: isize) -> Self {
        BinomialIter { n, k: 0 }
    }
}

impl Iterator for BinomialIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.k <= self.n {
            let res = binomial_coefficient(self.n, self.k);
            self.k += 1;
            Some(res)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    mod binomial {
        use crate::polynomial::{binomial_coefficient, BinomialIter};

        #[test]
        fn basics() {
            assert_eq!(binomial_coefficient(0, 0), 1);
            assert_eq!(binomial_coefficient(1, 0), 1);
            assert_eq!(binomial_coefficient(1, 1), 1);
            assert_eq!(binomial_coefficient(2, 1), 2);
            assert_eq!(binomial_coefficient(3, 1), 3);
            assert_eq!(binomial_coefficient(4, 2), 6);
        }

        #[test]
        fn iter() {
            assert_eq!(BinomialIter::new(0).collect::<Vec<isize>>(), vec![1]);
            assert_eq!(BinomialIter::new(1).collect::<Vec<isize>>(), vec![1, 1]);
            assert_eq!(BinomialIter::new(2).collect::<Vec<isize>>(), vec![1, 2, 1]);
            assert_eq!(
                BinomialIter::new(3).collect::<Vec<isize>>(),
                vec![1, 3, 3, 1]
            );
            assert_eq!(
                BinomialIter::new(4).collect::<Vec<isize>>(),
                vec![1, 4, 6, 4, 1]
            );
            assert_eq!(
                BinomialIter::new(5).collect::<Vec<isize>>(),
                vec![1, 5, 10, 10, 5, 1]
            );
        }
    }
}
