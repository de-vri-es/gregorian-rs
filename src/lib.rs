//! An implementation of the proleptic Gregorian calendar.
//! In this implementation, before the year 1 come year 0.
//! The library does not deal with times.
//!
//! The [`Date`] type represents a date (year, month and day),
//! the [`Year`] type represents a calendar year,
//! the [`Month`] type represents a calendar month,
//! and the [`YearMonth`] type represents a month of a specific year.
//!
//! # Example
//! ```
//! use gregorian::{Date, Month::*, Year, YearMonth};
//!
//! assert!(Year::new(2020).has_leap_day(), true);
//! assert!(YearMonth::new(1900, February).total_days() == 28);
//! assert!(YearMonth::new(2000, February).total_days() == 29);
//!
//! assert!(Year::new(2020).with_month(March).first_day() == Date::new(2020, March, 1).unwrap());
//! assert!(Year::new(2020).with_month(March).last_day() == Date::new(2020, March, 31).unwrap());
//!
//! assert!(Year::new(2020).first_day() == Date::new(2020, 1, 1).unwrap());
//! assert!(Year::new(2020).last_day() == Date::new(2020, 12, 31).unwrap());
//!
//! assert!(Date::new(2020, 2, 1).unwrap().day_of_year() == 32);
//! ```

mod date;
mod error;
mod month;
mod util;
mod year;
mod year_month;

pub use date::*;
pub use error::*;
pub use month::*;
pub use year::*;
pub use year_month::*;
