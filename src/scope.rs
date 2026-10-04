use std::path::Path;

/// Helper struct for comparing the 
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    parts: Vec<String>,
}

impl Scope {
    pub fn new(path: String) -> Self {
        Self {
            parts: path
                .split(r"\")
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        }
    }

    pub fn from(path: &Path) -> Self {
        let mut parts = Vec::new();

        for comp in path.components() {
            parts.push(comp.as_os_str().to_string_lossy().to_string())
        }

        Self { parts }
    }

    /// Checks if this scope is within or equal to the scope of `other`.
    ///
    /// `other` should be the longer scope.
    pub fn is_in(&self, other: &Scope) -> bool {
        for i in 1..self.parts.len() {
            if self.parts[i] != other.parts[i] {
                return false;
            }
        }
        true
    }
}
