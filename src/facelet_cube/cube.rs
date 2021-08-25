use crate::generic_cube::{Cube, Move, Face};
use crate::generic_cube::Face::*;

use super::moves::{MOVE_CONVERTER};

#[derive(Clone)]
pub struct FaceletCube {
    size: i32,
    faces: Vec<Face>
}

impl Cube for FaceletCube {
    fn new(size: i32) -> Self {
        FaceletCube {
            size: size,
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

    fn is_solved(&self) -> bool {
        let face_length = (self.size * self.size) as usize;

        all_equal(&self.faces[0 * face_length..1 * face_length]) &&
        all_equal(&self.faces[1 * face_length..2 * face_length]) &&
        all_equal(&self.faces[2 * face_length..3 * face_length]) &&
        all_equal(&self.faces[3 * face_length..4 * face_length]) &&
        all_equal(&self.faces[4 * face_length..5 * face_length]) &&
        all_equal(&self.faces[5 * face_length..6 * face_length])
    }

    fn get_state(&self) -> Vec<Face> {
        self.faces.clone()
    }

    fn apply_move(&self, mv: Move) -> Self {
        let mut faces = self.faces.clone();

        for (x, y) in &MOVE_CONVERTER.convert_move(self.size, mv).0 {
            faces[*y as usize] = self.faces[*x as usize];
        }

        FaceletCube {
            size: self.size,
            faces: faces
        }
    }
}

fn repeat<T: Clone>(element: T, count: i32) -> Vec<T> {
    std::iter::repeat(element).take(count as usize).collect()
}

fn all_equal<T: Clone + PartialEq>(arr: &[T]) -> bool {
    arr.iter().all(|x| *x == arr[0])
}