use std::cell::RefCell;

use crate::domain::cars::Cars;
use crate::domain::judge::Judge;
use crate::domain::racing_game_callbacks::RacingGameCallback;

pub struct RacingGame<'a> {
    judge: &'a (dyn Judge),
    callback: RefCell<Vec<&'a dyn RacingGameCallback>>,
    cars: Cars,
    count: u32,
}

impl<'a> RacingGame<'a> {
    pub fn new(nr_cars: u32, count: u32, judge: &'a dyn Judge) -> Self {
        RacingGame {
            cars: Cars::new(nr_cars),
            judge,
            callback: RefCell::new(vec![]),
            count,
        }
    }

    pub fn add_callback(&self, callback: &'a dyn RacingGameCallback) {
        self.callback.borrow_mut().push(callback);
    }

    pub fn race(&mut self) {
        for _ in 0..self.count {
            self.cars = self.cars.race(self.judge);
            for c in self.callback.borrow().iter() {
                c.on_raced(self.cars.positions());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::judge::Judge;
    use crate::domain::position::Position;
    use crate::domain::racing_game::RacingGame;
    use crate::domain::racing_game_callbacks::RacingGameCallback;
    use std::cell::RefCell;
    use std::ops::Add;

    struct Fixture {
        nr_on_race_called: RefCell<u32>,
        positions: RefCell<Vec<Position>>,
    }

    impl Fixture {
        fn new() -> Self {
            Fixture {
                nr_on_race_called: RefCell::new(0),
                positions: RefCell::new(vec![]),
            }
        }
    }

    impl Judge for Fixture {
        fn judge(&self) -> bool {
            true
        }
    }

    impl RacingGameCallback for Fixture {
        fn on_raced(&self, positions: Vec<Position>) {
            let nr_on_race_called = self.nr_on_race_called.take();
            self.nr_on_race_called.replace(nr_on_race_called + 1);
            self.positions.replace(positions);
        }
    }

    #[test]
    fn when_race_then_callback_called() {
        //given
        let f = Fixture::new();
        let mut r = RacingGame::new(3, 1, &f as &dyn Judge);
        r.add_callback(&f);

        //when
        r.race();

        //then
        assert_eq!(f.nr_on_race_called.take(), 1);
    }

    #[test]
    fn when_race_then_position_is_changed() {
        //given
        let f = Fixture::new();
        let mut r = RacingGame::new(3, 1, &f as &dyn Judge);
        r.add_callback(&f);

        //when
        r.race();

        //then
        for p in f.positions.take() {
            assert_eq!(p, Position::from(1));
        }
    }

    #[test]
    fn given_two_count_when_race_then_callback_called_twice() {
        //given
        let f = Fixture::new();
        let count = 2;
        let mut r = RacingGame::new(3, count, &f as &dyn Judge);
        r.add_callback(&f);

        //when
        r.race();

        //then
        assert_eq!(f.nr_on_race_called.take(), 2);
    }
}
