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
    pub fn filter_check(&self, field: &str, filter: &str) -> bool {
        match field {
            "name" => self.name.to_string(),
            "brewery" => self.brewery.to_string(),
            "year" => self.year.to_string(),
            "abv" => self.abv.to_string(),
            "style" => self.style.to_string(),
            "size" => self.size.to_string(),
            "stock" => self.stock.to_string(),
            "purchased" => self.purchased.to_string(),
            _ => return false,
        }
        .to_lowercase()
        .contains(filter)
    }

    pub fn compare_field(&self, other: &Self, field: &str) -> std::cmp::Ordering {
        match field {
            "name" => self.name.cmp(&other.name),
            "brewery" => self.brewery.cmp(&other.brewery),
            "year" => self.year.cmp(&other.year),
            "abv" => self
                .abv
                .partial_cmp(&other.abv)
                .unwrap_or(std::cmp::Ordering::Equal),
            "style" => self.style.cmp(&other.style),
            "size" => self.size.cmp(&other.size),
            "stock" => self.stock.cmp(&other.stock),
            "purchased" => self.purchased.cmp(&other.purchased),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_check() {
        let beer = Beer {
            name: IString::from("Test Beer"),
            brewery: IString::from("Test Brewery"),
            year: 2021,
            abv: 5.0,
            style: IString::from("Test Style"),
            size: 12,
            drunk: false,
            stock: 6,
            purchased: 0,
            ibu: None,
            collaborators: None,
            barrel_aged: false,
            barrel_type: None,
            brewed_with: None,
        };
        // test lowercase partial match
        assert_eq!(beer.filter_check("name", "test"), true);
        // test lowercase full match
        assert_eq!(beer.filter_check("name", "test beer"), true);
        // test uppercase full match
        assert_eq!(beer.filter_check("name", "Test Beer"), true);
        // test uppercase partial match
        assert_eq!(beer.filter_check("name", "Beer"), true);
        // test number partial match
        assert_eq!(ber.filter_check("year", "21"), true);
        // test number full match
        assert_eq!(ber.filter_check("year", "2021"), true);
        // test unmatching field
        assert_eq!(beer.filter_check("ibu", "5"), false);
    }

    #[test]
    fn test_compare_field() {
        let beer_a = Beer {
            name: IString::from("Test Beer A"),
            brewery: IString::from("Test Brewery A"),
            year: 2021,
            abv: 5.0,
            style: IString::from("Test Style"),
            size: 12,
            drunk: false,
            stock: 6,
            purchased: 0,
            ibu: None,
            collaborators: None,
            barrel_aged: false,
            barrel_type: None,
            brewed_with: None,
        };

        let beer_b = Beer {
            name: IString::from("Test Beer B"),
            brewery: IString::from("Test Brewery A"),
            year: 2021,
            abv: 4.5,
            style: IString::from("Test Style"),
            size: 12,
            drunk: false,
            stock: 6,
            purchased: 0,
            ibu: None,
            collaborators: None,
            barrel_aged: false,
            barrel_type: None,
            brewed_with: None,
        };

        // test string comparison
        assert_eq!(
            beer_a.compare_field(&beer_b, "name"),
            std::cmp::Ordering::Less
        );
        // test number comparison
        assert_eq!(
            beer_a.compare_field(&beer_b, "year"),
            std::cmp::Ordering::Equal
        );
        // test float comparison
        assert_eq!(
            beer_a.compare_field(&beer_b, "abv"),
            std::cmp::Ordering::Equal
        );
    }
}
