use crate::lib::backend::luck_mod::{CalculatableLuck, CalculatedLuck, DynamicLuck, Luck, LuckSource};
use super::perks::{Perk, SlipperyMeat, UpTheAnte};
use super::offerings::Offering;
use super::living_count::LivingCount;

const BASE_UNHOOK_CHANCE: f64 = 0.04;

type PerkSlot = Option<Perk>;
type OfferingSlot = Option<Offering>;

#[derive(Debug, Clone, Copy)]
pub struct Loadout {
    perks: [PerkSlot; 4],
    offering: OfferingSlot
}


// Personally active unhook modifiers from this player
impl Loadout {
    pub fn make_max_unhook(&self) -> u8 {
        let has_slippery = self.perks.iter()
            .any(|perk| if let Some(Perk::SlipperyMeat(_)) = perk { true } else { false });
        match has_slippery {
            true => 6,
            false => 3
        }
    }
    pub fn make_personal_luck(&self) -> Luck {
        let perk_luck: Luck = self.perks.iter()
            .map(|slot| slot.map(|perk| -> LuckSource { perk.into() }))
            .map(|slot| match slot {
                Some(LuckSource::Calculated(luck)) => luck.get_personal(),
                _ => 0.0
            })
            .sum();
        let offering_luck = self.offering
            .map(|offering| offering.personal_luck())
            .map(|x| x.get_personal())
            .unwrap_or(0.0);
        perk_luck + offering_luck
    }
}

// Globally active unhook modifiers from this player
impl Loadout {
    /// Returns a list of the dynamic luck sources that await calculation.
    pub fn get_dyn_luck(&self) -> Vec<DynamicLuck> {
        self.perks.iter()
            .filter_map(|&perk| perk)
            .map(|perk| -> LuckSource { perk.into() })
            .filter_map(|luck| luck.get_dynamic())
            .collect()
    }

    pub fn global_static_modifier(&self) -> Luck {
        let perk_luck: Luck = self.perks.iter()
            .map(|slot| slot.map(|perk| -> LuckSource { perk.into() }))
            .map(|slot| match slot {
                Some(LuckSource::Calculated(CalculatedLuck::Global(luck))) => luck,
                _ => 0.0
            })
            .sum();

        let offering_luck: Luck = self.offering
            .map(|offering| offering.personal_luck() )
            .map(|luck| luck.get_global() )
            .unwrap_or(0.0);

        perk_luck + offering_luck
    }
}

impl Default for Loadout {
    fn default() -> Self {
        Loadout {
            perks: [None; 4],
            offering: None
        }
    }
}

#[cfg(test)]
mod tests {
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