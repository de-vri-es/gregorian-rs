use crate::{
	Date,
	InvalidDayOfMonth,
	Month,
	Year,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct YearMonth {
	year: Year,
	month: Month,
}

impl YearMonth {
	pub fn new(year: impl Into<Year>, month: Month) -> Self {
		let year = year.into();
		Self { year, month }
	}

	pub fn year(self) -> Year {
		self.year
	}

	pub fn month(self) -> Month {
		self.month
	}

	pub fn total_days(self) -> u8 {
		match self.month {
			Month::January => 31,
			Month::February => if self.year.has_leap_day() { 29 } else { 28 },
			Month::March => 31,
			Month::April => 30,
			Month::May => 31,
			Month::June => 30,
			Month::July => 31,
			Month::August => 31,
			Month::September => 30,
			Month::October => 31,
			Month::November => 30,
			Month::December => 31,
		}
	}

	pub fn day_of_year(self) -> u16 {
		let leap_day_this_year = if self.year.has_leap_day() { 1 } else { 0 };
		match self.month {
			Month::January => 1,
			Month::February => 32,
			Month::March => 60 + leap_day_this_year,
			Month::April => 91 + leap_day_this_year,
			Month::May => 121 + leap_day_this_year,
			Month::June => 152 + leap_day_this_year,
			Month::July => 182 + leap_day_this_year,
			Month::August => 213 + leap_day_this_year,
			Month::September => 244 + leap_day_this_year,
			Month::October => 274 + leap_day_this_year,
			Month::November => 305 + leap_day_this_year,
			Month::December => 335 + leap_day_this_year,
		}
	}

	pub fn next(self) -> Self {
		if self.month == Month::December {
			Self::new(self.year.next(), Month::January)
		} else {
			Self::new(self.year, self.month.wrapping_next())
		}
	}

	pub fn prev(self) -> Self {
		if self.month == Month::January {
			Self::new(self.year.prev(), Month::December)
		} else {
			Self::new(self.year, self.month.wrapping_prev())
		}
	}

	pub fn with_day(self, day: u8) -> Result<Date, InvalidDayOfMonth> {
		InvalidDayOfMonth::check(self.year, self.month, day)?;
		unsafe {
			Ok(Date::new_unchecked(self.year, self.month, day))
		}
	}

	pub unsafe fn with_day_unchecked(self, day: u8) -> Date {
		Date::new_unchecked(self.year, self.month, day)
	}

	pub fn first_day(self) -> Date {
		Date {
			year: self.year,
			month: self.month,
			day: 1,
		}
	}

	pub fn last_day(self) -> Date {
		Date {
			year: self.year,
			month: self.month,
			day: self.total_days(),
		}
	}
}

impl std::fmt::Display for YearMonth {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:04}-{:02}", self.year.to_number(), self.month().to_number())
	}
}
