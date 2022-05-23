use crate::domain::car::Car;
use crate::domain::judge::Judge;
use crate::domain::name::Name;
use crate::RaceResult;

pub struct Cars {
    cars: Vec<Car>,
}

impl Cars {
    pub fn new(names: Vec<Name>) -> Self {
        let mut cars: Vec<Car> = vec![];
        for name in names {
            cars.push(Car::new(&name));
        }
        Cars { cars }
    }

    pub fn with_cars(cars: Vec<Car>) -> Self {
        Cars { cars }
    }

    pub fn race(&self, judge: &dyn Judge) -> Self {
        Cars::with_cars(self.cars.iter().map(|c| c.race(judge)).collect())
    }

    pub fn results(&self) -> Vec<RaceResult> {
        self.cars.iter().map(|c| c.result()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::cars::Cars;
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
    fn given_names_when_new_then_created() {
        let names = vec![Name::new("a").unwrap(), Name::new("b").unwrap()];
        let _ = Cars::new(names);
    }

    #[test]
    fn when_race_then_positions_are_increated() {
        let names = vec![Name::new("a").unwrap(), Name::new("b").unwrap()];

        let cars = Cars::new(names);

        let cars = cars.race(&F);

        let positions = cars.positions();
        for p in positions.into_iter() {
            assert_eq!(p, Position::from(1));
        }
    }

    #[test]
    fn when_results_then_return_results() {
        let names = vec![Name::new("a").unwrap(), Name::new("b").unwrap()];

        let cars = Cars::new(names);
        let cars = cars.race(&F);

        let results = cars.results();

        for r in results {
            assert_eq!(r.position(), Position::from(1));
        }
    }
}
