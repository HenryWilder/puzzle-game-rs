//! Applies properties to nouns.

/// A helper/modifier for describing how two elements affect each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// `[Noun(a), IS, Noun(b)]` => Each instance of `a` is immediately replaced with `b`.<br/>
    /// `[Noun(a), IS, Form(b)]` => Each instance of `a` participates in `b`.
    IS,
    /// `[Noun(a), HAS, Noun(b)]` => Each instance of `a` is replaced with `b` when destroyed.
    HAS,
    /// `[a, Operator(x), b, AND, c]` => `[a, x, b]` and `[a, x, c]`.<br/>
    /// `[a, AND, b, Operator(x), c]` => `[a, x, c]` and `[b, x, c]`.
    AND,
    /// Negates the operation.
    NOT,
    /// `[a, ON, b, Operator(x), Grammar(d)]`
    ON,
}
