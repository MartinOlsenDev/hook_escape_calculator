use super::offering::Offering;
use super::perk::{Perk, SlipperyMeat, UpTheAnte};
use crate::lib::backend::luck::{CalculatableLuck, CalculatedLuck, DynamicLuck, Luck, LuckSource};

const BASE_UNHOOK_CHANCE: f64 = 0.04;

type PerkSlot = Option<Perk>;
type OfferingSlot = Option<Offering>;

#[derive(Debug, Clone, Copy)]
pub struct Loadout {
    perks: [PerkSlot; 4], // todo: consider making it size 2
    offering: OfferingSlot,
}

// Personally active unhook modifiers from this player
impl Loadout {
    pub fn make_max_unhook(&self) -> u8 {
        let has_slippery = self.perks.iter().any(|perk| {
            if let Some(Perk::SlipperyMeat(_)) = perk {
                true
            } else {
                false
            }
        });
        match has_slippery {
            true => 6,
            false => 3,
        }
    }
    pub fn make_personal_luck(&self) -> Luck {
        let perk_luck: Luck = self
            .perks
            .iter()
            .map(|slot| slot.map(|perk| -> LuckSource { perk.into() }))
            .map(|slot| match slot {
                Some(LuckSource::Calculated(luck)) => luck.get_personal(),
                _ => 0.0,
            })
            .sum();
        let offering_luck = self
            .offering
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
        self.perks
            .iter()
            .filter_map(|&perk| perk)
            .map(|perk| -> LuckSource { perk.into() })
            .filter_map(|luck| luck.get_dynamic())
            .collect()
    }

    pub fn global_static_modifier(&self) -> Luck {
        let perk_luck: Luck = self
            .perks
            .iter()
            .map(|slot| slot.map(|perk| -> LuckSource { perk.into() }))
            .map(|slot| match slot {
                Some(LuckSource::Calculated(CalculatedLuck::Global(luck))) => luck,
                _ => 0.0,
            })
            .sum();

        let offering_luck: Luck = self
            .offering
            .map(|offering| offering.global_luck())
            .map(|luck| luck.get_global())
            .unwrap_or(0.0);

        perk_luck + offering_luck
    }
}

impl Default for Loadout {
    fn default() -> Self {
        Loadout {
            perks: [None; 4],
            offering: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::perk::*;
    use super::*;
    use crate::lib::backend::luck::TeamDynamicLuck;

    #[test]
    fn example0() {
        let survivor = Loadout {
            perks: [
                Some(Perk::SlipperyMeat(SlipperyMeat::One)),
                None,
                None,
                None,
            ],
            offering: Some(Offering::SaltStatuette),
        };
        assert_eq!(survivor.make_personal_luck(), 0.02);
        assert_eq!(survivor.make_max_unhook(), 6_u8);
        assert_eq!(survivor.global_static_modifier(), 0.02);
        assert_eq!(Vec::<DynamicLuck>::new(), survivor.get_dyn_luck());
    }
    #[test]
    fn example1() {
        let survivor = Loadout {
            perks: [None, Some(Perk::UpTheAnte(UpTheAnte::Three)), None, None],
            offering: Some(Offering::ChalkPouch),
        };
        assert_eq!(survivor.make_personal_luck(), 0.01);
        assert_eq!(survivor.make_max_unhook(), 3_u8);
        assert_eq!(survivor.global_static_modifier(), 0.0);
        assert_eq!(
            vec![DynamicLuck::Team(TeamDynamicLuck::UpTheAnte(
                UpTheAnte::Three
            ))],
            survivor.get_dyn_luck()
        );
    }
}
