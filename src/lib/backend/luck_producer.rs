pub trait Luck_Producer {
    fn make_personal(&self) -> f64;
    fn make_static_global(&self) -> f64;
    fn make_dyn_global_living_count(&self, living_count: u8) -> f64;
    pub fn make_luck(&self, living_count: u8) -> (f64, f64) {
        (
            &self.make_personal(),
            &self.static_global() + self.make_dyn_global_living_count(living_count),
        )
    }
}
