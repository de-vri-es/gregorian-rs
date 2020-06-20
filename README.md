# gregorian [![docs][docs-badge]][docs] [![tests][tests-badge]][tests]
[docs]: https://docs.rs/gregorian/
[tests]: https://github.com/de-vri-es/gregorian-rs/actions?query=workflow%3Atests
[docs-badge]: https://docs.rs/gregorian/badge.svg
[tests-badge]: https://github.com/de-vri-es/gregorian-rs/workflows/tests/badge.svg

An implementation of the proleptic Gregorian calendar.
In this implementation, before the year 1 come year 0.
The library does not deal with times.

The [`Date`][Date] type represents a date (year, month and day),
the [`Year`][Year] type represents a calendar year,
the [`Month`][Month] type represents a calendar month,
and the [`YearMonth`][YearMonth] type represents a month of a specific year.

[Date]: https://docs.rs/gregorian/latest/gregorian/struct.Date.html
[Year]: https://docs.rs/gregorian/latest/gregorian/struct.Year.html
[Month]: https://docs.rs/gregorian/latest/gregorian/enum.Month.html
[YearMonth]: https://docs.rs/gregorian/latest/gregorian/struct.YearMonth.html

## Example
```rust
use gregorian::{Date, Month::*, Year, YearMonth};

assert!(Year::new(2020).has_leap_day(), true);
assert!(YearMonth::new(1900, February).total_days() == 28);
assert!(YearMonth::new(2000, February).total_days() == 29);

assert!(Year::new(2020).with_month(March).first_day() == Date::new(2020, March, 1).unwrap());
assert!(Year::new(2020).with_month(March).last_day() == Date::new(2020, March, 31).unwrap());

assert!(Year::new(2020).first_day() == Date::new(2020, 1, 1).unwrap());
assert!(Year::new(2020).last_day() == Date::new(2020, 12, 31).unwrap());

assert!(Date::new(2020, 2, 1).unwrap().day_of_year() == 32);
```
