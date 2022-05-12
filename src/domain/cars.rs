use crate::domain::car::Car;
use crate::domain::judge::Judge;
use crate::domain::position::Position;

pub struct Cars {
    cars: Vec<Car>,
}

impl Cars {
    pub fn new(nr_cars: u32) -> Self {
        let mut cars: Vec<Car> = vec![];
        for _ in 0..nr_cars {
            cars.push(Car::new());
        }
        Cars { cars }
    }

    pub fn with_cars(cars: Vec<Car>) -> Self {
        Cars { cars }
    }

    pub fn race(&self, judge: &dyn Judge) -> Self {
        Cars::with_cars(self.cars.iter().map(|c| c.race(judge)).collect())
    }

    pub fn positions(&self) -> Vec<Position> {
        self.cars.iter().map(|c| c.position()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::cars::Cars;
    use crate::domain::judge::Judge;
    use crate::domain::position::Position;

    struct Fixture;
    impl Judge for Fixture {
        fn judge(&self) -> bool {
            true
        }
    }

    static F: Fixture = Fixture;

    #[test]
    fn given_nr_cars_when_new_then_created() {
        let nr_cars: u32 = 2;
        let _ = Cars::new(nr_cars);
    }

    #[test]
    fn when_race_then_positions_are_increated() {
        let cars = Cars::new(2);

        let cars = cars.race(&F);

        let positions = cars.positions();
        for p in positions.into_iter() {
            assert_eq!(p, Position::from(1));
        }
    }
}
