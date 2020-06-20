use crate::{Month, Year, YearMonth};

/// The string is not a valid date.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DateParseError {
	InvalidDateSyntax(InvalidDateSyntax),
	InvalidDate(InvalidDate),
}

/// The string does not follow the proper date syntax.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidDateSyntax {
	_private: (),
}

impl InvalidDateSyntax {
	pub fn new() -> Self {
		Self { _private: () }
	}
}

/// The date is not valid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvalidDate {
	InvalidMonthNumber(InvalidMonthNumber),
	InvalidDayForMonth(InvalidDayOfMonth),
}

impl From<core::convert::Infallible> for InvalidDate {
	fn from(_: core::convert::Infallible) -> Self {
		unreachable!()
	}
}

/// The month number is not valid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidMonthNumber {
	pub number: u8,
}

/// The day is not valid for the year and month.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidDayOfMonth {
	pub year: Year,
	pub month: Month,
	pub day: u8,
}

/// The day-of-year is not valid for the year.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidDayOfYear {
	pub year: Year,
	pub day: u16,
}

impl InvalidDayOfMonth {
	pub fn check(year: Year, month: Month, day: u8) -> Result<(), Self> {
		if day < 1 || day > YearMonth::new(year, month).total_days() {
			Err(Self { year, month, day })
		} else {
			Ok(())
		}
	}
}

impl From<InvalidDateSyntax> for DateParseError {
	fn from(other: InvalidDateSyntax) -> Self {
		Self::InvalidDateSyntax(other)
	}
}

impl From<InvalidDate> for DateParseError {
	fn from(other: InvalidDate) -> Self {
		Self::InvalidDate(other)
	}
}

impl From<InvalidMonthNumber> for InvalidDate {
	fn from(other: InvalidMonthNumber) -> Self {
		Self::InvalidMonthNumber(other)
	}
}

impl From<InvalidDayOfMonth> for InvalidDate {
	fn from(other: InvalidDayOfMonth) -> Self {
		Self::InvalidDayForMonth(other)
	}
}

#[cfg(feature = "std")]
mod std_support {
	use super::*;
	impl std::error::Error for DateParseError {}
	impl std::error::Error for InvalidDateSyntax {}
	impl std::error::Error for InvalidDate {}
	impl std::error::Error for InvalidMonthNumber {}
	impl std::error::Error for InvalidDayOfMonth {}
	impl std::error::Error for InvalidDayOfYear {}
}

impl core::fmt::Display for DateParseError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Self::InvalidDateSyntax(e) => write!(f, "{}", e),
			Self::InvalidDate(e) => write!(f, "{}", e),
		}
	}
}

impl core::fmt::Display for InvalidDateSyntax {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "invalid date syntax: expected \"YYYY-MM-DD\"")
	}
}

impl core::fmt::Display for InvalidDate {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Self::InvalidMonthNumber(e) => write!(f, "{}", e),
			Self::InvalidDayForMonth(e) => write!(f, "{}", e),
		}
	}
}

impl core::fmt::Display for InvalidMonthNumber {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "invalid month number: expected 1-12, got {}", self.number)
	}
}

impl core::fmt::Display for InvalidDayOfMonth {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(
			f,
			"invalid day for {} {}: expected 1-{}, got {}",
			self.month,
			self.year,
			YearMonth::new(self.year, self.month).total_days(),
			self.day,
		)
	}
}

impl core::fmt::Display for InvalidDayOfYear {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(
			f,
			"invalid day for of year for {}: expected 1-{}, got {}",
			self.year,
			self.year.total_days(),
			self.day,
		)
	}
}
