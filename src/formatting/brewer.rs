use crate::storage::Beer;

pub fn format(beer: &Beer) -> String {
    let Beer { brewery, .. } = beer;
    match &beer.collaborators {
        Some(collaborators) => {
            format!("{brewery} in collaboration with {collaborators}").to_string()
        }
        None => brewery.to_string(),
    }
}
