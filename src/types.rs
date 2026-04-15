use std::fmt::Display;
use std::ops::Add;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Div;
use std::ops::Index;
use std::ops::Mul;
use std::ops::Sub;

use crate::Result;
use crate::error::check_bound;

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
    /// let p = Percentage::new(10).unwrap();
    /// let result = 1000.0 * p.as_decimal();
    /// let result = 1000.0 * p; // also works cuz `Percentage` impls `Mul` for f64
    /// assert_eq!(result, 100.0);
    /// ```
    ///
    /// # Error
    ///
    /// a value outside 0..100 will cause [`crate::error::StatsError::InvalidRange`].
    /// Thereby, `Percentage` guarantees value is in betweem 0..10
    pub fn new(p: impl Into<f64>) -> Result<Self> {
        let v = p.into();
        check_bound(v, 0.0, 100.0)?;
        Ok(Self(v))
    }

    /// construct a new `Percentage` without bound check
    ///
    /// # Examples
    ///
    /// ```
    /// use airborne::types::Percentage;
    ///
    /// let p = Percentage::new_unchecked(10);
    /// let result = 1000.0 * p.as_decimal();
    /// assert_eq!(result, 100.0);
    /// ```
    pub fn new_unchecked(p: impl Into<f64>) -> Self {
        Self(p.into())
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
    /// let percentage = Percentage::new_unchecked(40);
    /// assert_eq!(percentage.inner(), 40.0);
    /// ```
    pub fn inner(&self) -> f64 {
        self.0
    }
}

impl Deref for Percentage {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Percentage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl Add<Percentage> for f64 {
    type Output = f64;

    fn add(self, rhs: Percentage) -> Self::Output {
        self + (self * rhs.as_decimal())
    }
}

impl Sub<Percentage> for f64 {
    type Output = f64;

    fn sub(self, rhs: Percentage) -> Self::Output {
        self - (self * rhs.as_decimal())
    }
}

impl Mul<Percentage> for f64 {
    type Output = f64;

    fn mul(self, rhs: Percentage) -> Self::Output {
        self * rhs.as_decimal()
    }
}

impl Div<Percentage> for f64 {
    type Output = f64;

    fn div(self, rhs: Percentage) -> Self::Output {
        self / rhs.as_decimal()
    }
}

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
            den: self.total(),
        })
    }

    pub fn percentage(&self, index: usize) -> Option<Percentage> {
        self.part(index).map(|f| f.as_percentage())
    }

    pub fn iter(&self) -> impl Iterator<Item = Fraction> {
        self.parts.iter().map(|&part| Fraction {
            num: part,
            den: self.total(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fraction {
    num: u32,
    den: u32,
}

impl Fraction {
    pub fn as_percentage(self) -> Percentage {
        if self.den == 0 {
            Percentage(0.0)
        } else {
            Percentage::from_frac(self.num, self.den)
        }
    }

    pub fn num(&self) -> u32 {
        self.num
    }
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

#[cfg(test)]
mod test {
    use crate::types::{Fraction, Percentage};

    #[test]
    fn ratio_t() {
        let ratio = ratio!(2:4:3:1);
        let first = ratio[0];
        assert_eq!(first, 2);
        let second_f = ratio.part(1).unwrap();
        assert_eq!(second_f, Fraction { num: 4, den: 10 });
        let second_perc = second_f.as_percentage();
        assert_eq!(second_perc, Percentage::new_unchecked(40));
        assert_eq!(format!("{}", second_perc), "40%".to_owned());
    }

    #[test]
    fn add_t() {
        let res = 1000.0 + Percentage::new_unchecked(10);
        assert_eq!(res, 1100.0);
    }

    #[test]
    fn sub_t() {
        let res = 1000.0 - Percentage::new_unchecked(10);
        assert_eq!(res, 900.0);
    }

    #[test]
    fn mul_t() {
        let res = 1000.0 * Percentage::new_unchecked(10);
        assert_eq!(res, 100.0);
    }

    #[test]
    fn div_t() {
        let res = 1000.0 / Percentage::new_unchecked(10);
        assert_eq!(res, 10000.0);
    }
}
