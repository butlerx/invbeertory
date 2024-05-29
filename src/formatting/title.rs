use crate::storage::Beer;

pub fn format(beer: &Beer) -> String {
    let Beer { name, year, .. } = beer;
    format!("{name} {year}").to_string()
}
