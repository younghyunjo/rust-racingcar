use std::cell::RefCell;

use crate::domain::cars::Cars;
use crate::domain::judge::Judge;
use crate::domain::racing_game_callbacks::RacingGameCallback;

struct RacingGame<'a> {
    judge: &'a (dyn Judge),
    callback: RefCell<Vec<&'a dyn RacingGameCallback>>,
    cars: Cars,
}

impl<'a> RacingGame<'a> {
    fn new(nr_cars: u32, judge: &'a dyn Judge) -> Self {
        RacingGame {
            cars: Cars::new(nr_cars),
            judge,
            callback: RefCell::new(vec![]),
        }
    }

    fn add_callback(&self, callback: &'a dyn RacingGameCallback) {
        self.callback.borrow_mut().push(callback);
    }

    fn race(&mut self) {
        self.cars = self.cars.race(self.judge);
        for c in self.callback.borrow().iter() {
            c.on_raced(self.cars.positions());
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

    struct Fixture {
        on_race_called: RefCell<bool>,
        positions: RefCell<Vec<Position>>,
    }

    impl Fixture {
        fn new() -> Self {
            Fixture {
                on_race_called: RefCell::new(false),
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
            self.on_race_called.replace(true);
            self.positions.replace(positions);
        }
    }

    #[test]
    fn when_race_then_callback_called() {
        //given
        let f = Fixture::new();
        let mut r = RacingGame::new(3, &f as &dyn Judge);
        r.add_callback(&f);

        //when
        r.race();

        //then
        assert_eq!(f.on_race_called.take(), true);
    }

    #[test]
    fn when_race_then_position_is_changed() {
        //given
        let f = Fixture::new();
        let mut r = RacingGame::new(3, &f as &dyn Judge);
        r.add_callback(&f);

        //when
        r.race();

        //then
        for p in f.positions.take() {
            assert_eq!(p, Position::from(1));
        }
    }
}
