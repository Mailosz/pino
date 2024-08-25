use std::ops;

pub struct Point{
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn new(x:f64, y:f64) -> Point {
        Point{x:x,y:y}
    }


    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn distance(&self, point : &Point) -> f64 {
        let x = self.x - point.x;
        let y = self.y - point.y;
        f64::sqrt(x * x + y * y)
    }

    /**
     * Direction in radians from 0,0 to this point
     */
    pub fn direction(&self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    /**
     * Direction in radians between two points
     */
    pub fn direction_to(&self, point : Point) -> f64 {
        f64::atan2(point.y - self.y, point.y - self.x)
    }

    /**
     * Returns a Point with the same direction, but length equal to 1.0
     */
    pub fn normalized(&self) -> Point {
        let len = self.length();
        Point::new(self.x / len, self.y / len)
    }

}

impl ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, p: Point) -> Point {
        Point::new(self.x + p.x, self.y + p.y)
    }
}

impl ops::Sub for Point {
    type Output = Self;
    
    fn sub(self, p: Self) -> Self::Output {
        Point::new(self.x - p.x, self.y - p.y)
    }
}

impl ops::Div for Point {
    type Output = Self;

    fn div(self, p: Self) -> Self::Output {
        Point::new(self.x / p.x, self.y / p.y)
    }
}

impl ops::Mul for Point {
    type Output = Self;

    fn mul(self, p: Self) -> Self::Output {
        Point::new(self.x * p.x, self.y * p.y)
    }
}

impl ops::Rem for Point {
    type Output = Self;
    
    fn rem(self, p: Self) -> Self::Output {
        Point::new(self.x % p.x, self.y % p.y)
    }
}

impl ops::Add<f64> for Point {
    type Output = Self;

    fn add(self, n: f64) -> Point {
        Point::new(self.x + n, self.y + n)
    }
}

impl ops::Sub<f64> for Point {
    type Output = Self;
    
    fn sub(self, n: f64) -> Self::Output {
        Point::new(self.x - n, self.y - n)
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, n: f64) -> Self::Output {
        Point::new(self.x / n, self.y / n)
    }
}

impl ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, n: f64) -> Self::Output {
        Point::new(self.x * n, self.y * n)
    }
}

impl ops::Rem<f64> for Point {
    type Output = Self;
    
    fn rem(self, n: f64) -> Self::Output {
        Point::new(self.x % n, self.y % n)
    }
}