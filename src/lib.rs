#![allow(dead_code)]
extern crate radish;

use std::fmt;
use std::str::{self, FromStr};

use radish::ascii::strtoi;

// ---------------------------------------------------------------------------
// Ported from datetime.h
// ---------------------------------------------------------------------------


// ----------------------------------------------------------------
//              time types + support macros
//
// String definitions for standard time quantities.
//
// These strings are the defaults used to form output time strings.
// Other alternative forms are hardcoded into token tables in datetime.c.
// ----------------------------------------------------------------
const DAGO       :&'static [u8] = b"ago";
const DCURRENT   :&'static [u8] = b"current";
const EPOCH      :&'static [u8] = b"epoch";
const INVALID    :&'static [u8] = b"invalid";
const EARLY      :&'static [u8] = b"-infinity";
const LATE       :&'static [u8] = b"infinity";
const NOW        :&'static [u8] = b"now";
const TODAY      :&'static [u8] = b"today";
const TOMORROW   :&'static [u8] = b"tomorrow";
const YESTERDAY  :&'static [u8] = b"yesterday";
const ZULU       :&'static [u8] = b"zulu";

const DMICROSEC  :&'static [u8] = b"usecond";
const DMILLISEC  :&'static [u8] = b"msecond";
const DSECOND    :&'static [u8] = b"second";
const DMINUTE    :&'static [u8] = b"minute";
const DHOUR      :&'static [u8] = b"hour";
const DDAY       :&'static [u8] = b"day";
const DWEEK      :&'static [u8] = b"week";
const DMONTH     :&'static [u8] = b"month";
const DQUARTER   :&'static [u8] = b"quarter";
const DYEAR      :&'static [u8] = b"year";
const DDECADE    :&'static [u8] = b"decade";
const DCENTURY   :&'static [u8] = b"century";
const DMILLENNIUM:&'static [u8] = b"millennium";
const DA_D       :&'static [u8] = b"ad";
const DB_C       :&'static [u8] = b"bc";
const DTIMEZONE  :&'static [u8] = b"timezone";

// Fundamental time field definitions for parsing.
//
// Meridian:  am, pm, or 24-hour style.
// Millennium: ad, bc
const AM   :i32 = 0;
const PM   :i32 = 1;
const HR24 :i32 = 2;

const AD   :i32 = 0;
const BC   :i32 = 1;

// Field types for time decoding.
//
// Can't have more of these than there are bits in an unsigned int
// since these are turned into bit masks during parsing and decoding.
//
// Furthermore, the values for YEAR, MONTH, DAY, HOUR, MINUTE, SECOND
// must be in the range 0..14 so that the associated bitmasks can fit
// into the left half of an INTERVAL's typmod value.  Since those bits
// are stored in typmods, you can't change them without initdb!

const RESERV        :i8 = 0;
const MONTH         :i8 = 1;
const YEAR          :i8 = 2;
const DAY           :i8 = 3;
const JULIAN        :i8 = 4;
/// fixed-offset timezone abbreviation
const TZ            :i8 = 5;
/// fixed-offset timezone abbrev, DST
const DTZ           :i8 = 6;
/// dynamic timezone abbreviation
const DYNTZ         :i8 = 7;
const IGNORE_DTF    :i8 = 8;
const AMPM          :i8 = 9;
const HOUR          :i8 = 10;
const MINUTE        :i8 = 11;
const SECOND        :i8 = 12;
const MILLISECOND   :i8 = 13;
const MICROSECOND   :i8 = 14;
const DOY           :i8 = 15;
const DOW           :i8 = 16;
const UNITS         :i8 = 17;
const ADBC          :i8 = 18;
const AGO           :i8 = 19; /// these are only for relative dates
const ABS_BEFORE    :i8 = 20;
const ABS_AFTER     :i8 = 21;
const ISODATE       :i8 = 22; // generic fields to help with parsing
const ISOTIME       :i8 = 23;
const WEEK          :i8 = 24; // these are only for parsing intervals
const DECADE        :i8 = 25;
const CENTURY       :i8 = 26;
const MILLENNIUM    :i8 = 27;
/// hack for parsing two-word timezone specs "MET DST" etc
const DTZMOD        :i8 = 28; // "DST" as a separate word
/// reserved for unrecognized string values
const UNKNOWN_FIELD :i8 = 31;



// Token field definitions for time parsing and decoding.
//
// Some field type codes (see above) use these as the "value" in datetktbl[].
// These are also used for bit masks in DecodeDateTime and friends
//  so actually restrict them to within [0,31] for now.
// - thomas 97/06/19
// Not all of these fields are used for masks in DecodeDateTime
//  so allow some larger than 31. - thomas 1997-11-17
//
// Caution: there are undocumented assumptions in the code that most of these
// values are not equal to IGNORE_DTF nor RESERV.  Be very careful when
// renumbering values in either of these apparently-independent lists :-(
const DTK_NUMBER     :i32 = 0;
const DTK_STRING     :i32 = 1;

const DTK_DATE       :i32 = 2;
const DTK_TIME       :i32 = 3;
const DTK_TZ         :i32 = 4;
const DTK_AGO        :i32 = 5;

const DTK_SPECIAL    :i32 = 6;
const DTK_INVALID    :i32 = 7;
const DTK_CURRENT    :i32 = 8;
const DTK_EARLY      :i32 = 9;
const DTK_LATE       :i32 = 10;
const DTK_EPOCH      :i32 = 11;
const DTK_NOW        :i32 = 12;
const DTK_YESTERDAY  :i32 = 13;
const DTK_TODAY      :i32 = 14;
const DTK_TOMORROW   :i32 = 15;
const DTK_ZULU       :i32 = 16;

const DTK_DELTA      :i32 = 17;
const DTK_SECOND     :i32 = 18;
const DTK_MINUTE     :i32 = 19;
const DTK_HOUR       :i32 = 20;
const DTK_DAY        :i32 = 21;
const DTK_WEEK       :i32 = 22;
const DTK_MONTH      :i32 = 23;
const DTK_QUARTER    :i32 = 24;
const DTK_YEAR       :i32 = 25;
const DTK_DECADE     :i32 = 26;
const DTK_CENTURY    :i32 = 27;
const DTK_MILLENNIUM :i32 = 28;
const DTK_MILLISEC   :i32 = 29;
const DTK_MICROSEC   :i32 = 30;
const DTK_JULIAN     :i32 = 31;

const DTK_DOW        :i32 = 32;
const DTK_DOY        :i32 = 33;
const DTK_TZ_HOUR    :i32 = 34;
const DTK_TZ_MINUTE  :i32 = 35;
const DTK_ISOYEAR    :i32 = 36;
const DTK_ISODOW     :i32 = 37;


// ---------------------------------------------------------------------------
// Ported from Timestamp.h
// ---------------------------------------------------------------------------
const MAX_TIMESTAMP_PRECISION :i32 = 6;
const MAX_INTERVAL_PRECISION  :i32 = 6;


// Assorted constants for datetime-related calculations
const DAYS_PER_YEAR    :f32 = 365.25; // assumes leap year every four years
const MONTHS_PER_YEAR  :i32  = 12;

// DAYS_PER_MONTH is very imprecise.  The more accurate value is
// 365.2425/12 = 30.436875, or '30 days 10:29:06'.  Right now we only
// return an integral number of days, but someday perhaps we should
// also return a 'time' value to be used as well.  ISO 8601 suggests
// 30 days.
const DAYS_PER_MONTH   :i32 = 30; // assumes exactly 30 days per month
const HOURS_PER_DAY    :i32 = 24; // assume no daylight savings time changes

// This doesn't adjust for uneven daylight savings time intervals or leap
// seconds, and it crudely estimates leap years.  A more accurate value
// for days per years is 365.2422.
const SECS_PER_YEAR    :i32 = (36525 * 864); /* avoid floating-point computation */
const SECS_PER_DAY     :i32 = 86400;
const SECS_PER_HOUR    :i32 = 3600;
const SECS_PER_MINUTE  :i32 = 60;
const MINS_PER_HOUR    :i32 = 60;

const USECS_PER_DAY    :i64 = 86400000000;
const USECS_PER_HOUR   :i64 = 3600000000;
const USECS_PER_MINUTE :i64 = 60000000;
const USECS_PER_SEC    :i64 = 1000000;

/// maximum allowed hour part
const MAX_TZDISP_HOUR  : i32 = 15;

// ---------------------------------------------------------------------------
// Ported from datetime.c
// ---------------------------------------------------------------------------

const DAY_TAB: [[i32;13];2] = [
  [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 0],
  [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 0]
];

// Removed NULL
const MONTHS: [&'static str;12] = [
   "Jan", "Feb", "Mar", "Apr", "May", "Jun",
   "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const DAYS: [&'static str;7] = [
  "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
];

pub struct DateToken {
  token: &'static [u8],
  ty: i8,
  value: i32
}

macro_rules! token {
  ($token:expr, $ty:expr, $value:expr) => {
    DateToken {
      token: $token,
      ty: $ty,
      value: $value
    }
  }
}

const DATEK_TBL: [DateToken;74] = [
  token!(EARLY, RESERV, DTK_EARLY),
  token!(DA_D, ADBC, AD),                     // "ad" for years > 0
  token!(b"allballs", RESERV, DTK_ZULU),      // 00:00:00
  token!(b"am", AMPM, AM),
  token!(b"apr", MONTH, 4),
  token!(b"april", MONTH, 4),
  token!(b"at", IGNORE_DTF, 0),               // "at" (throwaway)
  token!(b"aug", MONTH, 8),
  token!(b"august", MONTH, 8),
  token!(DB_C, ADBC, BC),                     // "bc" for years <= 0
  token!(DCURRENT, RESERV, DTK_CURRENT),      // "current" is always now
  token!(b"d", UNITS, DTK_DAY),               // "day of month" for ISO input
  token!(b"dec", MONTH, 12),
  token!(b"december", MONTH, 12),
  token!(b"dow", RESERV, DTK_DOW),            // day of week
  token!(b"doy", RESERV, DTK_DOY),            // day of year
  token!(b"dst", DTZMOD, SECS_PER_HOUR),
  token!(EPOCH, RESERV, DTK_EPOCH),           // "epoch" reserved for system epoch time
  token!(b"feb", MONTH, 2),
  token!(b"february", MONTH, 2),
  token!(b"fri", DOW, 5),
  token!(b"friday", DOW, 5),
  token!(b"h", UNITS, DTK_HOUR),              // "hour"
  token!(LATE, RESERV, DTK_LATE),             // "infinity" reserved for "late time"
  token!(INVALID, RESERV, DTK_INVALID),       // "invalid" reserved for bad time
  token!(b"isodow", RESERV, DTK_ISODOW),      // ISO day of week, Sunday == 7
  token!(b"isoyear", UNITS, DTK_ISOYEAR),     // year in terms of the ISO week date
  token!(b"j", UNITS, DTK_JULIAN),
  token!(b"jan", MONTH, 1),
  token!(b"january", MONTH, 1),
  token!(b"jd", UNITS, DTK_JULIAN),
  token!(b"jul", MONTH, 7),
  token!(b"julian", UNITS, DTK_JULIAN),
  token!(b"july", MONTH, 7),
  token!(b"jun", MONTH, 6),
  token!(b"june", MONTH, 6),
  token!(b"m", UNITS, DTK_MONTH),              // "month" for ISO input
  token!(b"mar", MONTH, 3),
  token!(b"march", MONTH, 3),
  token!(b"may", MONTH, 5),
  token!(b"mm", UNITS, DTK_MINUTE),            // "minute" for ISO input
  token!(b"mon", DOW, 1),
  token!(b"monday", DOW, 1),
  token!(b"nov", MONTH, 11),
  token!(b"november", MONTH, 11),
  token!(NOW, RESERV, DTK_NOW),                // current transaction time
  token!(b"oct", MONTH, 10),
  token!(b"october", MONTH, 10),
  token!(b"on", IGNORE_DTF, 0),                // "on" (throwaway)
  token!(b"pm", AMPM, PM),
  token!(b"s", UNITS, DTK_SECOND),             // "seconds" for ISO input
  token!(b"sat", DOW, 6),
  token!(b"saturday", DOW, 6),
  token!(b"sep", MONTH, 9),
  token!(b"sept", MONTH, 9),
  token!(b"september", MONTH, 9),
  token!(b"sun", DOW, 0),
  token!(b"sunday", DOW, 0),
  token!(b"t", ISOTIME, DTK_TIME),             // Filler for ISO time fields
  token!(b"thu", DOW, 4),
  token!(b"thur", DOW, 4),
  token!(b"thurs", DOW, 4),
  token!(b"thursday", DOW, 4),
  token!(TODAY, RESERV, DTK_TODAY),            // midnight
  token!(TOMORROW, RESERV, DTK_TOMORROW),      // tomorrow midnight
  token!(b"tue", DOW, 2),
  token!(b"tues", DOW, 2),
  token!(b"tuesday", DOW, 2),
  token!(b"undefined", RESERV, DTK_INVALID),   // pre-v6.1 invalid time
  token!(b"wed", DOW, 3),
  token!(b"wednesday", DOW, 3),
  token!(b"weds", DOW, 3),
  token!(b"y", UNITS, DTK_YEAR),               // "year" for ISO input
  token!(YESTERDAY, RESERV, DTK_YESTERDAY)     // yesterday midnight
];


const DELTATK_TBL: [DateToken;63] = [
  token!(b"@", IGNORE_DTF, 0),                 // postgres relative prefix
  token!(DAGO, AGO, 0),                        // "ago" indicates negative time offset
  token!(b"c", UNITS, DTK_CENTURY),            // "century" relative
  token!(b"cent", UNITS, DTK_CENTURY),         // "century" relative
  token!(b"centuries", UNITS, DTK_CENTURY),    // "centuries" relative
  token!(DCENTURY, UNITS, DTK_CENTURY),        // "century" relative
  token!(b"d", UNITS, DTK_DAY),                // "day" relative
  token!(DDAY, UNITS, DTK_DAY),                // "day" relative
  token!(b"days", UNITS, DTK_DAY),             // "days" relative
  token!(b"dec", UNITS, DTK_DECADE),           // "decade" relative
  token!(DDECADE, UNITS, DTK_DECADE),          // "decade" relative
  token!(b"decades", UNITS, DTK_DECADE),       // "decades" relative
  token!(b"decs", UNITS, DTK_DECADE),          // "decades" relative
  token!(b"h", UNITS, DTK_HOUR),               // "hour" relative
  token!(DHOUR, UNITS, DTK_HOUR),              // "hour" relative
  token!(b"hours", UNITS, DTK_HOUR),           // "hours" relative
  token!(b"hr", UNITS, DTK_HOUR),              // "hour" relative
  token!(b"hrs", UNITS, DTK_HOUR),             // "hours" relative
  token!(INVALID, RESERV, DTK_INVALID),        // reserved for invalid time
  token!(b"m", UNITS, DTK_MINUTE),             // "minute" relative
  token!(b"microsecon", UNITS, DTK_MICROSEC),  // "microsecond" relative
  token!(b"mil", UNITS, DTK_MILLENNIUM),       // "millennium" relative
  token!(b"millennia", UNITS, DTK_MILLENNIUM), // "millennia" relative
  token!(DMILLENNIUM, UNITS, DTK_MILLENNIUM),  // "millennium" relative
  token!(b"millisecon", UNITS, DTK_MILLISEC),  // relative
  token!(b"mils", UNITS, DTK_MILLENNIUM),      // "millennia" relative
  token!(b"min", UNITS, DTK_MINUTE),           // "minute" relative
  token!(b"mins", UNITS, DTK_MINUTE),          // "minutes" relative
  token!(DMINUTE, UNITS, DTK_MINUTE),          // "minute" relative
  token!(b"minutes", UNITS, DTK_MINUTE),       // "minutes" relative
  token!(b"mon", UNITS, DTK_MONTH),            // "months" relative
  token!(b"mons", UNITS, DTK_MONTH),           // "months" relative
  token!(DMONTH, UNITS, DTK_MONTH),            // "month" relative
  token!(b"months", UNITS, DTK_MONTH),
  token!(b"ms", UNITS, DTK_MILLISEC),
  token!(b"msec", UNITS, DTK_MILLISEC),
  token!(DMILLISEC, UNITS, DTK_MILLISEC),
  token!(b"mseconds", UNITS, DTK_MILLISEC),
  token!(b"msecs", UNITS, DTK_MILLISEC),
  token!(b"qtr", UNITS, DTK_QUARTER),          // "quarter" relative
  token!(DQUARTER, UNITS, DTK_QUARTER),        // "quarter" relative
  token!(b"s", UNITS, DTK_SECOND),
  token!(b"sec", UNITS, DTK_SECOND),
  token!(DSECOND, UNITS, DTK_SECOND),
  token!(b"seconds", UNITS, DTK_SECOND),
  token!(b"secs", UNITS, DTK_SECOND),
  token!(DTIMEZONE, UNITS, DTK_TZ),            // "timezone" time offset
  token!(b"timezone_h", UNITS, DTK_TZ_HOUR),   // timezone hour units
  token!(b"timezone_m", UNITS, DTK_TZ_MINUTE), // timezone minutes units
  token!(b"undefined", RESERV, DTK_INVALID),   // pre-v6.1 invalid time
  token!(b"us", UNITS, DTK_MICROSEC),          // "microsecond" relative
  token!(b"usec", UNITS, DTK_MICROSEC),        // "microsecond" relative
  token!(DMICROSEC, UNITS, DTK_MICROSEC),      // "microsecond" relative
  token!(b"useconds", UNITS, DTK_MICROSEC),    // "microseconds" relative
  token!(b"usecs", UNITS, DTK_MICROSEC),       // "microseconds" relative
  token!(b"w", UNITS, DTK_WEEK),               // "week" relative
  token!(DWEEK, UNITS, DTK_WEEK),              // "week" relative
  token!(b"weeks", UNITS, DTK_WEEK),           // "weeks" relative
  token!(b"y", UNITS, DTK_YEAR),               // "year" relative
  token!(DYEAR, UNITS, DTK_YEAR),              // "year" relative
  token!(b"years", UNITS, DTK_YEAR),           // "years" relative
  token!(b"yr", UNITS, DTK_YEAR),              // "year" relative
  token!(b"yrs", UNITS, DTK_YEAR)              // "years" relative
];


/// Calendar time to Julian date conversions.
/// Julian date is commonly used in astronomical applications,
///  since it is numerically accurate and computationally simple.
/// The algorithms here will accurately convert between Julian day
///  and calendar date for all non-negative Julian days
///  (i.e. from Nov 24, -4713 on).
///
/// These routines will be used by other date/time packages
/// - thomas 97/02/25
///
/// Rewritten to eliminate overflow problems. This now allows the
/// routines to work correctly for all Julian day counts from
/// 0 to 2147483647  (Nov 24, -4713 to Jun 3, 5874898) assuming
/// a 32-bit integer. Longer types should also work to the limits
/// of their precision.
pub fn date2j(mut y: i32, mut m: i32, d: i32) -> i32 {
  if m > 2 {
      m += 1;
      y += 4800;
  } else {
      m += 13;
      y += 4799;
  }

  let century: i32 = y / 100;
  let mut julian: i32 = y * 365 - 32167;
  julian += y / 4 - century + century / 4;
  julian += 7834 * m / 256 + d;

  julian
}

fn j2date(julian_day: u32) -> (i32, u32, u32) {
  let mut julian: u32 = julian_day;
  julian += 32044;
  let mut quad: u32 = julian / 146097;
  let extra: u32 = (julian - quad * 146097) * 4 + 3;
  julian += 60 + quad * 3 + extra / 146097;
  quad = julian / 1461;
  julian -= quad * 1461;
  let mut y: u32 = julian * 4 / 1461;

  julian = if y != 0 {
    ((julian + 305) % 365)
  } else {
    ((julian + 306) % 366) + 123
  };

  y += quad * 4;
  let year :i32 = (y - 4800) as i32;
  quad = julian * 2141 / 65536;
  let day: u32 = julian - 7834 * quad / 256;
  let month: u32 = (quad + 10) % MONTHS_PER_YEAR as u32 + 1;

  (year, month, day)
}


/// j2day - convert Julian date to day-of-week (0..6 == Sun..Sat)
///
/// Note: various places use the locution j2day(date - 1) to produce a
/// result according to the convention 0..6 = Mon..Sun.  This is a bit of
/// a crock, but will work as long as the computation here is just a modulo.
pub fn j2day(mut date: i32) -> i32 {
  date += 1;
  date %= 7;

  if date < 0 {
    date += 7;
  }

  date
}

#[derive(PartialEq, Eq)]
pub enum DateTimeParseError {
  BadFormat(String),
  TimezoneOverflow
}

impl fmt::Debug for DateTimeParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DateTimeParseError::BadFormat(ref s) => write!(f, "{}", s),
      DateTimeParseError::TimezoneOverflow => {
        write!(f, "overflow or underflow in timezone")
      }
    }
  }
}

/// Parse a string to a fractional second.
pub fn parse_fractional_second(s: &str) -> Result<i64, DateTimeParseError> {
  debug_assert!(s.len() > 1);
  debug_assert!(s.as_bytes()[0] == b'.');

  let part = &s[1..];
  match i64::from_str(part) {
    Ok(frac) => Ok(frac * 1000000),
    Err(e) => Err(DateTimeParseError::BadFormat(format!("{}: '{}'", e, s)))
  }
}

/// Parse a string to a timezone in seconds.
pub fn decode_timezone(tzstr: &str) -> Result<i32, DateTimeParseError> {
  let buf = tzstr.as_bytes();
  let mut hr: i32;
  let min;
  let mut remains;
  let mut sec = 0;

  let plus_or_minus = buf[0];
  if plus_or_minus != b'+' && plus_or_minus != b'-' {
    return Err(DateTimeParseError::BadFormat(
      format!("leading characer in timezone must be '+' or '-': '{}'", tzstr)));
  }

  let r = strtoi(buf, 1);
  hr = r.0;
  remains = r.1;

  if remains.is_some() && remains.unwrap()[0] == b':' {
    let r = strtoi(remains.unwrap(), 1);
    min = r.0;
    remains = r.1;

    if remains.is_some() && remains.unwrap()[0] == b':' {
      let r = strtoi(remains.unwrap(), 1);
      sec = r.0;
      remains = r.1;
    }
  } else if remains.is_none() && buf.len() > 3 {
    min = hr % 100;
    hr = hr / 100;
  } else {
    min = 0;
  }

  if hr < 0 || hr > MAX_TZDISP_HOUR {
    return Err(DateTimeParseError::TimezoneOverflow);
  }
  if min < 0 || min >= MINS_PER_HOUR {
    return Err(DateTimeParseError::TimezoneOverflow)
  }
  if sec < 0 || sec >= SECS_PER_MINUTE {
    return Err(DateTimeParseError::TimezoneOverflow)
  }

  let mut tz = (hr * MINS_PER_HOUR + min) * SECS_PER_MINUTE + sec;

  if plus_or_minus == b'-' {
    tz = -tz;
  }

  if remains.is_some() {
    return Err(DateTimeParseError::BadFormat(
      format!("bad format in timezone: '{}'", tzstr)));
  }

  Ok(-tz)
}

fn datebsearch<'a>(key: &[u8], data: &'a [DateToken]) -> &'a DateToken {

  let mut last = data.len() - 1;
  let position: usize;
  let mut result;

  while last >= 0 {
    position = last >> 1;

    result = key[0] - data[position].token[0];
    if result == 0 {
      unimplemented!()
    }

    break;
  }

  unimplemented!()
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::DateTimeParseError::*;

  #[test]
  fn test_parse_fractional_second() {
    assert_eq!(12345000000i64, parse_fractional_second(".12345").ok().unwrap());
  }

  #[test]
  fn test_parse_fractional_second_fail1() {
    let err = parse_fractional_second(".inv").err().unwrap();
    assert_eq!(BadFormat("invalid digit found in string: '.inv'".to_owned()),
      err);
  }

  #[test]
  fn test_j2day() {
    let jd = date2j(2016, 11, 11);
    assert_eq!(5, j2day(jd));
  }

  #[test]
  fn test_decode_timezone() {
    assert_eq!(-3600, decode_timezone("+1").ok().unwrap());
    assert_eq!(3600,  decode_timezone("-1").ok().unwrap());
    assert_eq!(-5400, decode_timezone("+1:30").ok().unwrap());
    assert_eq!(5400,  decode_timezone("-1:30").ok().unwrap());
  }

  #[test]
  fn test_decode_timezone_failure() {
    match decode_timezone("+17") {
      Err(TimezoneOverflow) => {},
      _ => assert!(false, "Overflow must happen")
    };

    match decode_timezone("+1:60") {
      Err(TimezoneOverflow) => {},
      _ => assert!(false, "Overflow must happen")
    };

    match decode_timezone("+1:0:60") {
      Err(TimezoneOverflow) => {},
      _ => assert!(false, "Overflow must happen")
    };
  }
}