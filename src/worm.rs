//! A worm.

use crate::spacial::{vector3i::Vector3i, direction3::Direction3};
pub mod segments;
use segments::*;

/// A worm.
pub struct Worm {
    head_position: Vector3i,
    segments: Option<WormSegments>,
}

// Basics

impl Worm {
    /// Construct a worm from head and segments.
    /// Each segment directs where the tail will go.
    ///
    /// Example:
    /// ```no_run
    /// use Direction3::*;
    /// Worm::new(Vector3i::new(0,0,0), [
    ///     East,
    ///     South,
    /// ]);
    /// ```
    /// produces the worm
    /// ```not_rust
    /// O--.
    ///    |
    /// ```
    pub fn new(head_position: Vector3i, segments: impl IntoIterator<Item = Direction3>) -> Self {
        Self {
            head_position,
            segments: Some(WormSegments::from_iter(segments)),
        }
    }

    /// Constructs a worm from head with no segments.
    /// If this is the player worm, it will be only a ring.
    /// If this is an NPC worm, it will be an edible dot.
    pub fn new_tailless(head_position: Vector3i) -> Self {
        Self {
            head_position,
            segments: None,
        }
    }

    /// The worm is just a head with no segments?
    pub fn is_tailless(&self) -> bool {
        self.segments.is_none()
    }

    /// The grid position of the head.
    /// Any other position requires iterating over [`Self::segment_positions()`].
    pub fn head_position(&self) -> Vector3i {
        self.head_position
    }

    /// Number of elements returned by [`Self::segment_positions()`].
    pub fn num_segments(&self) -> usize {
        match &self.segments {
            Some(segments) => segments.len() + 1,
            None => 1,
        }
    }
}

// Lengthen

/// Calling [`Worm::try_lengthen()`] requires the worm to be at least 1 segment long so the tail can be extended in that direction.
/// Otherwise, the direction must be specified.
///
/// This error can be handled semi-automatically by calling [`Self::resolve()`].
pub struct LengthenTaillessError<'worm>(&'worm mut Worm);

impl std::fmt::Debug for LengthenTaillessError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing tail to lengthen, need direction")
    }
}

impl LengthenTaillessError<'_> {
    /// Resolve and consumes the [`LengthenTaillessError`] by specifying the direction in which the tail should grow.
    pub fn resolve(self, direction: Direction3) {
        _ = self.0.segments.insert(WormSegments::from([direction]));
    }
}

impl Worm {
    /// Increases the length of the worm in the direction of its tail.
    /// Does not have awareness of the level geometry.
    ///
    /// Example:
    /// ```
    /// my_worm
    ///     .try_lengthen()
    ///     .unwrap_or_else(|err| {
    ///         let direction = prompt_direction();
    ///         err.resolve(direction)
    ///     });
    /// ```
    pub fn try_lengthen(&mut self) -> Result<(), LengthenTaillessError> {
        match &mut self.segments {
            Some(segments) => {
                let tail_direction = segments.tail_direction();
                segments.push_tail(tail_direction);
                Ok(())
            },
            None => Err(LengthenTaillessError(self)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod lengthen {
        use super::*;

        #[test]
        fn test_ok() {
            let mut worm = Worm::new(Vector3i::new(0, 0, 0), [Direction3::North]);

            let result = worm.try_lengthen();
            assert!(result.is_ok(), "lengthening existing tail should succeed");
            let segments = worm.segments.unwrap();
            assert_eq!(segments.len(), 2, "tail should be 2 segment long after lengthening");
            assert_eq!(segments.head_direction(), segments.tail_direction(), "tail direction should be same as the existing tail direction");
        }

        #[test]
        fn test_err() {
            let mut worm = Worm::new_tailless(Vector3i::new(0, 0, 0));

            let result = worm.try_lengthen();
            assert!(result.is_err(), "lengthening tailless should fail");
            result.unwrap_err().resolve(Direction3::North);
            assert!(worm.segments.is_some(), "resolving should create tail");
            let segments = worm.segments.unwrap();
            assert_eq!(segments.len(), 1, "tail should be 1 segment long after resolution");
            assert_eq!(segments.tail_direction(), Direction3::North, "tail direction should be the one used to resolve the error");
        }
    }
}

// Crawl

impl Worm {
    /// Pulls the worm's head in the requested direction without changing the worm's length.
    /// Does not have awareness of the level geometry.
    pub fn crawl(&mut self, crawl_direction: Direction3) {
        self.head_position += crawl_direction;
        if !self.is_tailless() {
            let mut segments = std::mem::take(&mut self.segments).unwrap();
            let new_head_direction = -crawl_direction;
            let current_head_direction = segments.head_direction();
            self.segments = if -current_head_direction != new_head_direction {
                segments.push_head(new_head_direction);
                segments.pop_tail().updated_segments
            } else {
                // reversing
                let current_tail_direction = segments.tail_direction();
                segments.push_tail(current_tail_direction);
                segments.pop_head().updated_segments
            };
        }
    }
}

// Segment Positions

impl Worm {
    /// Create an iterator over the worm's segments' world positions.
    /// The first element is always guaranteed to exist and will be the head position itself.
    ///
    /// Example
    /// ```no_run
    /// let worm: Worm = // ...
    /// let mut it = worm.segment_positions();
    /// let head_position = it.next().unwrap();
    /// let tail_position = it.last();
    /// ```
    pub fn segment_positions<'worm>(&'worm self) -> impl 'worm + Iterator<Item = Vector3i> {
        Some(self.head_position)
            .into_iter()
            .chain(self.segments.as_ref()
                .and_then(|segments| Some(
                    segments
                        .iter()
                        .scan(self.head_position, |headward_segment_position, &segment_direction| {
                            *headward_segment_position += segment_direction;
                            Some(*headward_segment_position)
                        })
                ))
                .into_iter()
                .flatten()
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod segment_positions {
        use super::*;

        #[test]
        fn test_tailless() {
            const HEAD_POS: Vector3i = Vector3i { x: 5, y: 3, z: 8 };
            let worm = Worm::new_tailless(HEAD_POS);

            let mut it = worm.segment_positions();
            let head = it.next();
            assert!(head.is_some(), "tailless worm should provide head position");
            assert_eq!(head.unwrap(), HEAD_POS, "head position should not be changed");
            assert!(it.next().is_none(), "tailless worm should only have head position");
        }

        #[test]
        fn test_normal() {
            const HEAD_POS: Vector3i = Vector3i { x: 2, y: 9, z: 1 };
            const DIRECTIONS: [Direction3; 7] = [
                Direction3::North,
                Direction3::North,
                Direction3::East,
                Direction3::Up,
                Direction3::West,
                Direction3::West,
                Direction3::Down,
            ];
            let worm = Worm::new(HEAD_POS, DIRECTIONS);
            let mut it = worm.segment_positions();
            let head = it.next();
            assert!(head.is_some(), "first element should be head position");
            assert_eq!(head.unwrap(), HEAD_POS, "head position should not be changed");
            let mut ongoing = HEAD_POS;
            for (position, direction) in it.zip(DIRECTIONS.iter()) {
                let new_position = ongoing + *direction;
                // println!("{ongoing:?} + {direction:?} gave {new_position:?}");
                ongoing = new_position;
                assert_eq!(position, ongoing, "positions should match");
            }
        }
    }
}
