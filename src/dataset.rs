use std::marker::PhantomData;

use crate::error::{StatsError, Result};
use crate::marker::{Marker, Population, Sample};
use crate::numeric::Numeric;

pub struct DataSet<T: Numeric, M: Marker = Population> {
    pub(crate) data: Vec<T>,
    _marker: PhantomData<M>,
}

// -- Accessors
impl<T: Numeric, M: Marker> DataSet<T, M> {
    #[inline]
    pub fn push(&mut self, value: T) {
        self.data.push(value)
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T: Numeric, M: Marker> DataSet<T, M> {
    pub fn new(data: impl IntoIterator<Item = T>) -> Result<Self> {
        let data: Vec<T> = data.into_iter().collect();
        if data.is_empty() {
            return Err(StatsError::EmptyDataSet);
        }
        Ok(Self {
            data,
            _marker: PhantomData,
        })
    }

    pub fn marker_name(&self) -> &'static str {
        M::NAME
    }

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

pub type SampleData<T> = DataSet<T, Sample>;

pub type PopulationData<T> = DataSet<T, Population>;
