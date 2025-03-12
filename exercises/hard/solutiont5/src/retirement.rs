use core::fmt;
use std::{fmt::{format, Display}, result};

use chrono::{Datelike, Duration, NaiveDate};

pub fn retire_time(time: &str, tp: &str) -> String {
    format!(
        "{}",
        calculate_retirement(time, get_personnel_type(tp)).unwrap()
    )
}

#[derive(Debug)]
enum PersonnelType {
    MaleGeneral,
    Female55,
    Female50,
}

#[derive(Debug)]
struct RetirementPolicy {
    normal_retire_date: f64,
    new_retire_date: f64,
    delay_ratio: u32,
}

struct RetirementResult {
    retirement_date: String,
    retirement_age: String,
    delay_months: u32,
}

impl fmt::Display for RetirementResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.retirement_date, self.retirement_age, self.delay_months
        )
    }
}

fn get_personnel_type(tp: &str) -> PersonnelType {
    match tp {
        "男职工" => PersonnelType::MaleGeneral,
        "女职工" => PersonnelType::Female55,
        "原法定退休年龄55周岁女职工" => PersonnelType::Female55,
        _ => PersonnelType::Female50,
    }
}

fn get_policy(personnel: &PersonnelType) -> RetirementPolicy {
    match personnel {
        PersonnelType::MaleGeneral => RetirementPolicy {
            normal_retire_date: 60.0,
            new_retire_date: 63.0,
            delay_ratio: 4,
        },
        PersonnelType::Female55 => RetirementPolicy {
            normal_retire_date: 55.0,
            new_retire_date: 58.0,
            delay_ratio: 4,
        },
        PersonnelType::Female50 => RetirementPolicy {
            normal_retire_date: 50.0,
            new_retire_date: 55.0,
            delay_ratio: 2,
        },
    }
}

fn calculate_retirement(birth_str: &str, personnel: PersonnelType) -> Option<RetirementResult> {
    let birth_date = parse_date(birth_str)?;
    let policy = get_policy(&personnel);

    let orig_retire_months = (policy.normal_retire_date * 12.0).round() as i32;
    let orig_retire_date = add_months(birth_date, orig_retire_months);

    // 政策开始日期
    let policy_start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

    let mut effective_month = 0;
    if orig_retire_date >= policy_start {
        let months_diff = (orig_retire_date.year_ce().1 - policy_start.year_ce().1) * 12
            + (orig_retire_date.month0() - policy_start.month0());
        effective_month = months_diff / policy.delay_ratio + 1;

        let max_extra =
            ((policy.new_retire_date - policy.normal_retire_date) * 12.0).round() as u32;
        if effective_month > max_extra {
            effective_month = max_extra;
        }
    }

    let total_retire_months = orig_retire_months + effective_month as i32;
    let retirement_date = add_months(birth_date, total_retire_months);

    let duration = retirement_date.signed_duration_since(birth_date);
    let age_years = duration.num_days() as f64 / 365.25;
    // println!("{}", age_years.round());

    Some(RetirementResult {
        retirement_date: format!(
            "{}-{}",
            retirement_date.year_ce().1,
            fmt_month(retirement_date.month0() + 1)
        ),
        retirement_age: process_float_num(age_years),
        delay_months: effective_month,
    })
}

fn fmt_month(months: u32) -> String {
    let mut result = format!("{}", months);
    if result.len() < 2 {
        result = format!("0{}", months);
    }

    result
}

fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let total_months = (date.year_ce().1 * 12) as i32 + date.month0() as i32 + months;
    let new_year = total_months / 12;
    let new_month = (total_months % 12) + 1;
    let new_day = (date.day0() + 1).min(days_in_month(new_year, new_month as u32));
    NaiveDate::from_ymd_opt(new_year, new_month as u32, new_day).unwrap()
}

fn process_float_num(num: f64) -> String {
    let rounded_num = (num * 100.0).round() / 100.0;
    if rounded_num == num.floor() || rounded_num == num.round() {
        // 检查是否为整数（无小数部分）
        format!("{}", rounded_num)
    } else {
        format!("{:.2}", num) // 否则保留两位小数
    }
}

fn parse_date(s: &str) -> Option<NaiveDate> {
    let parts: Vec<&str> = s.split('-').collect();
    match parts.len() {
        2 => {
            let full_str = format!("{}-{}-01", parts[0], parts[1]);
            NaiveDate::parse_from_str(&full_str, "%Y-%m-%d").ok()
        }
        3 => NaiveDate::parse_from_str(s, "%Y-%m-%d").ok(),
        _ => None,
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
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
        _ => 30,
    }
}

// 判断是否闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
