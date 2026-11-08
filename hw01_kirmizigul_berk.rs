// Homework 1 — Vectors, slices, structs and methods
// Author: Berk Kirmizigul

// ---------------------------------------------------------------
// Part 1 — Vectors and slices
// ---------------------------------------------------------------

/// 1. Each element is the sum of all elements up to and including it.
fn running_sum(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(values.len());
    let mut sum = 0;
    for &v in values {
        sum += v;
        result.push(sum);
    }
    result
}

/// 2. Multiplies every element by 2, in place.
fn double_in_place(values: &mut [i32]) {
    for x in values.iter_mut() {
        *x *= 2;
    }
}

/// 3. Keeps the first occurrence of each value, preserving order.
fn remove_duplicates(values: &mut Vec<i32>) {
    let mut result: Vec<i32> = Vec::new();
    for &v in values.iter() {
        if !result.contains(&v) {
            result.push(v);
        }
    }
    *values = result;
}

/// 4. The largest element, or None for an empty slice.
fn largest(values: &[i32]) -> Option<i32> {
    let mut iter = values.iter();
    let mut max = *iter.next()?;
    for &v in iter {
        if v > max {
            max = v;
        }
    }
    Some(max)
}

// ---------------------------------------------------------------
// Part 2 — Structs and methods
// ---------------------------------------------------------------

// 7. What each derive gives us:
//
// - Debug:     lets us print a Point with {:?} (e.g. in println! or
//              in assert_eq! failure messages).
// - Clone:     adds the .clone() method, which makes an explicit copy.
// - Copy:      a Point is copied automatically (bit by bit) whenever it
//              is assigned or passed by value, instead of being moved.
//              This is allowed because both fields (f64) are Copy too.
//              Copy requires Clone, so Clone must also be derived.
// - PartialEq: lets us compare points with == and !=, which
//              assert_eq! also needs.
//
// What breaks if we remove Copy:
// Point becomes a "move" type, like String. Passing it by value or
// assigning it now moves ownership, so the original can no longer be used:
//
//     let p = Point::new(1.0, 2.0);
//     r.contains(p);   // p is moved into contains
//     r.contains(p);   // error[E0382]: use of moved value: `p`
//
// It also breaks reading a Point field out of a borrowed struct, e.g.
// `let tl = self.top_left;` inside a &self method gives
// error[E0507]: cannot move out of `self.top_left` which is behind a
// shared reference — we are only borrowing self, so we cannot take
// ownership of its field. In both cases we would have to write
// .clone() explicitly or pass &Point instead.
// (This file itself still compiles without Copy, because every Point
// passed by value is used only once — the errors above appear as soon
// as a point is reused after being passed or moved.)
//
// Why: without Copy, Rust's ownership rules say every value has exactly
// one owner, and passing by value transfers that ownership. Copy is the
// opt-in that says "this type is cheap and safe to duplicate silently".

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// 5. The point (0, 0).
    const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

    /// 5. Associated function (constructor).
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// 6. Euclidean distance between two points.
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {
    /// 8. Constructor.
    fn new(top_left: Point, width: f64, height: f64) -> Self {
        Rectangle { top_left, width, height }
    }

    /// 8. A rectangle whose width and height are both `size`.
    fn square(top_left: Point, size: f64) -> Self {
        Rectangle::new(top_left, size, size)
    }

    /// 8. Area = width * height.
    fn area(&self) -> f64 {
        self.width * self.height
    }

    /// 9. y grows downwards (like on a screen); edges count as inside.
    fn contains(&self, p: Point) -> bool {
        let left = self.top_left.x;
        let top = self.top_left.y;
        let right = left + self.width;
        let bottom = top + self.height;
        p.x >= left && p.x <= right && p.y >= top && p.y <= bottom
    }

    /// 10. Compares sizes only, not positions.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    /// 10. Multiplies width and height; top_left stays where it is.
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    // Part 1
    println!("running_sum([1, 2, 3, 4]) = {:?}", running_sum(&[1, 2, 3, 4]));

    let mut v = [1, -2, 3];
    double_in_place(&mut v);
    println!("double_in_place([1, -2, 3]) = {:?}", v);

    let mut d = vec![3, 1, 3, 2, 1];
    remove_duplicates(&mut d);
    println!("remove_duplicates([3, 1, 3, 2, 1]) = {:?}", d);

    println!("largest([3, 9, 2]) = {:?}", largest(&[3, 9, 2]));
    println!("largest([]) = {:?}", largest(&[]));
    println!();

    // Part 2
    let p = Point::new(3.0, 4.0);
    println!("{:?}.distance(ORIGIN) = {}", p, p.distance(&Point::ORIGIN));

    let mut r = Rectangle::new(Point::ORIGIN, 4.0, 3.0);
    let sq = Rectangle::square(Point::new(10.0, 10.0), 3.0);
    println!("r.area() = {}", r.area());
    println!("r.contains((1, 2)) = {}", r.contains(Point::new(1.0, 2.0)));
    println!("r.contains((5, 1)) = {}", r.contains(Point::new(5.0, 1.0)));
    println!("r.can_hold(&sq) = {}, sq.can_hold(&r) = {}", r.can_hold(&sq), sq.can_hold(&r));
    r.scale(2.0);
    println!("after r.scale(2.0): {:?}", r);
}

// ---------------------------------------------------------------
// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_sum(&[]), Vec::<i32>::new());
    }

    #[test]
    fn t2_double_in_place() {
        let mut v = [1, -2, 3];
        double_in_place(&mut v);
        assert_eq!(v, [2, -4, 6]);
    }

    #[test]
    fn t3_remove_duplicates() {
        let mut v = vec![3, 1, 3, 2, 1];
        remove_duplicates(&mut v);
        assert_eq!(v, vec![3, 1, 2]);
    }

    #[test]
    fn t4_largest() {
        assert_eq!(largest(&[3, 9, 2]), Some(9));
        assert_eq!(largest(&[-5, -2]), Some(-2));
        assert_eq!(largest(&[]), None);
    }

    #[test]
    fn t5_point_new_origin() {
        assert_eq!(Point::new(1.0, 2.0), Point { x: 1.0, y: 2.0 });
        assert_eq!(Point::ORIGIN, Point::new(0.0, 0.0));
    }

    #[test]
    fn t6_distance() {
        assert_eq!(Point::new(3.0, 4.0).distance(&Point::ORIGIN), 5.0);
    }

    #[test]
    fn t8_rectangle() {
        assert_eq!(Rectangle::new(Point::ORIGIN, 4.0, 3.0).area(), 12.0);
        let sq = Rectangle::square(Point::ORIGIN, 3.0);
        assert_eq!((sq.width, sq.height), (3.0, 3.0));
    }

    #[test]
    fn t9_contains() {
        let r = Rectangle::new(Point::ORIGIN, 4.0, 3.0);
        assert!(r.contains(Point::new(1.0, 2.0)));
        assert!(r.contains(Point::new(4.0, 3.0)));
        assert!(!r.contains(Point::new(5.0, 1.0)));
        assert!(!r.contains(Point::new(1.0, -1.0)));
    }

    #[test]
    fn t10_can_hold_and_scale() {
        let mut r = Rectangle::new(Point::ORIGIN, 4.0, 3.0);
        let sq = Rectangle::square(Point::new(10.0, 10.0), 3.0);
        assert!(r.can_hold(&sq));
        assert!(!sq.can_hold(&r));
        r.scale(2.0);
        assert_eq!(r.width, 8.0);
        assert_eq!(r.height, 6.0);
        assert_eq!(r.top_left, Point::ORIGIN);
    }
}
