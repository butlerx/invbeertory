use implicit_clone::unsync::{IArray, IString};
use rand::Rng;
use std::collections::HashSet;

const BACKGROUND: &str = "#f2f0ec";

pub fn default_background() -> IString {
    IString::from(BACKGROUND)
}

pub fn generate_unique_colors(count: usize) -> IArray<IString> {
    let mut colors = HashSet::new();
    let mut rng = rand::thread_rng();

    while colors.len() < count {
        let color = IString::from(format!("#{:06x}", rng.gen::<u32>() & 0x00FF_FFFF));
        colors.insert(color);
    }

    colors.into_iter().collect()
}
