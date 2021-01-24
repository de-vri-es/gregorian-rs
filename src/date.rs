use crate::{DateParseError, InvalidDate, InvalidDayOfMonth, InvalidDateSyntax, Month, Year, YearMonth};
use crate::util::{modulo_i16, modulo_i32};

/// The total number of days in 400 years.
const DAYS_IN_400_YEAR: i32 = 400 * 365 + 97;

/// The number of days since year 0 for 1970-01-01.
const UNIX_EPOCH: i32 = DAYS_IN_400_YEAR * 4 + 370 * 365 + 90;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A calendar date consting of a year, month and day.
///
/// All dates in the library use the proleptic Gregorian calendar with a year 0.
pub struct Date {
	pub(crate) year: Year,
	pub(crate) month: Month,
	pub(crate) day: u8,
}

impl Date {
	/// Create a new date from a year, month and day.
	///
	/// Month and day numbers start at 1.
	pub fn new<Y, M>(year: Y, month: M, day: u8) -> Result<Self, InvalidDate>
	where
		Y: Into<Year>,
		M: core::convert::TryInto<Month>,
		InvalidDate: From<M::Error>,
	{
		let year_month = YearMonth::new(year, month.try_into()?);
		Ok(year_month.with_day(day)?)
	}

	/// Create a new date from a year, month and day.
	///
	/// Day numbers start at 1.
	pub const fn new_const(year: Year, month: Month, day: u8) -> Result<Self, InvalidDayOfMonth> {
		YearMonth::new_const(year, month).with_day(day)
	}

	/// Create a new date without checking the validity.
	///
	/// Month and day numbers start at 1.
	///
	/// # Safety
	/// Although this is currently not the case,
	/// future implementations may rely on date validity for memory safety
	pub const unsafe fn new_unchecked(year: Year, month: Month, day: u8) -> Self {
		Self { year, month, day }
	}

	/// Get the current date in the local time zone.
	#[cfg(feature = "std")]
	pub fn today() -> Self {
		unsafe {
			let time = libc::time(std::ptr::null_mut());
			let mut tm: libc::tm = std::mem::zeroed();
			if libc::localtime_r(&time, &mut tm).is_null() {
				panic!("failed to determine current time in local time zone");
			}
			let year = Year::new(tm.tm_year as i16 + 1900);
			let month = Month::new_unchecked(tm.tm_mon as u8 + 1);
			let day = tm.tm_mday as u8; // Weirdly, tm_mday is 1 based while tm_mon is zero based.
			Date::new_unchecked(year, month, day)
		}
	}

	/// Get the current date of the UTC time zone.
	#[cfg(feature = "std")]
	pub fn today_utc() -> Self {
		let seconds = std::time::SystemTime::now()
			.duration_since(std::time::SystemTime::UNIX_EPOCH)
			.unwrap()
			.as_secs();
		let days = seconds / 60 / 60 / 24;
		Self::from_days_since_year_zero(UNIX_EPOCH + days as i32)
	}

	/// Get the date for a unix timestamp.
	///
	/// The timestamp is interpreted as number of seconds since 1 January 1970 00:00,
	/// not including any leap seconds.
	pub const fn from_unix_timestamp(seconds: i64) -> Self {
		let days = seconds / (24 * 3600);
		let days = if seconds < 0 && seconds != days * 24 * 3600 {
			days - 1
		} else {
			days
		};

		Self::from_days_since_year_zero(UNIX_EPOCH + days as i32)
	}

	/// Get the unix timestamp for a date.
	///
	/// The timestamp is the number of seconds since 1 January 1970 00:00.
	///
	/// The returned timestamp is valid for time 00:00 of the date.
	pub const fn to_unix_timestamp(self) -> i64 {
		let days = self.days_since_year_zero() - UNIX_EPOCH;
		60 * 60 * 24 * days as i64
	}

	/// Get the year.
	pub const fn year(self) -> Year {
		self.year
	}

	/// Get the month.
	pub const fn month(self) -> Month {
		self.month
	}

	/// Get the day of the month.
	pub const fn day(self) -> u8 {
		self.day
	}

	/// Get the year and month as [`YearMonth`].
	pub const fn year_month(self) -> YearMonth {
		YearMonth::new_const(self.year(), self.month())
	}

	/// Get the day of the year.
	///
	/// The returned number is 1-based.
	/// For January 1, this function will return 1.
	pub const fn day_of_year(self) -> u16 {
		crate::raw::day_of_year(self.month, self.day, self.year.has_leap_day())
	}

	/// The number of days remaining in the year, including the current date.
	///
	/// For Janury 1 this will return 365 in a non-leap year or 366 in a leap year.
	/// For December 31, this will return 1.
	pub const fn days_remaining_in_year(self) -> u16 {
		self.year.total_days() - self.day_of_year() + 1
	}

	/// Get the total number of days since 1 January 0000.
	///
	/// The returned value is zero-based.
	/// For 1 January 0000, this function returns 0.
	#[allow(clippy::identity_op)]
	pub const fn days_since_year_zero(self) -> i32 {
		let years = modulo_i16(self.year().to_number(), 400);
		let whole_cycles = (self.year().to_number() - years) / 400;

		// Plus one because year 0 is a leap year.
		let leap_days = years / 4 - years / 100 + 1;
		// But -1 in leap years because they're taken care of in self.day_of_year().
		let leap_days = leap_days - if self.year.has_leap_day() { 1 } else { 0 };

		let from_years = whole_cycles as i32 * DAYS_IN_400_YEAR + years as i32 * 365 + leap_days as i32;

		from_years + self.day_of_year() as i32 - 1
	}

	/// Get the date corresponding to a number of days since the year zero.
	///
	/// For this function, day 0 is 1 January of year 0.
	#[rustfmt::skip]
	pub const fn from_days_since_year_zero(days: i32) -> Self {
		// Get the day index in the current 400 year cycle,
		// and the number of passed 400 year cycles.
		let day_index = modulo_i32(days, DAYS_IN_400_YEAR);
		let whole_cycles = (days - day_index) / DAYS_IN_400_YEAR;

		// How many leaps days did not happen at year 100, 200 and 300?
		let pretend_leap_days;
		if day_index >= 300 * 365 + 73 + 31 + 28 {
			pretend_leap_days = 3;
		} else if day_index >= 200 * 365 + 49 + 31 + 28 {
			pretend_leap_days = 2;
		} else if day_index >= 100 * 365 + 25 + 31 + 28 {
			pretend_leap_days = 1;
		} else {
			pretend_leap_days = 0;
		}

		// How many four year intervals passed, and how many days since then?
		let four_year_cycles       = (day_index + pretend_leap_days) / (4 * 365 + 1);
		let day_of_four_year_cycle = (day_index + pretend_leap_days) % (4 * 365 + 1);

		// How many years passed since the 4 year interval?
		let year_of_four_year_cycle = (day_of_four_year_cycle - 1) / 365;

		// Calculate the day of the year.
		// We added pretendsies leap days for year 100, 200 and 300,
		// so we can ignore the fact that those years actually don't have one.
		let day_of_year = day_of_four_year_cycle - (year_of_four_year_cycle * 365);
		let day_of_year = day_of_year - if day_of_four_year_cycle >= 366 { 1 } else { 0 };

		// Compensate for 1 based year-of-day numbers.
		let day_of_year = day_of_year + 1;

		// Put it all together.
		let year = 400 * whole_cycles + 4 * four_year_cycles + year_of_four_year_cycle;
		let year = Year::new(year as i16);

		// Lie about leap years for year 100, 200 and 300 because we added pretend leaps days.
		let (month, day_of_month) = match crate::raw::month_and_day_from_day_of_year(day_of_year as u16, year_of_four_year_cycle == 0) {
			Ok(x) => x,
			// TODO: replace with unreachable! when const_panic is stabilized.
			Err(()) => (Month::January, 1),
		};

		unsafe { year.with_month(month).with_day_unchecked(day_of_month) }
	}

	/// Get a [`Date`] object for the next day.
	pub const fn next(self) -> Date {
		if self.day == self.year_month().total_days() {
			self.year_month().next().first_day()
		} else {
			Self {
				year: self.year,
				month: self.month,
				day: self.day + 1,
			}
		}
	}

	/// Get a [`Date`] object for the previous day.
	pub const fn prev(self) -> Date {
		if self.day == 1 {
			self.year_month().prev().last_day()
		} else {
			Self {
				year: self.year,
				month: self.month,
				day: self.day - 1,
			}
		}
	}

	/// Compute a date by adding days.
	pub const fn add_days(self, days: i32) -> Self {
		Self::from_days_since_year_zero(self.days_since_year_zero() + days)
	}

	/// Compute a date by subtracting days.
	pub const fn sub_days(self, days: i32) -> Self {
		Self::from_days_since_year_zero(self.days_since_year_zero() - days)
	}

	/// Compute a date by adding a number of months.
	///
	/// The resulting date may not be valid.
	/// You can call [`InvalidDayOfMonth::next_valid()`] or [`InvalidDayOfMonth::prev_valid()`]
	/// to get the first day of the next month or the last day of resulting month.
	pub const fn add_months(self, months: i32) -> Result<Self, InvalidDayOfMonth> {
		self.year_month().add_months(months).with_day(self.day())
	}

	/// Compute a date by subtracting a number of months.
	///
	/// The resulting date may not be valid.
	/// You can call [`InvalidDayOfMonth::next_valid()`] or [`InvalidDayOfMonth::prev_valid()`]
	/// to get the first day of the next month or the last day of resulting month.
	pub const fn sub_months(self, months: i32) -> Result<Self, InvalidDayOfMonth> {
		self.year_month().add_months(months).with_day(self.day())
	}

	/// Compute a date by adding a number of years.
	///
	/// The resulting date may not be valid.
	/// You can call [`InvalidDayOfMonth::next_valid()`] or [`InvalidDayOfMonth::prev_valid()`]
	/// to get the first day of the next month or the last day of resulting month.
	pub const fn add_years(self, years: i16) -> Result<Self, InvalidDayOfMonth> {
		self.year_month().add_years(years).with_day(self.day())
	}

	/// Compute a date by subtracting a number of years.
	///
	/// The resulting date may not be valid.
	/// You can call [`InvalidDayOfMonth::next_valid()`] or [`InvalidDayOfMonth::prev_valid()`]
	/// to get the first day of the next month or the last day of resulting month.
	pub const fn sub_years(self, years: i16) -> Result<Self, InvalidDayOfMonth> {
		self.year_month().add_years(years).with_day(self.day())
	}
}

impl core::str::FromStr for Date {
	type Err = DateParseError;

	fn from_str(data: &str) -> Result<Self, Self::Err> {
		// Extract fields.
		let mut fields = data.splitn(3, '-');
		let year = fields.next().unwrap();
		let month = fields.next().ok_or_else(InvalidDateSyntax::new)?;
		let day = fields.next().ok_or_else(InvalidDateSyntax::new)?;

		// Parse fields as numbers.
		let year: i16 = year.parse().map_err(|_| InvalidDateSyntax::new())?;
		let month: u8 = month.parse().map_err(|_| InvalidDateSyntax::new())?;
		let day: u8 = day.parse().map_err(|_| InvalidDateSyntax::new())?;

		// Return date.
		Ok(Self::new(year, month, day)?)
	}
}

impl core::fmt::Display for Date {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:04}-{:02}-{:02}", self.year.to_number(), self.month.to_number(), self.day)
	}
}

impl core::fmt::Debug for Date {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "Date({})", self)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use assert2::assert;

	#[test]
	fn new() {
		assert!(let Ok(_) = Date::new(2020, 1, 2));
		assert!(Date::new(2020, 1, 2).unwrap().year() == 2020);
		assert!(Date::new(2020, 1, 2).unwrap().month() == 1);
		assert!(Date::new(2020, 1, 2).unwrap().day() == 2);

		assert!(let Ok(_) = Date::new(2020, 2, 29));
		assert!(let Err(_) = Date::new(2020, 2, 30));
		assert!(let Ok(_) = Date::new(2019, 2, 28));
		assert!(let Err(_) = Date::new(2019, 2, 29));
	}

	#[test]
	#[cfg(feature = "std")]
	fn today() {
		// Got nothing to check it against really, but lets see at least that it does not panic,
		// and that it's atleast 2021.
		assert!(Date::today().year() >= 2021);
		assert!(Date::today_utc().year() >= 2021);
	}

	#[test]
	fn next() {
		assert!(Date::new(2020, 1, 2).unwrap().next() == Date::new(2020, 1, 3).unwrap());
		assert!(Date::new(2020, 1, 31).unwrap().next() == Date::new(2020, 2, 1).unwrap());
		assert!(Date::new(2020, 12, 31).unwrap().next() == Date::new(2021, 1, 1).unwrap());
	}

	#[test]
	fn day_of_year() {
		assert!(Date::new(2019, 1, 1).unwrap().day_of_year() == 1);
		assert!(Date::new(2019, 2, 1).unwrap().day_of_year() == 32);
		assert!(Date::new(2019, 3, 1).unwrap().day_of_year() == 60);
		assert!(Date::new(2019, 4, 1).unwrap().day_of_year() == 91);
		assert!(Date::new(2019, 5, 1).unwrap().day_of_year() == 121);
		assert!(Date::new(2019, 6, 1).unwrap().day_of_year() == 152);
		assert!(Date::new(2019, 7, 1).unwrap().day_of_year() == 182);
		assert!(Date::new(2019, 8, 1).unwrap().day_of_year() == 213);
		assert!(Date::new(2019, 9, 1).unwrap().day_of_year() == 244);
		assert!(Date::new(2019, 10, 1).unwrap().day_of_year() == 274);
		assert!(Date::new(2019, 11, 1).unwrap().day_of_year() == 305);
		assert!(Date::new(2019, 12, 1).unwrap().day_of_year() == 335);

		assert!(Date::new(2020, 1, 1).unwrap().day_of_year() == 1);
		assert!(Date::new(2020, 2, 1).unwrap().day_of_year() == 32);
		assert!(Date::new(2020, 3, 1).unwrap().day_of_year() == 61);
		assert!(Date::new(2020, 4, 1).unwrap().day_of_year() == 92);
		assert!(Date::new(2020, 5, 1).unwrap().day_of_year() == 122);
		assert!(Date::new(2020, 6, 1).unwrap().day_of_year() == 153);
		assert!(Date::new(2020, 7, 1).unwrap().day_of_year() == 183);
		assert!(Date::new(2020, 8, 1).unwrap().day_of_year() == 214);
		assert!(Date::new(2020, 9, 1).unwrap().day_of_year() == 245);
		assert!(Date::new(2020, 10, 1).unwrap().day_of_year() == 275);
		assert!(Date::new(2020, 11, 1).unwrap().day_of_year() == 306);
		assert!(Date::new(2020, 12, 1).unwrap().day_of_year() == 336);

		assert!(Date::new(2019, 1, 2).unwrap().day_of_year() == 2);
		assert!(Date::new(2019, 1, 31).unwrap().day_of_year() == 31);
		assert!(Date::new(2019, 2, 2).unwrap().day_of_year() == 33);
		assert!(Date::new(2019, 2, 28).unwrap().day_of_year() == 59);
		assert!(Date::new(2019, 12, 31).unwrap().day_of_year() == 365);

		assert!(Date::new(2020, 12, 31).unwrap().day_of_year() == 366);

		let mut date = Date::new(2020, 1, 1).unwrap();
		for i in 1..=366 {
			assert!(date.day_of_year() == i);
			date = date.next();
		}

		let mut date = Date::new(2021, 1, 1).unwrap();
		for i in 1..=365 {
			assert!(date.day_of_year() == i);
			date = date.next();
		}
	}

	#[test]
	fn days_since_year_zero() {
		assert!(Date::new(0, 1, 1).unwrap().days_since_year_zero() == 0);
		assert!(Date::new(400, 1, 1).unwrap().days_since_year_zero() == 1 * (400 * 365 + 97));
		assert!(Date::new(800, 1, 1).unwrap().days_since_year_zero() == 2 * (400 * 365 + 97));
		assert!(Date::new(-400, 1, 1).unwrap().days_since_year_zero() == -1 * (400 * 365 + 97));
		assert!(Date::new(-800, 1, 1).unwrap().days_since_year_zero() == -2 * (400 * 365 + 97));

		assert!(Date::new(1, 1, 1).unwrap().days_since_year_zero() == 366);
		assert!(Date::new(0, 12, 31).unwrap().days_since_year_zero() == 365);
		assert!(Date::new(399, 12, 31).unwrap().days_since_year_zero() == 400 * 365 + 97 - 1);
		assert!(Date::new(-1, 12, 31).unwrap().days_since_year_zero() == -1);

		assert!(Date::new(396, 1, 1).unwrap().days_since_year_zero() == 396 * 365 + 96);

		assert!(Date::new(-2, 1, 1).unwrap().days_since_year_zero() == -2 * 365);
		assert!(Date::new(-3, 1, 1).unwrap().days_since_year_zero() == -3 * 365);
		assert!(Date::new(-4, 1, 1).unwrap().days_since_year_zero() == -4 * 365 - 1);
		assert!(Date::new(-100, 1, 1).unwrap().days_since_year_zero() == -100 * 365 - 24);
		assert!(Date::new(-400, 1, 1).unwrap().days_since_year_zero() == -400 * 365 - 97);

		let mut date = Date::new(0, 1, 1).unwrap();
		for i in 0..=10 * DAYS_IN_400_YEAR {
			assert!(date.days_since_year_zero() == i);
			date = date.next();
		}

		let mut date = Date::new(0, 1, 1).unwrap();
		for i in (-10 * DAYS_IN_400_YEAR..=0).rev() {
			assert!(date.days_since_year_zero() == i);
			date = date.prev();
		}
	}

	#[test]
	fn from_days_since_year_zero() {
		assert!(Date::from_days_since_year_zero(0) == Date::new(0, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(1 * (400 * 365 + 97)) == Date::new(400, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(2 * (400 * 365 + 97)) == Date::new(800, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-1 * (400 * 365 + 97)) == Date::new(-400, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-2 * (400 * 365 + 97)) == Date::new(-800, 1, 1).unwrap());

		assert!(Date::from_days_since_year_zero(366) == Date::new(1, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(365) == Date::new(0, 12, 31).unwrap());
		assert!(Date::from_days_since_year_zero(400 * 365 + 97 - 1) == Date::new(399, 12, 31).unwrap());
		assert!(Date::from_days_since_year_zero(-1) == Date::new(-1, 12, 31).unwrap());

		assert!(Date::from_days_since_year_zero(396 * 365 + 96) == Date::new(396, 1, 1).unwrap());

		assert!(Date::from_days_since_year_zero(-2 * 365) == Date::new(-2, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-3 * 365) == Date::new(-3, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-4 * 365 - 1) == Date::new(-4, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(366) == Date::new(1, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(100 * 365 + 25 + 31 + 27) == Date::new(100, 2, 28).unwrap());
		assert!(Date::from_days_since_year_zero(100 * 365 + 25 + 31 + 28) == Date::new(100, 3, 1).unwrap());
		assert!(Date::from_days_since_year_zero(100 * 365 + 25 + 31 + 29) == Date::new(100, 3, 2).unwrap());
		assert!(Date::from_days_since_year_zero(100 * 365 + 25 + 31 + 30) == Date::new(100, 3, 3).unwrap());
		assert!(Date::from_days_since_year_zero(101 * 365 + 25) == Date::new(101, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(100 * 365 + 25 + 31 + 28) == Date::new(100, 3, 1).unwrap());
		assert!(Date::from_days_since_year_zero(200 * 365 + 49) == Date::new(200, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(300 * 365 + 73) == Date::new(300, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-100 * 365 - 24) == Date::new(-100, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-400 * 365 - 97) == Date::new(-400, 1, 1).unwrap());

		assert!(Date::from_days_since_year_zero(370 * 365 + 90) == Date::new(370, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(770 * 365 + 97 + 90) == Date::new(770, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(UNIX_EPOCH) == Date::new(1970, 1, 1).unwrap());

		let mut date = Date::new(0, 1, 1).unwrap();
		for i in 0..=10 * DAYS_IN_400_YEAR {
			assert!(Date::from_days_since_year_zero(i) == date);
			date = date.next();
		}

		let mut date = Date::new(0, 1, 1).unwrap();
		for i in (-10 * DAYS_IN_400_YEAR..=0).rev() {
			assert!(Date::from_days_since_year_zero(i) == date);
			date = date.prev();
		}
	}

	#[test]
	fn add_days() {
		assert!(Date::new(2020, 1, 1).unwrap().add_days(1) == Date::new(2020, 1, 2).unwrap());
		assert!(Date::new(2020, 1, 1).unwrap().add_days(31) == Date::new(2020, 2, 1).unwrap());
		assert!(Date::new(2020, 1, 1).unwrap().add_days(366) == Date::new(2021, 1, 1).unwrap());
		assert!(Date::new(2020, 1, 1).unwrap().add_days(366 + 365) == Date::new(2022, 1, 1).unwrap());

		assert!(Date::new(2000, 1, 1).unwrap().add_days(100 * 365 + 24) == Date::new(2099, 12, 31).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(100 * 365 + 25) == Date::new(2100, 1, 1).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(100 * 365 + 26) == Date::new(2100, 1, 2).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(100 * 365 + 25 + 58) == Date::new(2100, 2, 28).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(100 * 365 + 25 + 59) == Date::new(2100, 3, 1).unwrap());

		assert!(Date::new(2000, 1, 1).unwrap().add_days(200 * 365 + 48) == Date::new(2199, 12, 31).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(200 * 365 + 49) == Date::new(2200, 1, 1).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(200 * 365 + 50) == Date::new(2200, 1, 2).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(200 * 365 + 49 + 58) == Date::new(2200, 2, 28).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(200 * 365 + 49 + 59) == Date::new(2200, 3, 1).unwrap());

		assert!(Date::new(2000, 1, 1).unwrap().add_days(300 * 365 + 73) == Date::new(2300, 1, 1).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(300 * 365 + 72) == Date::new(2299, 12, 31).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(300 * 365 + 74) == Date::new(2300, 1, 2).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(300 * 365 + 73 + 58) == Date::new(2300, 2, 28).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(300 * 365 + 73 + 59) == Date::new(2300, 3, 1).unwrap());

		assert!(Date::new(2000, 1, 1).unwrap().add_days(400 * 365 + 96) == Date::new(2399, 12, 31).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(400 * 365 + 97) == Date::new(2400, 1, 1).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(400 * 365 + 98) == Date::new(2400, 1, 2).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(400 * 365 + 97 + 58) == Date::new(2400, 2, 28).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(400 * 365 + 97 + 59) == Date::new(2400, 2, 29).unwrap());
		assert!(Date::new(2000, 1, 1).unwrap().add_days(400 * 365 + 97 + 60) == Date::new(2400, 3, 1).unwrap());
	}

	#[test]
	fn add_years() {
		assert!(Date::new(2020, 1, 1).unwrap().add_years(1).unwrap() == Date::new(2021, 1, 1).unwrap());
		assert!(Date::new(2000, 2, 29).unwrap().add_years(400).unwrap() == Date::new(2400, 2, 29).unwrap());
		assert!(Date::new(2000, 2, 29).unwrap().add_years(100).unwrap_err().prev_valid() == Date::new(2100, 2, 28).unwrap());
		assert!(Date::new(2000, 2, 29).unwrap().add_years(100).unwrap_err().next_valid() == Date::new(2100, 3, 1).unwrap());
	}

	#[test]
	fn add_months() {
		assert!(Date::new(2021, 1, 31).unwrap().add_months(2).unwrap() == Date::new(2021, 3, 31).unwrap());
		assert!(Date::new(2021, 1, 31).unwrap().add_months(1).unwrap_err().prev_valid() == Date::new(2021, 2, 28).unwrap());
		assert!(Date::new(2021, 1, 31).unwrap().add_months(1).unwrap_err().next_valid() == Date::new(2021, 3, 1).unwrap());

		assert!(Date::new(2021, 1, 31).unwrap().add_months(14).unwrap() == Date::new(2022, 3, 31).unwrap());
		assert!(Date::new(2021, 1, 31).unwrap().add_months(13).unwrap_err().prev_valid() == Date::new(2022, 2, 28).unwrap());
		assert!(Date::new(2021, 1, 31).unwrap().add_months(13).unwrap_err().next_valid() == Date::new(2022, 3, 1).unwrap());
	}

	#[test]
	fn parse() {
		assert!("2020-01-02".parse::<Date>().unwrap().year() == 2020);
		assert!("2020-01-02".parse::<Date>().unwrap().month() == 1);
		assert!("2020-01-02".parse::<Date>().unwrap().day() == 2);
		assert!(let Err(DateParseError::InvalidDateSyntax(_)) = "not-a-date".parse::<Date>());
		assert!(let Err(DateParseError::InvalidDate(_)) = "2019-30-12".parse::<Date>());
	}

	#[test]
	fn from_unix_timestamp() {
		const SECONDS_IN_DAY: i64 = 60 * 60 * 24;
		assert!(Date::from_unix_timestamp(0) == Date::new(1970, 1, 1).unwrap());
		assert!(Date::from_unix_timestamp(SECONDS_IN_DAY) == Date::new(1970, 1, 2).unwrap());
		assert!(Date::from_unix_timestamp(1592611200) == Date::new(2020, 06, 20).unwrap());
		assert!(Date::from_unix_timestamp(1592697599) == Date::new(2020, 06, 20).unwrap());
		assert!(Date::from_unix_timestamp(1592697600) == Date::new(2020, 06, 21).unwrap());
	}

	#[test]
	fn to_unix_timestamp() {
		const SECONDS_IN_DAY: i64 = 60 * 60 * 24;
		assert!(Date::new(1970, 1, 1).unwrap().to_unix_timestamp() == 0);
		assert!(Date::new(1970, 1, 2).unwrap().to_unix_timestamp() == SECONDS_IN_DAY);
		assert!(Date::new(2020, 06, 20).unwrap().to_unix_timestamp() == 1592611200);
	}

	#[test]
	fn format() {
		assert!(format!("{}", Date::new(2020, Month::January, 2).unwrap()) == "2020-01-02");
		assert!(format!("{:?}", Date::new(2020, Month::January, 2).unwrap()) == "Date(2020-01-02)");
	}
}
