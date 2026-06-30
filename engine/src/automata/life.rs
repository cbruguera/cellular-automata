use ndarray::{ArrayD, IxDyn};
use serde::{Deserialize, Serialize};

use crate::grid::{Boundary, BoundaryCondition, Grid};
use crate::neighborhood::Neighborhood;
use super::Automaton;

#[derive(Clone, Serialize, Deserialize)]
pub struct LifeRule {
    pub birth: Vec<u8>,
    pub survival: Vec<u8>,
}

impl Default for LifeRule {
    fn default() -> Self {
        LifeRule { birth: vec![3], survival: vec![2, 3] }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LifeConfig {
    pub shape: Vec<usize>,
    #[serde(default)]
    pub rule: LifeRule,
    #[serde(default)]
    pub boundary: BoundaryCondition,
    #[serde(default = "default_neighborhood")]
    pub neighborhood: Neighborhood,
}

fn default_neighborhood() -> Neighborhood {
    Neighborhood::Moore { radius: 1 }
}

pub struct Life {
    grid: Grid<u8>,
    rule: LifeRule,
    neighborhood: Neighborhood,
    generation: u64,
    population: u64,
}

impl Life {
    pub fn new(config: LifeConfig) -> Self {
        let boundary = match config.boundary {
            BoundaryCondition::Toroidal => Boundary::Toroidal,
            BoundaryCondition::Dead => Boundary::Fixed(0u8),
        };
        let mut life = Life {
            grid: Grid::new(&config.shape, boundary),
            rule: config.rule,
            neighborhood: config.neighborhood,
            generation: 0,
            population: 0,
        };
        life.randomize();
        life
    }
}

impl Automaton for Life {
    fn step(&mut self) {
        let shape = self.grid.cells.shape().to_vec();
        let offsets = self.neighborhood.offsets(shape.len());
        let rule = &self.rule;
        let grid = &self.grid;

        let new_cells = ArrayD::from_shape_fn(IxDyn(&shape), |idx| {
            let center: Vec<i64> = (0..shape.len()).map(|i| idx[i] as i64).collect();
            let alive = grid.get(&center) == 1;
            let n: u8 = offsets
                .iter()
                .map(|off| {
                    let nb: Vec<i64> = center.iter().zip(off).map(|(&a, &b)| a + b).collect();
                    grid.get(&nb)
                })
                .sum();
            if alive {
                if rule.survival.contains(&n) { 1u8 } else { 0u8 }
            } else {
                if rule.birth.contains(&n) { 1u8 } else { 0u8 }
            }
        });

        self.grid.cells = new_cells;
        self.generation += 1;
        self.population = self.grid.cells.iter().map(|&c| c as u64).sum();
    }

    fn generation(&self) -> u64 { self.generation }
    fn population(&self) -> u64 { self.population }

    fn shape(&self) -> Vec<usize> {
        self.grid.cells.shape().to_vec()
    }

    fn render_rgba(&self, buf: &mut [u8]) {
        let shape = self.grid.cells.shape();
        let w = shape[0];
        let h = if shape.len() > 1 { shape[1] } else { 1 };

        for y in 0..h {
            for x in 0..w {
                // for N > 2 dimensions, hold all other axes at index 0
                let idx_vec: Vec<usize> = (0..shape.len())
                    .map(|i| match i { 0 => x, 1 => y, _ => 0 })
                    .collect();
                let cell = self.grid.cells[IxDyn(&idx_vec)];
                let base = (y * w + x) * 4;
                let v = if cell == 1 { 220u8 } else { 15u8 };
                buf[base] = v;
                buf[base + 1] = v;
                buf[base + 2] = v;
                buf[base + 3] = 255;
            }
        }
    }

    fn randomize(&mut self) {
        let mut seed_bytes = [0u8; 8];
        getrandom::getrandom(&mut seed_bytes).unwrap_or(());
        let mut seed = u64::from_le_bytes(seed_bytes);
        for cell in self.grid.cells.iter_mut() {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            *cell = if seed >> 63 == 1 { 1 } else { 0 };
        }
        self.generation = 0;
        self.population = self.grid.cells.iter().map(|&c| c as u64).sum();
    }

    fn clear(&mut self) {
        self.grid.cells.fill(0);
        self.generation = 0;
        self.population = 0;
    }
}
