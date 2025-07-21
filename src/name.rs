use std::hash::{Hash, Hasher};

pub struct Name {
    separator: usize,
    name: String,
}

impl Name {
    pub fn new(name: impl Into<String>) -> Option<Self> {
        let name = name.into();
        let separator = name.find(':')?;

        // Make sure that the identifier is valid.
        if !is_valid_name_segment(&name[..separator])
            || !is_valid_name_segment(&name[separator + 1..])
        {
            return None;
        }

        Some(Self { separator, name })
    }

    /// Returns the scope name of the identifier.
    pub fn scope(&self) -> &str {
        &self.name[..self.separator]
    }

    /// Returns the name of the identifier without the scope.
    pub fn unqualified(&self) -> &str {
        &self.name[self.separator..]
    }

    /// Returns the full qualified name of the identifier.
    pub fn qualified(&self) -> &str {
        &self.name
    }
}

// Checks if a segment of the name (i.e., scope or unqualified name) is valid.
// It must be non-empty, the first character must be ASCII lowercase, and the rest must be ASCII alphanumeric or '_'.
pub fn is_valid_name_segment(segment: &str) -> bool {
    let mut chars = segment.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    if !first.is_ascii_lowercase() {
        return false;
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

impl From<&'static str> for Name {
    fn from(name: &'static str) -> Self {
        Self::new(name).expect("invalid name")
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.qualified()
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.qualified().hash(state);
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.qualified())
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Name(\"{}\")", self.qualified())
    }
}

impl std::cmp::PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.qualified() == other.qualified()
    }
}

impl std::cmp::Eq for Name {}

impl Clone for Name {
    fn clone(&self) -> Self {
        Self {
            separator: self.separator,
            name: self.name.clone(),
        }
    }
}
