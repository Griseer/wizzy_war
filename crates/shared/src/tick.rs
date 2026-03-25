// shared/tick.rs
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(pub u32);


impl AddAssign<u32> for Tick {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

