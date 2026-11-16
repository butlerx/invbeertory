use implicit_clone::unsync::{IArray, IString};

const BACKGROUND: &str = "#f2f0ec";

/// Hue step between consecutive colours. The golden angle spreads hues
/// evenly so neighbouring series never share a similar colour.
const GOLDEN_ANGLE: f64 = 137.507_764;

/// Lightness bands cycled per colour. Kept away from 0% and 100% so no
/// series renders as black or blends into the light chart background.
const LIGHTNESS: [u8; 3] = [45, 58, 70];

pub fn default_background() -> IString {
    IString::from(BACKGROUND)
}

pub fn generate_unique_colors(count: usize) -> IArray<IString> {
    let hues = std::iter::successors(Some(0.0_f64), |hue| Some((hue + GOLDEN_ANGLE) % 360.0));
    hues.zip(LIGHTNESS.iter().cycle())
        .take(count)
        .map(|(hue, lightness)| IString::from(format!("hsl({hue:.1}, 65%, {lightness}%)")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn colours_are_unique_and_deterministic() {
        let a = generate_unique_colors(255);
        let b = generate_unique_colors(255);
        assert_eq!(a, b);
        let unique: HashSet<_> = a.iter().collect();
        assert_eq!(unique.len(), 255);
    }

    #[test]
    fn colours_are_never_black() {
        for colour in generate_unique_colors(255).iter() {
            assert!(!colour.ends_with(" 0%)"), "{colour} is black");
        }
    }
}
