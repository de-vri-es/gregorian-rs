# gregorian [![docs][docs-badge]][docs] [![tests][tests-badge]][tests]
[docs]: https://docs.rs/gregorian/
[tests]: https://github.com/de-vri-es/gregorian-rs/actions?query=workflow%3Atests
[docs-badge]: https://docs.rs/gregorian/badge.svg
[tests-badge]: https://github.com/de-vri-es/gregorian-rs/workflows/tests/badge.svg

An implementation of the proleptic Gregorian calendar, compatible with ISO 8601.
Amongst others, that means that the calendar has a year zero preceeding the year 1.

This create does not deal with times or time zones.

The [`Date`] type represents a date (year, month and day),
the [`Year`] type represents a calendar year,
the [`Month`] type represents a calendar month,
and the [`YearMonth`] type represents a month of a specific year.

Where possible, things are implemented as `const fn`.
Currently, this excludes trait implementations and functions that rely on traits.

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

## Rounding invalid dates
When you use [`Date::add_years()`] or [`Date::add_months()`], you can get invalid dates.
These are reported with an [`InvalidDayOfMonth`] error which has the
[`next_valid()`][InvalidDayOfMonth::next_valid] and [`prev_valid()`][InvalidDayOfMonth::prev_valid] methods.
Those can be used to get the next or previous valid date instead.

Additionally, there is an extension trait for `Result<Date, InvalidDayOfMonth>` with the
[`or_next_valid()`][DateResultExt::or_next_valid] and [`or_prev_valid()`][DateResultExt::or_prev_valid] methods.
This allows you to directly round the date on the `Result` object.

```rust
use gregorian::{Date, DateResultExt};
let date = Date::new(2020, 1, 31).unwrap();
assert!(date.add_months(1).or_next_valid() == Date::new(2020, 3, 1).unwrap());
assert!(date.add_months(1).or_prev_valid() == Date::new(2020, 2, 29).unwrap());
```

[`Date`]: https://docs.rs/gregorian/latest/gregorian/struct.Date.html
[`Year`]: https://docs.rs/gregorian/latest/gregorian/struct.Year.html
[`YearMonth`]: https://docs.rs/gregorian/latest/gregorian/struct.YearMonth.html
[`Month`]: https://docs.rs/gregorian/latest/gregorian/struct.Month.html
[`InvalidDayOfMonth`]: https://docs.rs/gregorian/latest/gregorian/struct.InvalidDayOfMonth.html

[`Date::add_years()`]: https://docs.rs/gregorian/latest/gregorian/struct.Date.html#method.add_years
[`Date::add_months()`]: https://docs.rs/gregorian/latest/gregorian/struct.Date.html#method.add_months
[DateResultExt::or_next_valid]: https://docs.rs/gregorian/latest/gregorian/trait.DateResultExt.html#method.or_next_valid
[DateResultExt::or_prev_valid]: https://docs.rs/gregorian/latest/gregorian/trait.DateResultExt.html#method.or_prev_valid
[InvalidDayOfMonth::next_valid]: https://docs.rs/gregorian/latest/gregorian/struct.InvalidDayOfMonth.html#method.next_valid
[InvalidDayOfMonth::prev_valid]: https://docs.rs/gregorian/latest/gregorian/struct.InvalidDayOfMonth.html#method.prev_valid
