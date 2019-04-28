use std::ops::{Div, Mul};

pub struct Polynomial(Vec<Term>);

pub struct Binomial(Term, Term);

impl Binomial {
    fn expand(exp: i16) -> Polynomial {
        unimplemented!()
    }
}

struct Term {
    coefficient: isize,
    exponent: isize,
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

impl Mul for Term {
    type Output = Term;

    fn mul(self, rhs: Self) -> Self {
        Term {
            coefficient: self.coefficient * rhs.coefficient,
            exponent: self.exponent + rhs.exponent,
        }
    }
}

fn binomial_coefficient(n: isize, mut k: isize) -> isize {
    let mut res = 1;

    if (k > n - k) {
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