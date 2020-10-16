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
	fn year_month_fmt() {
		assert!(format!("{}", Year::new(2020).with_month(January)) == "2020-01");
		assert!(format!("{:?}", Year::new(2020).with_month(January)) == "YearMonth(2020-01)");
	}
}
