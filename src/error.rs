use crate::{
	Year,
	Month,
	YearMonth,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DateParseError {
	InvalidDateSyntax(InvalidDateSyntax),
	InvalidDate(InvalidDate),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidDateSyntax {
	pub data: String,
}

impl InvalidDateSyntax {
	pub fn new(data: impl Into<String>) -> Self {
		Self { data: data.into() }
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvalidDate {
	InvalidMonthNumber(InvalidMonthNumber),
	InvalidDayForMonth(InvalidDayOfMonth),
}

impl From<std::convert::Infallible> for InvalidDate {
	fn from(_: std::convert::Infallible) -> Self {
		unreachable!()
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidMonthNumber {
	pub number: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidDayOfMonth {
	pub year: Year,
	pub month: Month,
	pub day: u8,
}

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

impl std::error::Error for DateParseError {}
impl std::error::Error for InvalidDateSyntax {}
impl std::error::Error for InvalidDate {}
impl std::error::Error for InvalidMonthNumber {}
impl std::error::Error for InvalidDayOfMonth {}
impl std::error::Error for InvalidDayOfYear {}

impl std::fmt::Display for DateParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::InvalidDateSyntax(e) => write!(f, "{}", e),
			Self::InvalidDate(e) => write!(f, "{}", e),
		}
	}
}

impl std::fmt::Display for InvalidDateSyntax {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "invalid date syntax: expected \"YYYY-MM-DD\", got {:?}", self.data)
	}
}

impl std::fmt::Display for InvalidDate {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::InvalidMonthNumber(e) => write!(f, "{}", e),
			Self::InvalidDayForMonth(e) => write!(f, "{}", e),
		}
	}
}

impl std::fmt::Display for InvalidMonthNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "invalid month number: expected 1-12, got {}", self.number)
	}
}

impl std::fmt::Display for InvalidDayOfMonth {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl std::fmt::Display for InvalidDayOfYear {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"invalid day for of year for {}: expected 1-{}, got {}",
			self.year,
			self.year.total_days(),
			self.day,
		)
	}
}
