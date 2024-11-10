use chrono::Datelike;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub(super) fn format_date_html(date: &chrono::NaiveDate) -> String {
    let day = date.day();
    format!(
        "{day}<sup>{}</sup> {}, {}",
        ordinal_suffix(day),
        MONTHS[date.month0() as usize],
        date.year()
    )
}

fn ordinal_suffix(n: u32) -> &'static str {
    let last = n % 10;
    let last_two = n % 100;

    match (last, last_two) {
        (_, 11 | 12 | 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        (_, _) => "th",
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn suffix() {
        #[track_caller]
        fn check(n: u32, s: &str) {
            assert_eq!(format!("{n}{}", ordinal_suffix(n)), s);
        }

        check(0, "0th");
        check(1, "1st");
        check(2, "2nd");
        check(3, "3rd");
        check(4, "4th");
        check(5, "5th");
        check(6, "6th");
        check(7, "7th");
        check(8, "8th");
        check(9, "9th");

        check(10, "10th");
        check(11, "11th");
        check(12, "12th");
        check(13, "13th");
        check(14, "14th");
        check(15, "15th");
        check(16, "16th");
        check(17, "17th");
        check(18, "18th");
        check(19, "19th");

        check(20, "20th");
        check(21, "21st");
        check(22, "22nd");
        check(23, "23rd");
        check(24, "24th");
        check(25, "25th");
        check(26, "26th");
        check(27, "27th");
        check(28, "28th");
        check(29, "29th");
    }

    #[test]
    fn html() {
        assert_eq!(
            format_date_html(&NaiveDate::from_ymd_opt(2021, 10, 3).unwrap()),
            "3<sup>rd</sup> October, 2021"
        );
    }
}
