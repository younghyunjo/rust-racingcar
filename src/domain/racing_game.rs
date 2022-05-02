use crate::domain::cars::Cars;
use crate::domain::location::Location;
use crate::domain::movement_decider::MovementDecider;
use std::borrow::Borrow;

pub trait RacingGameCallback {
    fn on_race_single_track(&self, locations: &Vec<Location>);
}

pub struct RacingGame<M>
where
    M: MovementDecider + Copy + Clone,
{
    nr_cars: u32,
    cars: Cars<M>,
    callbacks: Option<Box<dyn RacingGameCallback>>,
}

pub struct RandomDecider;

impl MovementDecider for RandomDecider {
    fn decide(&self) -> bool {
        true
    }
}

#[derive(Copy, Clone)]
struct GoDecider2;

impl MovementDecider for GoDecider2 {
    fn decide(&self) -> bool {
        true
    }
}

impl<M: MovementDecider + Copy + Clone> RacingGame<M> {
    pub fn new(nr_cars: u32, decider: M) -> Self {
        let cars = Cars::with_nr_car(nr_cars, decider.clone());
        RacingGame {
            nr_cars,
            cars,
            callbacks: None,
        }
    }

    pub fn add_callback(&mut self, callbacks: Box<dyn RacingGameCallback>) {
        self.callbacks = Some(callbacks);
    }

    pub fn race(&mut self) {
        self.cars.race();
        let l = self.cars.location();
        self.callbacks.as_ref().unwrap().on_race_single_track(&l);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::movement_decider::MovementDecider;

    #[derive(Copy, Clone)]
    struct TestFixture {
        on_race_sing_track_called: bool,
    }

    impl TestFixture {
        fn new() -> TestFixture {
            TestFixture {
                on_race_sing_track_called: false,
            }
        }
    }

    impl MovementDecider for TestFixture {
        fn decide(&self) -> bool {
            true
        }
    }

    impl RacingGameCallback for TestFixture {
        fn on_race_single_track(&self, _locations: &Vec<Location>) {
            for l in _locations {
                println!("{}", l.current());
            }
        }
    }
    static mut TEST_FIXTURE: TestFixture = TestFixture {
        on_race_sing_track_called: false,
    };

    #[test]
    fn when_new_then_created() {
        // when
        let _ = RacingGame::new(10, unsafe { TEST_FIXTURE });
    }

    #[test]
    fn when_rance_then_on_race_single_track_called() {
        let mut racing_game = RacingGame::new(10, unsafe { TEST_FIXTURE });

        unsafe {
            racing_game.add_callback(Box::new(TEST_FIXTURE));
        }
        racing_game.race();

        unsafe {
            assert_eq!(TEST_FIXTURE.on_race_sing_track_called, true);
        }
    }
}
