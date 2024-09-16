//! Properties that nouns can participate in.

/// A trait or property that can be conditionally participated in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Property {
    /// Controlled by player input.
    YOU,
}
