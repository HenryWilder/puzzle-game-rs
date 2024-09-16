//! The atoms of rules.

pub mod noun;
pub mod operator;
pub mod property;

use noun::Noun;
use operator::Operator;
use property::Property;

/// A noun, operator, or property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Word {
    /// A noun.
    Noun(Noun),

    /// An operator.
    Operator(Operator),

    /// A property.
    Property(Property),
}
