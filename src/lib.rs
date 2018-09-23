extern crate slotmap;

use slotmap::{Key, SlotMap};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Blue,
    Green,
    Orange,
    Red,
    White,
    Yellow,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Face {
    colors: [Color; 9],
}

impl Face {
    const TOP_LEFT: usize = 0;
    const TOP: usize = 1;
    const TOP_RIGHT: usize = 2;
    const LEFT: usize = 3;
    const CENTER: usize = 4;
    const RIGHT: usize = 5;
    const BOTTOM_LEFT: usize = 6;
    const BOTTOM: usize = 7;
    const BOTTOM_RIGHT: usize = 8;

    pub fn new(tl: Color, t: Color, tr: Color, l: Color, c: Color, r: Color, bl: Color, b: Color, br: Color) -> Self {
        Face {
            colors: [tl, t, tr, l, c, r, bl, b, br]
        }
    }

    fn is_one_color(&self) -> bool {
        let first_color = self.colors[0];

        for color in self.colors[1..6].iter() {
            if *color != first_color {
                return false
            }
        }

        true
    }
}

const CORNERS: [usize; 4] = [Face::TOP_LEFT, Face::TOP_RIGHT, Face::BOTTOM_LEFT, Face::BOTTOM_RIGHT];
const EDGES: [usize; 4] = [Face::LEFT, Face::TOP, Face::RIGHT, Face::BOTTOM];

// 0 is front, 1 is left, 2 is back, 3 is right, 4 is top, 5 is bottom
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cube3x3x3 {
    faces: [Face; 6],
}

impl Cube3x3x3 {
    const BLUE: usize = 0;
    const GREEN: usize = 1;
    const ORANGE: usize = 2;
    const RED: usize = 3;
    const WHITE: usize = 4;
    const YELLOW: usize = 5;

    pub fn new(faces: [Face; 6]) -> Result<Self, &'static str> {
        let cube = Cube3x3x3 {
            faces,
        };

        if !cube.is_valid() {
            return Err("Invalid start state");
        }

        Ok(cube)
    }

    fn is_valid(&self) -> bool {
        let mut center_counts = [0u8; 6];
        let mut edges_counts = [0u8; 6];
        let mut corner_counts = [0u8; 6];

        for face in self.faces.iter() {
            match face.colors[Face::CENTER] {
                Color::Blue => center_counts[Self::BLUE] += 1,
                Color::Green => center_counts[Self::GREEN] += 1,
                Color::Orange => center_counts[Self::ORANGE] += 1,
                Color::Red => center_counts[Self::RED] += 1,
                Color::White => center_counts[Self::WHITE] += 1,
                Color::Yellow => center_counts[Self::YELLOW] += 1,
            }

            for edge in EDGES.iter() {
                match face.colors[*edge] {
                    Color::Blue => edges_counts[Self::BLUE] += 1,
                    Color::Green => edges_counts[Self::GREEN] += 1,
                    Color::Orange => edges_counts[Self::ORANGE] += 1,
                    Color::Red => edges_counts[Self::RED] += 1,
                    Color::White => edges_counts[Self::WHITE] += 1,
                    Color::Yellow => edges_counts[Self::YELLOW] += 1,
                }
            }

            for corner in CORNERS.iter() {
                match face.colors[*corner] {
                    Color::Blue => corner_counts[Self::BLUE] += 1,
                    Color::Green => corner_counts[Self::GREEN] += 1,
                    Color::Orange => corner_counts[Self::ORANGE] += 1,
                    Color::Red => corner_counts[Self::RED] += 1,
                    Color::White => corner_counts[Self::WHITE] += 1,
                    Color::Yellow => corner_counts[Self::YELLOW] += 1,
                }
            }
        }

        for center_count in center_counts.iter() {
            if *center_count != 1 {
                return false;
            }
        }

        for edge_count in edges_counts.iter() {
            if *edge_count != 4 {
                return false;
            }
        }

        for corner_count in corner_counts.iter() {
            if *corner_count != 4 {
                return false;
            }
        }

        true
    }

    pub fn is_solved(&self) -> bool {
        for face in self.faces.iter() {
            if !face.is_one_color() {
                return false;
            }
        }

        true
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Move {
    TopRotateLeft,
    TopRotateRight,
    BottomRotateLeft,
    BottomRotateRight,
    LeftRotateUp,
    LeftRotateDown,
    RightRotateUp,
    RightRotateDown,
}

// DFS Solver
#[derive(Clone, Debug)]
pub struct CubeSolver {
    cube: Option<Cube3x3x3>,
    nodes: SlotMap<Move>, // Slotmap even needed?
    moves: Vec<Move>,
}

impl CubeSolver {
    pub fn new(cube: Cube3x3x3) -> Self {
        CubeSolver {
            cube: Some(cube),
            nodes: SlotMap::new(),
            moves: Vec::new(),
        }
    }

    pub fn solve(&mut self) -> Option<Cube3x3x3> {
        // ...

        self.cube.take()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        use super::Color::{Blue as B, Green as G, Orange as O, Red as R, White as W, Yellow as Y};
        use super::{Face, Cube3x3x3, CubeSolver};

        let faces = [
            Face::new(Y, Y, G, G, O, O, G, W, B),
            Face::new(R, O, G, Y, B, O, Y, B, W),
            Face::new(B, G, W, G, R, R, O, G, B),
            Face::new(R, W, O, B, G, Y, W, B, Y),
            Face::new(B, W, Y, W, W, B, R, O, W),
            Face::new(O, R, O, Y, Y, R, R, R, G),
        ];
        let cube = Cube3x3x3::new(faces).unwrap();
        let mut solver = CubeSolver::new(cube);
        let solved_cube = solver.solve();

        assert!(solved_cube.unwrap().is_solved());
    }
}
