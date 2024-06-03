use crate::storage::Beer;
use implicit_clone::unsync::IString;

fn barrel_aged(beer: &Beer) -> IString {
    if beer.barrel_aged {
        IString::from("Barrel Aged")
    } else {
        IString::default()
    }
}

fn brewed_in(beer: &Beer) -> IString {
    match &beer.barrel_type {
        Some(barrel_type) => IString::from(format!("in {barrel_type} Barrels")),
        None => IString::default(),
    }
}

fn brewed_with(beer: &Beer) -> IString {
    match &beer.brewed_with {
        Some(brewed_with) => IString::from(format!("with {brewed_with}")),
        None => IString::default(),
    }
}

pub fn format(beer: &Beer) -> IString {
    IString::from(
        format!(
            "{} {} {} {}",
            barrel_aged(beer),
            beer.style,
            brewed_with(beer),
            brewed_in(beer)
        )
        .trim()
        .replace("  ", " "),
    )
}
