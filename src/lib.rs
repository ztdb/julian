#![allow(dead_code)]
use std::fmt;
use std::str::FromStr;

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
const DAGO       :&'static str = "ago";
const DCURRENT   :&'static str = "current";
const EPOCH      :&'static str = "epoch";
const INVALID    :&'static str = "invalid";
const EARLY      :&'static str = "-infinity";
const LATE       :&'static str = "infinity";
const NOW        :&'static str = "now";
const TODAY      :&'static str = "today";
const TOMORROW   :&'static str = "tomorrow";
const YESTERDAY  :&'static str = "yesterday";
const ZULU       :&'static str = "zulu";

const DMICROSEC  :&'static str = "usecond";
const DMILLISEC  :&'static str = "msecond";
const DSECOND    :&'static str = "second";
const DMINUTE    :&'static str = "minute";
const DHOUR      :&'static str = "hour";
const DDAY       :&'static str = "day";
const DWEEK      :&'static str = "week";
const DMONTH     :&'static str = "month";
const DQUARTER   :&'static str = "quarter";
const DYEAR      :&'static str = "year";
const DDECADE    :&'static str = "decade";
const DCENTURY   :&'static str = "century";
const DMILLENNIUM:&'static str = "millennium";
const DA_D       :&'static str = "ad";
const DB_C       :&'static str = "bc";
const DTIMEZONE  :&'static str = "timezone";

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

const DATEK_TBL: [(&'static str, i8, i32);74] = [
  (EARLY, RESERV, DTK_EARLY),
  (DA_D, ADBC, AD),                     // "ad" for years > 0
  ("allballs", RESERV, DTK_ZULU),       // 00:00:00
  ("am", AMPM, AM),
  ("apr", MONTH, 4),
  ("april", MONTH, 4),
  ("at", IGNORE_DTF, 0),                // "at" (throwaway)
  ("aug", MONTH, 8),
  ("august", MONTH, 8),
  (DB_C, ADBC, BC),                     // "bc" for years <= 0
  (DCURRENT, RESERV, DTK_CURRENT),      // "current" is always now
  ("d", UNITS, DTK_DAY),                // "day of month" for ISO input
  ("dec", MONTH, 12),
  ("december", MONTH, 12),
  ("dow", RESERV, DTK_DOW),             // day of week
  ("doy", RESERV, DTK_DOY),             // day of year
  ("dst", DTZMOD, SECS_PER_HOUR),
  (EPOCH, RESERV, DTK_EPOCH),           // "epoch" reserved for system epoch time
  ("feb", MONTH, 2),
  ("february", MONTH, 2),
  ("fri", DOW, 5),
  ("friday", DOW, 5),
  ("h", UNITS, DTK_HOUR),               // "hour"
  (LATE, RESERV, DTK_LATE),             // "infinity" reserved for "late time"
  (INVALID, RESERV, DTK_INVALID),       // "invalid" reserved for bad time
  ("isodow", RESERV, DTK_ISODOW),       // ISO day of week, Sunday == 7
  ("isoyear", UNITS, DTK_ISOYEAR),      // year in terms of the ISO week date
  ("j", UNITS, DTK_JULIAN),
  ("jan", MONTH, 1),
  ("january", MONTH, 1),
  ("jd", UNITS, DTK_JULIAN),
  ("jul", MONTH, 7),
  ("julian", UNITS, DTK_JULIAN),
  ("july", MONTH, 7),
  ("jun", MONTH, 6),
  ("june", MONTH, 6),
  ("m", UNITS, DTK_MONTH),              // "month" for ISO input
  ("mar", MONTH, 3),
  ("march", MONTH, 3),
  ("may", MONTH, 5),
  ("mm", UNITS, DTK_MINUTE),            // "minute" for ISO input
  ("mon", DOW, 1),
  ("monday", DOW, 1),
  ("nov", MONTH, 11),
  ("november", MONTH, 11),
  (NOW, RESERV, DTK_NOW),               // current transaction time
  ("oct", MONTH, 10),
  ("october", MONTH, 10),
  ("on", IGNORE_DTF, 0),                // "on" (throwaway)
  ("pm", AMPM, PM),
  ("s", UNITS, DTK_SECOND),             // "seconds" for ISO input
  ("sat", DOW, 6),
  ("saturday", DOW, 6),
  ("sep", MONTH, 9),
  ("sept", MONTH, 9),
  ("september", MONTH, 9),
  ("sun", DOW, 0),
  ("sunday", DOW, 0),
  ("t", ISOTIME, DTK_TIME),             // Filler for ISO time fields
  ("thu", DOW, 4),
  ("thur", DOW, 4),
  ("thurs", DOW, 4),
  ("thursday", DOW, 4),
  (TODAY, RESERV, DTK_TODAY),           // midnight
  (TOMORROW, RESERV, DTK_TOMORROW),     // tomorrow midnight
  ("tue", DOW, 2),
  ("tues", DOW, 2),
  ("tuesday", DOW, 2),
  ("undefined", RESERV, DTK_INVALID),   // pre-v6.1 invalid time
  ("wed", DOW, 3),
  ("wednesday", DOW, 3),
  ("weds", DOW, 3),
  ("y", UNITS, DTK_YEAR),               // "year" for ISO input
  (YESTERDAY, RESERV, DTK_YESTERDAY)    // yesterday midnight
];


const DELTATK_TBL: [(&'static str, i8, i32);63] = [
  ("@", IGNORE_DTF, 0),                 // postgres relative prefix
  (DAGO, AGO, 0),                       // "ago" indicates negative time offset
  ("c", UNITS, DTK_CENTURY),            // "century" relative
  ("cent", UNITS, DTK_CENTURY),         // "century" relative
  ("centuries", UNITS, DTK_CENTURY),    // "centuries" relative
  (DCENTURY, UNITS, DTK_CENTURY),       // "century" relative
  ("d", UNITS, DTK_DAY),                // "day" relative
  (DDAY, UNITS, DTK_DAY),               // "day" relative
  ("days", UNITS, DTK_DAY),             // "days" relative
  ("dec", UNITS, DTK_DECADE),           // "decade" relative
  (DDECADE, UNITS, DTK_DECADE),         // "decade" relative
  ("decades", UNITS, DTK_DECADE),       // "decades" relative
  ("decs", UNITS, DTK_DECADE),          // "decades" relative
  ("h", UNITS, DTK_HOUR),               // "hour" relative
  (DHOUR, UNITS, DTK_HOUR),             // "hour" relative
  ("hours", UNITS, DTK_HOUR),           // "hours" relative
  ("hr", UNITS, DTK_HOUR),              // "hour" relative
  ("hrs", UNITS, DTK_HOUR),             // "hours" relative
  (INVALID, RESERV, DTK_INVALID),       // reserved for invalid time
  ("m", UNITS, DTK_MINUTE),             // "minute" relative
  ("microsecon", UNITS, DTK_MICROSEC),  // "microsecond" relative
  ("mil", UNITS, DTK_MILLENNIUM),       // "millennium" relative
  ("millennia", UNITS, DTK_MILLENNIUM), // "millennia" relative
  (DMILLENNIUM, UNITS, DTK_MILLENNIUM), // "millennium" relative
  ("millisecon", UNITS, DTK_MILLISEC),  // relative
  ("mils", UNITS, DTK_MILLENNIUM),      // "millennia" relative
  ("min", UNITS, DTK_MINUTE),           // "minute" relative
  ("mins", UNITS, DTK_MINUTE),          // "minutes" relative
  (DMINUTE, UNITS, DTK_MINUTE),         // "minute" relative
  ("minutes", UNITS, DTK_MINUTE),       // "minutes" relative
  ("mon", UNITS, DTK_MONTH),            // "months" relative
  ("mons", UNITS, DTK_MONTH),           // "months" relative
  (DMONTH, UNITS, DTK_MONTH),           // "month" relative
  ("months", UNITS, DTK_MONTH),
  ("ms", UNITS, DTK_MILLISEC),
  ("msec", UNITS, DTK_MILLISEC),
  (DMILLISEC, UNITS, DTK_MILLISEC),
  ("mseconds", UNITS, DTK_MILLISEC),
  ("msecs", UNITS, DTK_MILLISEC),
  ("qtr", UNITS, DTK_QUARTER),          // "quarter" relative
  (DQUARTER, UNITS, DTK_QUARTER),       // "quarter" relative
  ("s", UNITS, DTK_SECOND),
  ("sec", UNITS, DTK_SECOND),
  (DSECOND, UNITS, DTK_SECOND),
  ("seconds", UNITS, DTK_SECOND),
  ("secs", UNITS, DTK_SECOND),
  (DTIMEZONE, UNITS, DTK_TZ),           // "timezone" time offset
  ("timezone_h", UNITS, DTK_TZ_HOUR),   // timezone hour units
  ("timezone_m", UNITS, DTK_TZ_MINUTE), // timezone minutes units
  ("undefined", RESERV, DTK_INVALID),   // pre-v6.1 invalid time
  ("us", UNITS, DTK_MICROSEC),          // "microsecond" relative
  ("usec", UNITS, DTK_MICROSEC),        // "microsecond" relative
  (DMICROSEC, UNITS, DTK_MICROSEC),     // "microsecond" relative
  ("useconds", UNITS, DTK_MICROSEC),    // "microseconds" relative
  ("usecs", UNITS, DTK_MICROSEC),       // "microseconds" relative
  ("w", UNITS, DTK_WEEK),               // "week" relative
  (DWEEK, UNITS, DTK_WEEK),             // "week" relative
  ("weeks", UNITS, DTK_WEEK),           // "weeks" relative
  ("y", UNITS, DTK_YEAR),               // "year" relative
  (DYEAR, UNITS, DTK_YEAR),             // "year" relative
  ("years", UNITS, DTK_YEAR),           // "years" relative
  ("yr", UNITS, DTK_YEAR),              // "year" relative
  ("yrs", UNITS, DTK_YEAR)              // "years" relative
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
pub enum DateTimeError {
  BadFormat(String)
}

impl fmt::Debug for DateTimeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DateTimeError::BadFormat(ref s) => write!(f, "{}", s)
    }
  }
}

pub fn parse_fractional_second(cp: &str) -> Result<i64, DateTimeError> {
  debug_assert!(cp.len() > 1);
  debug_assert!(cp.as_bytes()[0] == b'.');

  let part = &cp[1..];
  match i64::from_str(part) {
    Ok(frac) => Ok(frac * 1000000),
    Err(e) => Err(DateTimeError::BadFormat(format!("{}: '{}'", e, cp)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::DateTimeError::*;

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
}