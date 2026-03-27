use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::compute::{ComputeFloat, N};
use crate::error::{Result, StatsError};
use crate::marker::{Marker, Population, Sample};
use crate::numeric::Numeric;

/// A statistically-typed, generic dataset.
///
/// The two parameters are:
///
/// `T` => Any type which satisfies `Numeric` trait. (`i32`, `f64`, `u16`, ...)
/// `M` => [`Sample`] or [`Population`] => defaults to [`Population`]
///
/// Since `M` defaults to `Population`,
/// `DataSet<i32>` and `DataSet<i32, Population>` are exactly the same type.
///
/// ## Example
///
/// ### DataSet construction
///
/// ```ignore
/// use airborne::DataSet;
///
///// this won't compile
/// let data = DataSet::new([1, 2, 3, 4, 5]).unwrap(); // type annotations needed for `dataset::DataSet<i32, _>`
///                                                                // cannot satisfy `_: marker::Marker`
/// ```
///
/// valid constructions:
///
/// ```rust
/// use airborne::{DataSet, SampleData, PopulationData, Population, Sample};
///
/// fn build_data_set() {
///     // Same same but different!
///
///     // build via type annotation
///     let data_sample_01: DataSet<i32, Sample> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
///     let data_population_01: DataSet<i32, Population> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
///
///     // Default
///     let _data_default_population: DataSet<i32> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
///
///     // type alias
///     let pr_sample: SampleData<i32> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
///     let portfolio_return: PopulationData<i32> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
///
///     // .into_{}()
///     let _into_sample = portfolio_return.into_sample(); // type:  DataSet<i32, Sample>
///     let _into_population = data_sample_01.into_population(); // type:  DataSet<i32>    
///                                                                           // .as_{}_clone()
///     let _sample_clone = data_population_01.as_sample_clone(); // type:  DataSet<i32, Sample>
///     let _as_population_clone = pr_sample.as_population_clone(); // type:  DataSet<i32>
/// }
/// ```
pub struct DataSet<T: Numeric, M: Marker = Population> {
    pub(crate) data: Vec<T>,
    _marker: PhantomData<M>,
}

/// type alias for `DataSet` with `Sample` marker prefixed.
pub type SampleData<T> = DataSet<T, Sample>;

/// type alias for `DataSet` with `Population` marker prefixed.
pub type PopulationData<T> = DataSet<T, Population>;

// -- Accessors
impl<T: Numeric, M: Marker> DataSet<T, M> {
    /// Add a data point in place.
    #[inline]
    pub fn push(&mut self, value: T) {
        self.data.push(value)
    }

    // return len of inner Pec
    #[inline]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    /// Always `false` — construction rejects empty datasets.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Borrow the underlying slice.
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Consume and return the underlying `Vec`.
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Append values from an iterator in place.
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

impl<T: Numeric, M: Marker> DataSet<T, M> {
    /// Creates a new DataSet from any iterator
    ///
    /// # Errors
    ///
    /// This function will return an error ([`StatsError::EmptyDataset`]) if the `data` is empty.
    pub fn new(data: impl IntoIterator<Item = T>) -> Result<Self> {
        let data: Vec<T> = data.into_iter().collect();
        if data.is_empty() {
            return Err(StatsError::EmptyIterator);
        }
        Ok(Self {
            data,
            _marker: PhantomData,
        })
    }

    /// Name of the marker type: `"Sample"` or `"Population"`.
    pub fn marker_name(&self) -> &'static str {
        M::NAME
    }

    /// Returns the dof denominator of this [`DataSet<T, M>`].
    /// The effective denominator for variance/covariance: `n − DOF_OFFSET`.
    ///
    /// # Errors
    ///
    /// This function will return an error [`StatsError::InsufficientData`]
    /// if when `n ≤ DOF_OFFSET`
    /// (most commonly: sample variance with only one data point).
    pub fn dof_denominator(&self) -> Result<usize> {
        let n = self.len();
        let offset = M::DOF_OFFSET;

        // bound check (no zero or negative number)
        if n <= offset {
            return Err(StatsError::InsufficientData {
                needed: offset + 1,
                got: n,
            });
        }

        Ok(n - offset)
    }
    
    pub fn dof_denominator_n(&self) -> Result<N> {
        self.dof_denominator().map(N::cf_from_usize)
    }

    #[inline]
    pub(crate) fn len_n(&self) -> N {
        N::cf_from_usize(self.data.len())
    }
    
    /// Convert `DataSet` to `Vec<N>`
    /// This is the single entry point for all functions.
    pub(crate) fn to_n_vec(&self) -> Result<Vec<N>> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                let f = v.to_f64().ok_or(StatsError::ConversionError { index: i })?;
                if !f.is_finite() {
                    return Err(StatsError::InvalidValue { index: i });
                }
                Ok(N::cf_from_f64(f))
            })
            .collect()
    }

    // Lazy iterator variant
    // to be used when we don't need random access.
    pub(crate) fn to_n_iter(&self) -> impl Iterator<Item = N> {
            self.data.iter().map(|&v| {
                // NaN/inf is already validated by `to_n_vec` as public boundary.
                // this is for internal chaining after validation.
                N::cf_from_f64(v.to_f64().expect("validated"))
            })
    }
}

impl<T: Numeric, M: Marker> DataSet<T, M> {
    /// Re-interpret this dataset as a **sample** dataset (zero allocation).
    pub fn into_sample(self) -> SampleData<T> {
        DataSet {
            data: self.data,
            _marker: PhantomData,
        }
    }

    /// Re-interpret this dataset as a **population** dataset (zero allocation).
    pub fn into_population(self) -> PopulationData<T> {
        DataSet {
            data: self.data,
            _marker: PhantomData,
        }
    }

    /// Clone and re-interpret as a sample dataset.
    pub fn as_sample_clone(&self) -> SampleData<T> {
        DataSet {
            data: self.data.clone(),
            _marker: PhantomData,
        }
    }

    /// Clone and re-interpret as a population dataset.
    pub fn as_population_clone(&self) -> PopulationData<T> {
        DataSet {
            data: self.data.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: Numeric, M: Marker> Debug for DataSet<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataSet<{}>({:?})", M::NAME, self.data)
    }
}

impl<T: Numeric, M: Marker> std::fmt::Display for DataSet<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataSet<{}>[n={}]", M::NAME, self.len())
    }
}

impl<T, M> IntoIterator for DataSet<T, M>
where
    T: Numeric,
    M: Marker,
{
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T, M> IntoIterator for &'a DataSet<T, M>
where
    T: Numeric,
    M: Marker,
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

#[macro_export]
macro_rules! data_set {
      ($($x:expr),+ $(,)?) => (
        $crate::DataSet::new(
            // Using the intrinsic produces a dramatic improvement in stack usage for
            // unoptimized programs using this code path to construct large Vecs.
            [$($x),+]
        )
    );
}

impl<T, M> Deref for DataSet<T, M>
where
    T: Numeric,
    M: Marker,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, M> DerefMut for DataSet<T, M>
where
    T: Numeric,
    M: Marker,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T, M> AsRef<[T]> for DataSet<T, M>
where
    T: Numeric,
    M: Marker,
{
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::marker::{Sample, Population};

    #[test]
    fn test_new_success() {
        let data = vec![1, 2, 3];
        let ds = DataSet::<i32, Population>::new(data).unwrap();
        assert_eq!(ds.len(), 3);
        assert!(!ds.is_empty());
    }

    #[test]
    fn test_new_empty_error() {
        let data: Vec<i32> = vec![];
        let result = DataSet::<i32, Population>::new(data);
        assert!(result.is_err());
        // Verify it returns StatsError::EmptyIterator
    }

    #[test]
    fn test_push_and_extend() {
        let mut ds = DataSet::<i32, Sample>::new([1]).unwrap();
        ds.push(2);
        ds.extend([3, 4]);
        assert_eq!(ds.data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_dof_denominator() {
        // Population DOF_OFFSET is usually 0
        let pop = DataSet::<i32, Population>::new([10, 20]).unwrap();
        assert_eq!(pop.dof_denominator().unwrap(), 2);

        // Sample DOF_OFFSET is usually 1
        let samp = DataSet::<i32, Sample>::new([10, 20]).unwrap();
        assert_eq!(samp.dof_denominator().unwrap(), 1);

        // Test InsufficientData error
        let tiny_samp = DataSet::<i32, Sample>::new([10]).unwrap();
        assert!(tiny_samp.dof_denominator().is_err());
    }

    #[test]
    fn test_conversions_reinterpret() {
        let ds = DataSet::<i32, Population>::new([1, 2]).unwrap();
        
        // Test into_ conversions (consuming)
        let sample = ds.into_sample();
        assert_eq!(sample.marker_name(), "Sample");
        
        let pop = sample.into_population();
        assert_eq!(pop.marker_name(), "Population");

        // Test clone conversions (non-consuming)
        let s_clone = pop.as_sample_clone();
        let p_clone = s_clone.as_population_clone();
        
        assert_eq!(p_clone.len(), 2);
    }

    #[test]
    fn test_to_n_vec_and_iter() {
        let ds = DataSet::<f64, Population>::new([1.1, 2.2]).unwrap();
        
        // Test successful conversion
        let n_vec = ds.to_n_vec().unwrap();
        assert_eq!(n_vec.len(), 2);

        // Test internal iter (usually used after validation)
        let n_iter_count = ds.to_n_iter().count();
        assert_eq!(n_iter_count, 2);
    }

    #[test]
    fn test_formatting_traits() {
        let ds = DataSet::<i32, Population>::new([1, 2]).unwrap();
        
        // Debug
        let debug_str = format!("{:?}", ds);
        assert!(debug_str.contains("DataSet<Population>([1, 2])"));

        // Display
        let display_str = format!("{}", ds);
        assert_eq!(display_str, "DataSet<Population>[n=2]");
    }

    #[test]
    fn test_deref_and_slices() {
        let mut ds = DataSet::<i32, Population>::new([1, 2, 3]).unwrap();
        
        // Deref to slice
        assert_eq!(ds.as_slice(), &[1, 2, 3]);
        assert_eq!(ds.len(), 3);
        
        // DerefMut
        ds[0] = 10;
        assert_eq!(ds[0], 10);
        
        // AsRef
        let r: &[i32] = ds.as_ref();
        assert_eq!(r[0], 10);
    }

    #[test]
    fn test_iterators() {
        let ds = DataSet::<i32, Population>::new([1, 2]).unwrap();

        // Ref Iterator
        for val in &ds {
            assert!(*val > 0);
        }

        // IntoIterator (consuming)
        let vec: Vec<i32> = ds.into_iter().collect();
        assert_eq!(vec, vec![1, 2]);
    }

    #[test]
    fn test_macro_construction() {
        let ds: Result<DataSet<i32, Population>> = data_set![1, 2, 3];
        assert!(ds.is_ok());
        assert_eq!(ds.unwrap().len(), 3);
    }

    #[test]
    fn test_into_vec() {
        let ds = DataSet::<i32, Population>::new([1, 2]).unwrap();
        let v = ds.into_vec();
        assert_eq!(v, vec![1, 2]);
    }
}
