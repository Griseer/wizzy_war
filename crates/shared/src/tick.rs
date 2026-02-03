// shared/tick.rs
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServerTick(pub u64);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputTick(pub u64);

impl AddAssign<u64> for ServerTick {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl AddAssign<u64> for InputTick {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}
