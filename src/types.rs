use std::ops::Deref;
use std::ops::DerefMut;

use crate::Result;
use crate::error::check_bound;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(f64);

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
    /// assert_eq!(result, 100.0);
    /// ```
    ///
    /// # Error
    ///
    /// a value outside 0..100 will cause [`StatsError::InvalidRange`].
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
    /// let perc = Percentage::from_frac(5, 10);
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

impl Into<f64> for Percentage {
    fn into(self) -> f64 {
        self.0
    }
}
