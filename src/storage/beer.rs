use implicit_clone::{unsync::IString, ImplicitClone};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone, PartialOrd)]
pub struct Beer {
    pub name: IString,
    pub brewery: IString,
    pub year: i16,
    pub abv: f64,
    pub style: IString,
    pub size: i64,
    pub drunk: bool,
    pub stock: i64,
    pub purchased: i64,
    pub ibu: Option<f64>,
    pub collaborators: Option<IString>,
    pub barrel_aged: bool,
    pub barrel_type: Option<IString>,
    pub brewed_with: Option<IString>,
}

impl std::fmt::Display for Beer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({}%)", self.name, self.year, self.abv)
    }
}

impl ImplicitClone for Beer {}

impl Beer {
    pub fn filter_check(&self, field: usize, filter: &str) -> bool {
        match field {
            0 => self.name.to_string(),
            1 => self.brewery.to_string(),
            2 => self.year.to_string(),
            3 => self.abv.to_string(),
            4 => self.style.to_string(),
            5 => self.size.to_string(),
            6 => self.stock.to_string(),
            7 => self.purchased.to_string(),
            _ => return false,
        }
        .to_lowercase()
        .contains(filter)
    }
}
