use bon::bon;
use derive_getters::Getters;
use nutype::nutype;

use super::{
    offering,
    offering::OfferingSlot,
    perk::{PerkName, TierSlot},
};
use crate::constants::misc as k;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Getters)]
pub struct SurvivorUpdate {
    id: SurvivorId,
    update: SurvivorUpdateData,
}

#[bon]
impl SurvivorUpdate {
    fn from_perk(id: SurvivorId, perk: PerkName, tier: TierSlot) -> Self {
        Self {
            id,
            update: SurvivorUpdateData::LoadoutUpdate(LoadoutUpdate::Perk(PerkUpdate {
                perk,
                value: tier,
            })),
        }
    }
    #[builder]
    pub fn perk(id: SurvivorId, perk: PerkName, tier: TierSlot) -> Self {
        Self::from_perk(id, perk, tier)
    }
    #[builder]
    pub fn perk_usize(id: usize, perk: PerkName, tier: TierSlot) -> Result<Self, SurvivorIdError> {
        let id = SurvivorId::try_new(id)?;
        Ok(Self::from_perk(id, perk, tier))
    }
    fn from_offering(id: SurvivorId, offering: OfferingSlot) -> Self {
        Self {
            id,
            update: SurvivorUpdateData::LoadoutUpdate(LoadoutUpdate::Offering(offering)),
        }
    }
    #[builder]
    pub fn offering(id: SurvivorId, offering: OfferingSlot) -> Self {
        Self::from_offering(id, offering)
    }
    #[builder]
    pub fn offering_usize(
        id: usize,
        offering: offering::OfferingSlot,
    ) -> Result<Self, SurvivorIdError> {
        let id = SurvivorId::try_new(id)?;
        Ok(Self::from_offering(id, offering))
    }
    #[builder]
    pub fn living_status(id: SurvivorId, alive: bool) -> Self {
        Self {
            id,
            update: SurvivorUpdateData::Life(alive),
        }
    }
    fn from_data(id: SurvivorId, update: SurvivorUpdateData) -> Self {
        Self { id, update }
    }
    #[builder]
    pub fn data(id: SurvivorId, update: SurvivorUpdateData) -> Self {
        Self::from_data(id, update)
    }
    #[builder]
    pub fn data_usize(id: usize, update: SurvivorUpdateData) -> Result<Self, SurvivorIdError> {
        let id = SurvivorId::try_new(id)?;
        Ok(Self::from_data(id, update))
    }
}

#[nutype(
    validate(less = k::TEAM_MAX_CAPACITY),
    derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsRef, Deref, Hash, Display)
)]
pub struct SurvivorId(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SurvivorUpdateData {
    LoadoutUpdate(LoadoutUpdate),
    Life(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadoutUpdate {
    Perk(PerkUpdate),
    Offering(OfferingSlot),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Getters)]
pub struct PerkUpdate {
    perk: PerkName,
    value: TierSlot,
}

#[cfg(test)]
pub mod arb {
    use super::*;
    use proptest::prelude::*;
    use super::super::{perk, offering};

    prop_compose! {
        pub fn survivor_update_data()(
            id in 0..{k::TEAM_MAX_CAPACITY-1},
            is_alive in 0..1,
            offering in offering::arb::offering_slot_strategy(),
            perk_name in perk::arb::name(),
            perk_tier_slot in perk::arb::tier_slot(),
            choice in 0..2
        ) -> SurvivorUpdate {
            let id: SurvivorId = SurvivorId::try_new(id).expect("choice should be in team capacity");
            
            match choice {
                0 => SurvivorUpdate::living_status().id(id).alive(is_alive != 0).call(),
                1 => SurvivorUpdate::offering().id(id).offering(offering).call(),
                2 => SurvivorUpdate::perk().id(id).perk(perk_name).tier(perk_tier_slot).call(),
                _ => unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{team, perk};
    use proptest::prelude::*;

    /// Given an update and a team, returns true iff the application of
    /// the update on the team would cause no changes. Because this
    /// is used to prove that Team::alter works, Team::alter may
    /// not be used in this function.
    fn is_noop(su: SurvivorUpdate, t: team::Team) -> bool {
        let survivor = t.get_player(su.id);

        match su.update {
            SurvivorUpdateData::LoadoutUpdate(LoadoutUpdate::Offering(o)) => {
                survivor.offering() == &o
            },
            SurvivorUpdateData::LoadoutUpdate(LoadoutUpdate::Perk(p)) => {
                let update_tier: Option<perk::Tier> = p.value().into_inner();
                let current_tier: Option<perk::Tier> = survivor.get_perk_tier(*p.perk()).copied();
                current_tier == update_tier
            },
            SurvivorUpdateData::Life(is_alive) => survivor.is_alive() == is_alive,
        }
    }

    proptest! {
        #[test]
        fn update_changes_team(
            update in arb::survivor_update_data(),
            mut team in team::arb::team()
        ) {
            prop_assume! { !is_noop(update, team) };
            team.alter(update);
            prop_assert!(is_noop(update, team))
        }
    }
}