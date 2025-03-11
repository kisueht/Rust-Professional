pub fn time_info(time: &str) -> String {
    print!("time:{}---", time);
    match parse_date(time) {
        Some((year, month, day)) => {
            let day_of_year = calc_day_of_year(year, month, day);
            let mut week_number = (day_of_year / 7) + 1;
            if 52 < week_number {
                week_number = 1;
            }
            let wd = weekdy(year, month, day);
            let weekday_str = match wd {
                0 => "7",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                _ => "未知",
            };

            let total_days = if is_leap_year(year) { 366 } else { 365 };
            let remaining_days = total_days - day_of_year;
            let days_until_cny = days_until_chinise_new_year(year, month, day);
            let dasys_until_trading = days_until_next_trading_day(year, month, day);

            let output = format!("{},{},{},{},{},{}", week_number, weekday_str, day_of_year, remaining_days, days_until_cny, dasys_until_trading);
            println!("{}", output);
            return output;
        },
        None => {
            format!("无效的时间格式！")
        }
    }
}

fn parse_date(s: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = s.split("-").collect();
    if parts.len() != 3 {
        return None;
    }

    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let day = parts[2].parse::<u32>().ok()?;
    Some((year, month, day))
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => if is_leap_year(year) {
            29
        } else {
            28
        },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

fn calc_day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let mut doy = 0;
    for m in 1..month {
        doy += days_in_month(year, m);
    }
    doy + day
}

fn weekdy(year: i32, month: u32, day: u32) -> u32 {
    let t = [7, 3, 2, 5, 7, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    let m = month;
    if m < 3 {
        y -= 1;
    }
    ((y + y/4 - y/100 + y/400 + t[(m - 1) as usize] + day as i32) % 7) as u32
}

fn next_day(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    let dim = days_in_month(year, month);
    if day < dim {
        (year, month, day + 1)
    } else if month < 12 {
        (year, month + 1, 1)
    } else {
        (year + 1, 1, 1)
    }
}

fn get_chinese_new_year(year: i32) -> Option<(i32, u32, u32)> {
    let table = [
        (2000, 2, 5),
        (2001, 1, 24),
        (2002, 2, 12),
        (2003, 2, 1),
        (2004, 1, 22),
        (2005, 2, 9),
        (2006, 1, 29),
        (2007, 2, 18),
        (2008, 2, 7),
        (2009, 1, 26),
        (2010, 2, 14),
        (2011, 2, 3),
        (2012, 1, 23),
        (2013, 2, 10),
        (2014, 1, 31),
        (2015, 2, 19),
        (2016, 2, 8),
        (2017, 1, 28),
        (2018, 2, 16),
        (2019, 2, 5),
        (2020, 1, 25),
        (2021, 2, 12),
        (2022, 2, 1),
        (2023, 1, 22),
        (2024, 2, 10),
        (2025, 1, 29),
        (2026, 2, 17),
    ];

    for &(y, m, d) in &table {
        if y == year {
            return Some((y, m , d));
        }
    }

    None
}

fn days_until_chinise_new_year(year: i32, month: u32, day: u32) -> u32 {
    let current_doy = calc_day_of_year(year, month, day);
    if let Some((_, cny_month, cny_day)) = get_chinese_new_year(year) {
        let cny_doy = calc_day_of_year(year, cny_month, cny_day);
        if current_doy < cny_doy {
            return cny_doy - current_doy;
        }
    }

    let next_year = year + 1;
    if let Some((_, cny_month, cny_day)) = get_chinese_new_year(next_year) {
        let total = if is_leap_year(year) {
            366
        } else {
            365
        };
        let remaining = total - current_doy;
        let next_cny_doy = calc_day_of_year(next_year, cny_month, cny_day);
        return remaining + next_cny_doy;
    }

    0
}

fn days_until_next_trading_day(year: i32, month: u32, day: u32) -> u32 {
    let mut count = 0;
    let mut y = year;
    let mut m = month;
    let mut d = day;

    loop {
        let (ny, nm, nd) = next_day(y, m, d);
        count += 1;
        y = ny;
        m = nm;
        d = nd;
        let wd = weekdy(y, m, d);
        if wd >= 1 && wd <= 5 && !is_holiday(y, m, d) {
            break;
        }
    }

    count - 1
}

fn is_holiday(year: i32, month: u32, day: u32) -> bool {
    let date_str = format!("{:04}-{:02}-{:02}", year, month, day);

    let holidays = [
        "2025-01-01",                   // 元旦
        "2025-01-28", "2025-01-29", "2025-01-30", "2025-01-31", "2025-02-03", "2025-02-04", // 春节假期
        "2025-04-04",                   // 清明节（示例）
        "2025-05-01", "2025-05-02", "2025-05-05", // 劳动节假期
        "2025-06-02",                   // 端午节（示例）
        "2025-10-01", "2025-10-02", "2025-10-03", "2025-10-06", "2025-10-07", "2025-10-08", // 国庆假期
        "2026-01-01",                   // 元旦
    ];

    holidays.contains(&date_str.as_str())
}