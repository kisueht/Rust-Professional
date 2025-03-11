/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    if n == 0 {
        return 0;
    }

    let base_matrix = Matrix2x2 {
        a: 1,
        b: 1,
        c: 1,
        d: 0,
    };
    let powered_matrix = base_matrix.power(n - 1);
    powered_matrix.a
}



#[derive(Debug, Clone, Copy)]
struct Matrix2x2 {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Matrix2x2 {
    fn identity() -> Self {
        Matrix2x2 {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        }
    }

    fn multiply(self, other: Matrix2x2) -> Self {
        Matrix2x2 {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }

    fn power(self, mut exp: i32) -> Self {
        let mut result = Matrix2x2::identity();
        let mut base = self;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.multiply(base);
            }
            base = base.multiply(base);
            exp /= 2;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
