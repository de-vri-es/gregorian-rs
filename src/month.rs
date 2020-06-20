use crate::{
	InvalidMonthNumber,
	Year,
	YearMonth,
};

/// All months in order as array.
pub const MONTHS: [Month; 12] = [
	January,
	February,
	March,
	April,
	May,
	June,
	July,
	August,
	September,
	October,
	November,
	December,
];

/// A month on the Gregorian calendar.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Month {
	January = 1,
	February = 2,
	March = 3,
	April = 4,
	May = 5,
	June = 6,
	July = 7,
	August = 8,
	September = 9,
	October = 10,
	November = 11,
	December = 12,
}

pub use Month::*;

impl Month {
	/// Create a new month from a month number.
	///
	/// The number must be in the range 1-12 (inclusive).
	pub fn new(month: u8) -> Result<Self, InvalidMonthNumber> {
		match month {
			1 => Ok(Self::January),
			2 => Ok(Self::February),
			3 => Ok(Self::March),
			4 => Ok(Self::April),
			5 => Ok(Self::May),
			6 => Ok(Self::June),
			7 => Ok(Self::July),
			8 => Ok(Self::August),
			9 => Ok(Self::September),
			10 => Ok(Self::October),
			11 => Ok(Self::November),
			12 => Ok(Self::December),
			number => Err(InvalidMonthNumber { number }),
		}
	}

	/// Create a new month from a month number, without checking for validity.
	///
	/// # Safety
	/// The month number will be transmuted directly to [`Month`].
	/// If the number is not a valid enum variant, this triggers undefined behaviour.
	pub unsafe fn new_unchecked(month: u8) -> Self {
		core::mem::transmute(month)
	}

	/// Get the month number in the range 1-12.
	pub fn to_number(self) -> u8 {
		self as u8
	}

	/// Combine the month with a year to create a [`YearMonth`].
	pub fn with_year(self, year: impl Into<Year>) -> YearMonth {
		YearMonth::new(year, self)
	}

	/// Add a number of months, wrapping back to January after December.
	pub fn wrapping_add(self, count: u8) -> Self {
		let index = self.to_number() - 1;
		let index = index.wrapping_add(count) % 12;
		unsafe {
			Self::new_unchecked(index + 1)
		}
	}

	/// Add a number of months, wrapping back to December after January.
	pub fn wrapping_sub(self, count: u8) -> Self {
		self.wrapping_add(12 - count % 12)
	}

	/// Get the next month, wrapping back to January after December.
	pub fn wrapping_next(self) -> Self {
		self.wrapping_add(1)
	}

	/// Get the previous month, wrapping back to December after January.
	pub fn wrapping_prev(self) -> Self {
		self.wrapping_add(11)
	}
}

impl core::convert::TryFrom<u8> for Month {
	type Error = InvalidMonthNumber;

	fn try_from(other: u8) -> Result<Self, Self::Error> {
		Self::new(other)
	}
}

impl From<Month> for u8 {
	fn from(other: Month) -> Self {
		other.to_number()
	}
}

impl PartialEq<u8> for Month {
	fn eq(&self, other: &u8) -> bool {
		self.to_number() == *other
	}
}

impl core::fmt::Display for Month {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		// Delegate to Debug.
		write!(f, "{:?}", self)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use assert2::assert;

	#[test]
	fn test_number() {
		assert!(let Err(InvalidMonthNumber { number: 0 }) = Month::new(0));
		assert!(let Err(InvalidMonthNumber { number: 13 }) = Month::new(13));
		assert!(let Ok(January) = Month::new(1));
		assert!(let Ok(February) = Month::new(2));
		assert!(let Ok(March) = Month::new(3));
		assert!(let Ok(April) = Month::new(4));
		assert!(let Ok(May) = Month::new(5));
		assert!(let Ok(June) = Month::new(6));
		assert!(let Ok(July) = Month::new(7));
		assert!(let Ok(August) = Month::new(8));
		assert!(let Ok(September) = Month::new(9));
		assert!(let Ok(October) = Month::new(10));
		assert!(let Ok(November) = Month::new(11));
		assert!(let Ok(December) = Month::new(12));

		unsafe {
			assert!(Month::new_unchecked(1) == January);
			assert!(Month::new_unchecked(2) == February);
			assert!(Month::new_unchecked(3) == March);
			assert!(Month::new_unchecked(4) == April);
			assert!(Month::new_unchecked(5) == May);
			assert!(Month::new_unchecked(6) == June);
			assert!(Month::new_unchecked(7) == July);
			assert!(Month::new_unchecked(8) == August);
			assert!(Month::new_unchecked(9) == September);
			assert!(Month::new_unchecked(10) == October);
			assert!(Month::new_unchecked(11) == November);
			assert!(Month::new_unchecked(12) == December);
		}

		assert!(January.to_number() == 1);
		assert!(February.to_number() == 2);
		assert!(March.to_number() == 3);
		assert!(April.to_number() == 4);
		assert!(May.to_number() == 5);
		assert!(June.to_number() == 6);
		assert!(July.to_number() == 7);
		assert!(August.to_number() == 8);
		assert!(September.to_number() == 9);
		assert!(October.to_number() == 10);
		assert!(November.to_number() == 11);
		assert!(December.to_number() == 12);

		assert!(January == 1);
		assert!(February == 2);
		assert!(March == 3);
		assert!(April == 4);
		assert!(May == 5);
		assert!(June == 6);
		assert!(July == 7);
		assert!(August == 8);
		assert!(September == 9);
		assert!(October == 10);
		assert!(November == 11);
		assert!(December == 12);
	}

	#[test]
	fn test_with_year() {
		assert!(January.with_year(2020) == YearMonth::new(2020, January));
		assert!(February.with_year(-1200) == YearMonth::new(-1200, February));
	}

	#[test]
	fn test_wrapping_add() {
		assert!(January.wrapping_add(1) == February);
		assert!(January.wrapping_add(2) == March);
		assert!(January.wrapping_add(13) == February);
	}

	#[test]
	fn test_wrapping_sub() {
		assert!(January.wrapping_sub(1) == December);
		assert!(January.wrapping_sub(2) == November);
		assert!(January.wrapping_sub(13) == December);
	}
}
