use crate::storage::Beer;
use implicit_clone::unsync::IString;

pub fn format(beer: &Beer) -> IString {
    let Beer { brewery, .. } = beer;
    match &beer.collaborators {
        Some(collaborators) => {
            IString::from(format!("{brewery} in collaboration with {collaborators}"))
        }
        None => brewery.clone(),
    }
}
