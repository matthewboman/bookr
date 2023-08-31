use std::ops::Deref;

const forbidden_characters: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

fn contains_forbidden_characters(s: &str) -> bool {
    s.chars().any(|g| forbidden_characters.contains(&g))
}

#[derive(Debug)]
pub struct StringInput(String);

impl StringInput {
    pub fn parse(s: String) -> String {
        if contains_forbidden_characters(&s) {
            let s = s.replace(&forbidden_characters[..], "");

            s
        } else {
            s
        }
    }
}

impl AsRef<str> for StringInput {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct OptionalStringInput(Option<String>);

impl OptionalStringInput {
    pub fn parse(s: &Option<String>) -> OptionalStringInput {
        match s {
            Some(s) => {
                let s = StringInput::parse(s.to_owned());
                let is_empty_or_whitespace = s.trim().is_empty();

                if is_empty_or_whitespace {
                    Self(None)
                } else {
                    Self(Some(s.to_string()))
                }
            }
            None => Self(None)
        }
    }
}

impl Deref for OptionalStringInput {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct NewOptionalStringInput(Option<String>);

impl NewOptionalStringInput {
    pub fn parse(s: Option<String>) -> Option<String> {
        match s {
            Some(s) => {
                let s = StringInput::parse(s);
                let is_empty_or_whitespace = s.trim().is_empty();

                if is_empty_or_whitespace {
                    None
                } else {
                    Some(s.to_string())
                }
            }
            None => None
        }
    }
}

impl Deref for NewOptionalStringInput {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{StringInput, OptionalStringInput};

    #[test]
    fn forbidden_characters_are_stripped_from_strings() {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        for c in &forbidden_characters {
            let c = c.to_string();

            assert_eq!(StringInput::parse(c), "");
        }
    }

    #[test]
    fn parsed_string_is_the_same() {
        let input  = "this is the input".to_string();
        let output = StringInput::parse(input.clone());

        assert_eq!(input, output);
    }

    #[test]
    fn empty_string_returns_none() {
        let input = "".to_string();
        let res   = OptionalStringInput::parse(&Some(input));

        assert_eq!(res.is_none(), true);
    }
}