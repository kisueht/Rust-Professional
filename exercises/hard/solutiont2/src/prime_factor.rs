use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;
use num_traits::{One, Zero, ToPrimitive};
use rand::{Rng, thread_rng};

pub fn find_max_prime_factor(mut number: u128) -> u128 {
    max_prime_factor(number.to_biguint().unwrap()).to_u128().unwrap()
}

fn max_prime_factor(mut number: BigUint) -> BigUint {
    let one = One::one();
    if number <= one {
        return number;
    }

    if miller_rabin(&number) {
        return number;
    }

    let factor = pollards_rho(&number);
    let other = &number / &factor;
    let max_fac1 = max_prime_factor(factor);
    let max_fac2 = max_prime_factor(other);
    if max_fac1 > max_fac2 {
        max_fac1
    } else {
        max_fac2
    }
}

fn miller_rabin(n: &BigUint) -> bool {
    let two = 2u32.to_biguint().unwrap();
    let one = One::one();

    if *n < two {
        return false;
    }

    if *n == two {
        return true;
    }

    if n.is_even() {
        return false;
    }

    let mut d: BigUint = n - &one;
    let mut s = 0;
    while d.is_even() {
        d /= &two;
        s += 1;
    }

    let small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    'outer: for &a in &small_primes {
        if n <= &a.to_biguint().unwrap() {
            continue;
        }
        let a_big = a.to_biguint().unwrap();
        let mut x = a_big.modpow(&d, n);
        if x == one || x == n - &one {
            continue;
        }
        for _ in 0..(s - 1) {
            x = x.modpow(&two, n);
            if x == n - &one {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn pollards_rho(n: &BigUint) -> BigUint {
    let one = One::one();
    let two = 2u32.to_biguint().unwrap();

    if n.is_even() {
        return two;
    }

    let mut rng = thread_rng();
    let c = rng.gen_biguint_range(&one, &n);
    let mut x = rng.gen_biguint_range(&one, &n);
    let mut y = x.clone();
    let mut d = one.clone();

    while d == one {
        x = (x.modpow(&two, n) + &c) % n;
        y = (y.modpow(&two, n) + &c) % n;
        y = (y.modpow(&two, n) + &c) % n;

        let diff = if x > y {
            &x - &y
        } else { &y - &x};
        d = diff.gcd(n);
        if d == *n {
            return pollards_rho(n);
        }
    }
    d
}
