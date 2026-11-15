// Homework 4 — Either: our own version of Result
// Author: Berk Kirmizigul

/// Left holds the error, Right holds the value ("right" = correct).
#[derive(Debug, PartialEq)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

// ---------------------------------------------------------------
// 1. Methods of Either
// ---------------------------------------------------------------

impl<L, R> Either<L, R> {
    /// Puts a value in Right.
    fn new(x: R) -> Either<L, R> {
        Either::Right(x)
    }

    /// Applies `f` to the Right value; a Left is passed on unchanged.
    fn map<U>(self, f: fn(R) -> U) -> Either<L, U> {
        match self {
            Either::Left(e) => Either::Left(e),
            Either::Right(x) => Either::Right(f(x)),
        }
    }

    /// Like map, but `f` can fail itself, so it already returns an Either.
    /// Its result is returned as it is (not wrapped again).
    /// A Left is passed on unchanged, and `f` is not called.
    fn flat_map<U>(self, f: fn(R) -> Either<L, U>) -> Either<L, U> {
        match self {
            Either::Left(e) => Either::Left(e),
            Either::Right(x) => f(x),
        }
    }
}

// ---------------------------------------------------------------
// 2. divide and reciprocal
// ---------------------------------------------------------------

fn divide(a: f64, b: f64) -> Either<String, f64> {
    if b == 0.0 {
        // {} prints 8.0 as "8", which matches the expected message.
        Either::Left(format!("cannot divide {} by {}", a, b))
    } else {
        Either::new(a / b)
    }
}

/// 1 / x, computed using divide.
fn reciprocal(x: f64) -> Either<String, f64> {
    divide(1.0, x)
}

// ---------------------------------------------------------------
// 3. sqrt
// ---------------------------------------------------------------

fn sqrt(x: f64) -> Either<String, f64> {
    if x < 0.0 {
        Either::Left(format!("cannot take the square root of {}", x))
    } else {
        Either::new(x.sqrt())
    }
}

// ---------------------------------------------------------------
// Chaining with flat_map: the first step that fails decides the error
// ---------------------------------------------------------------

fn calculate(a: f64, b: f64) -> Either<String, f64> {
    divide(a, b).flat_map(reciprocal).flat_map(sqrt)
}

fn main() {
    let cases = [(1.0, 16.0), (8.0, 0.0), (0.0, 5.0), (-1.0, 4.0)];
    for (a, b) in cases {
        println!(
            "a = {:>2}, b = {:>2}  ->  {:?}",
            a,
            b,
            calculate(a, b)
        );
    }

    println!();
    // map example: double the value inside Right
    let doubled: Either<String, f64> = Either::new(21.0).map(|x| x * 2.0);
    println!("Either::new(21.0).map(|x| x * 2.0) = {:?}", doubled);
}

// ---------------------------------------------------------------
// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn left(s: &str) -> Either<String, f64> {
        Either::Left(s.to_string())
    }

    #[test]
    fn t1_new() {
        let e: Either<String, i32> = Either::new(5);
        assert_eq!(e, Either::Right(5));
    }

    #[test]
    fn t1_map() {
        let r: Either<String, i32> = Either::new(5);
        assert_eq!(r.map(|x| x * 2), Either::Right(10));

        let l: Either<String, i32> = Either::Left("error".to_string());
        assert_eq!(l.map(|x| x * 2), Either::Left("error".to_string()));
    }

    #[test]
    fn t1_map_changes_type() {
        let r: Either<String, i32> = Either::new(42);
        assert_eq!(r.map(|x| x.to_string()), Either::Right("42".to_string()));
    }

    #[test]
    fn t1_flat_map() {
        assert_eq!(Either::new(16.0).flat_map(sqrt), Either::Right(4.0));
        assert_eq!(
            Either::new(-4.0).flat_map(sqrt),
            left("cannot take the square root of -4")
        );
        assert_eq!(left("earlier error").flat_map(sqrt), left("earlier error"));
    }

    #[test]
    fn t2_divide_reciprocal() {
        assert_eq!(divide(8.0, 4.0), Either::Right(2.0));
        assert_eq!(divide(8.0, 0.0), left("cannot divide 8 by 0"));
        assert_eq!(reciprocal(4.0), Either::Right(0.25));
        assert_eq!(reciprocal(0.0), left("cannot divide 1 by 0"));
    }

    #[test]
    fn t3_sqrt() {
        assert_eq!(sqrt(16.0), Either::Right(4.0));
        assert_eq!(sqrt(-4.0), left("cannot take the square root of -4"));
    }

    #[test]
    fn chain_test_cases() {
        assert_eq!(calculate(1.0, 16.0), Either::Right(4.0));
        assert_eq!(calculate(8.0, 0.0), left("cannot divide 8 by 0"));
        assert_eq!(calculate(0.0, 5.0), left("cannot divide 1 by 0"));
        assert_eq!(calculate(-1.0, 4.0), left("cannot take the square root of -4"));
    }
}
