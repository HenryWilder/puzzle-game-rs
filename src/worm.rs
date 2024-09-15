use crate::cell::*;
pub mod segments;
use segments::*;

pub struct Worm {
    head: Vector3i,
    segments: Option<WormSegments>,
}

impl Worm {
    pub fn new(head: Vector3i, tail: impl Into<WormSegments>) -> Self {
        Self {
            head,
            segments: Some(tail.into()),
        }
    }

    pub fn new_tailless(head: Vector3i) -> Self {
        Self {
            head,
            segments: None,
        }
    }

    /// The worm is just a head with no segments
    pub fn is_tailless(&self) -> bool {
        self.segments.is_none()
    }

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

    /// Pulls the worm's head in the requested direction without changing the worm's length.
    /// Does not have awareness of the level geometry.
    pub fn crawl(&mut self, direction: Direction3) {
        self.head += direction;
        if !self.is_tailless() {
            let mut segments = std::mem::take(&mut self.segments).unwrap();
            segments.push_head(-direction);
            self.segments = segments.pop_tail().updated_segments;
        }
    }

    pub fn is_rotateable() {

    }
}

pub struct LengthenTaillessError<'worm>(&'worm mut Worm);

impl std::fmt::Debug for LengthenTaillessError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing tail to lengthen, need direction")
    }
}

impl<'worm> LengthenTaillessError<'worm> {
    pub fn resolve(mut self, direction: Direction3) {
        self.0.segments = Some(WormSegments::new(direction));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lengthen_ok() {
        let mut worm = Worm::new(Vector3i::new(0, 0, 0), Direction3::North);

        let result = worm.try_lengthen();
        assert!(result.is_ok(), "lengthening existing tail should succeed");
        let segments = worm.segments.unwrap();
        assert_eq!(segments.len(), 2, "tail should be 2 segment long after lengthening");
        assert_eq!(segments.head_direction(), segments.tail_direction(), "tail direction should be same as the existing tail direction");
    }

    #[test]
    fn test_lengthen_err() {
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
