use crate::compute::{ComputeFloat, N, to_n_vec};
use crate::error::Result;
use crate::marker::Marker;
use crate::numeric::Numeric;
use crate::{DataSet, n_from_usize, n_sum};

pub trait Dispersion {
    fn variance(&self) -> Result<f64>;
    fn std_dev(&self) -> Result<f64>;
    // fn covariance(&self, other: &[f64]) -> Result<f64>;
}

impl<T: Numeric, M: Marker> DataSet<T, M> {
    pub(crate) fn variance_n(&self) -> Result<N> {
        let v = to_n_vec(&self.data)?;
        let n = n_from_usize!(v.len());
        let mu = n_sum!(v.iter().copied()) / n; // first pass: mean
        let denom = n_from_usize!(self.dof_denominator()?);
        let ss = n_sum!(v.iter().map(|&x| (x - mu).cf_powi(2))); // second pass: SS
        Ok(ss / denom)
    }

    pub(crate) fn std_dev_n(&self) -> Result<N> {
        let v = self.variance_n()?;
        Ok(v.cf_sqrt())
    }
}

impl<T: Numeric, M: Marker> Dispersion for DataSet<T, M> {
    fn variance(&self) -> Result<f64> {
        self.variance_n().map(|v| v.cf_to_f64())
    }

    fn std_dev(&self) -> Result<f64> {
        self.std_dev_n().map(|sd| sd.cf_to_f64())
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
