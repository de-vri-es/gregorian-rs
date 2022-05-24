main:
  * Fix `Date::today()` for Windows.

Version 0.2.1:
  * Add `Date::today()` and `Date::today_utc()`.

Version 0.2.0:
  * Make `Debug` string representations more compact.
  * `Month::wrapping_add/sub` now take an `i8` instead of `u8`.
  * Add `YearMonth::add/sub_years` and `YearMonth::add/sub_months`.
  * Add `Date::add/sub_years` and `Date::add/sub_months`.
  * Add `InvalidDayOfMonth::next/prev_valid()` to get the nearest valid date.
  * Add `DateResultExt` extension trait for `Result<Date, InvalidDayOfMonth>` to easily round invalid dates.
  * Rename `InvalidDate::InvalidDayForMonth` to `InvalidDate::InvalidDayOfMonth` to match the wrapped struct.
  * Rename `InvalidDayOfYear::day` to `day_of year`.
  * Make many functions `const`.

Version 0.1.1:
  * Fix `Date::add_days` for days in years 100, 200 and 300 of the 400 year cycle.

Version 0.1.0:
  * Initial release.
