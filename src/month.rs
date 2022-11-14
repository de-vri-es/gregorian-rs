use crate::{InvalidMonthNumber, Year, YearMonth};

/// All months in order as array.
pub const MONTHS: [Month; 12] = [
	January, February, March, April, May, June, July, August, September, October, November, December,
];

/// A month on the Gregorian calendar.
#[repr(u8)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize),
	serde(try_from = "u8", into = "u8")
)]
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
	pub const fn new(month: u8) -> Result<Self, InvalidMonthNumber> {
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
	pub const fn to_number(self) -> u8 {
		self as u8
	}

	const fn from_number(number: u8) -> Self {
		match number {
			1 => Self::January,
			2 => Self::February,
			3 => Self::March,
			4 => Self::April,
			5 => Self::May,
			6 => Self::June,
			7 => Self::July,
			8 => Self::August,
			9 => Self::September,
			10 => Self::October,
			11 => Self::November,
			12 => Self::December,
			//TODO: Replace this with unreachable!() when const_panic is stabilized.
			_ => Self::January,
		}
	}

	/// Combine the month with a year to create a [`YearMonth`].
	pub fn with_year(self, year: impl Into<Year>) -> YearMonth {
		YearMonth::new(year, self)
	}

	/// Add a number of months, wrapping back to January after December.
	pub const fn wrapping_add(self, count: i8) -> Self {
		let count = if count < 0 {
			(count % 12) + 12
		} else {
			count % 12
		};
		let index = (self.to_number() as i8 - 1 + count) % 12;
		Self::from_number(index as u8 + 1)
	}

	/// Add a number of months, wrapping back to December after January.
	pub const fn wrapping_sub(self, count: i8) -> Self {
		// Take remainder after dividing by 12 before negating,
		// to prevent negating i8::MIN.
		self.wrapping_add(-(count % 12))
	}

	/// Get the next month, wrapping back to January after December.
	pub const fn wrapping_next(self) -> Self {
		self.wrapping_add(1)
	}

	/// Get the previous month, wrapping back to December after January.
	pub const fn wrapping_prev(self) -> Self {
		self.wrapping_add(-1)
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
	use assert2::{assert, let_assert};

	#[test]
	fn to_number() {
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
	fn with_year() {
		assert!(January.with_year(2020) == YearMonth::new(2020, January));
		assert!(February.with_year(-1200) == YearMonth::new(-1200, February));
	}

	#[test]
	fn wrapping_add() {
		assert!(January.wrapping_add(0) == January);
		assert!(January.wrapping_add(1) == February);
		assert!(January.wrapping_add(2) == March);
		assert!(January.wrapping_add(12) == January);
		assert!(January.wrapping_add(13) == February);

		assert!(January.wrapping_add(-1) == December);
		assert!(January.wrapping_add(-2) == November);
		assert!(January.wrapping_add(-12) == January);
		assert!(January.wrapping_add(-13) == December);
	}

	#[test]
	fn wrapping_sub() {
		assert!(January.wrapping_sub(0) == January);
		assert!(January.wrapping_sub(1) == December);
		assert!(January.wrapping_sub(2) == November);
		assert!(January.wrapping_sub(12) == January);
		assert!(January.wrapping_sub(13) == December);

		assert!(January.wrapping_sub(-0) == January);
		assert!(January.wrapping_sub(-1) == February);
		assert!(January.wrapping_sub(-2) == March);
		assert!(January.wrapping_sub(-12) == January);
		assert!(January.wrapping_sub(-13) == February);
	}

	#[test]
	#[cfg(feature = "std")]
	fn format() {
		assert!(format!("{}", January) == "January");
		assert!(format!("{:?}", January) == "January");
	}

	#[test]
	fn serde() {
		#[derive(Debug, serde::Deserialize, serde::Serialize)]
		struct Container {
			month: Month,
		}

		#[track_caller]
		fn serialize(month: Month) -> String {
			let_assert!(Ok(serialized) = serde_yaml::to_string(&Container { month }));
			serialized
		}

		#[track_caller]
		fn deserialize(data: &str) -> Month {
			let_assert!(Ok(deserialized) = serde_yaml::from_str::<Container>(data));
			deserialized.month
		}

		assert!(serialize(Month::January) == "month: 1\n");
		assert!(serialize(Month::February) == "month: 2\n");
		assert!(serialize(Month::March) == "month: 3\n");
		assert!(serialize(Month::April) == "month: 4\n");
		assert!(serialize(Month::May) == "month: 5\n");
		assert!(serialize(Month::June) == "month: 6\n");
		assert!(serialize(Month::July) == "month: 7\n");
		assert!(serialize(Month::August) == "month: 8\n");
		assert!(serialize(Month::September) == "month: 9\n");
		assert!(serialize(Month::October) == "month: 10\n");
		assert!(serialize(Month::November) == "month: 11\n");
		assert!(serialize(Month::December) == "month: 12\n");

		assert!(deserialize("month: 1\n") == Month::January);
		assert!(deserialize("month: 2\n") == Month::February);
		assert!(deserialize("month: 3\n") == Month::March);
		assert!(deserialize("month: 4\n") == Month::April);
		assert!(deserialize("month: 5\n") == Month::May);
		assert!(deserialize("month: 6\n") == Month::June);
		assert!(deserialize("month: 7\n") == Month::July);
		assert!(deserialize("month: 8\n") == Month::August);
		assert!(deserialize("month: 9\n") == Month::September);
		assert!(deserialize("month: 10\n") == Month::October);
		assert!(deserialize("month: 11\n") == Month::November);
		assert!(deserialize("month: 12\n") == Month::December);

		let_assert!(Err(e) = serde_yaml::from_str::<Container>("month: 0"));
		assert!(e.to_string() == "invalid month number: expected 1-12, got 0");
		let_assert!(Err(e) = serde_yaml::from_str::<Container>("month: 13"));
		assert!(e.to_string() == "invalid month number: expected 1-12, got 13");
	}
}
