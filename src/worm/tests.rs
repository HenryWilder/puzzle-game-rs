use super::*;

#[cfg(test)]
mod from_str {
    use super::*;

    #[test]
    fn test_doc() {
        use Direction3::*;
        let head_position = Vector3i::new(5, 3, 8);
        /******************
         *        .--.    *
         * o------:--:==- *
         *      --'       *
         ******************/
        let worm1 = Worm::from_str(head_position, ">>>^>v>o<<xv<").unwrap();
        let worm2 = Worm::new(head_position, [East, East, East, North, East, South, East, Down, West, West, Up, South, West]);
        let segs1 = worm1.segment_positions();
        let segs2 = worm2.segment_positions();
        for (seg1, seg2) in segs1.zip(segs2) {
            assert_eq!(seg1, seg2);
        }
    }
}

#[cfg(test)]
mod lengthen {
    use super::*;

    #[test]
    fn test_ok() {
        let mut worm = Worm::new(Vector3i::new(0, 0, 0), [Direction3::North]);
        assert!(worm.segments.as_ref().is_some_and(|s| s.len() == 1), "constructor not working");

        let result = worm.try_lengthen();
        assert!(result.is_ok(), "lengthening existing tail should succeed");
        let segments = worm.segments.unwrap();
        assert_eq!(segments.len(), 2, "tail should be 2 segment long after lengthening");
        assert_eq!(segments.head_direction(), segments.tail_direction(), "tail direction should be same as the existing tail direction");
    }

    #[test]
    fn test_err() {
        let mut worm = Worm::new(Vector3i::new(0, 0, 0), []);
        assert!(worm.segments.is_none(), "constructor not working");

        let result = worm.try_lengthen();
        assert!(result.is_err(), "lengthening tailless should fail");
        result.unwrap_err().resolve(Direction3::North);
        assert!(worm.segments.is_some(), "resolving should create tail");
        let segments = worm.segments.unwrap();
        assert_eq!(segments.len(), 1, "tail should be 1 segment long after resolution");
        assert_eq!(segments.tail_direction(), Direction3::North, "tail direction should be the one used to resolve the error");
    }
}

mod segment_positions {
    use super::*;

    #[test]
    fn test_tailless() {
        const HEAD_POS: Vector3i = Vector3i { x: 5, y: 3, z: 8 };
        let worm = Worm::new(HEAD_POS, []);

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
