
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
const AM   :i8 = 0;
const PM   :i8 = 1;
const HR24 :i8 = 2;

const AD   :i8 = 0;
const BC   :i8 = 1;

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
/// these are only for relative dates
const AGO           :i8 = 19;
const ABS_BEFORE    :i8 = 20;
const ABS_AFTER     :i8 = 21;
/// generic fields to help with parsing
const ISODATE       :i8 = 22;
const ISOTIME       :i8 = 23;
/// these are only for parsing intervals
const WEEK          :i8 = 24;
const DECADE        :i8 = 25;
const CENTURY       :i8 = 26;
const MILLENNIUM    :i8 = 27;
/// hack for parsing two-word timezone specs "MET DST" etc
const DTZMOD        :i8 = 28; // "DST" as a separate word
/// reserved for unrecognized string values
const UNKNOWN_FIELD :i8 = 31;

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

const DATEK_TBL: [(&'static str, u8, i32);1] = [
  ("abc", 1, 1)
];