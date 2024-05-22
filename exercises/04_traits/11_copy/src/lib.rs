// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.
use std::ops::Add;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

// use std::ops::Add;
impl Add<WrappingU32> for WrappingU32 {
    type Output = WrappingU32;
    fn add(self, rhs: WrappingU32) -> Self {
        WrappingU32 {
            value: self.value + rhs.value
        }    
    }
}

impl Add<u32> for WrappingU32 {
    type Output = WrappingU32;
    fn add(self, rhs: u32) -> Self {
        WrappingU32 {
            value: self.value + rhs
        }
    }
}

impl Add<WrappingU32> for u32 {
    type Output = WrappingU32;
    fn add(self, rhs: WrappingU32) -> WrappingU32 {
        WrappingU32 {
            value: self + rhs.value
        }    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
