use crate::spacial::{vector3i::Vector3i, direction3::Direction3};
pub mod segments;
use segments::*;

pub struct Worm {
    head_position: Vector3i,
    segments: Option<WormSegments>,
}

// Basics

impl Worm {
    pub fn new(head_position: Vector3i, segments: impl Into<WormSegments>) -> Self {
        Self {
            head_position,
            segments: Some(segments.into()),
        }
    }

    pub fn new_tailless(head_position: Vector3i) -> Self {
        Self {
            head_position,
            segments: None,
        }
    }

    /// The worm is just a head with no segments
    pub fn is_tailless(&self) -> bool {
        self.segments.is_none()
    }
}

// Lengthen

pub struct LengthenTaillessError<'worm>(&'worm mut Worm);

impl std::fmt::Debug for LengthenTaillessError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing tail to lengthen, need direction")
    }
}

impl LengthenTaillessError<'_> {
    pub fn resolve(mut self, direction: Direction3) {
        self.0.segments = Some(WormSegments::new(direction));
    }
}

impl Worm {
    /// Increases the length of the worm in the direction of its tail.
    /// Does not have awareness of the level geometry.
    ///
    /// Example:
    /// ```no_run
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
            let mut worm = Worm::new(Vector3i::new(0, 0, 0), Direction3::North);

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

pub struct InvalidDirectionError {}

impl std::fmt::Debug for InvalidDirectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "worm cannot pass through itself except to create a closed loop")
    }
}

impl Worm {
    /// Pulls the worm's head in the requested direction without changing the worm's length.
    /// Does not have awareness of the level geometry.
    pub fn try_crawl(&mut self, crawl_direction: Direction3) -> Result<(), InvalidDirectionError> {
        if !self.is_tailless() {
            let new_head_direction = -crawl_direction;
            let current_head_direction = self.segments.as_ref().unwrap().head_direction();
            // Cannot crawl into own neck
            if -current_head_direction == new_head_direction {
                return Err(InvalidDirectionError {});
            }
            let mut segments = std::mem::take(&mut self.segments).unwrap();
            segments.push_head(new_head_direction);
            self.segments = segments.pop_tail().updated_segments;
        }
        self.head_position += crawl_direction;
        Ok(())
    }
}

// Segment Positions

impl Worm {
    pub fn segment_positions<'worm>(&'worm self) -> impl Iterator<Item = Vector3i> + 'worm {
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
