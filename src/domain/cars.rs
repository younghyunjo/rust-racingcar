use crate::domain::car::Car;
use crate::domain::location::Location;
use crate::domain::movement_decider::MovementDecider;

pub struct Cars<M>
where
    M: MovementDecider + Clone + Copy,
{
    cars: Vec<Car<M>>,
}

impl<M: MovementDecider + Clone + Copy> Cars<M> {
    pub fn new(cars: Vec<Car<M>>) -> Self {
        Cars { cars }
    }

    pub fn race(&mut self) {
        for n in 0..self.cars.len() {
            self.cars[n] = self.cars[n].race();
        }
    }

    pub fn location(&self) -> Vec<Location> {
        self.cars.iter().map(|car| car.location()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Copy, Clone)]
    struct GoDecider;
    impl MovementDecider for GoDecider {
        fn decide(&self) -> bool {
            true
        }
    }
    static GO_DECIDER: GoDecider = GoDecider;

    #[test]
    fn when_new_then_created() {
        //given
        let car = Car::new(GO_DECIDER);

        //when
        let _ = Cars::new(vec![car]);

        //then
        //no compile error
    }

    #[test]
    fn given_go_decider_when_race_then_location_is_increased() {
        //given
        let car = Car::new(GO_DECIDER);
        let mut cars = Cars::new(vec![car]);

        //when
        cars.race();

        //then
        assert_eq!(cars.location()[0].current(), 1);
    }

    #[test]
    fn given_two_cars_when_race_then_two_cars_are_moved() {
        let car0 = Car::new(GO_DECIDER);
        let car1 = Car::new(GO_DECIDER);

        let mut cars = Cars::new(vec![car0, car1]);

        cars.race();

        let locations = cars.location();


        assert_eq!(locations[0].current(), 1);
        assert_eq!(locations[1].current(), 1);
    }
}
