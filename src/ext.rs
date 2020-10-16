use crate::{Date, InvalidDayOfMonth};

/// Extension for `Result<Date>` to round invalid dates.
trait DateResultExt {
	/// Get the date or the next valid date.
	///
	/// This function gives the first day of the next month for the invalid date.
	/// It ignores any excess days in the invalid date.
	fn or_next_valid(&self) -> Date;

	/// Get the date or the next valid date.
	///
	/// This function gives the last day of the current month for the invalid date.
	/// It ignores any excess days in the invalid date.
	fn or_prev_valid(&self) -> Date;
}

impl DateResultExt for Result<Date, InvalidDayOfMonth> {
	fn or_next_valid(&self) -> Date {
		self.unwrap_or_else(|e| e.next_valid())
	}

	fn or_prev_valid(&self) -> Date {
		self.unwrap_or_else(|e| e.prev_valid())
	}
}
