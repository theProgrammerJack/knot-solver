use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul};

#[derive(Eq, PartialEq, Debug)]
pub struct Polynomial(Vec<Term>);

impl Polynomial {
    fn from_vec(mut terms: Vec<Term>) -> Self {
        terms.retain(|t| !t.is_zero());
        terms.sort_unstable_by(Term::compare_exponent);
        Polynomial(terms)
    }

    fn empty() -> Self {
        Polynomial::from_vec(Vec::new())
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Term> + '_ {
        self.0.iter()
    }

    pub fn remove_zero_terms(&mut self) {
        self.0.retain(|t| !t.is_zero());
    }
}

impl IntoIterator for Polynomial {
    type Item = Term;
    type IntoIter = <Vec<Term> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let terms = self
            .iter()
            .rev()
            .map(|t| format!("{}", t))
            .collect::<Vec<_>>()
            .join(" + ");
        write!(f, "{}", terms)
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
        if !rhs.is_zero() {
            match self.0.binary_search_by(|t| t.compare_exponent(&rhs)) {
                Ok(i) => {
                    let old = self.0.remove(i);
                    if old.coefficient + rhs.coefficient != 0 {
                        self.0.insert(
                            i,
                            Term {
                                // Need to manually construct term since adding terms returns a polynomial.
                                coefficient: old.coefficient + rhs.coefficient,
                                exponent: old.exponent,
                            },
                        );
                    }
                }
                Err(i) => self.0.insert(i, rhs),
            }
        }
    }
}

impl Sum for Polynomial {
    fn sum<I: Iterator<Item = Polynomial>>(iter: I) -> Self {
        let mut p = Polynomial::empty();
        for polynomial in iter {
            p += polynomial;
        }
        p
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

impl AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Polynomial) {
        for term in rhs {
            *self += term;
        }
    }
}

impl Mul<Term> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Term) -> Self::Output {
        Polynomial::from_vec(self.into_iter().map(|t| t * rhs).collect())
    }
}

impl From<Term> for Polynomial {
    fn from(term: Term) -> Self {
        Polynomial::from_vec(vec![term])
    }
}

pub struct Binomial(pub Term, pub Term);

impl Binomial {
    pub fn expand(self, exp: isize) -> Polynomial {
        Polynomial::from_vec(
            BinomialIter::new(exp)
                .map(|(c, k)| self.0.pow(k) * self.1.pow(exp - k) * c)
                .collect(),
        )
    }
}

#[derive(Eq, Debug, Clone, Copy)]
pub struct Term {
    coefficient: isize,
    exponent: isize,
}

impl Term {
    pub fn new(coefficient: isize, exponent: isize) -> Self {
        Term {
            coefficient,
            exponent,
        }
    }

    pub fn pow(&self, exponent: isize) -> Self {
        Term {
            coefficient: self.coefficient.pow(exponent as u32),
            exponent: self.exponent * exponent,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.coefficient == 0
    }

    pub fn zero() -> Self {
        Term::new(0, 0)
    }

    pub fn is_one(&self) -> bool {
        self.exponent == 0 && self.coefficient == 1
    }

    pub fn one() -> Self {
        Term::new(1, 0)
    }

    pub fn compare_exponent(&self, other: &Term) -> Ordering {
        self.exponent.cmp(&other.exponent)
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

impl Mul<isize> for Term {
    type Output = Term;

    fn mul(self, rhs: isize) -> Self::Output {
        Term {
            coefficient: self.coefficient * rhs,
            exponent: self.exponent,
        }
    }
}

impl Add for Term {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        if self.exponent == rhs.exponent {
            Polynomial::from_vec(vec![Term {
                coefficient: self.coefficient + rhs.coefficient,
                exponent: self.exponent,
            }])
        } else {
            Polynomial::from_vec(vec![self, rhs])
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}A^{}", self.coefficient, self.exponent)
    }
}

fn binomial_coefficient(n: isize, mut k: isize) -> isize {
    let mut res = 1;

    if k > n - k {
        k = n - k;
    }

    for i in 0..k {
        res *= n - i;
        res /= i + 1;
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
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.k <= self.n {
            let res = binomial_coefficient(self.n, self.k);
            self.k += 1;
            Some((res, self.k - 1))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    mod binomial {
        use crate::polynomial::{binomial_coefficient, Binomial, BinomialIter, Polynomial, Term};

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
            assert_eq!(
                BinomialIter::new(0).map(|t| t.0).collect::<Vec<isize>>(),
                vec![1]
            );
            assert_eq!(
                BinomialIter::new(1).map(|t| t.0).collect::<Vec<isize>>(),
                vec![1, 1]
            );
            assert_eq!(
                BinomialIter::new(2).map(|t| t.0).collect::<Vec<isize>>(),
                vec![1, 2, 1]
            );
            assert_eq!(
                BinomialIter::new(3).map(|t| t.0).collect::<Vec<isize>>(),
                vec![1, 3, 3, 1]
            );
            assert_eq!(
                BinomialIter::new(4).map(|t| t.0).collect::<Vec<isize>>(),
                vec![1, 4, 6, 4, 1]
            );
            assert_eq!(
                BinomialIter::new(5).map(|t| t.0).collect::<Vec<isize>>(),
                vec![1, 5, 10, 10, 5, 1]
            );
        }

        #[test]
        fn expand() {
            assert_eq!(
                Polynomial::from_vec(vec![Term::new(1, 4), Term::new(2, 3), Term::new(1, 2)]),
                Binomial(Term::new(1, 1), Term::new(1, 2)).expand(2)
            );

            assert_eq!(
                Polynomial::from_vec(vec![
                    Term::new(8, 3),
                    Term::new(36, 6),
                    Term::new(54, 9),
                    Term::new(27, 12)
                ]),
                Binomial(Term::new(2, 1), Term::new(3, 4)).expand(3)
            );
        }
    }

    mod term {
        use crate::polynomial::{Polynomial, Term};

        #[test]
        fn display() {
            let t1 = Term::new(3, 5);
            assert_eq!("3A^5", format!("{}", t1));

            let t1 = Term::new(7, 9);
            assert_eq!("7A^9", format!("{}", t1));
        }

        #[test]
        fn pow() {
            assert_eq!(Term::new(4, 6), Term::new(2, 3).pow(2));
            assert_eq!(Term::new(3, 2), Term::new(3, 2).pow(1));
            assert_eq!(Term::new(1, 3), Term::new(1, 1).pow(3));
            assert_eq!(Term::new(8, 9), Term::new(2, 3).pow(3));
        }

        #[test]
        fn mul() {
            assert_eq!(Term::new(3, 6) * Term::new(6, 3), Term::new(18, 9));
            assert_eq!(Term::new(4, 7) * Term::new(1, 1), Term::new(4, 8));
            assert_eq!(Term::new(2, 2) * Term::new(5, 5), Term::new(10, 7));
            assert_eq!(Term::new(8, 23) * Term::new(20, 7), Term::new(160, 30));
        }

        #[test]
        fn add() {
            assert_eq!(
                Term::new(3, 4) + Term::new(4, 5),
                Polynomial::from_vec(vec![Term::new(3, 4), Term::new(4, 5)])
            );

            assert_eq!(
                Term::new(3, 4) + Term::new(4, 4),
                Polynomial::from_vec(vec![Term::new(7, 4)])
            );
        }

        #[test]
        fn is_zero() {
            assert!(Term::new(0, 1).is_zero());
            assert!(Term::new(0, -1).is_zero());
        }
    }

    mod polynomial {
        use crate::polynomial::{Polynomial, Term};

        #[test]
        fn display() {
            assert_eq!(
                "3A^4",
                format!("{}", Polynomial::from_vec(vec![Term::new(3, 4)]))
            );
            assert_eq!(
                "3A^4 + 4A^3",
                format!(
                    "{}",
                    Polynomial::from_vec(vec![Term::new(3, 4), Term::new(4, 3)])
                )
            );
            assert_eq!(
                "3A^4 + 4A^3",
                format!(
                    "{}",
                    Polynomial::from_vec(vec![Term::new(4, 3), Term::new(3, 4)])
                )
            );

            assert_eq!(
                "3A^4 + 4A^3",
                format!(
                    "{}",
                    Polynomial::from(Term::new(3, 4)) + Polynomial::from(Term::new(4, 3))
                )
            );
        }

        #[test]
        fn add_term() {
            assert_eq!(
                Polynomial::from(Term::new(3, 4)) + Polynomial::from(Term::new(4, 3)),
                Polynomial::from_vec(vec![Term::new(3, 4), Term::new(4, 3)])
            );

            assert_eq!(
                Polynomial::from_vec(vec![Term::new(3, 4), Term::new(4, 3)])
                    + Polynomial::from(Term::new(4, 3)),
                Polynomial::from_vec(vec![Term::new(3, 4), Term::new(8, 3)])
            );
        }

        #[test]
        fn add_polynomial() {
            assert_eq!(
                Polynomial::from_vec(vec![Term::new(3, 4), Term::new(4, 3)])
                    + Polynomial::from_vec(vec![Term::new(3, 4), Term::new(4, 3)]),
                Polynomial::from_vec(vec![Term::new(6, 4), Term::new(8, 3)])
            );

            assert_eq!(
                Polynomial::from_vec(vec![Term::new(3, 4), Term::new(4, 3)])
                    + Polynomial::from_vec(vec![Term::new(3, 7), Term::new(4, 22)]),
                Polynomial::from_vec(vec![
                    Term::new(3, 4),
                    Term::new(4, 3),
                    Term::new(3, 7),
                    Term::new(4, 22)
                ])
            );
        }
    }
}
