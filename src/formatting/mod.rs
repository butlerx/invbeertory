use implicit_clone::unsync::IString;

pub mod brewer;
pub mod style;
pub mod title;

pub fn capitalise(word: &str) -> IString {
    let mut chars = word.chars();
    match chars.next() {
        None => IString::default(),
        Some(f) => IString::from(f.to_uppercase().collect::<String>() + chars.as_str()),
    }
}
