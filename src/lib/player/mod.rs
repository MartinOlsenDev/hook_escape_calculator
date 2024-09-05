mod loadout;
mod luck;

pub use loadout::Loadout;
pub use luck::offerings::Offering;


pub struct Player {
    pub loadout: Loadout,
    pub is_alive: bool
}

impl Default for Player {
    fn default() -> Self {
        todo!()
    }
}
