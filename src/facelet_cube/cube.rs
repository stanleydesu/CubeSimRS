use crate::generic_cube::{Cube, Move, Face};
use crate::generic_cube::Face::*;

use super::moves::{convert_move};

/// A Rubik's Cube with stickers stored sequentially in a 1-dimensional array.
/// 
/// Each move is implemented as an array of index mappings of the form ``(from_idx, to_idx)``.
/// A move is then applied by swapping all pieces as specified by these index mappings.
/// 
/// Applying moves for the ``FaceletCube`` is more efficient than the ``GeoCube``, but
/// it is harder to define moves from scratch. Instead of deriving index mappings from scratch,
/// we first implement a GeoCube move, then use our conversion function to map the move
/// to a FaceletCube move.
#[derive(Clone)]
pub struct FaceletCube {
    size: i32,
    faces: Vec<Face>
}

impl Cube for FaceletCube {
    fn new(size: i32) -> Self {
        Self {
            size,
            faces: vec![
                repeat(U, size * size),
                repeat(R, size * size),
                repeat(F, size * size),
                repeat(D, size * size),
                repeat(L, size * size),
                repeat(B, size * size),
            ].concat()
        }
    }

    fn size(&self) -> i32 {
        self.size
    }

    fn get_state(&self) -> Vec<Face> {
        self.faces.clone()
    }

    fn mask(&self, mask: &[i32]) -> Self {
        let masked_faces = self.faces
                               .iter()
                               .enumerate()
                               .map(|(i, &x)| if mask.contains(&(i as i32)) { x } else { Face::X } )
                               .collect();

        Self { faces: masked_faces, ..*self }
    }

    fn apply_move(&self, mv: Move) -> Self {
        let mut faces = self.faces.clone();

        for (x, y) in convert_move(self.size, mv).0 {
            faces[y as usize] = self.faces[x as usize];
        }

        Self { 
            size: self.size, 
            faces
        }
    }
}

fn repeat<T: Clone>(element: T, count: i32) -> Vec<T> {
    std::iter::repeat(element).take(count as usize).collect()
}