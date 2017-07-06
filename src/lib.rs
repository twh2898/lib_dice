#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features,
        unused_import_braces, unused_qualifications)]

//! A library for parcing dice rolls using nom.
//!
//! ## Usage
//! ```no-run
//! extern crate lib_dice;

//! fn main() {
//! 	let roll_1 = lib_dice::roll(1, 8, 0); // 1d8 + 0
//! 	let roll_2 = lib_dice::roll_from_str("2d6 + 7");
//! }
//! ```

extern crate rand;
use rand::Rng;

#[macro_use]
extern crate nom;
use nom::IResult;

use std::str::{FromStr, from_utf8_unchecked};

mod error;
use error::DiceFormatError;

#[allow(dead_code)]
fn buf_to_i32(buf: &[u8]) -> i32 {
    FromStr::from_str(unsafe { from_utf8_unchecked(buf) }).unwrap()
}

#[allow(dead_code)]
named!(parse_dice <(usize, u8, i32)>, do_parse!(
    count: call!(nom::digit) >>
    tag!("d") >>
    dice: call!(nom::digit) >>
    opt!(complete!(tag!(" + "))) >>
    bonus: opt!(complete!(call!(nom::digit))) >>
    ((buf_to_i32(count) as usize, buf_to_i32(dice) as u8, buf_to_i32(bonus.unwrap_or(b"0"))))
));

/// Simulate a random dice roll using the rand crate.
///
/// ## Example
/// ```no-run
/// let roll_1 = roll(1, 8, 0); // 1d8 + 0
/// let roll_2 = roll(2, 6, 7); // 2d6 + 7
/// ```
pub fn roll(count: usize, dice: u8, bonus: i32) -> i32 {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| rng.gen_range(0, dice as i32))
        .fold(0i32, |total, value| total + value) + bonus + 1
}

/// Parse a dice roll from a sting in one of the following formats
///
/// ## Example
/// ```no-run
/// let roll_1 = roll_from_str("1d8").unwrap();  // roll a single 8 sided dice
/// let roll_2 = roll_from_str("2d6 + 7").unwrap();  // roll 2 6 sided dice with a +7 bonus
/// ```
///
/// ## Error
///
/// A lib_dice::DiceFormatError will be thrown if the dice string is malformed or empty.
pub fn roll_from_str(cmd: &str) -> Result<i32, DiceFormatError> {
    return match parse_dice(cmd.as_bytes()) {
        IResult::Done(_, dice) => Ok(roll(dice.0, dice.1, dice.2)),
        _ => Err(DiceFormatError::new(cmd))
    };
}

#[test]
fn test_roll() {
    // Valid dice strings
    assert_eq!(nom::IResult::Done(&[][..], (1, 8, 0)), parse_dice(b"1d8"));
    assert_eq!(nom::IResult::Done(&[][..], (1, 8, 0)), parse_dice(b"1d8 + 0"));
    assert_eq!(nom::IResult::Done(&[][..], (3, 4, 2)), parse_dice(b"3d4 + 2"));

    // Malformed Dice Strings
    assert_eq!(nom::IResult::Done(&[][..], (1, 8, 0)), parse_dice(b"1d8 + "));
    assert_eq!(nom::IResult::Done(&b"+"[..], (1, 8, 0)), parse_dice(b"1d8+"));
}
