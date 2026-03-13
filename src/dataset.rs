use std::fmt::Debug;
use std::marker::PhantomData;

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
/// ```rust,no_run
/// // this won't compile
/// let data = DataSet::new([1, 2, 3, 4, 5]).unwrap(); // type annotations needed for `dataset::DataSet<i32, _>`
///                                                                // cannot satisfy `_: marker::Marker`
/// ```
///
/// valid constructions:
///
/// ```rust
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
            return Err(StatsError::EmptyDataset);
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
