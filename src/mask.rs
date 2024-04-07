use crate::game::{Coords, Mask};

pub const STAR_MASK: Mask = match Mask::new(&[
    false, true, false,
    true, true, true,
    false, true, false,
], 3, Coords { x: 1, y: 1 }) {
    Some(mask) => mask,
    None => panic!("Invalid mask STAR"),
};

pub const NEAR_MASK: Mask = match Mask::new(&[
    false, true, false,
    true, false, true,
    false, true, false,
], 3, Coords { x: 1, y: 1 }) {
    Some(mask) => mask,
    None => panic!("Invalid mask NEAR"),
};

pub const FAR_MASK: Mask = match Mask::new(&[
    true, true, true,
    true, false, true,
    true, true, true,
], 3, Coords { x: 1, y: 1 }) {
    Some(mask) => mask,
    None => panic!("Invalid mask FAR"),
};

pub const ROW_MASK: Mask = match Mask::new(&[
    true, true, true,
], 3, Coords { x: 1, y: 0 }) {
    Some(mask) => mask,
    None => panic!("Invalid mask ROW"),
};

pub const COLUMN_MASK: Mask = match Mask::new(&[
    true,
    true,
    true,
], 1, Coords { x: 0, y: 1 }) {
    Some(mask) => mask,
    None => panic!("Invalid mask COLUMN"),
};
