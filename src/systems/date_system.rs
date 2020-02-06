extern crate specs;
use crate::gamelog::{GameLog, GeneralLog, WorldStatLog};
use specs::prelude::*;

pub struct DateSystem {}

impl<'a> System<'a> for DateSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, Date>,
        WriteExpect<'a, WorldStatLog>,
        WriteExpect<'a, GeneralLog>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (_log, mut date, mut world_logs, mut general_logs) = data;

        date.new_day();

        let buf = format!(
            "\n********Day {} of year {}********",
            date.get_day(),
            date.get_year()
        );
        world_logs.entries.push(buf.clone());
        general_logs.entries.push(buf.clone());
    }
}

#[derive(Clone)]
pub struct Date {
    days_count: i64,
}

impl Date {
    pub const YEAR_DURATION: i64 = 365;

    #[allow(clippy::new_without_default)]
    pub fn new() -> Date {
        Date { days_count: 0 }
    }

    pub fn new_day(&mut self) {
        self.days_count += 1;
    }

    pub fn pass_time(&mut self, days_passed: i32) {
        self.days_count += days_passed as i64;
    }

    pub fn get_day(&self) -> i32 {
        (self.days_count % Self::YEAR_DURATION) as i32
    }
    pub fn get_year(&self) -> i32 {
        (self.days_count / Self::YEAR_DURATION) as i32
    }

    pub fn get_total_day(&self) -> i64 {
        self.days_count
    }

    pub fn get_date(&self) -> Date {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn date_test() {
        let mut date = Date::new();

        assert_ne!(Date::YEAR_DURATION, 0);

        assert_eq!(date.get_total_day(), 0);
        assert_eq!(date.get_day(), 0);
        assert_eq!(date.get_year(), 0);

        date.new_day();

        assert_eq!(date.get_total_day(), 1);
        assert_eq!(date.get_day(), 1);
        assert_eq!(date.get_year(), (1 / Date::YEAR_DURATION) as i32);

        let time_passed = 56982;
        date.pass_time(time_passed);

        assert_eq!(date.get_total_day(), time_passed as i64 + 1); //seem likke I am rewriting the function in the test, prety stupid
        assert_eq!(
            date.get_day(),
            (time_passed + 1) % Date::YEAR_DURATION as i32
        );
        assert_eq!(
            date.get_year(),
            ((time_passed + 1) / Date::YEAR_DURATION as i32)
        );
    }
}
