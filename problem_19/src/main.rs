use chrono::{NaiveDate, Datelike, Weekday};

fn main() {
    let mut sum = 0;
    for year in 1901..2001 {
        for day in 0..366 {
            if let Some(date) = NaiveDate::from_yo_opt(year, day){
                if date.weekday() == Weekday::Sun && date.day() == 1 {
                    sum += 1;
                }
            }
        }
    }
    println!("{}", sum);
}
