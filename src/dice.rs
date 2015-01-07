use std::rand;
use std::rand::Rng;

#[derive(Copy)]
pub struct Dice(pub uint, pub uint);

pub fn rand<T : PartialOrd + rand::distributions::range::SampleRange>(lower: T, upper: T) -> T {
    rand::thread_rng().gen_range::<T>(lower, upper)
}

impl Dice {
    pub fn roll(&self) -> uint {
        let lower = self.0;
        let upper = lower * self.1 + 1;
        rand(lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::Dice;
    use std::iter::repeat;

    // run enough tests to populate the histogram
    macro_rules! assert_dice(
        ($num:expr, $sides:expr) => ({
           let dice = Dice($num, $sides);
           let mut dist : Vec<uint> = repeat(0u).take($num * $sides + 1).collect();
            for _ in range(0, $num * $sides * 1000) {
                let roll = dice.roll();
                dist[roll as uint] += 1;
            }

            assert!(dist.iter().take($num - 1).all(|g| *g == 0), "dist {}d{} {}", $num, $sides, dist[0u..$num]);
            assert!(dist.iter().skip($num).all(|g| *g > 0),      "dist {}d{} {}", $num, $sides, dist[$num..]);
        }));

    #[test]
    fn dice() {
        for num in range(1u, 4) {
            for sides in range(1u, 20) {
                assert_dice!(num, sides);
            }
        }
    }
}
