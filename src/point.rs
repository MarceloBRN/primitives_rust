use std::ops::*;
use num::{Integer};

#[derive(Copy, Clone, Debug, Default)]
#[allow(unused_variables, dead_code)]
pub struct Point<T: Integer>{
    pub x: T,
    pub y: T,
}


#[allow(unused_variables, dead_code, non_snake_case)]
impl<T: Copy + Clone + Default + Integer> Point<T>{
    pub fn new(x: T, y: T) -> Self{
        Point::<T>{
            x: x,
            y: y,
        }
    }

    pub fn default() -> Self{
        Point::<T>{
            x: Default::default(),
            y: Default::default(),
        }
    }

    pub fn xy(&self) -> (T, T){
        (self.x, self.y)
    }

    pub fn is_zero(&self) -> bool {
        if self.x.is_zero() && self.y.is_zero() { true } else { false }
    }
}

impl From<Point<i32>> for Point<i64>{
    #[inline]
    fn from(point: Point<i32>) -> Point<i64>{
        Point::<i64> {
            x: point.x as i64,
            y: point.y as i64,
        }
    }
}

impl From<Point<i32>> for Point<i128>{
    #[inline]
    fn from(point: Point<i32>) -> Point<i128>{
        Point::<i128> {
            x: point.x as i128,
            y: point.y as i128,
        }
    }
}

impl From<Point<i64>> for Point<i32>{
    #[inline]
    fn from(point: Point<i64>) -> Point<i32>{
        Point::<i32> {
            x: point.x as i32,
            y: point.y as i32,
        }
    }
}

impl From<Point<i64>> for Point<i128>{
    #[inline]
    fn from(point: Point<i64>) -> Point<i128>{
        Point::<i128> {
            x: point.x as i128,
            y: point.y as i128,
        }
    }
}

impl From<Point<i128>> for Point<i32>{
    #[inline]
    fn from(point: Point<i128>) -> Point<i32>{
        Point::<i32> {
            x: point.x as i32,
            y: point.y as i32,
        }
    }
}

impl From<Point<i128>> for Point<i64>{
    #[inline]
    fn from(point: Point<i128>) -> Point<i64>{
        Point::<i64> {
            x: point.x as i64,
            y: point.y as i64,
        }
    }
}


impl <T: Copy + Clone + Add<Output=T> + Integer> Add for Point<T>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
} 

impl <T: Copy + Clone + Sub<Output=T> + Integer> Sub for Point<T>{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// impl <T: Copy + Clone + Integer + From<U>, U: Copy + Clone + Integer + Float> Mul<U> for Point<T>{
//     type Output = Self;
//     fn mul(self, rhs: U) -> Self {
//         Point{
//             x: self.x * T::from(rhs),
//             y: self.y * T::from(rhs),
//         }
//     }
// } 

impl <T: Copy + Clone + Integer + From<i32>> Mul<i32> for Point<T>{
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Point{
            x: self.x * T::from(rhs),
            y: self.y * T::from(rhs),
        }
    }
} 

impl <T: Copy + Clone + Integer + From<i64>> Mul<i64> for Point<T>{
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        Point{
            x: self.x * T::from(rhs),
            y: self.y * T::from(rhs),
        }
    }
} 

impl <T: Copy + Clone + Integer + From<i128>> Mul<i128> for Point<T> {
    type Output = Self;
    fn mul(self, rhs: i128) -> Self {
        Point{
            x: self.x * T::from(rhs),
            y: self.y * T::from(rhs),
        }
    }
} 

impl Mul<f32> for Point<i32> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Point{
            x: ((self.y as f32) * rhs) as i32,
            y: ((self.y as f32) * rhs) as i32,
        }
    }
} 

impl Mul<f64> for Point<i32>{
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Point{
            x: ((self.x as f64) * rhs) as i32,
            y: ((self.y as f64) * rhs) as i32,
        }
    }
} 

impl Mul<f64> for Point<i64>{
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Point{
            x: ((self.x as f64) * rhs) as i64,
            y: ((self.x as f64) * rhs) as i64,
        }
    }
}

impl Mul<f32> for Point<i64>{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Point{
            x: ((self.x as f32) * rhs) as i64,
            y: ((self.x as f32) * rhs) as i64,
        }
    }
}

impl Mul<f32> for Point<i128>{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Point{
            x: ((self.x as f32) * rhs) as i128,
            y: ((self.x as f32) * rhs) as i128,
        }
    }
} 

impl Mul<f64> for Point<i128>{
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Point{
            x: ((self.x as f64) * rhs) as i128,
            y: ((self.x as f64) * rhs) as i128,
        }
    }
} 

impl <T: Copy + Clone + Neg<Output=T> + Integer> Neg for Point<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::Point;

    #[allow(unused_mut, unused_variables)]
    #[test]
    fn point_test() {
        let p1 = Point::<i32>::new(100, 200);
        assert_eq!((100, 200), p1.xy());
        let p2 = Point::<i32>::default();
        assert_eq!((0, 0), p2.xy());
        let p3 = Point::<i64>::new(300, 400);
        assert_eq!((300, 400), p3.xy());
        let p4 = Point::<i64>::default();
        assert_eq!((0, 0), p4.xy());
        let p5 = Point::<i128>::new(10, 20);
        assert_eq!((10, 20), p5.xy());
        let p6 = Point::<i128>::default();
        assert_eq!((0, 0), p6.xy());

        let p2 = Point::<i64>::from(p1);
        assert_eq!((100, 200), p2.xy());
        let p4 = Point::<i128>::from(p3);
        assert_eq!((300, 400), p4.xy());
        let p6 = Point::<i128>::from(p1);
        assert_eq!((100, 200), p6.xy());

        let p2 = Point::<i32>::from(p5);
        assert_eq!((10, 20), p2.xy());
        let p4 = Point::<i32>::from(p5);
        assert_eq!((10, 20), p4.xy());
        let p6 = Point::<i64>::from(p5);
        assert_eq!((10, 20), p6.xy());

        let psum = p1 + Point::<i32>::from(p3) + Point::<i32>::from(p5);
        assert_eq!((410, 620), psum.xy());

        let psub = psum - p1 - Point::<i32>::from(p3) - Point::<i32>::from(p5);
        assert_eq!((0, 0), psub.xy());

        let pmulint = p1 * 100 + Point::<i32>::from(p3 * 100) + Point::<i32>::from(p5 * 100);
        assert_eq!((41000, 62000), pmulint.xy());

        let pmulfloat = Point::<i32>::from(p1 * 0.5) + Point::<i32>::from(p3 * 0.5) + Point::<i32>::from(p5 * 0.5);
        assert_eq!((205, 255), pmulfloat.xy());

        let pneg = -p1;
        assert_eq!((-100, -200), pneg.xy());
    }
}