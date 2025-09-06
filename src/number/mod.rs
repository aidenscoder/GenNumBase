use core::fmt;
use std::ops::*;

/// Constructs a IterNum object, 
/// that acts like a resizable order of list.
pub struct IterNum {
    value:f64,
    count:i64
}

fn add<T:Add>(x:T,y:T) -> <T as Add>::Output {
    x+y
}

fn sub<T:Sub>(x:T,y:T) -> <T as Sub>::Output {
    x-y
}

fn div<T:Div>(x:T,y:T) -> <T as Div>::Output {
    x/y
}

fn mul<T:Mul>(x:T,y:T) -> <T as Mul>::Output {
    x*y
}

impl IterNum {
    pub fn new(value:f64) -> Self {
        Self {value:value, count:0i64}
    }

    pub fn as_vec(self) -> Vec<i64> {
        let mut vector:Vec<i64> = vec![];
        for i in self {
            vector.push(i);
        }
        vector
    }
}

/// Yields a set of number, always greater or equal to than 0. 
impl Iterator for IterNum {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.value.floor() as i64 {
            Some(self.count)
        }
        else {
            self.count = 0;
            None
        }
    }
}

/// Preforms Addition on the IterNum value.
impl Add for IterNum {
    type Output = IterNum;
    fn add(self, rhs: Self) -> Self::Output {
        let value = add(self.value,rhs.value);
        Self {value:value, count:0i64}
    }
}

/// Preforms Subtraction on the IterNum value.
impl Sub for IterNum {
    type Output = IterNum;
    fn sub(self, rhs: Self) -> Self::Output {
        let value = sub(self.value,rhs.value);
        Self {value:value, count:0i64}
    }
}

/// Preforms Division on the IterNum value.
impl Div for IterNum {
    type Output = IterNum;
    fn div(self, rhs: Self) -> Self::Output {
        let value = div(self.value,rhs.value);
        Self {value:value, count:0i64}
    }
}

/// Preforms Multiplication on the IterNum value.
impl Mul for IterNum {
    type Output = IterNum;
    fn mul(self, rhs: Self) -> Self::Output {
        let value = mul(self.value,rhs.value);
        Self {value:value, count:0i64}
    }
}

/// Preforms equality checks
impl PartialEq for IterNum {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}


/// Preforms greater than, lesser than
/// greater or equal, and lesser or equal
impl PartialOrd for IterNum {
    fn ge(&self, other: &Self) -> bool {
        self.value > other.value
    }
    fn gt(&self, other: &Self) -> bool {
        self.value >= other.value
    }
    fn le(&self, other: &Self) -> bool {
        self.value < other.value
    }
    fn lt(&self, other: &Self) -> bool {
        self.value <= other.value
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

/// Preforms Addition and then assigns it to the value.
impl AddAssign for IterNum {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

/// Preforms Subtraction and then assigns it to the value.
impl SubAssign for IterNum {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

/// Preforms Multiplication and then assigns it to the value.
impl MulAssign for IterNum {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

/// Preforms Division and then assigns it to the value.
impl DivAssign for IterNum {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

/// Flip the value of the number, if positive, 
/// it will become negatitve, and vise versa.
impl Neg for IterNum {
    type Output = IterNum;
    fn neg(self) -> Self::Output {
        Self {value:-self.value,count:0}
    }
}

impl fmt::Display for IterNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.value)
    }
}

#[macro_export]
macro_rules! num {
    ($x:literal) => {
        crate::number::IterNum::new($x as f64)
    };
}

