use crate::domain::location::Location;
use crate::domain::movement_decider::MovementDecider;

#[derive(Copy, Clone)]
pub struct Car<M>
where
    M: MovementDecider + Clone + Copy,
{
    decider: M,
    location: Location,
}

impl<M: MovementDecider + Clone + Copy> Car<M> {
    pub fn new(movement_decider: M) -> Self {
        Car {
            decider: movement_decider,
            location: Location::new(),
        }
    }

    pub fn race(&self) -> Self {
        if self.decider.decide() == true {
            return Car {
                decider: self.decider,
                location: self.location.increase(),
            };
        }

        *self
    }

    pub fn location(&self) -> Location {
        self.location.clone()
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

    #[derive(Copy, Clone)]
    struct StopDecider;
    impl MovementDecider for StopDecider {
        fn decide(&self) -> bool {
            false
        }
    }
    static STOP_DECIDER: StopDecider = StopDecider;

    #[test]
    fn when_new_then_instance_created() {
        //when
        let _car = Car::new(GO_DECIDER);

        //then
        //no compile error
    }

    #[test]
    fn given_go_decider_when_race_then_position_increased() {
        //given
        let car = Car::new(GO_DECIDER);

        //when
        let car = car.race();

        //then
        assert_eq!(1, car.location().current());
    }

    #[test]
    fn given_stop_decider_when_race_then_position_increased() {
        //given
        let car = Car::new(STOP_DECIDER);

        //when
        let car = car.race();

        //then
        assert_eq!(0, car.location().current());
    }

    #[test]
    fn when_location_then_location() {
        //given
        let car = Car::new(STOP_DECIDER);

        //when
        let location = car.location();

        //then
        assert_eq!(0, location.current());
    }
}
