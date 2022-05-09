use crate::domain2::judge::Judge;
use crate::domain2::position::Position;

struct Car {
    position: Position,
}

impl Car {
    fn new() -> Self {
        Car {
            position: Position::new(),
        }
    }

    fn race(&mut self, j: &dyn Judge) {
        if j.judge() == true {
            self.position = self.position.increase();
        }
    }

    fn position(&self) -> &Position {
        &self.position
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
        let mut c = Car::new();

        //when
        c.race(&F);

        //then
        assert_eq!(c.position(), &Position::with_position(1));
    }
}
