use crate::{
	Date,
	December,
	January,
	Month,
	YearMonth,
	InvalidDayOfYear,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Year {
	year: i16,
}

impl Year {
	pub fn new(year: i16) -> Self {
		Self { year }
	}

	pub fn to_number(self) -> i16 {
		self.year
	}

	pub fn has_leap_day(self) -> bool {
		self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0)
	}

	pub fn total_days(self) -> u16 {
		if self.has_leap_day() {
			366
		} else {
			365
		}
	}

	pub fn next(self) -> Self {
		self + 1
	}

	pub fn prev(self) -> Self {
		self - 1
	}

	pub fn with_month(self, month: Month) -> YearMonth {
		YearMonth::new(self, month)
	}

	pub fn with_day_of_year(self, day: u16) -> Result<Date, InvalidDayOfYear> {
		if day < 1 || day > self.total_days() {
			return Err(InvalidDayOfYear { year: self, day });
		}

		#[allow(array_into_iter)]
		for month in self.months().into_iter().rev() {
			if day >= month.day_of_year() {
				let day_of_month = (day - month.day_of_year()) as u8 + 1;
				return Ok(unsafe { month.with_day_unchecked(day_of_month) });
			}
		}

		unreachable!()
	}

	pub fn first_month(self) -> YearMonth {
		self.with_month(January)
	}

	pub fn last_month(self) -> YearMonth {
		self.with_month(December)
	}

	pub fn months(self) -> [YearMonth; 12] {
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

	pub fn first_day(self) -> Date {
		Date {
			year: self,
			month: January,
			day: 1,
		}
	}

	pub fn last_day(self) -> Date {
		Date {
			year: self,
			month: December,
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
	fn partial_cmp(&self, other: &i16) -> Option<std::cmp::Ordering> {
		Some(self.to_number().cmp(other))
	}
}

impl std::ops::Add<i16> for Year {
	type Output = Self;

	fn add(self, other: i16) -> Self {
		Self::new(self.to_number() + other)
	}
}

impl std::ops::Sub<i16> for Year {
	type Output = Self;

	fn sub(self, other: i16) -> Self {
		Self::new(self.to_number() - other)
	}
}

impl std::ops::AddAssign<i16> for Year {
	fn add_assign(&mut self, other: i16) {
		self.year += other
	}
}

impl std::ops::SubAssign<i16> for Year {
	fn sub_assign(&mut self, other: i16) {
		self.year -= other
	}
}

impl std::fmt::Display for Year {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:04}", self.year)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use assert2::assert;

	#[test]
	fn test_is_leap_year() {
		assert!(Year::new(2020).has_leap_day() == true);
		assert!(Year::new(2021).has_leap_day() == false);
		assert!(Year::new(1900).has_leap_day() == false);
		assert!(Year::new(2000).has_leap_day() == true);
	}
}
