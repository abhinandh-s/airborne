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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranks_empty() {
        let data: [f64; 0] = [];
        let result = ranks(&data);
        assert!(result.is_empty());
    }

    #[test]
    fn test_ranks_single_element() {
        let data = [10.0];
        let result = ranks(&data);
        // Rank 1 for a single element: (1 + 1) / 2 = 1.0
        assert_eq!(result, vec![1.0]);
    }

    #[test]
    fn test_ranks_distinct_values() {
        // Data is unsorted, but values are distinct
        let data = [10.0, 30.0, 20.0];
        let result = ranks(&data);
        // Sorted order: 10.0 (idx 0), 20.0 (idx 2), 30.0 (idx 1)
        // Ranks: 10.0 -> 1.0, 20.0 -> 2.0, 30.0 -> 3.0
        assert_eq!(result, vec![1.0, 3.0, 2.0]);
    }

    #[test]
    fn test_ranks_all_ties() {
        let data = [5.0, 5.0, 5.0];
        let result = ranks(&data);
        // (1 + 3) / 2 = 2.0 for all
        assert_eq!(result, vec![2.0, 2.0, 2.0]);
    }

    #[test]
    fn test_ranks_mixed_ties() {
        // 1.0 (idx 0), 2.0 (idx 1), 2.0 (idx 2), 3.0 (idx 3), 3.0 (idx 4), 3.0 (idx 5)
        let data = [1.0, 2.0, 2.0, 3.0, 3.0, 3.0];
        let result = ranks(&data);

        // 1.0: rank 1.0
        // 2.0: average of rank 2 and 3 = 2.5
        // 3.0: average of rank 4, 5, and 6 = (4 + 6) / 2 = 5.0
        assert_eq!(result, vec![1.0, 2.5, 2.5, 5.0, 5.0, 5.0]);
    }

    #[test]
    fn test_ranks_unsorted_with_ties() {
        let data = [40.0, 10.0, 40.0, 20.0];
        // Sorted: 10.0 (idx 1), 20.0 (idx 3), 40.0 (idx 0, 2)
        // 10.0 -> Rank 1
        // 20.0 -> Rank 2
        // 40.0 -> Rank (3+4)/2 = 3.5
        let result = ranks(&data);
        assert_eq!(result, vec![3.5, 1.0, 3.5, 2.0]);
    }

    #[test]
    #[should_panic(expected = "unexpected NaN")]
    fn test_ranks_panics_on_nan() {
        // Your function uses expect("unexpected NaN"), so we test that it actually panics
        let data = [1.0, f64::NAN, 2.0];
        ranks(&data);
    }
}
