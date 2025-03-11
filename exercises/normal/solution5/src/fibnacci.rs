
pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut result = 0;
    let mut fib_n = 0;
    for i in 0.. {
        fib_n = fib(i);
        if fib_n < threshold {
            if fib_n % 2 > 0 {
                result += fib_n;
            }
        } else {
            break;
        }
    }

    result
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
