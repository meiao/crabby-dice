use super::{parse_dice, Roller};

pub struct Loaded {}

impl Roller for Loaded {
    fn roll(dice: &str) -> Result<u8, &str> {
        match parse_dice(dice) {
            Ok((number_dice, dice_size)) => Ok(number_dice * dice_size),
            Err(error_message) => Err(error_message),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Loaded;
    use crate::dice::Roller;

    #[test]
    fn test_roll() {
        assert_eq!(20, Loaded::roll("1d20").unwrap());
        assert_eq!(4, Loaded::roll("1d4").unwrap());
        assert_eq!(12, Loaded::roll("1d12").unwrap());
        assert_eq!(40, Loaded::roll("2d20").unwrap());
        assert_eq!(12, Loaded::roll("3d4").unwrap());
        assert_eq!(48, Loaded::roll("4d12").unwrap());
    }
}
