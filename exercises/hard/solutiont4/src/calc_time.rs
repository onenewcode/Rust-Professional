use std::collections::{HashMap, HashSet};

fn is_leapyear(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn str_to_num(s: &str) -> u32 {
    let tmp = s.as_bytes();
    let mut number = 0;
    for num in tmp.iter() {
        number = number * 10 + (*num - b'0') as u32
    }
    number
}

fn date_to_week_day(year: u32, month: u32, day: u32) -> u32 {
    // 根据日期计算当天是周几
    let c = (year / 100) as i32; // 20
    let mut y = (year % 100) as i32; // 25
    let mut m: i32 = 0;
    let d: i32 = day as i32;
    if month == 1 || month == 2 {
        m = month as i32 + 12;
        y -= 1;
    } else {
        m = month as i32;
    };
    let mut h = (y + y/4 + c/4 - 2*c + (13*(m as i32 + 1)/5 + d - 1)) % 7;
    if h == 0 {
        h = 7;
    }

    h as u32
}

fn date_to_week_count(year: u32, month: u32, day: u32) -> u32 {
    let mut week_day = date_to_week_day(year, 1, 1); // 该年的1月1日是周几
    if week_day == 7 {
        week_day = 0;
    }
    let (days, _) = count_days(year, month, day); // 计算该年已经过了多少天

    let mut ans = 0;
    if week_day > 4 { // 1月1日所在的周是去年的最后一周
        if day <= 7-week_day+1 { // 是否是去年的最后一周
            ans = date_to_week_count(year-1, 12, 31);
        } else {
            ans = (days + week_day - 3) / 7 + 1;
        }
    } else {
        ans = (days + week_day - 2) / 7 + 1;
    }

    if month == 12 {
        let cur_year_last_week_day = date_to_week_day(year, 12, 31);
        if cur_year_last_week_day < 4 && 31-day < cur_year_last_week_day { // 是否是明年的第一周
            1
        } else {
            ans
        }
    } else {
        ans
    }
}

fn count_days(year: u32, month: u32, day: u32) -> (u32, u32) {
    let mut ans_0 = 0;
    let mut ans_1 = 0;
    let mut month2days: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leapyear(year) {
        month2days[2] = 29;
    }
    let is_leapyear = is_leapyear(year);
    for m in 1..month {
        ans_0 += month2days[m as usize];
    }
    ans_0 += day;

    if is_leapyear {
        ans_1 = 366 - ans_0;
    } else {
        ans_1 = 365 - ans_0;
    }

    (ans_0, ans_1)
}

fn compute_diff_of_date(year0: u32, month0: u32, day0: u32, year1: u32, month1: u32, day1: u32) -> u32 {
    if year0 == year1 {
        let (days_0, _) = count_days(year0, month0, day0);
        let (days_1, _) = count_days(year1, month1, day1);
        days_0 - days_1
    } else {
        let (days_0, _) = count_days(year0, month0, day0);
        let (_, days_1) = count_days(year1, month1, day1);
        days_0 + days_1
    }
}

fn date_to_newyear(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> u32 {
    let mut newyear_year = year;
    let mut newyear_month = 0;
    let mut newyear_day = 0;
    (newyear_month, newyear_day) = *date2newyear.get(&year).expect("date_to_newyear error");
    if month > newyear_month || (month == newyear_month && day > newyear_day) {
        (newyear_month, newyear_day) = *date2newyear.get(&(year+1)).expect("date_to_newyear error");
        newyear_year = year + 1;
    }
    compute_diff_of_date(newyear_year, newyear_month, newyear_day,
                         year, month, day)
}

fn next_day(year: u32, month: u32, day: u32) -> (u32, u32, u32) {
    let mut month2days: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leapyear(year) {
        month2days[2] = 29;
    }
    if month2days[month as usize] == day { // 该月的最后一天
        if month == 12 { // 该年的最后一天
            (year+1, 1, 1)
        } else {
            (year, month+1, 1)
        }
    } else {
        (year, month, day+1)
    }
}

fn is_holiday(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> bool {
    let mut month2days: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leapyear(year) {
        month2days[2] = 29;
    }
    let mut holidays: HashSet<(u32, u32)> = HashSet::new();

    // 元旦
    holidays.insert((1, 1));

    // 春节
    let (mut newyear_month, mut newyear_day) = *date2newyear.get(&year).expect("date_to_newyear error");
    if newyear_day != 1 {
        holidays.insert((newyear_month, newyear_day-1));
    } else {
        holidays.insert((newyear_month-1, month2days[newyear_month as usize - 1]));
    }
    for _ in 0..7 {
        holidays.insert((newyear_month, newyear_day));
        (_, newyear_month, newyear_day) = next_day(year, newyear_month, newyear_day);
    }

    // 劳动节
    holidays.insert((5, 1));
    holidays.insert((5, 2));
    holidays.insert((5, 3));
    holidays.insert((5, 4));
    holidays.insert((5, 5));
    if holidays.contains(&(month, day)) {
        true
    } else {
        let week_day = date_to_week_day(year, month, day);
        match week_day {
            1..=5 => false,
            _ => true,
        }
    }
}

fn date_to_trading_days(year: u32, month: u32, day: u32, date2newyear: &HashMap<u32, (u32, u32)>) -> u32 {
    let mut trading_day = next_day(year, month, day);
    while is_holiday(trading_day.0, trading_day.1, trading_day.2, date2newyear) {
        trading_day = next_day(trading_day.0, trading_day.1, trading_day.2);
    }
    compute_diff_of_date(trading_day.0, trading_day.1, trading_day.2, year, month, day) - 1
}

pub fn time_info(time: &str) -> String {
    let tmp: Vec<&str> = time.split('-').collect();
    let year = str_to_num(tmp[0]);
    let month = str_to_num(tmp[1]);
    let day = str_to_num(tmp[2]);

    let week_count = date_to_week_count(year, month, day);
    let week_day = date_to_week_day(year, month, day);

    let (days_0, days_1) = count_days(year, month, day);

    let mut date2newyear = HashMap::new();
    date2newyear.insert(2025, (1, 29));
    date2newyear.insert(2026, (2, 17));
    let days_2 = date_to_newyear(year, month, day, &date2newyear);

    let days_3 = date_to_trading_days(year, month, day, &date2newyear);

    format!("{},{},{},{},{},{}", week_count, week_day, days_0, days_1, days_2, days_3)
}