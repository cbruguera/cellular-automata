pub mod life;

pub trait Automaton {
    fn step(&mut self);
    fn generation(&self) -> u64;
    fn population(&self) -> u64;
    fn shape(&self) -> Vec<usize>;
    fn render_rgba(&self, buf: &mut [u8]);
    fn randomize(&mut self);
    fn clear(&mut self);
}
