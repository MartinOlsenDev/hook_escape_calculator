use super::luck_record::LoadoutLuckRecord;
use super::offering::Offering;
use super::perk::{Perk, COUNT_ALL_KNOWN_LUCK_PERKS, Tier, PerkName};
use arrayvec::ArrayVec;
use frunk::monoid;
use frunk::Semigroup;
use konst as kon;
use COUNT_ALL_KNOWN_LUCK_PERKS as COUNT_PERKS;

const PERKSLOT_COUNT: usize = kon::min!(COUNT_PERKS, 4_usize);
const SLIPPERY_INDEX: usize = 0;
const UTA_INDEX: usize = 1;

type PerkSlot = Option<Perk>;
type OfferingSlot = Option<Offering>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loadout {
    perks: [PerkSlot; PERKSLOT_COUNT],
    offering: OfferingSlot,
}

impl Loadout {
    pub fn set_slippery(&mut self, tier: Option<Tier>) {
        let sm = self.perks.get_mut(SLIPPERY_INDEX).expect("Slippery Meat index at 0 ought to exist.");

        *sm = tier.map(|t| Perk::new(PerkName::SlipperyMeat, t));
    }

    pub fn set_uta(&mut self, tier: Option<Tier>) {
        let uta = self.perks.get_mut(UTA_INDEX).expect("UTA index at 1 ought to exist.");

        *uta = tier.map(|t| Perk::new(PerkName::UpTheAnte, t));
    }

    pub fn set_offering(&mut self, new: Option<Offering>) {
        self.offering = new;
    }
}

impl Loadout {
    pub fn collate_luck(&self) -> LoadoutLuckRecord {
        let perk_record_list: ArrayVec<LoadoutLuckRecord, COUNT_PERKS> = self
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
