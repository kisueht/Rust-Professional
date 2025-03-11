pub fn goldbach_conjecture() -> String {
    let mut results = Vec::new();
    let mut n = 9;

    while results.len() < 2 {
        if n % 2 == 1 && !is_odd_num(n) {
            let max_k = (n as f64 / 2.0).sqrt() as u32;
            let mut is_valid = false;

            for k in 1..=max_k {
                let rem = n - 2 * k * k;
                if rem >= 2 && is_odd_num(rem) {
                    is_valid = true;
                    break;
                }
            }

            if !is_valid {
                results.push(n);
            }
        }

        n += 2;
    }

    format!("{},{}", results[0], results[1])
}

fn is_odd_num(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    let mut w = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }

    true
}
