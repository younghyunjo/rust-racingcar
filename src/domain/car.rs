use crate::domain::judge::Judge;
use crate::domain::name::Name;
use crate::domain::position::Position;
use crate::RaceResult;

pub struct Car {
    name: Name,
    position: Position,
}

impl Car {
    pub fn new(name: Name) -> Self {
        Car::with_position(name, Position::new())
    }

    pub fn with_position(name: Name, position: Position) -> Self {
        Car { name, position }
    }

    pub fn race(&self, judge: &dyn Judge) -> Car {
        if judge.judge() == true {
            return Car::with_position(self.name(), self.position.increase());
        }
        return Car::with_position(self.name(), self.position());
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn result(&self) -> RaceResult {
        RaceResult::new(&self.name, self.position.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::car::Car;
    use crate::domain::judge::Judge;
    use crate::domain::name::Name;
    use crate::domain::position::Position;

    struct Fixture;
    impl Judge for Fixture {
        fn judge(&self) -> bool {
            true
        }
    }

    static F: Fixture = Fixture;

    #[test]
    fn when_new_then_created() {
        let _ = Car::new(Name::new("".into()).unwrap());
    }

    #[test]
    fn when_race_then_position_increased() {
        //given
        let c = Car::new(Name::new("".into()).unwrap());

        //when
        let c = c.race(&F);

        //then
        assert_eq!(c.position(), Position::from(1));
    }

    #[test]
    fn when_name_then_returns_name() {
        //given
        let name = Name::new("name".into()).unwrap();
        let c = Car::new(name.clone());

        //when,then
        assert_eq!(c.name(), name);
    }

    #[test]
    fn when_result_then_return_results() {
        //given
        let name = Name::new("name".into()).unwrap();
        let c = Car::new(name.clone());

        let result = c.result();
        assert_eq!(result.name(), c.name());
        assert_eq!(result.position(), Position::from(0));
    }
}
