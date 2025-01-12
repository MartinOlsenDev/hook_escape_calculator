use super::luck_record::LoadoutLuckRecord;
use super::offering::Offering;
use super::perk::{Perk, COUNT_ALL_KNOWN_LUCK_PERKS};
use arrayvec::ArrayVec;
use frunk::monoid;
use frunk::Semigroup;
use konst as kon;
use COUNT_ALL_KNOWN_LUCK_PERKS as COUNT_PERKS;

const PERKSLOT_COUNT: usize = kon::min!(COUNT_PERKS, 4_usize);

type PerkSlot = Option<Perk>;
type OfferingSlot = Option<Offering>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loadout {
    perks: [PerkSlot; PERKSLOT_COUNT],
    offering: OfferingSlot,
}

impl Loadout {
    pub fn perk_mut(&mut self, i: usize) -> Option<&mut Perk> {
        self.perks.get_mut(i).and_then(|item| item.as_mut())
    }
    pub fn offering_mut(&mut self) -> Option<&mut Offering> {
        self.offering.as_mut()
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
            .unwrap_or(LoadoutLuckRecord::default());

        monoid::combine_all(&perk_record_list).combine(&offering_luck)
    }
}

impl Default for Loadout {
    fn default() -> Self {
        Loadout {
            perks: [None; 2],
            offering: None,
        }
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
