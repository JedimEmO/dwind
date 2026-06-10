use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A simple civil calendar date (proleptic Gregorian), used by the date
/// picker. Serializes to/from ISO-8601 (`YYYY-MM-DD`).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct CalendarDate {
    pub year: i32,
    /// 1-12
    pub month: u32,
    /// 1-31
    pub day: u32,
}

impl CalendarDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    /// Day of week with 0 = Monday .. 6 = Sunday (Sakamoto's method)
    pub fn day_of_week(self) -> u32 {
        const OFFSETS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

        let mut y = self.year;

        if self.month < 3 {
            y -= 1;
        }

        let sunday_based =
            (y + y / 4 - y / 100 + y / 400 + OFFSETS[(self.month - 1) as usize] + self.day as i32)
                .rem_euclid(7) as u32;

        // convert 0=Sunday to 0=Monday
        (sunday_based + 6) % 7
    }

    pub fn first_of_month(self) -> Self {
        Self::new(self.year, self.month, 1)
    }

    pub fn prev_month(year: i32, month: u32) -> (i32, u32) {
        if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        }
    }

    pub fn next_month(year: i32, month: u32) -> (i32, u32) {
        if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        }
    }

    /// Adds (or subtracts) whole days
    pub fn add_days(self, days: i32) -> Self {
        let mut year = self.year;
        let mut month = self.month;
        let mut day = self.day as i32 + days;

        while day < 1 {
            let (py, pm) = Self::prev_month(year, month);
            year = py;
            month = pm;
            day += Self::days_in_month(year, month) as i32;
        }

        while day > Self::days_in_month(year, month) as i32 {
            day -= Self::days_in_month(year, month) as i32;
            let (ny, nm) = Self::next_month(year, month);
            year = ny;
            month = nm;
        }

        Self::new(year, month, day as u32)
    }

    /// Moves by whole months, clamping the day to the target month's length
    pub fn add_months(self, months: i32) -> Self {
        let total = self.year * 12 + (self.month as i32 - 1) + months;
        let year = total.div_euclid(12);
        let month = (total.rem_euclid(12) + 1) as u32;
        let day = self.day.min(Self::days_in_month(year, month));

        Self::new(year, month, day)
    }

    pub fn month_name(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "",
        }
    }

    /// Today's date in the browser's local timezone. Falls back to the Unix
    /// epoch when no JS environment is available (e.g. native tests).
    pub fn today() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let now = web_sys::js_sys::Date::new_0();

            Self::new(
                now.get_full_year() as i32,
                now.get_month() + 1,
                now.get_date(),
            )
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::new(1970, 1, 1)
        }
    }
}

impl Display for CalendarDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarDateParseError;

impl Display for CalendarDateParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected an ISO-8601 date (YYYY-MM-DD)")
    }
}

impl FromStr for CalendarDate {
    type Err = CalendarDateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().splitn(3, '-');

        let year = parts
            .next()
            .and_then(|v| v.parse::<i32>().ok())
            .ok_or(CalendarDateParseError)?;
        let month = parts
            .next()
            .and_then(|v| v.parse::<u32>().ok())
            .ok_or(CalendarDateParseError)?;
        let day = parts
            .next()
            .and_then(|v| v.parse::<u32>().ok())
            .ok_or(CalendarDateParseError)?;

        if !(1..=12).contains(&month) || day < 1 || day > Self::days_in_month(year, month) {
            return Err(CalendarDateParseError);
        }

        Ok(Self::new(year, month, day))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        assert!(CalendarDate::is_leap_year(2024));
        assert!(CalendarDate::is_leap_year(2000));
        assert!(!CalendarDate::is_leap_year(1900));
        assert!(!CalendarDate::is_leap_year(2026));
    }

    #[test]
    fn month_lengths() {
        assert_eq!(CalendarDate::days_in_month(2026, 6), 30);
        assert_eq!(CalendarDate::days_in_month(2026, 7), 31);
        assert_eq!(CalendarDate::days_in_month(2024, 2), 29);
        assert_eq!(CalendarDate::days_in_month(2026, 2), 28);
    }

    #[test]
    fn weekdays() {
        // 2026-06-10 is a Wednesday (0=Mon => 2)
        assert_eq!(CalendarDate::new(2026, 6, 10).day_of_week(), 2);
        // 2024-02-29 was a Thursday
        assert_eq!(CalendarDate::new(2024, 2, 29).day_of_week(), 3);
        // 2000-01-01 was a Saturday
        assert_eq!(CalendarDate::new(2000, 1, 1).day_of_week(), 5);
    }

    #[test]
    fn day_arithmetic_crosses_boundaries() {
        let d = CalendarDate::new(2026, 1, 31).add_days(1);
        assert_eq!(d, CalendarDate::new(2026, 2, 1));

        let d = CalendarDate::new(2026, 1, 1).add_days(-1);
        assert_eq!(d, CalendarDate::new(2025, 12, 31));

        let d = CalendarDate::new(2024, 2, 28).add_days(7);
        assert_eq!(d, CalendarDate::new(2024, 3, 6));
    }

    #[test]
    fn month_arithmetic_clamps_days() {
        let d = CalendarDate::new(2026, 1, 31).add_months(1);
        assert_eq!(d, CalendarDate::new(2026, 2, 28));

        let d = CalendarDate::new(2026, 1, 15).add_months(-2);
        assert_eq!(d, CalendarDate::new(2025, 11, 15));

        let d = CalendarDate::new(2026, 12, 5).add_months(1);
        assert_eq!(d, CalendarDate::new(2027, 1, 5));
    }

    #[test]
    fn iso_roundtrip() {
        let d: CalendarDate = "2026-06-10".parse().unwrap();
        assert_eq!(d, CalendarDate::new(2026, 6, 10));
        assert_eq!(d.to_string(), "2026-06-10");

        assert!("2026-13-01".parse::<CalendarDate>().is_err());
        assert!("2026-02-29".parse::<CalendarDate>().is_err());
        assert!("not-a-date".parse::<CalendarDate>().is_err());
    }
}
