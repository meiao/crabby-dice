mod loaded;
mod regular;

use std::result::Result;
use std::vec::Vec;

pub use loaded::Loaded;
pub use regular::Regular;

pub trait Roller {
    fn roll(dice: &str) -> Result<u8, &str>;
}

fn parse_dice<'a, 'b>(dice: &'a str) -> Result<(u8, u8), &'b str> {
    let values: Vec<&str> = dice.split('d').collect();

    let number_dice_str = match values.get(0) {
        None => return Err("Invalid input string. Empty?"),
        Some(number_str) => number_str,
    };
    let number_dice = match number_dice_str.parse() {
        Ok(number) => number,
        Err(_) => return Err("Unable to parse number of dice"),
    };

    let dice_size_str = match values.get(1) {
        None => return Err("No 'd' found in input string."),
        Some(number_str) => number_str,
    };
    let dice_size = match dice_size_str.parse() {
        Ok(number) => number,
        Err(_) => return Err("Unable to parse size of dice"),
    };

    return Ok((number_dice, dice_size));
}

#[cfg(test)]
mod tests {

    use super::parse_dice;

    #[test]
    fn test_parser() {
        assert_eq!((1, 4), parse_dice("1d4").unwrap());
        assert_eq!((3, 6), parse_dice("3d6").unwrap());
        assert_eq!((2, 20), parse_dice("2d20").unwrap());
    }
}
