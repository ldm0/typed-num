#[cfg(feature = "bincode")]
pub mod bincode;
#[cfg(feature = "serde")]
pub mod serde;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Num<const N: i64>;
