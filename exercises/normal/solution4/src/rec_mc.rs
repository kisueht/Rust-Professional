pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    if amount == 0 {
        return 0;
    }

    let coins = vec![100, 50, 30, 20, 10, 5, 2, 1];

    let mut count = 0;
    let mut result = amount;
    for coin in coins {
        if result >= coin {
            count += result / coin;

            let rem: u32 = result % coin;
            if rem > 0 {
                result = rem;
            } else {
                break;
            }
        }
    }

    count
}
