use crate::storage::Beer;

fn barrel_aged(beer: &Beer) -> String {
    if beer.barrel_aged {
        "Barrel Aged".to_string()
    } else {
        String::new()
    }
}

fn brewed_in(beer: &Beer) -> String {
    match &beer.barrel_type {
        Some(barrel_type) => format!("in {barrel_type} Barrels"),
        None => String::new(),
    }
}

fn brewed_with(beer: &Beer) -> String {
    match &beer.brewed_with {
        Some(brewed_with) => format!("with {brewed_with}"),
        None => String::new(),
    }
}

pub fn format(beer: &Beer) -> String {
    format!(
        "{} {} {} {}",
        barrel_aged(beer),
        beer.style,
        brewed_with(beer),
        brewed_in(beer)
    )
    .trim()
    .replace("  ", " ") // Clean up any double spaces
    .to_string()
}
