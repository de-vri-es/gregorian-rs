use crate::{Date, InvalidDayOfYear, Month, YearMonth};

/// A calendar year.
///
/// All dates in the library use the proleptic Gregorian calendar with a year 0.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Year {
	year: i16,
}

impl Year {
	/// Create a new year from a number.
	pub const fn new(year: i16) -> Self {
		Self { year }
	}

	/// Get the year number.
	pub const fn to_number(self) -> i16 {
		self.year
	}

	/// Check if the year has a leap day.
	///
	/// In the proleptic Gregorian calendar with a year 0,
	/// the year 0 has a leap day.
	pub const fn has_leap_day(self) -> bool {
		self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0)
	}

	/// Get the total number of days in the year.
	///
	/// For leap years, this is 366.
	/// For other years, this is 365.
	pub const fn total_days(self) -> u16 {
		if self.has_leap_day() {
			366
		} else {
			365
		}
	}

	/// Get the next year.
	pub const fn next(self) -> Self {
		Self { year: self.year + 1 }
	}

	/// Get the previous year.
	pub const fn prev(self) -> Self {
		Self { year: self.year - 1 }
	}

	/// Combine the year with a month to create a [`YearMonth`].
	pub const fn with_month(self, month: Month) -> YearMonth {
		YearMonth::new_const(self, month)
	}

	/// Combine the year with a day-of-year to create a [`Date`].
	///
	/// Day-of-year numbers start a 1 for January 1.
	pub const fn with_day_of_year(self, day_of_year: u16) -> Result<Date, InvalidDayOfYear> {
		let (month, day_of_month) = match crate::raw::month_and_day_from_day_of_year(day_of_year, self.has_leap_day()) {
			Ok(x) => x,
			Err(()) => return Err(InvalidDayOfYear { year: self, day_of_year }),
		};

		Ok(unsafe { self.with_month(month).with_day_unchecked(day_of_month) })
	}

	/// Get the first month of the year as [`YearMonth`].
	pub const fn first_month(self) -> YearMonth {
		self.with_month(Month::January)
	}

	/// Get the last month of the year as [`YearMonth`].
	pub const fn last_month(self) -> YearMonth {
		self.with_month(Month::December)
	}

	/// Get all months of the year as [`YearMonth`] array.
	pub const fn months(self) -> [YearMonth; 12] {
		[
			self.with_month(Month::January),
			self.with_month(Month::February),
			self.with_month(Month::March),
			self.with_month(Month::April),
			self.with_month(Month::May),
			self.with_month(Month::June),
			self.with_month(Month::July),
			self.with_month(Month::August),
			self.with_month(Month::September),
			self.with_month(Month::October),
			self.with_month(Month::November),
			self.with_month(Month::December),
		]
	}

	/// Get the first day of the year as [`Date`].
	pub const fn first_day(self) -> Date {
		Date {
			year: self,
			month: Month::January,
			day: 1,
		}
	}

	/// Get the last day of the year as [`Date`].
	pub const fn last_day(self) -> Date {
		Date {
			year: self,
			month: Month::December,
			day: 31,
		}
	}
}

impl From<i16> for Year {
	fn from(other: i16) -> Self {
		Self::new(other)
	}
}

impl From<Year> for i16 {
	fn from(other: Year) -> i16 {
		other.to_number()
	}
}

impl PartialEq<i16> for Year {
	fn eq(&self, other: &i16) -> bool {
		self.to_number() == *other
	}
}

impl PartialOrd<i16> for Year {
	fn partial_cmp(&self, other: &i16) -> Option<core::cmp::Ordering> {
		Some(self.to_number().cmp(other))
	}
}

impl core::ops::Add<i16> for Year {
	type Output = Self;

	fn add(self, other: i16) -> Self {
		Self::new(self.to_number() + other)
	}
}

impl core::ops::Sub<i16> for Year {
	type Output = Self;

	fn sub(self, other: i16) -> Self {
		Self::new(self.to_number() - other)
	}
}

impl core::ops::AddAssign<i16> for Year {
	fn add_assign(&mut self, other: i16) {
		self.year += other
	}
}

impl core::ops::SubAssign<i16> for Year {
	fn sub_assign(&mut self, other: i16) {
		self.year -= other
	}
}

impl core::fmt::Display for Year {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:04}", self.year)
	}
}

impl core::fmt::Debug for Year {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "Year({:04})", self.year)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use assert2::assert;

	#[test]
	fn has_leap_day() {
		assert!(Year::new(2020).has_leap_day() == true);
		assert!(Year::new(2021).has_leap_day() == false);
		assert!(Year::new(1900).has_leap_day() == false);
		assert!(Year::new(2000).has_leap_day() == true);
	}

	#[test]
	fn with_day_of_year() {
		let mut date = Date::new(2020, 1, 1).unwrap();
		for i in 1..=366 {
			assert!(Year::new(2020).with_day_of_year(i).unwrap() == date);
			date = date.next();
		}

		let mut date = Date::new(2021, 1, 1).unwrap();
		for i in 1..=365 {
			assert!(Year::new(2021).with_day_of_year(i).unwrap() == date);
			date = date.next();
		}

		assert!(let Err(_) = Year::new(2020).with_day_of_year(0));
		assert!(let Err(_) = Year::new(2021).with_day_of_year(0));
		assert!(let Err(_) = Year::new(2020).with_day_of_year(367));
		assert!(let Err(_) = Year::new(2021).with_day_of_year(368));
	}

	#[test]
	fn months() {
		let year = Year::new(2020);
		let months = year.months();
		assert!(months[0] == year.with_month(Month::January));
		assert!(months[1] == year.with_month(Month::February));
		assert!(months[2] == year.with_month(Month::March));
		assert!(months[3] == year.with_month(Month::April));
		assert!(months[4] == year.with_month(Month::May));
		assert!(months[5] == year.with_month(Month::June));
		assert!(months[6] == year.with_month(Month::July));
		assert!(months[7] == year.with_month(Month::August));
		assert!(months[8] == year.with_month(Month::September));
		assert!(months[9] == year.with_month(Month::October));
		assert!(months[10] == year.with_month(Month::November));
		assert!(months[11] == year.with_month(Month::December));
	}

	#[test]
	#[cfg(feature = "std")]
	fn format_year() {
		assert!(format!("{}", Year::new(2020)) == "2020");
		assert!(format!("{:?}", Year::new(2020)) == "Year(2020)");
	}
}
