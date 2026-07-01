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
    ages: ArrayD<u32>,
    rule: LifeRule,
    neighborhood: Neighborhood,
    generation: u64,
    population: u64,
    color_mode: u8,
}

impl Life {
    pub fn new(config: LifeConfig) -> Self {
        let boundary = match config.boundary {
            BoundaryCondition::Toroidal => Boundary::Toroidal,
            BoundaryCondition::Dead => Boundary::Fixed(0u8),
        };
        let ages = ArrayD::zeros(IxDyn(&config.shape));
        let mut life = Life {
            grid: Grid::new(&config.shape, boundary),
            ages,
            rule: config.rule,
            neighborhood: config.neighborhood,
            generation: 0,
            population: 0,
            color_mode: 0,
        };
        life.randomize();
        life
    }
}

fn age_to_color(age: u32) -> (u8, u8, u8) {
    if age == 0 { return (13, 13, 13); }
    let t = ((age as f32 - 1.0) / 20.0).clamp(0.0, 1.0);
    let r = (255.0 - t * 55.0) as u8;
    let g = (220.0 - t * 20.0) as u8;
    let b = (160.0 + t * 40.0) as u8;
    (r, g, b)
}

impl Automaton for Life {
    fn step(&mut self) {
        let shape = self.grid.cells.shape().to_vec();
        let offsets = self.neighborhood.offsets(shape.len());

        let (new_cells, new_ages) = {
            let grid = &self.grid;
            let rule = &self.rule;
            let old_ages = &self.ages;

            let cells = ArrayD::from_shape_fn(IxDyn(&shape), |idx| {
                let center: Vec<i64> = (0..shape.len()).map(|i| idx[i] as i64).collect();
                let alive = grid.get(&center) == 1;
                let n: u8 = offsets.iter().map(|off| {
                    let nb: Vec<i64> = center.iter().zip(off).map(|(&a, &b)| a + b).collect();
                    grid.get(&nb)
                }).sum();
                if alive {
                    if rule.survival.contains(&n) { 1u8 } else { 0u8 }
                } else {
                    if rule.birth.contains(&n) { 1u8 } else { 0u8 }
                }
            });

            let ages = ArrayD::from_shape_fn(IxDyn(&shape), |idx| {
                if cells[idx.clone()] == 1 {
                    let center: Vec<i64> = (0..shape.len()).map(|i| idx[i] as i64).collect();
                    if grid.get(&center) == 1 {
                        old_ages[idx].saturating_add(1)
                    } else {
                        1u32
                    }
                } else {
                    0u32
                }
            });

            (cells, ages)
        };

        self.grid.cells = new_cells;
        self.ages = new_ages;
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
                let idx_vec: Vec<usize> = (0..shape.len())
                    .map(|i| match i { 0 => x, 1 => y, _ => 0 })
                    .collect();
                let idx = IxDyn(&idx_vec);
                let (r, g, b) = if self.color_mode == 1 {
                    age_to_color(self.ages[idx])
                } else {
                    if self.grid.cells[idx] == 1 { (204, 204, 204) } else { (13, 13, 13) }
                };
                let base = (y * w + x) * 4;
                buf[base]     = r;
                buf[base + 1] = g;
                buf[base + 2] = b;
                buf[base + 3] = 255;
            }
        }
    }

    fn set_color_mode(&mut self, mode: u8) { self.color_mode = mode; }

    fn randomize(&mut self) {
        let mut seed_bytes = [0u8; 8];
        getrandom::getrandom(&mut seed_bytes).unwrap_or(());
        let mut seed = u64::from_le_bytes(seed_bytes);
        for (cell, age) in self.grid.cells.iter_mut().zip(self.ages.iter_mut()) {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            *cell = if seed >> 63 == 1 { 1 } else { 0 };
            *age  = *cell as u32;
        }
        self.generation = 0;
        self.population = self.grid.cells.iter().map(|&c| c as u64).sum();
    }

    fn clear(&mut self) {
        self.grid.cells.fill(0);
        self.ages.fill(0);
        self.generation = 0;
        self.population = 0;
    }

    fn load_cells(&mut self, cells: &[(i32, i32)]) {
        if cells.is_empty() { return; }

        let shape = self.grid.cells.shape().to_vec();
        let w = shape[0] as i32;
        let h = if shape.len() > 1 { shape[1] as i32 } else { 1 };

        let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
        let max_x = cells.iter().map(|(x, _)| *x).max().unwrap();
        let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();
        let max_y = cells.iter().map(|(_, y)| *y).max().unwrap();

        let ox = (w - (max_x - min_x + 1)) / 2 - min_x;
        let oy = (h - (max_y - min_y + 1)) / 2 - min_y;

        self.grid.cells.fill(0);
        self.ages.fill(0);

        for (px, py) in cells {
            let gx = px + ox;
            let gy = py + oy;
            if gx >= 0 && gx < w && gy >= 0 && gy < h {
                let idx = IxDyn(&[gx as usize, gy as usize]);
                self.grid.cells[idx.clone()] = 1;
                self.ages[idx] = 1;
            }
        }

        self.generation = 0;
        self.population = self.grid.cells.iter().map(|&c| c as u64).sum();
    }
}
