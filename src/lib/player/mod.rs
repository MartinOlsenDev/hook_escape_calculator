mod loadout;
mod offerings;
mod perks;

pub use loadout::Loadout;
pub use perks::{SlipperyMeat, UpTheAnte};
pub use offerings::Offering;


pub struct Player {
    pub loadout: Loadout,
    pub is_alive: bool
}
