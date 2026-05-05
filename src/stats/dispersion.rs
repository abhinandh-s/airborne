use std::{
    ops::{AddAssign, Div, Mul, Sub},
    process::Output,
};

use num_traits::{FromPrimitive, One, Zero};

use crate::{
    DataSet, Population, Sample, StatsError,
    compute::{ComputeFloat, N},
    error::{Result, check_empty_set},
    marker::Marker,
    numeric::Numeric,
};

pub trait Dispersion<M: Marker = Population> {
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
pub(crate) fn variance_n(series: &[N], dof: N) -> Result<(N, N)> {
    check_empty_set(series)?;

    let mut n = N::cf_zero();
    let mut mean = N::cf_zero();
    let mut m2 = N::cf_zero();

    for &x in series {
        n += N::cf_one();
        let delta = x - mean;
        mean += delta / n;
        let delta2 = x - mean;
        m2 += delta * delta2;
    }

    Ok((mean, m2 / (dof)))
}

#[doc(hidden)]
#[macro_export]
macro_rules! variance_n {
    ($series:expr, $dof:expr) => {
        $crate::stats::dispersion::variance_n($series, $dof).map(|(_, v)| v)
    };
    ($series:expr, $dof:expr, mu) => {
        $crate::stats::dispersion::variance_n($series, $dof)
    };
}

#[test]
fn variance_n_macro_t() -> Result<()> {
    let series = &[2, 3, 4, 5].map(N::cf_from_usize);
    let dof = N::cf_from_usize(2);
    let _v1 = variance_n!(series, dof)?;
    let (_mu, _v) = variance_n!(series, dof, mu)?;
    Ok(())
}

pub(crate) fn std_dev_n(series: &[N], dof: N) -> Result<(N, N, N)> {
    let (mu, v) = variance_n(series, dof)?;
    Ok((mu, v, v.cf_sqrt()))
}

#[doc(hidden)]
#[macro_export]
macro_rules! std_dev_n {
    ($series:expr, $dof:expr) => {
        $crate::stats::dispersion::std_dev_n($series, $dof).map(|(_, _, sd)| sd)
    };
    ($series:expr, $dof:expr, mu) => {
        $crate::stats::dispersion::std_dev_n($series, $dof).map(|(mu, _, sd)| (mu, sd))
    };
    ($series:expr, $dof:expr, v) => {
        $crate::stats::dispersion::std_dev_n($series, $dof).map(|(_, v, sd)| (v, sd))
    };
    ($series:expr, $dof:expr, mu, v) => {
        $crate::stats::dispersion::std_dev_n($series, $dof)
    };
}

#[test]
fn std_dev_n_macro_t() -> Result<()> {
    let series = &[2, 3, 4, 5].map(N::cf_from_usize);
    let dof = N::cf_from_usize(2);
    let _v1 = std_dev_n!(series, dof)?;
    let (_mu, _v) = std_dev_n!(series, dof, mu)?;
    let (_mu, _v) = std_dev_n!(series, dof, v)?;
    let (_mu, _v, _sd) = std_dev_n!(series, dof, mu, v)?;
    Ok(())
}

impl<T: Numeric, M: Marker> Dispersion for DataSet<T, M> {
    fn variance(&self) -> Result<f64> {
        let series = self.to_n_vec()?;
        let dof = self.dof_denominator_n()?;
        let (_, v) = variance_n(&series, dof)?;
        Ok(v.cf_to_f64())
    }

    fn std_dev(&self) -> Result<f64> {
        let series = self.to_n_vec()?;
        let dof = self.dof_denominator_n()?;
        let (_, _, sd) = std_dev_n(&series, dof)?;
        Ok(sd.cf_to_f64())
    }

    fn normalize(&self) -> Result<Vec<f64>> {
        let v = self.to_n_vec()?;
        let min = v.iter().cloned().fold(N::cf_infinity(), N::cf_min);
        let max = v.iter().cloned().fold(N::cf_neg_infinity(), N::cf_max);
        let rng = max - min;
        if rng == N::cf_zero() {
            return Err(crate::StatsError::ZeroVariance);
        }

        let n = v.iter().map(|&x| (x - min) / rng);
        let res = n.map(N::cf_to_f64).collect();
        Ok(res)
    }

    type Output = f64;
}

pub fn variance<T, M: Marker>(data: &[T]) -> Result<T>
where
    T: Copy
        + Zero
        + One
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + AddAssign
        + FromPrimitive,
{
    check_empty_set(data)?;

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

    let count = T::from_usize(data.len()).unwrap();
    let dof = count - T::from_usize(M::DOF_OFFSET).ok_or(StatsError::ConversionErrorUnchecked)?;
    Ok(m2 / dof)
}

impl<T> Dispersion<Population> for [T]
where
    T: Copy
        + Zero
        + One
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + AddAssign
        + FromPrimitive,
{
    type Output = T;

    fn variance(&self) -> Result<T> {
        variance::<T, Population>(self)
    }

    fn std_dev(&self) -> Result<T> {
        todo!()
    }

    fn normalize(&self) -> Result<Vec<T>> {
        todo!()
    }
    //     type Output = T;
    //
    //     fn mean(&self) -> Result<Self::Output> {
    //         let count = self.len();
    //         // If the iterator is empty, return `Err` to avoid division by zero
    //         if count == 0 {
    //             return Err(StatsError::EmptyIterator);
    //         }
    //
    //         let sum: T = self.iter().copied().sum();
    //         // safety: I checked its not `0` & its of same type.
    //         // so, this must not panic!
    //         Ok(sum / T::from_usize(count).unwrap())
    //     }
}

#[cfg(test)]
mod test {
    use crate::stats::dispersion::variance;
    use crate::{DataSet, Sample};

    use super::Dispersion;

    // https://statisticsbyjim.com/calculators/variance-calculator/
    #[test]
    fn variance_t() {
        let slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0].map(|x| x as f64);
        let data: DataSet<i32> = DataSet::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        let v01 = data.variance().unwrap();
        assert_eq!(v01, 8.250000);

        let v: f64 = slice.variance().unwrap();
        variance::<f64, Sample>(slice);
        assert_eq!(v, 8.25);
    }

    // https://www.calculator.net/standard-deviation-calculator.html?numberinputs=10%2C+12%2C+23%2C+23%2C+16%2C+23%2C+21%2C+16&ctype=p&x=Calculate
    #[test]
    fn std_dev_t() {
        let data: DataSet<i32> = DataSet::from_iter([10, 12, 23, 23, 16, 23, 21, 16]);
        let sd = data.std_dev().unwrap();
        assert_eq!(sd, 4.898979485566356);
    }

    #[test]
    fn normalize_t() {
        let data: DataSet<i32> = DataSet::from_iter([10, 5, 0]);
        let sd = data.normalize().unwrap();
        assert_eq!(sd, vec![1.0, 0.5, 0.0]);
    }
}
