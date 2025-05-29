use crate::constants::misc as k;

use super::{
    luck_record::LoadoutLuckRecord,
    offering::OfferingSlot,
    perk::{Perk, PerkName, PerkSlot},
    update::{LoadoutUpdate, PerkUpdate},
};

const SLIPPERY_INDEX: usize = 0;
const UTA_INDEX: usize = 1;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loadout {
    perks: [PerkSlot; k::PERKSLOT_COUNT],
    offering: OfferingSlot,
}

impl Loadout {
    pub fn alter(&mut self, update: LoadoutUpdate) {
        match update {
            LoadoutUpdate::Offering(x) => self.offering = x,
            LoadoutUpdate::Perk(x) => self.perk_update(x),
        };
    }
    fn perk_update(&mut self, update: PerkUpdate) {
        let perk: &mut PerkSlot = self.get_perk_mut(*update.perk());
        let new = PerkSlot::new(
            update
                .value()
                .into_inner()
                .map(|t| Perk::new(*update.perk(), t)),
        );

        *perk = new;
    }
}

// accessors
impl Loadout {
    fn perk_index(perk: PerkName) -> usize {
        match perk {
            PerkName::SlipperyMeat => SLIPPERY_INDEX,
            PerkName::UpTheAnte => UTA_INDEX,
        }
    }
    pub fn get_perk(&self, perk: PerkName) -> &PerkSlot {
        let index = Self::perk_index(perk);
        let expect_msg = format!(
            "Const index of {:?} is {:?}, which should be <= possible max {}.",
            perk,
            index,
            k::PERKSLOT_COUNT - 1
        );
        self.perks.get(index).expect(&expect_msg)
    }
    pub fn get_offering(&self) -> &OfferingSlot {
        &self.offering
    }
}

// mutable accessors
impl Loadout {
    fn get_perk_mut(&mut self, perk: PerkName) -> &mut PerkSlot {
        let index = Self::perk_index(perk);

        self.perks
            .get_mut(index)
            .expect("{index} ought to be a valid index less than {COUNT_ALL_KNOWN_LUCK_PERKS}")
    }
}

// luck collater
impl Loadout {
    pub fn collate_luck(&self) -> LoadoutLuckRecord {
        let perk_records = self
            .perks
            .iter()
            .filter_map(|perk_slot| perk_slot.into_inner())
            .map(|perk| LoadoutLuckRecord::from(&perk));

        let offering_luck: LoadoutLuckRecord = self
            .offering
            .map(|offering| LoadoutLuckRecord::from(&offering))
            .unwrap_or_default();

        &offering_luck + &perk_records.fold(LoadoutLuckRecord::default(), |acc, x| &acc + &x)
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
