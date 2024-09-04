use crate::lib::luck::LuckContributor;
use crate::lib::player::offerings::Offering;
use super::{perks, SlipperyMeat};
use crate::lib::team::living_count::LivingCount;

const BASE_UNHOOK_CHANCE: f64 = 0.04;

pub struct Loadout {
    slippery_meat_status: Option<perks::SlipperyMeat>,
    up_the_ante_status: Option<perks::UpTheAnte>,
    offering: Option<Offering>
}


// Personally active unhook modifiers from this player
impl Loadout {
    pub fn make_max_unhook(&self) -> u8 {
        match self.slippery_meat_status {
            Some(_) => 6,
            None => 3
        }
    }

    pub fn make_personal_luck(&self) -> f64 {
        let slippery_contrib = if let Some(slippery) = &self.slippery_meat_status {
            LuckContributor::from(slippery).personal_bonus()
        } else { 0.0 };

        let offering_contrib = if let Some(ref offering) = &self.offering {
            LuckContributor::from(offering).personal_bonus()
        } else { 0.0 };

        slippery_contrib + offering_contrib
    }
}

// Globally active unhook modifiers from this player
impl Loadout {
    /// Given a number of living non-self players, returns the bonus from Up the Ante
    pub fn ante_calculator(&self, living_count: LivingCount) -> f64 {
        match &self.up_the_ante_status {
            Some(ref ante) => {
                let l = LuckContributor::from(ante);
                l.ante_coefficient().unwrap_or(0.0) * living_count.0 as f64
            },
            None => 0.0
        }
    }

    pub fn global_static_modifier(&self) -> f64 {
        self.offering.as_ref()
            .map(|o| LuckContributor::from(o).global_bonus())
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::player::UpTheAnte;
    use super::*;

    #[test]
    fn example0() {
        let survivor = Loadout {
            slippery_meat_status: Some(SlipperyMeat::One),
            up_the_ante_status: None,
            offering: Some(Offering::SaltStatuette)
        };
        assert_eq!(survivor.make_personal_luck(), 0.02);
        assert_eq!(survivor.make_max_unhook(), 6_u8);
        assert_eq!(survivor.ante_calculator(LivingCount(3)), 0.0);
        assert_eq!(survivor.global_static_modifier(), 0.02);
    }
    #[test]
    fn example1() {
        let survivor = Loadout {
            slippery_meat_status: None,
            up_the_ante_status: Some(UpTheAnte::Three),
            offering: Some(Offering::ChalkPouch)
        };
        assert_eq!(survivor.make_personal_luck(), 0.01);
        assert_eq!(survivor.make_max_unhook(), 3_u8);
        assert_eq!(survivor.ante_calculator(LivingCount(2)), 0.06);
        assert_eq!(survivor.global_static_modifier(), 0.0);
    }
}