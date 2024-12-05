use std::{collections::HashMap, ops::Add};

use advent_2019_common::{run_day_puzzle_solver, DayPuzzlePart};
use anyhow::{anyhow, Context, Error, Result};

type WirePositionScalar = i32;

/// ```markdown
/// ^ y
/// |
/// |
/// +====> x
/// ``````
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct WireMapVector2 {
    x: WirePositionScalar,
    y: WirePositionScalar,
}

impl Add for WireMapVector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl WireMapVector2 {
    /// Manhattan distance between this point and another one.
    pub fn distance_with(&self, rhs: Self) -> u32 {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum WireDirection {
    Right,
    Up,
    Left,
    Down,
}

impl TryFrom<char> for WireDirection {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match &value {
            'R' => WireDirection::Right,
            'U' => WireDirection::Up,
            'L' => WireDirection::Left,
            'D' => WireDirection::Down,
            _ => return Err(anyhow!(format!("unknown wire direction: {}", value))),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WireOffsetPosition {
    direction: WireDirection,
    length: WirePositionScalar,
}

impl WireOffsetPosition {
    pub fn as_unit_vector(&self) -> WireMapVector2 {
        match self.direction {
            WireDirection::Right => WireMapVector2 { x: 1, y: 0 },
            WireDirection::Up => WireMapVector2 { x: 0, y: 1 },
            WireDirection::Left => WireMapVector2 { x: -1, y: 0 },
            WireDirection::Down => WireMapVector2 { x: 0, y: -1 },
        }
    }
}

#[derive(Clone, Debug)]
struct Wire {
    directions: Vec<WireOffsetPosition>,
}

/// Structure: (coordinates, steps_from_origin)
type WirePath = HashMap<WireMapVector2, u32>;

impl TryFrom<String> for Wire {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let raw_directions = value.split(',');
        let mut directions = Vec::with_capacity(raw_directions.size_hint().0);
        for raw_direction in raw_directions {
            let mut chars = raw_direction.trim().chars();
            let direction = WireDirection::try_from(chars.next().with_context(|| {
                format!("Wire directions parsing error for token: {}", raw_direction)
            })?)?;
            let length_string: String = chars.into_iter().collect();
            let length: WirePositionScalar = length_string.parse().with_context(|| {
                format!(
                    "Wire directions parsing error for movement length: {}",
                    length_string
                )
            })?;
            directions.push(WireOffsetPosition { direction, length });
        }
        Ok(Self { directions })
    }
}

impl Wire {
    pub fn directions(&self) -> &Vec<WireOffsetPosition> {
        &self.directions
    }

    pub fn compute_path(&self, origin: WireMapVector2) -> WirePath {
        let mut current = origin.clone();
        let mut path = WirePath::with_capacity(1 + self.directions.len());
        path.insert(current, 0);
        let mut steps = 0;
        for direction in &self.directions {
            let direction_unit_vector = direction.as_unit_vector();
            for _ in 0..direction.length {
                steps += 1;
                current = current + direction_unit_vector;
                path.insert(current, steps);
            }
        }
        path
    }
}

fn compute_solution_1(wire1: Wire, wire2: Wire) -> Result<u32> {
    let origin = WireMapVector2 { x: 0, y: 0 };
    let path1 = wire1.compute_path(origin);
    let path2 = wire2.compute_path(origin);
    let mut intersections = vec![];
    for position1 in path1.keys() {
        if path2.contains_key(&position1) {
            intersections.push(*position1);
        }
    }

    if intersections.is_empty() {
        Err(anyhow!("compute_solution_1: no intersections found"))
    } else {
        let mut intersections_distances: Vec<u32> = intersections
            .iter()
            .map(|position| position.distance_with(origin))
            .collect();
        intersections_distances.sort();
        Ok(intersections_distances[1]) // skip origin intersection
    }
}

fn compute_solution_2(wire1: Wire, wire2: Wire) -> Result<u32> {
    let origin = WireMapVector2 { x: 0, y: 0 };
    let path1 = wire1.compute_path(origin);
    let path2 = wire2.compute_path(origin);
    let mut intersections_steps: Vec<u32> = vec![];
    for (position1, position1_steps) in path1.iter() {
        if let Some(position2_steps) = path2.get(position1) {
            intersections_steps.push(position1_steps + position2_steps);
        }
    }

    if intersections_steps.is_empty() {
        Err(anyhow!("compute_solution_2: no intersections found"))
    } else {
        intersections_steps.sort();
        Ok(intersections_steps[1]) // skip origin intersection
    }
}

fn main() -> Result<()> {
    // Part 1
    run_day_puzzle_solver(3, DayPuzzlePart::One, b'\n', |input: Vec<Wire>| {
        let wire1 = input[0].clone();
        let wire2 = input[1].clone();
        compute_solution_1(wire1, wire2)
    })?;

    // Part 2
    run_day_puzzle_solver(3, DayPuzzlePart::Two, b'\n', |input: Vec<Wire>| {
        let wire1 = input[0].clone();
        let wire2 = input[1].clone();
        compute_solution_2(wire1, wire2)
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        compute_solution_1, compute_solution_2, Wire, WireDirection::*, WireOffsetPosition,
    };

    #[test]
    fn test_compute_day_3_short_wire_path() {
        let path_wire_1 = Wire::try_from("R8,U5,L5,D3".to_string()).unwrap();
        let path_1_expected_offsets = [
            WireOffsetPosition {
                direction: Right,
                length: 8,
            },
            WireOffsetPosition {
                direction: Up,
                length: 5,
            },
            WireOffsetPosition {
                direction: Left,
                length: 5,
            },
            WireOffsetPosition {
                direction: Down,
                length: 3,
            },
        ];
        for (i, wire_1_direction) in path_wire_1.directions().iter().enumerate() {
            assert_eq!(wire_1_direction, &path_1_expected_offsets[i]);
        }

        let path_wire_2 = Wire::try_from("U7,R6,D4,L4".to_string()).unwrap();
        let path_2_expected_offsets = vec![
            WireOffsetPosition {
                direction: Up,
                length: 7,
            },
            WireOffsetPosition {
                direction: Right,
                length: 6,
            },
            WireOffsetPosition {
                direction: Down,
                length: 4,
            },
            WireOffsetPosition {
                direction: Left,
                length: 4,
            },
        ];
        for (i, wire_2_direction) in path_wire_2.directions().iter().enumerate() {
            assert_eq!(wire_2_direction, &path_2_expected_offsets[i]);
        }

        assert_eq!(compute_solution_1(path_wire_1, path_wire_2).unwrap(), 6);
    }

    #[test]
    fn test_compute_day_3_solution_1() {
        let path_1_wire_1 =
            Wire::try_from("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string()).unwrap();
        let path_1_wire_2 = Wire::try_from("U62,R66,U55,R34,D71,R55,D58,R83".to_string()).unwrap();
        assert_eq!(
            compute_solution_1(path_1_wire_1, path_1_wire_2).unwrap(),
            159
        );
    }

    #[test]
    fn test_compute_day_3_solution_2() {
        let path_1_wire_1 =
            Wire::try_from("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string()).unwrap();
        let path_1_wire_2 = Wire::try_from("U62,R66,U55,R34,D71,R55,D58,R83".to_string()).unwrap();
        assert_eq!(
            compute_solution_2(path_1_wire_1, path_1_wire_2).unwrap(),
            610
        );
    }
}
