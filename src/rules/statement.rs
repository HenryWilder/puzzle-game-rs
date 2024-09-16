use super::word::Word;
use super::word::{
    noun::Noun::*,
    operator::Operator::*,
    property::Property::*,
    Word::*,
};

/// A sequence of words forming
pub struct Statement(Vec<Word>);

impl FromIterator<Word> for Statement {
    fn from_iter<T: IntoIterator<Item = Word>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

// impl Statement {
//     // Break a statement apart using AND as a delimiter
//     pub fn split(&self) -> impl Iterator<Item = impl Iterator<Item = Statement>> {
//         // [a AND b IS x AND y] => [[a IS x], [a IS Y], [b IS x], [b IS y]]
//         self.0.iter()
//             .as_slice()
//             .split(|word| word == &Operator(AND))
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_split() {
//         let statement = Statement::from_iter([
//             Noun(WORM),
//             Operator(IS),
//             Property(YOU),
//         ]);
//     }
// }
