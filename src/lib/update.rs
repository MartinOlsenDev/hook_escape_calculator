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
