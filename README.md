# Julian

## What is Julian
 * A date/time library designed for data processing.

## Goals
 * Fast serialization, comparison, and arithmetic computation.
 * A wide range representation of date/time (4713 BC to far into the future)

## Plans
 * Use julian calendar in order to handle a wide range of chronicles and to (de)serialize date times efficiently.
   * Time with a high precision fraction second is represented as a signed long
   * Date is represented as a signed integer represented as days from Jan 1, 4713 BC.
   * Timestamp (date and time) with a 6 digit fraction second is represented as a signed long.
 * Standard as well as non-standard date/time string formats support
 * JIT-based date/time format parsing with pre-defined date/time format string
