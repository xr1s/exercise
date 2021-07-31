use std::ops::AddAssign;

use chrono::{Datelike, Duration, NaiveDate, Weekday};

fn solve<D>(mut since: D, until: D) -> usize
where
    D: PartialEq + Datelike + AddAssign<Duration>,
{
    let mut answer = 0;
    while since != until {
        if since.day() == 1 && since.weekday() == Weekday::Sun {
            answer += 1;
        }
        since += Duration::days(1);
    }
    return answer;
}

fn main() {
    let since = NaiveDate::from_ymd(1901, 1, 1);
    let until = NaiveDate::from_ymd(2001, 1, 1);
    println!("{}", solve(since, until));
}
