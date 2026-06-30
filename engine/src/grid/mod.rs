use ndarray::{ArrayD, IxDyn};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BoundaryCondition {
    #[default]
    Toroidal,
    Dead,
}

pub enum Boundary<T> {
    Toroidal,
    Fixed(T),
}

pub struct Grid<T> {
    pub cells: ArrayD<T>,
    boundary: Boundary<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(shape: &[usize], boundary: Boundary<T>) -> Self {
        Grid {
            cells: ArrayD::from_elem(IxDyn(shape), T::default()),
            boundary,
        }
    }

    pub fn get(&self, idx: &[i64]) -> T {
        let shape = self.cells.shape();
        let resolved: Option<Vec<usize>> = idx
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let n = shape[i] as i64;
                match &self.boundary {
                    Boundary::Toroidal => Some(x.rem_euclid(n) as usize),
                    Boundary::Fixed(_) => {
                        if x >= 0 && x < n { Some(x as usize) } else { None }
                    }
                }
            })
            .collect();

        match resolved {
            Some(idx_vec) => self.cells[IxDyn(&idx_vec)].clone(),
            None => match &self.boundary {
                Boundary::Fixed(v) => v.clone(),
                Boundary::Toroidal => unreachable!(),
            },
        }
    }
}
