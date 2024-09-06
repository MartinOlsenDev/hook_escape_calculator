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
