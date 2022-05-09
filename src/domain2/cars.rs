use crate::domain2::car::Car;
use crate::domain2::judge::Judge;
use crate::domain2::position::Position;

struct Cars {
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

    pub fn race(&mut self, judge: &dyn Judge) {
        for car in self.cars.iter_mut() {
            car.race(judge);
        }
    }

    pub fn positions(&self) -> Vec<Position> {
        self.cars.iter().map(|c| c.position()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain2::cars::Cars;
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
    fn given_nr_cars_when_new_then_created() {
        let nr_cars: u32 = 2;
        let _ = Cars::new(nr_cars);
    }

    #[test]
    fn when_race_then_positions_are_increated() {
        let mut cars = Cars::new(2);

        cars.race(&F);

        let positions = cars.positions();
        for p in positions.into_iter() {
            assert_eq!(p, Position::with_position(1));
        }
    }
}
