//! A worm.

use crate::spacial::{vector3i::Vector3i, direction3::Direction3};
pub mod segments;
use segments::*;

#[cfg(test)]
mod tests;

/// A worm.
pub struct Worm {
    head_position: Vector3i,
    segments: Option<WormSegments>,
}

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
    /// Construct a worm from head and segments.
    /// Each segment directs where the tail will go.
    ///
    /// Example:
    /// ```no_run
    /// use Direction3::*;
    /// /********
    ///  * o--. *
    ///  *    | *
    ///  ********/
    /// Worm::new(Vector3i::new(0,0,0), [
    ///     East,
    ///     South,
    /// ]);
    /// ```
    pub fn new(head_position: Vector3i, segments: impl IntoIterator<Item = Direction3>) -> Self {
        let mut segments = segments
            .into_iter()
            .peekable();

        let segments = segments
            .peek()
            .is_some()
            .then(|| WormSegments::from_iter(segments));

        Self {
            head_position,
            segments,
        }
    }

    pub fn try_new(
        head_position: Vector3i,
        segments: impl IntoIterator<Item = Result<Direction3, impl std::fmt::Debug>>
    ) -> Result<Self, impl std::fmt::Debug> {
        let mut segments = segments
            .into_iter()
            .peekable();

        let segments: Option<Result<WormSegments, _>> = segments
            .peek()
            .is_some()
            .then(|| segments.collect::<Result<_, _>>());

        if let Some(Err(e)) = segments {
            Err(e)
        } else {
            Ok(Self {
                head_position,
                segments: segments
                    .map(|inner| inner.unwrap()),
            })
        }
    }

    /// Construct a worm from a direction string.
    /// 'x' points towards the viewer and 'o' points away.
    /// 
    /// Example:
    /// ```
    /// use Direction3::*;
    /// let head_position = Vector3i::new(5, 3, 8);
    /// /******************
    ///  *        .--.    *
    ///  * o------:--:==- *
    ///  *      --'       *
    ///  ******************/
    /// let worm1 = Worm::from_str(head_position, ">>>^>v>o<<xv<");
    /// let worm2 = Worm::new(head_position, [
    ///     East,
    ///     East,
    ///     East,
    ///     North,
    ///     East,
    ///     South,
    ///     East,
    ///     Down,
    ///     West,
    ///     West,
    ///     Up,
    ///     South,
    ///     West,
    /// ]);
    /// let segs1 = worm1.segment_positions();
    /// let segs2 = worm2.segment_positions();
    /// for (seg1, seg2) in segs1.zip(segs2) {
    ///     assert_eq!(seg1, seg2);
    /// }
    /// ```
    pub fn from_str(head_position: Vector3i, segments: &str) -> Result<Self, impl '_ + std::fmt::Debug> {
        Self::try_new(head_position, segments
            .chars()
            .map(|ch|
                match ch {
                    '<' => Ok(Direction3::East),
                    '>' => Ok(Direction3::West),
                    '^' => Ok(Direction3::North),
                    'v' => Ok(Direction3::South),
                    'o' => Ok(Direction3::Down),
                    'x' => Ok(Direction3::Up),
                    _ => Err(format!("invalid character: '{ch}'"))
                }
            )
        )
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
