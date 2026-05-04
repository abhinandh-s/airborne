## General Philosophy

### Handling Precision

Precision is handled via `type N` and `trait ComputeFloat` in `compute.rs`.
It is the only file that is allowed to use feature flag "precision". 

For all internal computation we must only use type `N` and methods provided by `ComputeFloat`.
`cf_` and `_n` will switch between `f64` and `rust_decimal::Decimal` accrodingly to the feature flag.

1. methods of `ComputeFloat` starts with `cf_{method_name}`
2. an only internal funtions are allowed to return `N` and funtion name should end with `_n`
    eg: `pub(crate) fn {some_function}_n() -> Result<N>;`

### Why `DataSet`, not generic funtions which takes any with `Numeric` trait

`DataSet` is the heart of this crate almost every methods is implemented on it. 
It is because, in statistics there are Sample and Population Data. The only diffrence is ...

api design gets kind of wacky cause of that.

```rust
// 1. a normal api 

fn sample_variance(data: &[f64]) -> f64;
fn population_variance(data: &[f64]) -> f64;

fn sample_covariance(data: &[f64]) -> f64;
fn population_covariance(data: &[f64]) -> f64;

// and so on...

// 2. with traits 

trait Sample {}
trait Population {}

// still lot of duplication for just one thing, 
// 
// Population divides by len() of data
// Sample divides by len() - 1 of data.
// 
```

```rust
// How DataSet handles this

let data = DataSet::new([1, 2, 3, 4, 5]).unwrap(); // compiler will yell to put Type in there.
// thats by design 

let data: DataSet<i32> = DataSet::new([1, 2, 3, 4, 5]).unwrap(); // means we use Population(Default) for entire operations

let data: DataSet<i32, Population> = DataSet::new([1, 2, 3, 4, 5]).unwrap(); // we can also explicitly state that 
let data: DataSet<i32, Sample> = DataSet::new([1, 2, 3, 4, 5]).unwrap(); // divides by .len() - 1 in every operations on this dataset as needed 

// we can also use type alias 
let data: SampleData<T> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
let data: PopulationData<T> = DataSet::new([1, 2, 3, 4, 5]).unwrap();

// now we can call methods on it. No need to set Sample or Population again.
let mu = data.mean();
let v = data.variance();
let cv = data.covariance();
```

> [!NOTE]
> trait methods availabe under `Vec`, `[T]` or any other type which impl Deref to slice, assumes its `Population`.

### test coverage

src/compute is excluded
