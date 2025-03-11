use core::fmt;
use std::fmt::Display;

use chrono::{Datelike, NaiveDate, Duration};

pub fn retire_time(time: &str, tp: &str) -> String {
    format!("{}", calculate_retirement(time, get_personnel_type(tp)).unwrap())
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
    retire_date_from: i32,
    delay_ratio: f64,
}

struct RetirementResult {
    retirement_date: NaiveDate,
    retirement_age: f64,
    delay_months: u32,
}

impl fmt::Display for RetirementResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{:.2},{}", self.retirement_date, self.retirement_age, self.delay_months)
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
            retire_date_from: 1965,
            delay_ratio: 1.0 / 4.0,
        },
        PersonnelType::Female55 => RetirementPolicy {
            normal_retire_date: 55.0,
            retire_date_from: 1970,
            delay_ratio: 1.0 / 4.0,
        },
        PersonnelType::Female50 => RetirementPolicy {
            normal_retire_date: 50.0,
            retire_date_from:1975,
            delay_ratio: 1.0 / 2.0,
        },
    }
}

fn calculate_retirement(birth_str: &str, personnel: PersonnelType) -> Option<RetirementResult> {
    println!("{},{:?}", birth_str, &personnel);
    let birth_date = parse_date(birth_str)?;
    println!("{:?}", &birth_date.year_ce());
    let policy = get_policy(&personnel);

    let effective_month = (12.0 * (birth_date.year_ce().1 as i32 - policy.retire_date_from) as f64 + birth_date.month0() as f64 + 1.0) as f64 * policy.delay_ratio;

    let total_retire_months = (policy.normal_retire_date * 12.0).round() as i32 + effective_month as i32;

    let retire_year = birth_date.year_ce().1 as i32 + total_retire_months / 12;
    let retire_month = (birth_date.month0() as i32 + total_retire_months % 12) % 12 + 1;
    let retire_day = (birth_date.day0() + 1).min(days_in_month(retire_year, retire_month as u32));
    println!("{}-{}-{}", retire_year, retire_month, retire_day);
    let retirement_date = NaiveDate::from_ymd_opt(retire_year, retire_month as u32, retire_day);

    println!("{:?}", retirement_date);

    let duration = retirement_date?.signed_duration_since(birth_date);
    let age_years = duration.num_days() as f64 / 365.25;

    Some(RetirementResult{
        retirement_date: retirement_date?,
        retirement_age: age_years,
        delay_months: effective_month as u32,
    })
}

fn parse_date(s: &str) -> Option<NaiveDate> {
    let parts: Vec<&str> = s.split('-').collect();
    match parts.len() {
        2 => {
            let full_str = format!("{}-{}-01", parts[0], parts[1]);
            NaiveDate::parse_from_str(&full_str, "%Y-%m-%d").ok()
        },
        3 => NaiveDate::parse_from_str(s, "%Y-%m-%d").ok(),
        _ => None,
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => if is_leap_year(year) { 29 } else { 28 },
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