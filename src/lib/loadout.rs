use super::luck_record::LoadoutLuckRecord;
use super::offering::{Offering, OfferingSlot};
use super::perk::{Perk, PerkName, PerkSlot, Tier};

use arrayvec::ArrayVec;
use frunk::monoid;
use frunk::Semigroup;
use konst as kon;

use crate::constants::misc as k;

const PERKSLOT_COUNT: usize = kon::min!(k::COUNT_ALL_KNOWN_LUCK_PERKS, 4_usize);
const SLIPPERY_INDEX: usize = 0;
const UTA_INDEX: usize = 1;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loadout {
    perks: [PerkSlot; PERKSLOT_COUNT],
    offering: OfferingSlot,
}

// mutators
impl Loadout {
    pub fn set_perk_tier(&mut self, name: PerkName, tier: Option<Tier>) {
        let mut perk = self.get_perk_mut(name);

        match (&mut perk, tier) {
            (None, None) => (),
            (Some(_), None) => *perk = None,
            (None, Some(t)) => *perk = Some(Perk::new(name, t)),
            (Some(p), Some(t)) => p.set_tier(t),
        };
    }

    pub fn set_offering(&mut self, new: Option<Offering>) {
        self.offering = new;
    }
}

// accessors
impl Loadout {
    pub fn get_perk(&self, perk: PerkName) -> Option<&Perk> {
        let index = match perk {
            PerkName::SlipperyMeat => SLIPPERY_INDEX,
            PerkName::UpTheAnte => UTA_INDEX,
        };
        self.perks.get(index).and_then(Option::as_ref)
    }
    pub fn get_offering(&self) -> Option<&Offering> {
        self.offering.as_ref()
    }
}

// mutable accessors
impl Loadout {
    fn get_perk_mut(&mut self, name: PerkName) -> &mut Option<Perk> {
        let index = match name {
            PerkName::SlipperyMeat => SLIPPERY_INDEX,
            PerkName::UpTheAnte => UTA_INDEX,
        };

        self.perks
            .get_mut(index)
            .expect("{index} ought to be a valid index less than {COUNT_ALL_KNOWN_LUCK_PERKS}")
    }
}

// luck collater
impl Loadout {
    pub fn collate_luck(self) -> LoadoutLuckRecord {
        let perk_record_list: ArrayVec<LoadoutLuckRecord, { k::COUNT_ALL_KNOWN_LUCK_PERKS }> = self
            .perks
            .iter()
            .filter_map(|&perk_slot| perk_slot)
            .map(|perk| LoadoutLuckRecord::from(&perk))
            .collect();
        let offering_luck: LoadoutLuckRecord = self
            .offering
            .map(|offering| LoadoutLuckRecord::from(&offering))
            .unwrap_or_default();

        monoid::combine_all(&perk_record_list).combine(&offering_luck)
    }
}

//TODO: Refactor Tests for New Technique
/*#[cfg(test)]
mod tests {
    use super::super::perk::*;
    use super::*;
    use crate::lib::backend::luck::TeamDynamicLuck;

    #[test]
    fn example0() {
        let survivor = Loadout {
            perks: [Some(Perk::SlipperyMeat(SlipperyMeat::One)), None],
            offering: Some(Offering::SaltStatuette),
        };
        assert_eq!(survivor.make_personal_luck(), 0.02);
        assert_eq!(survivor.make_max_unhook(), 6_u8);
        assert_eq!(survivor.make_global_luck(), 0.02);
        assert_eq!(Vec::<DynamicLuck>::new(), survivor.get_dyn_luck());
    }
    #[test]
    fn example1() {
        let survivor = Loadout {
            perks: [None, Some(Perk::UpTheAnte(UpTheAnte::Three))],
            offering: Some(Offering::ChalkPouch),
        };
        assert_eq!(survivor.make_personal_luck(), 0.01);
        assert_eq!(survivor.make_max_unhook(), 3_u8);
        assert_eq!(survivor.make_global_luck(), 0.0);
        assert_eq!(
            vec![DynamicLuck::Team(TeamDynamicLuck::UpTheAnte(
                UpTheAnte::Three
            ))],
            survivor.get_dyn_luck()
        );
    }
}*/
