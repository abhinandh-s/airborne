#![allow(unused)]
use num_traits::ToPrimitive;


pub trait Numeric:
    Copy + PartialOrd + ToPrimitive + std::fmt::Debug + Send + Sync + 'static
{
}

impl<T> Numeric for T where
    T: Copy + PartialOrd + ToPrimitive + std::fmt::Debug + Send + Sync + 'static
{
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
