use super::{parse_dice, Roller};
use rand::{thread_rng, Rng};

pub struct Regular {}

impl Roller for Regular {
    fn roll(dice: &str) -> Result<u8, &str> {
        let (mut number_dice, dice_size) = match parse_dice(dice) {
            Ok((number_dice, dice_size)) => (number_dice, dice_size),
            Err(error_message) => return Err(error_message),
        };

        let mut sum = 0;
        let low = 1;
        let high = dice_size + 1; // since gen_range high limit is exclusive
        let mut rng = thread_rng();
        while number_dice > 0 {
            sum += rng.gen_range(low, high);
            number_dice = number_dice - 1;
        }

        return Ok(sum);
    }
}

#[cfg(test)]
mod tests {

    use super::Regular;
    use crate::dice::Roller;

    #[test]
    fn test_roll() {
        let roll = Regular::roll("1d6").unwrap();
        assert!(1 <= roll);
        assert!(roll <= 6);

        let roll = Regular::roll("2d4").unwrap();
        assert!(2 <= roll);
        assert!(roll <= 8);

        let roll = Regular::roll("5d20").unwrap();
        assert!(5 <= roll);
        assert!(roll <= 100);

        // this will lead to false positives, but should pass almost all the time
        let roll = Regular::roll("100d2").unwrap();
        assert!(135 <= roll);
        assert!(roll <= 165);
    }
}
