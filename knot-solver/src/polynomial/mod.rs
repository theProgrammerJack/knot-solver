use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul};
use rayon::prelude::*;

use num::{rational::Rational, One, Zero};

mod new;

#[macro_export]
macro_rules! term {
    ($c:literal A^ $e:literal) => {
        $crate::polynomial::Term::new($c, $e)
    };
    ($cn:literal / $cd:literal A^ $ec:literal / $ed:literal) => {
        $crate::polynomial::Term::new(
            num::rational::Rational::new($cn, $cd),
            num::rational::Rational::new($en, $ed),
        )
    };
    ($c:tt A^ $e:tt) => {
        $crate::polynomial::Term::new($c, $e)
    };
}

// macro_rules! polynomial {
//     ($c:literal ^ $e:literal $($tail:tt)*) => {
//         $crate::polynomial::polynomial::from($crate::polynomial::Term::new($c, $e)) + polynomial!($($tail)*)
//     };
//     ($c:literal ^ )
// }

/// A polynomial with one variable, represented by `A` in this documentation..
#[derive(Eq, PartialEq, Debug)]
pub struct Polynomial(Vec<Term>);

impl Polynomial {
    /// Creates a polynomial from a list of `Term`s.
    pub fn from_vec(mut terms: Vec<Term>) -> Self {
        terms.retain(|t| !t.is_zero());
        terms.sort_unstable_by(Term::compare_exponent);
        Polynomial(terms)
    }

    /// Creates a polynomial that is equal to 0.
    pub fn zero() -> Self {
        Polynomial::from_vec(Vec::new())
    }

    /// Returns a `DoubleEndedIterator` over the terms of the polynomials.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Term> + '_ {
        self.0.iter()
    }

    /// Removes all of the `Term`s equal to zero from the polynomial.
    pub fn remove_zero_terms(&mut self) {
        self.0.retain(|t| !t.is_zero());
    }

    pub fn par_iter(&self) -> impl IndexedParallelIterator<Item = &Term> + '_ {
        self.0.par_iter()
    }
}

impl IntoParallelIterator for Polynomial {
    type Iter = <Vec<Term> as IntoParallelIterator>::Iter;
    type Item = Term;

    fn into_par_iter(self) -> Self::Iter {
        self.0.into_par_iter()
    }
}

//impl IntoParallelIterator for &Polynomial {
//    type Iter = <Vec<Term> as IntoParallelIterator>::Iter;
//    type Item = Term;
//
//    fn into_par_iter(self) -> Self::Iter {
//        (&self.0).into_par_iter()
//    }
//}

impl From<Vec<Term>> for Polynomial {
    fn from(terms: Vec<Term>) -> Self {
        Polynomial::from_vec(terms)
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
                    if !(old.coefficient + rhs.coefficient).is_zero() {
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
        let mut p = Polynomial::zero();
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

/// A binomial.
pub struct Binomial(pub Term, pub Term);

impl Binomial {
    /// Raises the binomial to a power and returns the resulting polynomial.
    pub fn expand(self, exp: isize) -> Polynomial {
        Polynomial::from_vec(
            BinomialIter::new(exp)
                .map(|(c, k)| self.0.pow(k) * self.1.pow(exp - k) * c)
                .collect(),
        )
    }
}

/// One term consisting of a rational number coefficient and a rational number exponent for the
/// variable `A`.
#[derive(Eq, Debug, Clone, Copy)]
pub struct Term {
    coefficient: Rational,
    exponent: Rational,
}

impl Term {
    /// Creates a new `Term` with the given coefficient exponent.
    pub fn new<T: Into<Rational>>(coefficient: T, exponent: T) -> Self {
        Term {
            coefficient: coefficient.into(),
            exponent: exponent.into(),
        }
    }

    /// Raises the `Term` to the given power.
    pub fn pow(&self, exponent: isize) -> Self {
        Term {
            coefficient: self.coefficient.pow(exponent as i32),
            exponent: self.exponent * exponent,
        }
    }

    /// Returns whether this `Term` is equal to 0.
    pub fn is_zero(&self) -> bool {
        self.coefficient.is_zero()
    }

    /// Creates a `Term` that is equal to 0.
    pub fn zero() -> Self {
        Term::new(0, 0)
    }

    /// Returns whether this `Term` is equal to 1.
    pub fn is_one(&self) -> bool {
        self.exponent.is_zero() && self.coefficient.is_one()
    }

    /// Creates a `Term` that is equal to 1.
    pub fn one() -> Self {
        Term::new(1, 0)
    }

    /// Compares just the exponents of the two `Term`s.
    pub fn compare_exponent(&self, other: &Term) -> Ordering {
        self.exponent.cmp(&other.exponent)
    }

    /// Returns the coefficient of the `Term`.
    pub fn coefficient(&self) -> Rational {
        self.coefficient
    }

    /// Returns the exponent of the `Term`.
    pub fn exponent(&self) -> Rational {
        self.exponent
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

/// An iterator over the binomial coefficients for some `n`.
struct BinomialIter {
    n: isize,
    k: isize,
}

impl BinomialIter {
    /// Creates a new `BinomialIter` with the given `n`.
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

        #[test]
        fn macro_create() {
            assert_eq!(Term::new(3, 3), term!(3 A^ 3));
            assert_eq!(Term::new(-1, -3), term!(-1 A^ -3));
            assert_eq!(Term::new(3, 13), term!(3 A^ 13));
            assert_eq!(Term::new(-30, 56), term!(-30 A^ 56));
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
