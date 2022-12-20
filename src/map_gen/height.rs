use std::ops::{Sub, Add};

use bevy::prelude::Component;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Height {
    pub value: usize,
}
impl From<Height> for usize {
    fn from(value: Height) -> Self {
        value.value
    }
}
impl From<Height> for u32 {
    fn from(value: Height) -> Self {
        value.value as u32
    }
}
impl From<Height> for f64 {
    fn from(value: Height) -> Self {
        value.value as f64
    }
}
impl From<Height> for f32 {
    fn from(value: Height) -> Self {
        value.value as f32
    }
}
impl Sub<Height> for Height {
    type Output = Height;

    fn sub(self, rhs: Height) -> Self::Output {
        let value = self.value - rhs.value;
        Height{value}
    }
}
impl Add<Height> for Height {
    type Output = Height;

    fn add(self, rhs: Height) -> Self::Output {
        let value = self.value + rhs.value;
        Height{value}
    }
}