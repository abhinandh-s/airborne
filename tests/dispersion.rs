use airborne::{DataSet, Dispersion};

#[test]
fn name() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let dataset: DataSet<i32> = DataSet::from_iter(data);
    let _v = dataset.variance().unwrap();
    //    assert_eq!(v, N!(8.25));
}
