use crate::lib::player::perks::{UpTheAnte, SlipperyMeat};
use crate::lib::player::Offering;

pub enum LuckContributor{
    Personal(f64),
    Global(f64),
    SurvivorCount(f64)
}

pub enum LuckScope{
    Personal,
    Global
}

impl LuckContributor {
    pub fn personal_bonus(&self) -> f64 {
        match self {
            LuckContributor::Personal(value) => *value,
            _ => 0.0
        }
    }

    pub fn global_bonus(&self) -> f64 {
        match self {
            LuckContributor::Global(value) => *value,
            _ => 0.0
        }
    }

    pub fn ante_coefficient(&self) -> Option<f64> {
        Some(match self {
            LuckContributor::SurvivorCount(coeff) => *coeff,
            _ => None?
        })
    }
}

impl Default for LuckContributor {
    fn default() -> Self {
        LuckContributor::Personal(0.0)
    }
}

impl From<&UpTheAnte> for LuckContributor {
    fn from(perk: &UpTheAnte) -> Self {
        let per_person_modifier: f64 = perk.luck_mod();

        LuckContributor::SurvivorCount(per_person_modifier)
    }
}

impl From<&SlipperyMeat> for LuckContributor {
    fn from(perk: &SlipperyMeat) -> Self {
        LuckContributor::Personal(perk.luck_mod())
    }
}

impl From<&Offering> for LuckContributor {
    fn from(offering: &Offering) -> Self {
        match offering.luck_scope() {
            LuckScope::Personal => LuckContributor::Personal(offering.luck_value()),
            LuckScope::Global => LuckContributor::Global(offering.luck_value())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn personal_get() {
        let input = LuckContributor::Personal(0.01);
        let result = input.personal_bonus();
        assert_eq!(0.01, result);
    }
    #[test]
    fn personal_get_on_global() {
        let input = LuckContributor::Global(0.01);
        let result = input.personal_bonus();
        assert_eq!(0.0, result);
    }
    #[test]
    fn global_get() {
        let input = LuckContributor::Global(0.01);
        let result = input.global_bonus();
        assert_eq!(0.01, result);
    }
    #[test]
    fn slippery_to_luck() {
        let input = SlipperyMeat::Two;
        let result = LuckContributor::from(&input);
        assert_eq!(0.03, result.personal_bonus());
        assert_eq!(0.0, result.global_bonus());
        assert_eq!(None, result.ante_coefficient());
    }
    #[test]
    fn offering_to_luck() {
        let input = Offering::ChalkPouch;
        let result = LuckContributor::from(&input);
        assert_eq!(0.01, result.personal_bonus());
        assert_eq!(0.00, result.global_bonus());
        assert_eq!(None, result.ante_coefficient());
    }
}
