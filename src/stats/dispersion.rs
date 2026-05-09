//! Dispersion (trait)
//!
//! 1. variance
//!    - computed using Welford's algorithm
//!    - supports both Sample and Population data
//! 2. standard deviation
//! 3. normalize

use num_traits::{FromPrimitive, One, Zero};

use crate::Population;
use crate::{
    error::{Result, check_empty_set, is_valid_slice},
    marker::Marker,
    numeric::{NumExt, NumOps},
};

pub trait Dispersion<M: Marker> {
    type Output;
    fn variance(&self) -> Result<Self::Output>;
    fn std_dev(&self) -> Result<Self::Output>;
    fn normalize(&self) -> Result<Vec<Self::Output>>;
}

// Welford maintains a running mean and running sum-of-squared-deviations in one pass:
//
// for each xᵢ:
//     n    += 1
//     delta = xᵢ - mean
//     mean += delta / n
//     delta2 = xᵢ - mean        ← uses the *updated* mean
//     M2   += delta * delta2    ← M2 = Σ(xᵢ - mean)^2
//
// At the end: variance = M2 / n (population) or M2 / (n-1) (sample).
// No separate mean pass, no two-sum.
pub fn variance<T, M: Marker>(data: &[T]) -> Result<T>
where
    T: Zero + One + NumOps + FromPrimitive,
{
    let normalized_count = is_valid_slice::<T, M>(data)?;

    let mut n = T::zero();
    let mut mean = T::zero();
    let mut m2 = T::zero();

    for &x in data {
        n += T::one();
        let delta = x - mean;
        mean += delta / n;
        let delta2 = x - mean;
        m2 += delta * delta2;
    }

    Ok(m2 / normalized_count)
}

pub fn std_dev<T, M: Marker>(series: &[T]) -> Result<T>
where
    T: Zero + One + NumOps + NumExt + FromPrimitive,
{
    variance::<T, M>(series).map(|x| x.sqrt())?
}

pub fn normalize<T>(data: &[T]) -> Result<Vec<T>>
where
    T: NumOps + Zero,
{
    check_empty_set(data)?;

    let mut min = &data[0];
    let mut max = &data[0];

    for val in data.iter() {
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    let rng = *max - *min;

    if rng == T::zero() {
        return Err(crate::StatsError::ZeroVariance);
    }
    let n = data.iter().map(|&x| (x - *min) / rng);
    let res = n.collect();
    Ok(res)
}

impl<T> Dispersion<Population> for [T]
where
    T: Zero + One + NumOps + NumExt + FromPrimitive,
{
    type Output = T;

    fn variance(&self) -> Result<T> {
        variance::<T, Population>(self)
    }

    fn std_dev(&self) -> Result<T> {
        std_dev::<T, Population>(self)
    }

    fn normalize(&self) -> Result<Vec<T>> {
        check_empty_set(self)?;

        let mut min = &self[0];
        let mut max = &self[0];

        for val in self.iter() {
            if val < min {
                min = val;
            }
            if val > max {
                max = val;
            }
        }

        let rng = *max - *min;

        if rng == T::zero() {
            return Err(crate::StatsError::ZeroVariance);
        }
        let n = self.iter().map(|&x| (x - *min) / rng);
        let res = n.collect();
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::compute::ComputeFloat;
    use crate::compute::N;
    use crate::stats::dispersion::variance;
    use crate::stats::std_dev;
    use crate::{Population, Sample};

    use super::Dispersion;

    // https://statisticsbyjim.com/calculators/variance-calculator/
    #[test]
    fn variance_t() {
        let slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0].map(|x| nusize!(x));
        let v = slice.variance().unwrap();
        let _v = variance::<N, Sample>(slice);
        assert_eq!(v, nf64!(8.25));
    }

    // https://www.calculator.net/standard-deviation-calculator.html?numberinputs=10%2C+12%2C+23%2C+23%2C+16%2C+23%2C+21%2C+16&ctype=p&x=Calculate
    #[test]
    fn std_dev_t() {
        let data = [10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0];
        let sd = std_dev::<f64, Population>(&data).unwrap();
        assert_eq!(sd, 4.898979485566356);
    }

    #[test]
    fn normalize_t() {
        let data = [10.0, 5.0, 0.0];
        let sd = data.normalize().unwrap();
        assert_eq!(sd, vec![1.0, 0.5, 0.0]);
    }
}
