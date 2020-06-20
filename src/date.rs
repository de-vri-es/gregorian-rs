use crate::{
	Month,
	Year,
	YearMonth,
	InvalidDate,
	InvalidDateSyntax,
	DateParseError,
	util::Modulo,
};

/// The total number of days in 400 years.
const DAYS_IN_400_YEAR : i32 = 400 * 365 + 97;

/// The number of days since year 0 for 1970-01-01.
const UNIX_EPOCH: i32 = DAYS_IN_400_YEAR * 4 + 370 * 365 + 90;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

	/// Create a new date without checking the validity.
	///
	/// Month and day numbers start at 1.
	pub unsafe fn new_unchecked(year: Year, month: Month, day: u8) -> Self {
		Self { year, month, day }
	}

	/// Get the date for a unix timestamp.
	///
	/// The timestamp is interpreted as number of seconds since 1 January 1970 00:00,
	/// not including any leap seconds.
	pub fn from_unix_timestamp(seconds: i64) -> Self {
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
	pub fn to_unix_timestamp(self) -> i64 {
		let days = self.days_since_year_zero() - UNIX_EPOCH;
		60 * 60 * 24 * i64::from(days)
	}

	/// Get the year.
	pub fn year(self) -> Year {
		self.year
	}

	/// Get the month.
	pub fn month(self) -> Month {
		self.month
	}

	/// Get the day of the month.
	pub fn day(self) -> u8 {
		self.day
	}

	/// Get the year and month as [`YearMonth`].
	pub fn year_month(self) -> YearMonth {
		YearMonth::new(self.year(), self.month())
	}

	/// Get the day of the year.
	///
	/// The returned number is 1-based.
	/// For January 1, this function will return 1.
	pub fn day_of_year(self) -> u16 {
		let leap_day_this_year = if self.year.has_leap_day() { 1 } else { 0 };
		let days_to_month_start = match self.month {
			Month::January => 0,
			Month::February => 31,
			Month::March => 59 + leap_day_this_year,
			Month::April => 90 + leap_day_this_year,
			Month::May => 120 + leap_day_this_year,
			Month::June => 151 + leap_day_this_year,
			Month::July => 181 + leap_day_this_year,
			Month::August => 212 + leap_day_this_year,
			Month::September => 243 + leap_day_this_year,
			Month::October => 273 + leap_day_this_year,
			Month::November => 304 + leap_day_this_year,
			Month::December => 334 + leap_day_this_year,
		};

		days_to_month_start + u16::from(self.day)
	}

	/// The number of days remaining in the year, including the current date.
	///
	/// For Janury 1 this will return 365 in a non-leap year or 366 in a leap year.
	/// For December 31, this will return 1.
	pub fn days_remaining_in_year(self) -> u16 {
		self.year.total_days() - self.day_of_year() + 1
	}

	/// Get the total number of days since 1 January 0000.
	///
	/// The returned value is zero-based.
	/// For 1 January 0000, this function returns 0.
	pub fn days_since_year_zero(self) -> i32 {
		let years = ((self.year().to_number() % 400) + 400) % 400;
		let whole_cycles = (self.year().to_number() - years) / 400;

		// Plus one because year 0 is a leap year.
		let leap_days = years / 4 - years / 100 + 1;
		// But -1 in leap years because they're taken care of in self.day_of_year().
		let leap_days = leap_days - if self.year.has_leap_day() { 1 } else { 0 };

		let from_years = 0
			+ i32::from(whole_cycles) * DAYS_IN_400_YEAR
			+ i32::from(years) * 365
			+ i32::from(leap_days);

		from_years + i32::from(self.day_of_year()) - 1
	}

	/// Get the date corresponding to a number of days since the year zero.
	///
	/// For this function, day 0 is 1 January 0000.
	pub fn from_days_since_year_zero(days: i32) -> Self {
		// Get the day index in the current 400 year cycle,
		// and the number of passed 400 year cycles.
		let day_index = days.modulo(DAYS_IN_400_YEAR);
		let whole_cycles = (days - day_index) / DAYS_IN_400_YEAR;

		// How many leaps days did not happen at year 100, 200 and 300?
		let pretend_leap_days = if day_index >= 300 * 365 + 73 + 31 + 29 {
			3
		} else if day_index >= 200 * 365 + 49 + 31 + 29 {
			2
		} else if day_index >= 100 * 365 + 25 + 31 + 29 {
			1
		} else {
			0
		};

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
		year.with_day_of_year(day_of_year as u16).unwrap()
	}

	/// Get a [`Date`] object for the next day.
	pub fn next(self) -> Date {
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
	pub fn prev(self) -> Date {
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
	pub fn add_days(self, days: i32) -> Self {
		Self::from_days_since_year_zero(self.days_since_year_zero() + days)
	}

	/// Compute a date by subtracting days.
	pub fn sub_days(self, days: i32) -> Self {
		Self::from_days_since_year_zero(self.days_since_year_zero() - days)
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
		let year : i16 = year.parse().map_err(|_| InvalidDateSyntax::new())?;
		let month : u8 = month.parse().map_err(|_| InvalidDateSyntax::new())?;
		let day : u8 = day.parse().map_err(|_| InvalidDateSyntax::new())?;

		// Return date.
		Ok(Self::new(year, month, day)?)
	}
}

impl core::fmt::Display for Date {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:04}-{:02}-{:02}", self.year.to_number(), self.month.to_number(), self.day)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use assert2::assert;

	#[test]
	fn test_make_date() {
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
	fn test_next_date() {
		assert!(Date::new(2020, 1, 2).unwrap().next() == Date::new(2020, 1, 3).unwrap());
		assert!(Date::new(2020, 1, 31).unwrap().next() == Date::new(2020, 2, 1).unwrap());
		assert!(Date::new(2020, 12, 31).unwrap().next() == Date::new(2021, 1, 1).unwrap());
	}

	#[test]
	fn test_day_of_year() {
		assert!(Date::new(2019, 1, 1).unwrap().day_of_year() == 1);
		assert!(Date::new(2019, 2, 1).unwrap().day_of_year() == 32);
		assert!(Date::new(2019, 3, 1).unwrap().day_of_year() == 60);
		assert!(Date::new(2019, 4, 1).unwrap().day_of_year() == 91);
		assert!(Date::new(2019, 5, 1).unwrap().day_of_year() == 121);
		assert!(Date::new(2019, 6 , 1).unwrap().day_of_year() == 152);
		assert!(Date::new(2019, 7 , 1).unwrap().day_of_year() == 182);
		assert!(Date::new(2019, 8 , 1).unwrap().day_of_year() == 213);
		assert!(Date::new(2019, 9 , 1).unwrap().day_of_year() == 244);
		assert!(Date::new(2019, 10, 1).unwrap().day_of_year() == 274);
		assert!(Date::new(2019, 11, 1).unwrap().day_of_year() == 305);
		assert!(Date::new(2019, 12, 1).unwrap().day_of_year() == 335);

		assert!(Date::new(2020, 1, 1).unwrap().day_of_year() == 1);
		assert!(Date::new(2020, 2, 1).unwrap().day_of_year() == 32);
		assert!(Date::new(2020, 3, 1).unwrap().day_of_year() == 61);
		assert!(Date::new(2020, 4, 1).unwrap().day_of_year() == 92);
		assert!(Date::new(2020, 5, 1).unwrap().day_of_year() == 122);
		assert!(Date::new(2020, 6 , 1).unwrap().day_of_year() == 153);
		assert!(Date::new(2020, 7 , 1).unwrap().day_of_year() == 183);
		assert!(Date::new(2020, 8 , 1).unwrap().day_of_year() == 214);
		assert!(Date::new(2020, 9 , 1).unwrap().day_of_year() == 245);
		assert!(Date::new(2020, 10, 1).unwrap().day_of_year() == 275);
		assert!(Date::new(2020, 11, 1).unwrap().day_of_year() == 306);
		assert!(Date::new(2020, 12, 1).unwrap().day_of_year() == 336);

		assert!(Date::new(2019, 1, 2).unwrap().day_of_year() == 2);
		assert!(Date::new(2019, 1, 31).unwrap().day_of_year() == 31);
		assert!(Date::new(2019, 2, 2).unwrap().day_of_year() == 33);
		assert!(Date::new(2019, 2, 28).unwrap().day_of_year() == 59);
		assert!(Date::new(2019, 12, 31).unwrap().day_of_year() == 365);

		assert!(Date::new(2020, 12, 31).unwrap().day_of_year() == 366);
	}

	#[test]
	fn test_days_since_year_zero() {
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
	}

	#[test]
	fn test_from_days_since_year_zero() {
		assert!(Date::from_days_since_year_zero(0) == Date::new(0, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(1 * (400 * 365 + 97)) == Date::new(400, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(2 * (400 * 365 + 97)) == Date::new(800, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-1 * (400 * 365 + 97)) ==Date::new(-400, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-2 * (400 * 365 + 97)) ==Date::new(-800, 1, 1).unwrap());

		assert!(Date::from_days_since_year_zero(366) == Date::new(1, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(365) == Date::new(0, 12, 31).unwrap());
		assert!(Date::from_days_since_year_zero(400 * 365 + 97 - 1) == Date::new(399, 12, 31).unwrap());
		assert!(Date::from_days_since_year_zero(-1) == Date::new(-1, 12, 31).unwrap());

		assert!(Date::from_days_since_year_zero(396 * 365 + 96) == Date::new(396, 1, 1).unwrap());

		assert!(Date::from_days_since_year_zero(-2 * 365) == Date::new(-2, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-3 * 365) == Date::new(-3, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-4 * 365 - 1) == Date::new(-4, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero( 366) == Date::new(1, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero( 100 * 365 + 25 + 31 + 27) == Date::new(100, 2, 28).unwrap());
		assert!(Date::from_days_since_year_zero( 100 * 365 + 25 + 31 + 28) == Date::new(100, 3, 1).unwrap());
		assert!(Date::from_days_since_year_zero( 101 * 365 + 25) == Date::new(101, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero( 100 * 365 + 25 + 31 + 28) == Date::new(100, 3, 1).unwrap());
		assert!(Date::from_days_since_year_zero( 200 * 365 + 49) == Date::new(200, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero( 300 * 365 + 73) == Date::new(300, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-100 * 365 - 24) == Date::new(-100, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(-400 * 365 - 97) == Date::new(-400, 1, 1).unwrap());

		assert!(Date::from_days_since_year_zero(370 * 365 + 90) == Date::new(370, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(770 * 365 + 97 + 90) == Date::new(770, 1, 1).unwrap());
		assert!(Date::from_days_since_year_zero(UNIX_EPOCH) == Date::new(1970, 1, 1).unwrap());
	}

	#[test]
	fn test_add_days() {
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
	fn test_parse_date() {
		assert!("2020-01-02".parse::<Date>().unwrap().year() == 2020);
		assert!("2020-01-02".parse::<Date>().unwrap().month() == 1);
		assert!("2020-01-02".parse::<Date>().unwrap().day() == 2);
		assert!(let Err(DateParseError::InvalidDateSyntax(_)) = "not-a-date".parse::<Date>());
		assert!(let Err(DateParseError::InvalidDate(_)) = "2019-30-12".parse::<Date>());
	}

	#[test]
	fn test_from_unix_timestamp() {
		const SECONDS_IN_DAY: i64 = 60 * 60 * 24;
		assert!(Date::from_unix_timestamp(0) == Date::new(1970, 1, 1).unwrap());
		assert!(Date::from_unix_timestamp(SECONDS_IN_DAY) == Date::new(1970, 1, 2).unwrap());
		assert!(Date::from_unix_timestamp(1592611200) == Date::new(2020, 06, 20).unwrap());
		assert!(Date::from_unix_timestamp(1592697599) == Date::new(2020, 06, 20).unwrap());
		assert!(Date::from_unix_timestamp(1592697600) == Date::new(2020, 06, 21).unwrap());
	}

	#[test]
	fn test_to_unix_timestamp() {
		const SECONDS_IN_DAY: i64 = 60 * 60 * 24;
		assert!(Date::new(1970, 1, 1).unwrap().to_unix_timestamp() == 0);
		assert!(Date::new(1970, 1, 2).unwrap().to_unix_timestamp() == SECONDS_IN_DAY);
		assert!(Date::new(2020, 06, 20).unwrap().to_unix_timestamp() == 1592611200);
	}
}
