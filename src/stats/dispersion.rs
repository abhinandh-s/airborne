use crate::DataSet;
use crate::compute::{ComputeFloat, N};
use crate::error::Result;
use crate::marker::Marker;
use crate::numeric::Numeric;

use super::mean_n;

pub trait Dispersion {
    fn variance(&self) -> Result<f64>;
    fn std_dev(&self) -> Result<f64>;
}

pub(crate) fn variance_n(series: &[N], dof: N) -> N {
    let mu = mean_n(series);
    let ss = N::cf_sum(series.iter().map(|&x| (x - mu).cf_powi(2)));

    ss / dof
}

pub(crate) fn std_dev_n(series: &[N], dof: N) -> N {
    variance_n(series, dof).cf_sqrt()
}

impl<T: Numeric, M: Marker> Dispersion for DataSet<T, M> {
    fn variance(&self) -> Result<f64> {
        let series = self.to_n_vec()?;
        let dof = self.dof_denominator_n()?;
        Ok(variance_n(&series, dof).cf_to_f64())
    }

    fn std_dev(&self) -> Result<f64> {
        let series = self.to_n_vec()?;
        let dof = self.dof_denominator_n()?;
        Ok(std_dev_n(&series, dof).cf_to_f64())
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
}
