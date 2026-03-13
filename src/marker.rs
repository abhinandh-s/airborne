mod private {
    pub trait Sealed {}
}

/// Compile-time marker encoding **sample** or **population** semantics.
///
/// This trait is **sealed**: only [`Sample`] and [`Population`] implement it.
/// The single associated constant `DOF_OFFSET` drives the Bessel-correction
/// logic inside variance and covariance without any runtime dispatch.
///
/// | Marker       | `DOF_OFFSET` | Variance denominator |
/// |--------------|-------------|----------------------|
/// | `Sample`     | `1`         | `n − 1`              |
/// | `Population` | `0`         | `n`                  |
pub trait Marker: private::Sealed + std::fmt::Debug + Clone + Copy + Send + Sync + 'static {
    /// Subtracted from `n` when computing variance / covariance denominators.
    const DOF_OFFSET: usize;
    /// Human-readable label ("Sample" or "Population").
    const NAME: &'static str;
}

/// Marker indicating the dataset is a **sample** from a larger population.
///
/// Variance and covariance use Bessel's correction: divide by `n − 1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sample;

impl private::Sealed for Sample {}

impl Marker for Sample {
    const DOF_OFFSET: usize = 1;
    const NAME: &'static str = "Sample";
}

/// Marker indicating the dataset **is** the entire population.
///
/// Variance and covariance divide by `n` with no correction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Population;

impl private::Sealed for Population {}

impl Marker for Population {
    const DOF_OFFSET: usize = 0;
    const NAME: &'static str = "Population";
}
