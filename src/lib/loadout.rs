use crate::constants::misc as k;

use super::{
    luck_record::LoadoutLuckRecord,
    offering::OfferingSlot,
    perk::{Perk, PerkName, PerkSlot},
    update::{LoadoutUpdate, PerkUpdate},
};

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
    /// Any implementation of perk_label is acceptable so long
    /// as its inverse function exists, it returns unsigned, and 
    /// tests::all_perks_valid_index passes
    const fn perk_label(perk: PerkName) -> usize {
        match perk {
            PerkName::SlipperyMeat => 0,
            PerkName::UpTheAnte => 1,
        }
    }
    pub fn get_perk(&self, perk: PerkName) -> &PerkSlot {
        let index = Self::perk_label(perk);
        let expect_msg = format!(
            "Const index of {:?} is {:?}, which should be <= possible max {}.",
            perk,
            index,
            k::PERKSLOT_COUNT - 1
        );
        self.perks.get(index).expect(&expect_msg)
    }
    pub fn offering(&self) -> &OfferingSlot {
        &self.offering
    }
}

// mutable accessors
impl Loadout {
    fn get_perk_mut(&mut self, perk: PerkName) -> &mut PerkSlot {
        let index = Self::perk_label(perk);

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

        perk_records.fold(offering_luck, |acc, x| &acc + &x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use super::super::perk;

    proptest! {
        #[test]
        fn all_perks_valid_index(perk in perk::arb::name()) {
            assert!(Loadout::perk_label(perk) < k::PERKSLOT_COUNT)
        }
    }

}
