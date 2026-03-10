use num_traits::ToPrimitive;

use crate::error::{Result, StatsError};

pub trait Numeric:
    Copy + PartialOrd + ToPrimitive + std::fmt::Debug + Send + Sync + 'static
{
}

impl<T> Numeric for T where
    T: Copy + PartialOrd + ToPrimitive + std::fmt::Debug + Send + Sync + 'static
{
}

/// Convert one value, annotating the index for error messages.
#[inline]
pub(crate) fn to_f64_at<T: Numeric>(val: T, index: usize) -> Result<f64> {
    val.to_f64().ok_or(StatsError::ConversionError { index })
}
/// Convert an entire slice to `Vec<f64>`, rejecting NaN and ±∞.
pub(crate) fn to_f64_vec<T: Numeric>(data: &[T]) -> Result<Vec<f64>> {
    data.iter()
        .enumerate()
        .map(|(index, &val)| {
            let f = val.to_f64().ok_or(StatsError::ConversionError { index })?;

            if !f.is_finite() {
                Err(StatsError::InvalidValue { index })
            } else {
                Ok(f)
            }
        })
        .collect()
}

/// Sort a `Vec<f64>` ascending (safe after NaN has been rejected).
pub(crate) fn sort_asc(mut v: Vec<f64>) -> Vec<f64> {
    // SAFETY: NaN values are rejected by to_f64_vec before reaching here.
    v.sort_unstable_by(|a, b| a.partial_cmp(b).expect("NaN after validation"));
    v
}

/// Compute ranks (1-based, average-rank for ties) of a sorted-index array.
/// `data` must already be finite (NaN-free).
pub(crate) fn ranks(data: &[f64]) -> Vec<f64> {
    let n = data.len();
    // Build (value, original_index) pairs, sort by value.
    let mut indexed: Vec<(f64, usize)> = data
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    indexed.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).expect("unexpected NaN"));

    let mut rank_vec = vec![0.0f64; n];
    let mut i = 0;
    while i < n {
        // Find the extent of a tie group.
        let mut j = i + 1;
        while j < n && indexed[j].0 == indexed[i].0 {
            j += 1;
        }
        // Average rank for the group (1-based).
        let avg_rank = (i + 1 + j) as f64 / 2.0;
        for k in i..j {
            rank_vec[indexed[k].1] = avg_rank;
        }
        i = j;
    }
    rank_vec
}
