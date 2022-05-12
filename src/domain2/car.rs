use crate::domain2::judge::Judge;
use crate::domain2::position::Position;

pub struct Car {
    position: Position,
}

impl Car {
    pub fn new() -> Self {
        Car {
            position: Position::new(),
        }
    }

    pub fn with_position(position: Position) -> Self {
        Car { position }
    }

    pub fn race(&self, judge: &dyn Judge) -> Car {
        if judge.judge() == true {
            return Car::with_position(self.position.increase());
        }
        return Car::with_position(self.position());
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain2::car::Car;
    use crate::domain2::judge::Judge;
    use crate::domain2::position::Position;

    struct Fixture;
    impl Judge for Fixture {
        fn judge(&self) -> bool {
            true
        }
    }

    static F: Fixture = Fixture;

    #[test]
    fn when_new_then_created() {
        let _ = Car::new();
    }

    #[test]
    fn when_race_then_position_increased() {
        //given
        let c = Car::new();

        //when
        let c = c.race(&F);

        //then
        assert_eq!(c.position(), Position::with_position(1));
    }
}
