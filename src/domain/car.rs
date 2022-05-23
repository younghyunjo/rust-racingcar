use crate::domain::judge::Judge;
use crate::domain::name::Name;
use crate::domain::position::Position;

pub struct Car {
    position: Position,
    name: Name,
}

impl Car {
    pub fn new(name: &Name) -> Self {
        Car {
            position: Position::new(),
            name: name.clone(),
        }
    }

    pub fn with_position(name: &Name, position: Position) -> Self {
        Car {
            position,
            name: name.clone(),
        }
    }

    pub fn race(&self, judge: &dyn Judge) -> Car {
        if judge.judge() == true {
            return Car::with_position(&self.name(), self.position.increase());
        }
        return Car::with_position(&self.name(), self.position());
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }

    pub fn name(&self) -> Name {
        self.name.clone()
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
        let _ = Car::new(&Name::new("").unwrap());
    }

    #[test]
    fn when_race_then_position_increased() {
        //given
        let c = Car::new(&Name::new("").unwrap());

        //when
        let c = c.race(&F);

        //then
        assert_eq!(c.position(), Position::from(1));
    }

    #[test]
    fn when_name_then_returns_name() {
        //given
        let name = Name::new("name").unwrap();
        let c = Car::new(&name.clone());

        //when,then
        assert_eq!(c.name(), name);
    }
}
