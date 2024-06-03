use crate::storage::Beer;
use implicit_clone::unsync::IString;

pub fn format(beer: &Beer) -> IString {
    let Beer { name, year, .. } = beer;
    IString::from(format!("{name} {year}"))
}
