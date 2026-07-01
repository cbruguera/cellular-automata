pub mod life;

pub trait Automaton {
    fn step(&mut self);
    fn generation(&self) -> u64;
    fn population(&self) -> u64;
    fn shape(&self) -> Vec<usize>;
    fn render_rgba(&self, buf: &mut [u8]);
    fn randomize(&mut self);
    fn clear(&mut self);
    fn load_cells(&mut self, cells: &[(i32, i32)]);
    fn set_color_mode(&mut self, _mode: u8) {}
}
