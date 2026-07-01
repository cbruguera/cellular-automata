use wasm_bindgen::prelude::*;
use serde::Deserialize;

mod grid;
mod neighborhood;
mod automata;
mod rle;

use automata::{life::{Life, LifeConfig}, Automaton};

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum AutomatonConfig {
    Life(LifeConfig),
}

#[wasm_bindgen]
pub struct Simulation {
    inner: Box<dyn Automaton>,
    pixel_buf: Vec<u8>,
}

#[wasm_bindgen]
impl Simulation {
    pub fn create(config_json: &str) -> Result<Simulation, JsValue> {
        let config: AutomatonConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let automaton: Box<dyn Automaton> = match config {
            AutomatonConfig::Life(c) => Box::new(Life::new(c)),
        };

        let shape = automaton.shape();
        let pixel_buf = vec![0u8; shape[0] * shape.get(1).copied().unwrap_or(1) * 4];

        Ok(Simulation { inner: automaton, pixel_buf })
    }

    pub fn load_rle(&mut self, rle: &str) -> Result<(), JsValue> {
        let cells = rle::parse(rle)
            .map_err(|e| JsValue::from_str(&e))?;
        self.inner.load_cells(&cells);
        Ok(())
    }

    pub fn set_color_mode(&mut self, mode: u8) { self.inner.set_color_mode(mode); }

    pub fn step(&mut self) { self.inner.step(); }
    pub fn render(&mut self) { self.inner.render_rgba(&mut self.pixel_buf); }
    pub fn randomize(&mut self) { self.inner.randomize(); }
    pub fn clear(&mut self) { self.inner.clear(); }

    pub fn pixel_buf_ptr(&self) -> *const u8 { self.pixel_buf.as_ptr() }
    pub fn pixel_buf_len(&self) -> usize { self.pixel_buf.len() }

    pub fn generation(&self) -> u64 { self.inner.generation() }
    pub fn population(&self) -> u64 { self.inner.population() }
    pub fn width(&self) -> usize { self.inner.shape()[0] }
    pub fn height(&self) -> usize { self.inner.shape().get(1).copied().unwrap_or(1) }
}
