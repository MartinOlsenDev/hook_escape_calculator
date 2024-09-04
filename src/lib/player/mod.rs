mod loadout;
mod offerings;
pub mod perks;

pub use loadout::Loadout;
pub use offerings::Offering;


pub struct Player {
    pub loadout: Loadout,
    pub is_alive: bool
}
