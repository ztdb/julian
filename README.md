# Julian

## Objective
This is a date/time library specialized for data processing. Its features are as follows:
 * Use julian calendar in order to handle a wide range of chronicles and to (de)serialize date times efficiently.
   * Time with a high precision fraction second is represented as a signed long.
   * Date is represented as a signed integer.
   * Timestamp (date and time) with a 6 digit fraction second is represented as a signed long.
 * Julian aims at recognizing standard as well as non-standard date/time strint formats.
   * It works well without any predefined format string.
