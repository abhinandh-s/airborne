use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::compute::{ComputeFloat, N};
use crate::error::{Result, StatsError};
use crate::marker::{Marker, Population, Sample};
use crate::numeric::Numeric;

/// ## DataSet
///
/// A statistically-typed, generic dataset.
///
/// `DataSet` provides a type-safe way to handle statistical calculations by marking
/// data as either [`Sample`] or a [`Population`] at compile time. This ensures that
/// operations like variance or standard deviation use the correct [Degree of Freedom](https://en.wikipedia.org/wiki/Degrees_of_freedom_(statistics)) (DOF).
///
/// ## Type parameters
///
/// - `T` => Any type which satisfies `Numeric` trait. (`i32`, `f64`, `u16`, ...)
/// - `M` => [`Sample`] or [`Population`] => defaults to [`Population`]
///
/// Since `M` defaults to `Population`,
/// `DataSet<i32>` and `DataSet<i32, Population>` are exactly the same type.
///
/// ## Example
///
/// ### DataSet construction
///
/// ```rust,ignore
/// use airborne::DataSet;
///
///// this won't compile
/// let data = DataSet::from_iter([1, 2, 3, 4, 5]); // type annotations needed for `dataset::DataSet<i32, _>`
///                                                                // cannot satisfy `_: marker::Marker`
/// ```
///
/// valid constructions:
///
/// ```rust
/// use airborne::{DataSet, SampleData, PopulationData, Population, Sample};
///
/// fn build_data_set() {
///     // build via type annotation
///     let data_sample_01: DataSet<i32, Sample> = DataSet::from_iter([1, 2, 3, 4, 5]);
///     let data_population_01: DataSet<i32, Population> = DataSet::from_iter([1, 2, 3, 4, 5]);
///
///     // Default
///     let _data_default_population: DataSet<i32> = DataSet::from_iter([1, 2, 3, 4, 5]);
///
///     // type alias
///     let pr_sample: SampleData<i32> = DataSet::from_iter([1, 2, 3, 4, 5]);
///     let portfolio_return: PopulationData<i32> = DataSet::from_iter([1, 2, 3, 4, 5]);
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

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Borrow the underlying slice.
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// construct `DataSet` from any `AsRef<[T]>` (slice, array ref, vec ref, etc. )
    #[inline]
    pub fn from_slice(data: impl AsRef<[T]>) -> Self {
        Self::from_iter(data.as_ref().iter().copied())
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
    // coverage: exclude
    /// Creates a new DataSet from any iterator
    ///
    /// # Errors
    ///
    /// This function will return an error ([`StatsError::EmptyIterator`]) if the `data` is empty.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Name of the marker type: `"Sample"` or `"Population"`.
    pub fn marker_name(&self) -> &'static str {
        M::NAME // coverage: exclude
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
                needed: offset + 1, // coverage: exclude
                got: n,             // coverage: exclude
            });
        }

        Ok(n - offset)
    }

    pub(crate) fn dof_denominator_n(&self) -> Result<N> {
        self.dof_denominator().map(N::cf_from_usize)
    }

    #[inline]
    pub(crate) fn len_n(&self) -> N {
        N::cf_from_usize(self.data.len())
    }

    /// Convert `DataSet` to `Vec<N>`
    /// This is the single entry point for all functions.
    pub(crate) fn to_n_vec(&self) -> Result<Vec<N>> {
        if self.data.is_empty() {
            return Err(StatsError::EmptyIterator);
        }
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
            // is_empty,
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

// # impl
//   - Default
//   - Debug
//   - Display
//   - FromIterator<T>
//   - IntoIterator for DataSet<T, M>
//   - IntoIterator for &'a DataSet<T, M>
//   - Deref
//   - DerefMut
//   - AsRef<[T]>
//   - From:
//      - Vec<T>
//      - [T; N]
//      - &[T; N]
//      - [T]

impl<T: Numeric, M: Marker> Default for DataSet<T, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Numeric, M: Marker> std::fmt::Debug for DataSet<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataSet<{}>({:?})", M::NAME, self.data)
    }
}

impl<T: Numeric, M: Marker> std::fmt::Display for DataSet<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Numeric, M: Marker> From<Vec<T>> for DataSet<T, M> {
    fn from(v: Vec<T>) -> Self {
        Self::from_iter(v)
    }
}

impl<T: Numeric, M: Marker, const N: usize> From<[T; N]> for DataSet<T, M> {
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<T: Numeric, M: Marker, const N: usize> From<&[T; N]> for DataSet<T, M> {
    fn from(arr: &[T; N]) -> Self {
        Self::from_slice(arr)
    }
}

impl<T: Numeric, M: Marker> From<&[T]> for DataSet<T, M> {
    fn from(s: &[T]) -> Self {
        Self::from_slice(s)
    }
}

impl<T, M> FromIterator<T> for DataSet<T, M>
where
    T: Numeric,
    M: Marker,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut data = Vec::new();
        for item in iter {
            data.push(item);
        }

        DataSet {
            data,
            _marker: std::marker::PhantomData,
        }
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

/// Creates a [`DataSet`] containing the arguments.
///
/// `dataset!` allows `DataSet`s to be defined with the same syntax as array expressions.
/// There are two forms of this macro:
///
/// * Create a [`DataSet`] containing a given list of elements:
///
/// ```rust
/// use airborne::dataset;
/// use airborne::DataSet;
///
/// let v: DataSet<i32> = dataset![1, 2, 3];
/// assert_eq!(v[0], 1);
/// assert_eq!(v[1], 2);
/// assert_eq!(v[2], 3);
/// ```
///
/// * Create a [`DataSet`] from a given element and size:
///
/// ```rust
/// use airborne::dataset;
/// use airborne::DataSet;
///
/// let v: DataSet<i32> = dataset![1; 3];
/// assert_eq!(*v, [1, 1, 1]);
/// ```
///
/// Also, note that `dataset![expr; 0]` is allowed, and produces an empty vector.
/// This will still evaluate `expr`, however, and immediately drop the resulting value, so
/// be mindful of side effects.
#[macro_export]
macro_rules! dataset {
    () => (
        $crate::DataSet::new()
    );
    ($elem:expr; $n:expr) => (
       $crate::DataSet::from_slice([$elem; $n])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::DataSet::from_iter(
            [$($x),+]
        )
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        DataSet, Marker, Sample, StatsError,
        compute::{ComputeFloat, N},
        marker::{self, Population},
    };

    #[test]
    fn test_new_success() {
        let data = [1, 2, 3].into_iter();
        let ds = DataSet::<i32, Population>::from_iter(data);
        assert_eq!(ds.len(), 3);
        assert!(!ds.is_empty());
    }

    #[test]
    fn test_from() {
        let my_slice = &[1.0, 2.1];
        let my_array = [1.0];

        let _a: DataSet<f64, Sample> = [1.0, 2.0, 3.0].into(); // .into() via From<[T;N]>
        let _b: DataSet<f64, Sample> = vec![1.0].into(); // .into() via From<Vec>
        let _c: DataSet<f64, Sample> = DataSet::from([1.0, 2.0, 3.0]); // [T; N]
        let _d: DataSet<f64, Sample> = DataSet::from(&[1.0, 2.0, 3.0]); // &[T; N]
        let _e: DataSet<f64, Sample> = DataSet::from(vec![1.0, 2.0, 3.0]); // Vec<T>
        let _f: DataSet<f64, Sample> = DataSet::from(my_slice); // &[T]
        let _g: DataSet<f64, Sample> = DataSet::from_slice(my_array); // any AsRef<[T]>
        let _h = DataSet::<i32, Population>::from(&[1, 2, 3]);
    }

    #[test]
    fn test_new_empty_error() {
        let data = [];
        let result = DataSet::<i32, Population>::from_iter(data);
        assert!(result.to_n_vec().is_err());
        // Verify it returns StatsError::EmptyIterator
    }

    #[test]
    fn test_push_and_extend() {
        let mut ds = DataSet::<i32, Sample>::from([1]);
        ds.push(2);
        ds.extend([3, 4]);
        assert_eq!(ds.data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_dof_denominator() {
        // Population DOF_OFFSET is usually 0
        let pop = DataSet::<i32, Population>::from([10, 20]);
        assert_eq!(pop.dof_denominator().unwrap(), 2);

        // Sample DOF_OFFSET is usually 1
        let samp = DataSet::<i32, Sample>::from([10, 20]);
        assert_eq!(samp.dof_denominator().unwrap(), 1);

        // Test InsufficientData error
        let tiny_samp = DataSet::<i32, Sample>::from([10]);
        assert!(tiny_samp.dof_denominator().is_err());
    }

    #[test]
    fn test_conversions_reinterpret() {
        let ds = DataSet::<i32, Population>::from([1, 2]);

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
        let ds = DataSet::<f64, Population>::from([1.1, 2.2]);

        // Test successful conversion
        let n_vec = ds.to_n_vec().unwrap();
        assert_eq!(n_vec.len(), 2);

        // Test internal iter (usually used after validation)
        let n_iter_count = ds.to_n_iter().count();
        assert_eq!(n_iter_count, 2);
    }

    #[test]
    fn test_formatting_traits() {
        let ds = DataSet::<i32, Population>::from([1, 2]);

        // Debug
        let debug_str = format!("{:?}", ds);
        assert!(debug_str.contains("DataSet<Population>([1, 2])"));

        // Display
        let display_str = format!("{}", ds);
        assert_eq!(display_str, "[1, 2]");
    }

    #[test]
    fn test_deref_and_slices() {
        let mut ds = DataSet::<i32, Population>::from([1, 2, 3]);

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
        let ds = DataSet::<i32, Population>::from([1, 2]);

        // Ref Iterator
        for val in &ds {
            assert!(*val > 0);
        }

        // IntoIterator (consuming)
        let vec: Vec<i32> = ds.into_iter().collect();
        assert_eq!(vec, vec![1, 2]);
    }

    #[test]
    fn test_into_vec() {
        let ds = DataSet::<i32, Population>::from([1, 2]);
        let v = ds.into_vec();
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn marker_name_t() {
        let ds = DataSet::<i32, Population>::from([1, 2]);
        let m = ds.marker_name();
        let n = <marker::Population as Marker>::NAME;
        assert_eq!("Population", m);
        assert_eq!(n, m);
    }

    #[test]
    fn test_dof_denominator_error_coverage() {
        let ds = DataSet::<f64, Sample>::from(vec![1.0]);
        let result = ds.dof_denominator();

        assert!(result.is_err());

        assert_eq!(
            result,
            Err(StatsError::InsufficientData { needed: 2, got: 1 })
        );
    }

    #[test]
    fn len_n_t() {
        let ds = DataSet::<i32, Population>::from([1, 2]);
        let m = ds.len_n();
        assert_eq!(N::cf_from_usize(2), m);
    }

    #[test]
    #[should_panic]
    // called `Result::unwrap()` on an `Err` value: InvalidValue { index: 1 }
    fn to_n_vec_must_fail_on_inf_t() {
        let d: DataSet<f64> = DataSet::from([10.0, f64::NAN, 2.0]);
        d.to_n_vec().unwrap();
    }
}
