
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