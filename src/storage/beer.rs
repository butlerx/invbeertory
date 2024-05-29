use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Beer {
    pub name: String,
    pub brewery: String,
    pub year: i16,
    pub abv: f64,
    pub style: String,
    pub size: i64,
    pub drunk: bool,
    pub stock: i64,
    pub purchased: i64,
    pub ibu: Option<f64>,
    pub collaborators: Option<String>,
    pub barrel_aged: bool,
    pub barrel_type: Option<String>,
    pub brewed_with: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Stock {
    pub beers: Vec<Beer>,
}
