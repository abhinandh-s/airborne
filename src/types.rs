use std::fmt::Display;
use std::ops::Index;
use std::ops::Sub;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(f64);

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl Percentage {
    /// construct a new `Percentage`
    ///
    /// # Examples
    ///
    /// ```
    /// use airborne::types::Percentage;
    ///
    /// let p = Percentage::new(10);
    /// let result = 1000.0 * p.as_decimal();
    /// let result_02 = 1000.0 * p; // also works cuz `Percentage` impls `Mul` for f64
    /// assert_eq!(result, 100.0);
    /// assert_eq!(result, result_02);
    /// ```
    pub fn new(p: impl Into<f64>) -> Self {
        let v = p.into();
        Self(v)
    }

    /// construct `Percentage` from fraction
    ///
    /// ```
    /// use airborne::types::Percentage;
    ///
    /// let perc = Percentage::from_frac(5, 10); // (5 / 10 ) * 100 = 50%
    /// assert_eq!(perc.inner(), 50.0);
    /// ```
    pub fn from_frac(num: impl Into<f64>, deno: impl Into<f64>) -> Self {
        Self((num.into() / deno.into()) * 100.0)
    }

    /// return `Percentage` as decimal
    ///  
    /// > `Percentage(10)` => 0.10
    ///
    /// ```
    /// use airborne::types::Percentage;
    ///
    /// let perc = Percentage::from_frac(5, 10);
    /// assert_eq!(perc.as_decimal(), 0.5);
    ///
    /// let result = 1000.0 * perc.as_decimal();
    /// assert_eq!(result, 500.0);
    /// ```
    pub fn as_decimal(&self) -> f64 {
        self.0 / 100.0
    }

    /// return innner value of `Percentage`
    ///
    /// Percentage(10) => 10
    ///
    /// # Examples
    ///
    /// ```
    /// use airborne::types::Percentage;
    ///
    /// let percentage = Percentage::new(40);
    /// assert_eq!(percentage.inner(), 40.0);
    /// ```
    pub fn inner(&self) -> f64 {
        self.0
    }
}

impl From<f64> for Percentage {
    fn from(value: f64) -> Self {
        Percentage(value)
    }
}

impl From<Percentage> for f64 {
    fn from(val: Percentage) -> Self {
        val.0
    }
}

macro_rules! impl_ops {
    ($trait:ident, $fn:ident, $symbol:tt) => {
        impl std::ops::$trait<f64> for Percentage {
            type Output = f64;

            fn $fn(self, rhs: f64) -> Self::Output {
                rhs $symbol (rhs * self.as_decimal())
            }
        }

        impl std::ops::$trait<Percentage> for f64 {
            type Output = f64;

            fn $fn(self, rhs: Percentage) -> Self::Output {
                self $symbol (self * rhs.as_decimal())
            }
        }

    }
}

macro_rules! impl_ops_02 {
    ($trait:ident, $fn:ident, $symbol:tt) => {
        impl std::ops::$trait<f64> for Percentage {
            type Output = f64;

            fn $fn(self, rhs: f64) -> Self::Output {
                rhs $symbol self.as_decimal()
            }
        }

        impl std::ops::$trait<Percentage> for f64 {
            type Output = f64;

            fn $fn(self, rhs: Percentage) -> Self::Output {
                self $symbol rhs.as_decimal()
            }
        }

    }
}

impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);
impl_ops_02!(Mul, mul, *);
impl_ops_02!(Div, div, /);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ratio {
    parts: Vec<u32>,
    total: u32,
}

impl Ratio {
    pub fn new(parts: impl IntoIterator<Item = u32>) -> Self {
        let parts: Vec<u32> = parts.into_iter().collect();
        let total = parts.iter().sum();
        Self { parts, total }
    }

    pub const fn len(&self) -> usize {
        self.parts.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub const fn total(&self) -> u32 {
        self.total
    }

    pub fn part(&self, index: usize) -> Option<Fraction> {
        self.parts.get(index).map(|&f| Fraction {
            num: f,
            denom: self.total(),
        })
    }

    pub fn percentage(&self, index: usize) -> Option<Percentage> {
        self.part(index).map(|f| f.as_percentage())
    }

    pub fn iter(&self) -> impl Iterator<Item = Fraction> {
        self.parts.iter().map(|&part| Fraction {
            num: part,
            denom: self.total(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fraction {
    num: u32,
    denom: u32,
}

impl Fraction {
    pub fn new(num: u32, denom: u32) -> Self {
        Self { num, denom }
    }

    pub fn as_percentage(self) -> Percentage {
        if self.denom == 0 {
            Percentage(0.0)
        } else {
            Percentage::from_frac(self.num, self.denom)
        }
    }

    pub fn num(&self) -> u32 {
        self.num
    }

    pub fn denom(&self) -> u32 {
        self.denom
    }

    pub fn set_num(&mut self, num: u32) {
        self.num = num;
    }

    pub fn set_denom(&mut self, denom: u32) {
        self.denom = denom;
    }

    pub fn simplify(&mut self) {
        if self.num == 0 {
            self.denom = 1;
            return;
        }
        let common = gcd(self.num, self.denom);
        self.num /= common;
        self.denom /= common;
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let l = self.num as u64 * rhs.denom as u64;
        let r = rhs.num as u64 * self.denom as u64;
        if l < r {
            panic!("Fraction subtraction resulted in a negative value");
        }
        Self {
            num: (l - r) as u32,
            denom: (self.denom as u64 * rhs.denom as u64) as u32,
        }
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

impl Index<usize> for Ratio {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.parts[index]
    }
}

#[macro_export]
macro_rules! ratio {
    ($first:literal $(: $rest:expr)+ $(,)?  ) => {
        {
            $crate::types::Ratio::new([$first, $($rest),+ ])
        }
    }
}

/// Debits and credits in double-entry bookkeeping are entries
/// made in account ledgers to record changes in value resulting
/// from business transactions. A debit entry in an account
/// represents a transfer of value to that account,
/// and a credit entry represents a transfer from the account.
/// Each transaction transfers value from credited accounts to debited accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntryType {
    /// repr Debit entry
    Debit,
    /// repr Credit entry
    Credit,
}

/// repr Debit entry
/// alias for EntryType::Debit
pub const DEBIT: EntryType = crate::types::EntryType::Debit;

/// repr Credit entry
/// alias for EntryType::Credit
pub const CREDIT: EntryType = crate::types::EntryType::Credit;

enum AccountType {
    Asset,
    Liability,
    Capital,
    Expense,
    Revenue,
}

impl AccountType {
    fn balance(&self) -> EntryType {
        match self {
            AccountType::Liability | AccountType::Capital | AccountType::Revenue => CREDIT,
            AccountType::Expense | AccountType::Asset => DEBIT,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::types::{Fraction, Percentage};

    #[test]
    fn fraction_sub_t() {
        let f1 = Fraction::new(3, 5);
        let f2 = Fraction::new(3, 6);
        let f_res = f1 - f2;
        assert_eq!(f_res, Fraction::new(3, 30))
    }

    #[test]
    fn ratio_t() {
        let ratio = ratio!(2:4:3:1);
        let first = ratio[0];
        assert_eq!(first, 2);
        let second_f = ratio.part(1).unwrap();
        assert_eq!(second_f, Fraction { num: 4, denom: 10 });
        let second_perc = second_f.as_percentage();
        assert_eq!(format!("{}", second_perc), "40%".to_owned());
    }

    macro_rules! test_ops {
        ($fn:ident, $sy:tt, $r1:expr, $r2:expr) => {
            #[test]
            fn $fn() {
                let res = 1000.0 $sy Percentage::new(10);
                assert_eq!(res, $r1);
                let res = Percentage::new(10) $sy 1000.0;
                assert_eq!(res, $r2);
            }
        }
    }

    test_ops!(add, +, 1100.0, 1100.0);
    test_ops!(sub, -, 900.0, 900.0);

    #[test]
    fn mul_t() {
        let res = 1000.0 * Percentage::new(10);
        assert_eq!(res, 100.0);
    }

    #[test]
    fn div_t() {
        let res = 1000.0 / Percentage::new(10);
        assert_eq!(res, 10000.0);
    }
}
