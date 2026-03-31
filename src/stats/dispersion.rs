use crate::{
    DataSet,
    compute::{ComputeFloat, N},
    error::{Result, check_empty_set},
    marker::Marker,
    numeric::Numeric,
};

pub trait Dispersion {
    fn variance(&self) -> Result<f64>;
    fn std_dev(&self) -> Result<f64>;
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
}

#[cfg(test)]
mod test {
    use crate::DataSet;

    use super::Dispersion;

    // https://statisticsbyjim.com/calculators/variance-calculator/
    #[test]
    fn variance_t() {
        let data: DataSet<i32> = DataSet::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]).unwrap();
        let variance = data.variance().unwrap();
        assert_eq!(variance, 8.250000);
    }

    // https://www.calculator.net/standard-deviation-calculator.html?numberinputs=10%2C+12%2C+23%2C+23%2C+16%2C+23%2C+21%2C+16&ctype=p&x=Calculate
    #[test]
    fn std_dev_t() {
        let data: DataSet<i32> = DataSet::new([10, 12, 23, 23, 16, 23, 21, 16]).unwrap();
        let sd = data.std_dev().unwrap();
        assert_eq!(sd, 4.898979485566356);
    }
}
