use crate::Month;

/// Get the month and day of month for a day of year.
pub const fn month_and_day_from_day_of_year(day_of_year: u16, leap_year: bool) -> Result<(Month, u8), ()> {
	if !leap_year {
		match day_of_year {
			1..=31 => Ok((Month::January, day_of_year as u8)),
			32..=59 => Ok((Month::February, (day_of_year - 31) as u8)),
			60..=90 => Ok((Month::March, (day_of_year - 59) as u8)),
			91..=120 => Ok((Month::April, (day_of_year - 90) as u8)),
			121..=151 => Ok((Month::May, (day_of_year - 120) as u8)),
			152..=181 => Ok((Month::June, (day_of_year - 151) as u8)),
			182..=212 => Ok((Month::July, (day_of_year - 181) as u8)),
			213..=243 => Ok((Month::August, (day_of_year - 212) as u8)),
			244..=273 => Ok((Month::September, (day_of_year - 243) as u8)),
			274..=304 => Ok((Month::October, (day_of_year - 273) as u8)),
			305..=334 => Ok((Month::November, (day_of_year - 304) as u8)),
			335..=365 => Ok((Month::December, (day_of_year - 334) as u8)),
			0 | 366..= u16::MAX => Err(()),
		}
	} else {
		match day_of_year {
			1..=31 => Ok((Month::January, day_of_year as u8)),
			32..=60 => Ok((Month::February, (day_of_year - 31) as u8)),
			61..=91 => Ok((Month::March, (day_of_year - 60) as u8)),
			92..=121 => Ok((Month::April, (day_of_year - 91) as u8)),
			122..=152 => Ok((Month::May, (day_of_year - 121) as u8)),
			153..=182 => Ok((Month::June, (day_of_year - 152) as u8)),
			183..=213 => Ok((Month::July, (day_of_year - 182) as u8)),
			214..=244 => Ok((Month::August, (day_of_year - 213) as u8)),
			245..=274 => Ok((Month::September, (day_of_year - 244) as u8)),
			275..=305 => Ok((Month::October, (day_of_year - 274) as u8)),
			306..=335 => Ok((Month::November, (day_of_year - 305) as u8)),
			336..=366 => Ok((Month::December, (day_of_year - 335) as u8)),
			0 | 367..= u16::MAX => Err(()),
		}
	}
}

/// Get the numbers of days in a month.
pub const fn days_in_month(month: Month, leap_year: bool) -> u8 {
	match month {
		Month::January => 31,
		Month::February => if leap_year { 29 } else { 28 },
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

/// Get the day of year for the start of a month.
pub const fn start_day_of_year(month: Month, leap_year: bool) -> u16 {
	if !leap_year {
		match month {
			Month::January => 1,
			Month::February => 32,
			Month::March => 60,
			Month::April => 91,
			Month::May => 121,
			Month::June => 152,
			Month::July => 182,
			Month::August => 213,
			Month::September => 244,
			Month::October => 274,
			Month::November => 305,
			Month::December => 335,
		}
	} else {
		match month {
			Month::January => 1,
			Month::February => 32,
			Month::March => 61,
			Month::April => 92,
			Month::May => 122,
			Month::June => 153,
			Month::July => 183,
			Month::August => 214,
			Month::September => 245,
			Month::October => 275,
			Month::November => 306,
			Month::December => 336,
		}
	}
}

/// Get the day of year for a day of month.
pub const fn day_of_year(month: Month, day_of_month: u8, leap_year: bool) -> u16 {
	start_day_of_year(month, leap_year) - 1 + day_of_month as u16
}
