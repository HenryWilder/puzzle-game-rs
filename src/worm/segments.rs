use std::collections::VecDeque;
use crate::spacial::direction3::Direction3;

#[derive(Debug)]
pub struct WormSegments(VecDeque<Direction3>);

pub(super) struct PopResult {
    pub old_direction: Direction3,
    pub updated_segments: Option<WormSegments>,
}

impl FromIterator<Direction3> for WormSegments {
    fn from_iter<T: IntoIterator<Item = Direction3>>(iter: T) -> Self {
        let mut iter = iter.into_iter().peekable();
        assert!(iter.peek().is_some());
        Self(iter.collect())
    }
}

impl<const N: usize> From<[Direction3; N]> for WormSegments {
    fn from(value: [Direction3; N]) -> Self {
        assert_ne!(N, 0);
        Self(value.into())
    }
}

impl From<Direction3> for WormSegments {
    fn from(value: Direction3) -> Self {
        Self(VecDeque::from([value]))
    }
}

impl WormSegments {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn head_direction(&self) -> Direction3 {
        *self.0
            .front()
            .expect("WormSegments cannot be empty")
    }

    pub fn tail_direction(&self) -> Direction3 {
        *self.0
            .back()
            .expect("WormSegments cannot be empty")
    }

    pub(super) fn push_head(&mut self, direction: Direction3) {
        self.0.push_front(direction);
    }

    pub(super) fn push_tail(&mut self, direction: Direction3) {
        self.0.push_back(direction);
    }

    pub(super) fn pop_head(mut self) -> PopResult {
        let old_direction = self.0
            .pop_front()
            .expect("WormSegments cannot be empty");

        PopResult {
            old_direction,
            updated_segments: (!self.0.is_empty()).then_some(self),
        }
    }

    pub(super) fn pop_tail(mut self) -> PopResult {
        let old_direction = self.0
            .pop_back()
            .expect("WormSegments cannot be empty");

        PopResult {
            old_direction,
            updated_segments: (!self.0.is_empty()).then_some(self),
        }
    }

    pub(super) fn iter(&self) -> std::collections::vec_deque::Iter<'_, Direction3> {
        self.0.iter()
    }
}
