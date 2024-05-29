use rand::Rng;
use std::collections::HashSet;

pub fn generate_unique_colors(count: usize) -> Vec<String> {
    let mut colors = HashSet::new();
    let mut rng = rand::thread_rng();

    while colors.len() < count {
        let color = format!("#{:06x}", rng.gen::<u32>() & 0x00FF_FFFF);
        colors.insert(color);
    }

    colors.into_iter().collect()
}
