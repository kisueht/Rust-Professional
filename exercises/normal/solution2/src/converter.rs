use std::collections::{HashMap, VecDeque};

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑

    let num = split_num_str(num_str);
    let from_base: u32 = (*num.1).parse().unwrap();

    if from_base == to_base {
        return num.0.to_string();
    }

    let decimal = convert_x_10(num.0, from_base);
    convert_10_base(decimal, to_base)
}

fn split_num_str(num_str: &str) -> (&str, &str) {
    let result: Vec<&str> = num_str.split('(').map(|s| s.trim_matches(')')).collect();

    (result[0], result[1])
}

fn convert_x_10(nums: &str, from_base: u32) -> u32 {
    if 10 == from_base {
        return nums.parse::<u32>().unwrap();
    }

    let mut hashmap = HashMap::new();
    for (i, c) in "0123456789abcdef".chars().enumerate() {
        hashmap.insert(c, i as u32);
    }

    let mut decimal: u32 = 0;
    for n in nums.chars() {
        let rem = hashmap.get(&n.to_ascii_lowercase()).unwrap();
        if *rem >= from_base {
            return 0;
        }

        decimal = decimal
            .checked_mul(from_base)
            .and_then(|v| v.checked_add(*rem))
            .ok_or("number overflow!!!")
            .unwrap();
    }

    decimal
}

/**
 * 10进制转换2~16进制
 */
fn convert_10_base(mut dec_num: u32, to_base: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    let mut rem_stack = VecDeque::new();

    while dec_num > 0 {
        rem_stack.push_front(dec_num % to_base);
        dec_num /= to_base as u32;
    }

    let mut result_str = String::new();
    for re in rem_stack {
        result_str.push(digits[re as usize]);
    }

    result_str
}
