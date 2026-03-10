mod private {
    pub trait Sealed {}
}

pub trait Marker: private::Sealed + std::fmt::Debug + Clone + Copy + Send + Sync + 'static {
    const DOF_OFFSET: usize;
    const NAME: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sample;

impl private::Sealed for Sample {}

impl Marker for Sample {
    const DOF_OFFSET: usize = 1;
    const NAME: &'static str = "sample";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Population;

impl private::Sealed for Population {}

impl Marker for Population {
    const DOF_OFFSET: usize = 0;
    const NAME: &'static str = "population";
}
