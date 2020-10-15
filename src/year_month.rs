use crate::{Date, InvalidDayOfMonth, Month, Year};

/// A month of a specific year.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct YearMonth {
	year: Year,
	month: Month,
}

impl YearMonth {
	/// Create a new year-month.
	pub fn new(year: impl Into<Year>, month: Month) -> Self {
		let year = year.into();
		Self { year, month }
	}

	/// Get the year.
	pub fn year(self) -> Year {
		self.year
	}

	/// Get the month as [`Month`].
	pub fn month(self) -> Month {
		self.month
	}

	/// Get the total number of days in the month.
	///
	/// This function accounts for leap-days,
	/// so it reports 29 days for February of leap-years,
	/// and 28 days for other years.
	pub fn total_days(self) -> u8 {
		crate::raw::days_in_month(self.month, self.year.has_leap_day())
	}

	/// Get the day-of-year on which the month starts.
	///
	/// Day-of-year numbers are 1-based.
	pub fn day_of_year(self) -> u16 {
		crate::raw::start_day_of_year(self.month, self.year.has_leap_day())
	}

	/// Get the next month as [`YearMonth`].
	///
	/// After December, this function returns January of the next year.
	pub fn next(self) -> Self {
		if self.month == Month::December {
			Self::new(self.year.next(), Month::January)
		} else {
			Self::new(self.year, self.month.wrapping_next())
		}
	}

	/// Get the previous month as [`YearMonth`].
	///
	/// After January, this function returns December of the previous year.
	pub fn prev(self) -> Self {
		if self.month == Month::January {
			Self::new(self.year.prev(), Month::December)
		} else {
			Self::new(self.year, self.month.wrapping_prev())
		}
	}

	/// Get a new [`YearMonth`] by adding a number of years.
	pub fn add_years(self, years: i16) -> Self {
		(self.year() + years).with_month(self.month())
	}

	/// Get a new [`YearMonth`] by subtracting a number of years.
	pub fn sub_years(self, years: i16) -> Self {
		(self.year() - years).with_month(self.month())
	}

	/// Get a new [`YearMonth`] by adding a number of months.
	pub fn add_months(self, months: i32) -> Self {
		// Split calculation for years and months.
		let months = i32::from(self.month().to_number() - 1) + months;
		let mut year = self.year() + (months / 12) as i16;
		let month = Month::January.wrapping_add((months % 12) as i8);

		// If we subtract months, we must decrease the year too.
		if months % 12 < 0 {
			year -= 1;
		}

		year.with_month(month)
	}

	/// Get a new [`YearMonth`] by subtracting a number of months.
	pub fn sub_months(self, months: i32) -> Self {
		// This breaks for i32::MIN, but that would overflow the year counter anyway.
		self.add_months(-months)
	}

	/// Combine the year and month with a day, to create a full [`Date`].
	pub fn with_day(self, day: u8) -> Result<Date, InvalidDayOfMonth> {
		InvalidDayOfMonth::check(self.year, self.month, day)?;
		unsafe { Ok(Date::new_unchecked(self.year, self.month, day)) }
	}

	/// Combine the year and month with a day, without checking for validity.
	///
	/// # Safety
	/// Although this is currently not the case, future implementations may rely on date validity for memory safety
	pub unsafe fn with_day_unchecked(self, day: u8) -> Date {
		Date::new_unchecked(self.year, self.month, day)
	}

	/// Get the first day of the month as [`Date`].
	pub fn first_day(self) -> Date {
		Date {
			year: self.year,
			month: self.month,
			day: 1,
		}
	}

	/// Get the last day of the month as [`Date`].
	pub fn last_day(self) -> Date {
		Date {
			year: self.year,
			month: self.month,
			day: self.total_days(),
		}
	}
}

impl core::fmt::Display for YearMonth {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:04}-{:02}", self.year.to_number(), self.month().to_number())
	}
}

impl core::fmt::Debug for YearMonth {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "YearMonth({})", self)
	}
}

#[cfg(test)]
mod test {
	use crate::*;
	use assert2::assert;

	#[test]
	fn test_add_months() {
		for i in -200..=200 {
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 1) == Year::new(2000 + i as i16).with_month(February));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 2) == Year::new(2000 + i as i16).with_month(March));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 3) == Year::new(2000 + i as i16).with_month(April));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 4) == Year::new(2000 + i as i16).with_month(May));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 5) == Year::new(2000 + i as i16).with_month(June));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 6) == Year::new(2000 + i as i16).with_month(July));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 7) == Year::new(2000 + i as i16).with_month(August));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 8) == Year::new(2000 + i as i16).with_month(September));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 9) == Year::new(2000 + i as i16).with_month(October));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 10) == Year::new(2000 + i as i16).with_month(November));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 11) == Year::new(2000 + i as i16).with_month(December));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + 12) == Year::new(2001 + i as i16).with_month(January));

			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -1) == Year::new(1999 + i as i16).with_month(December));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -2) == Year::new(1999 + i as i16).with_month(November));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -3) == Year::new(1999 + i as i16).with_month(October));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -4) == Year::new(1999 + i as i16).with_month(September));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -5) == Year::new(1999 + i as i16).with_month(August));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -6) == Year::new(1999 + i as i16).with_month(July));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -7) == Year::new(1999 + i as i16).with_month(June));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -8) == Year::new(1999 + i as i16).with_month(May));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -9) == Year::new(1999 + i as i16).with_month(April));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -10) == Year::new(1999 + i as i16).with_month(March));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -11) == Year::new(1999 + i as i16).with_month(February));
			assert!(Year::new(2000).with_month(January).add_months(i * 12 + -12) == Year::new(1999 + i as i16).with_month(January));
		}
	}

	#[test]
	fn year_month_fmt() {
		assert!(format!("{}", Year::new(2020).with_month(January)) == "2020-01");
		assert!(format!("{:?}", Year::new(2020).with_month(January)) == "YearMonth(2020-01)");
	}
}
