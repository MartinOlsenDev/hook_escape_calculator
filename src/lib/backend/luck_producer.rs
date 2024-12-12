use super::luck_record::LoadoutLuckRecord;

pub trait LuckProducer {
    fn make_luck(&self) -> LoadoutLuckRecord;
}
