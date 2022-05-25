use crate::domain::car::Car;
use std::cell::RefCell;

use crate::domain::cars::Cars;
use crate::domain::judge::Judge;
use crate::domain::name::Name;
use crate::domain::racing_game_callbacks::RacingGameCallback;
use crate::domain::winners::Winners;
use crate::{Position, RaceResult};

pub struct RacingGame<'a> {
    judge: Option<&'a (dyn Judge)>,
    callback: RefCell<Vec<&'a dyn RacingGameCallback>>,
    cars: Cars,
    count: u32,
}

impl<'a> RacingGame<'a> {
    pub fn new(names: Vec<Name>, count: u32, judge: &'a dyn Judge) -> Self {
        RacingGame {
            cars: Cars::new(names),
            judge: Some(judge),
            callback: RefCell::new(vec![]),
            count,
        }
    }

    pub fn with_results(race_results: Vec<RaceResult>) -> Self {
        let mut cars: Vec<Car> = vec![];
        for r in race_results {
            cars.push(Car::with_position(&r.name(), r.position()));
        }

        RacingGame {
            cars: Cars::with_cars(cars),
            judge: None,
            callback: RefCell::new(vec![]),
            count: 0,
        }
    }

    pub fn winners(&self) -> Winners {
        let mut winners_name: Vec<Name> = vec![];
        let mut winner_position: Position = Position::from(0);
        for r in self.cars.results() {
            if r.position() > winner_position {
                winners_name.clear();
                winners_name.push(r.name());
            } else if r.position() == winner_position {
                winners_name.push(r.name());
            }
        }

        Winners::new(winners_name)
    }

    pub fn add_callback(&self, callback: &'a dyn RacingGameCallback) {
        self.callback.borrow_mut().push(callback);
    }

    pub fn race(&mut self) {
        for _ in 0..self.count {
            self.cars = self.cars.race(self.judge.unwrap());
            for c in self.callback.borrow().iter() {
                c.on_raced(self.cars.results());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::judge::Judge;
    use crate::domain::name::Name;
    use crate::domain::position::Position;
    use crate::domain::racing_game::RacingGame;
    use crate::domain::racing_game_callbacks::RacingGameCallback;
    use crate::RaceResult;
    use std::cell::RefCell;

    struct Fixture {
        nr_on_race_called: RefCell<u32>,
        positions: RefCell<Vec<Position>>,
        race_results: RefCell<Vec<RaceResult>>,
    }

    impl Fixture {
        fn new() -> Self {
            Fixture {
                nr_on_race_called: RefCell::new(0),
                positions: RefCell::new(vec![]),
                race_results: RefCell::new(vec![]),
            }
        }
    }

    impl Judge for Fixture {
        fn judge(&self) -> bool {
            true
        }
    }

    impl RacingGameCallback for Fixture {
        fn on_raced(&self, result: Vec<RaceResult>) {
            let nr_on_race_called = self.nr_on_race_called.take();
            self.nr_on_race_called.replace(nr_on_race_called + 1);
            self.race_results.replace(result);
        }
    }

    #[test]
    fn given_race_results_when_with_results_then_racing_game_created() {
        //given
        let race_results = vec![
            RaceResult::new(&Name::new("a").unwrap(), Position::from(1)),
            RaceResult::new(&Name::new("b").unwrap(), Position::from(2)),
            RaceResult::new(&Name::new("c").unwrap(), Position::from(3)),
        ];

        //when, then
        RacingGame::with_results(race_results);
    }

    #[test]
    fn given_race_results_when_winner() {
        //given
        let race_results = vec![
            RaceResult::new(&Name::new("a").unwrap(), Position::from(1)),
            RaceResult::new(&Name::new("b").unwrap(), Position::from(2)),
            RaceResult::new(&Name::new("c").unwrap(), Position::from(3)),
        ];
        let racing_game = RacingGame::with_results(race_results);

        //when
        let winners = racing_game.winners();

        //then
        assert_eq!(winners.names(), vec![Name::new("c").unwrap()]);
    }

    #[test]
    fn given_names_when_new_then_created() {
        let f = Fixture::new();
        let names = vec![Name::new("a").unwrap(), Name::new("b").unwrap()];

        let _ = RacingGame::new(names, 1, &f as &dyn Judge);
    }

    #[test]
    fn when_race_then_callback_called() {
        //given
        let f = Fixture::new();
        let names = vec![
            Name::new("a").unwrap(),
            Name::new("b").unwrap(),
            Name::new("c").unwrap(),
        ];
        let mut r = RacingGame::new(names, 1, &f as &dyn Judge);
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
        let names = vec![
            Name::new("a").unwrap(),
            Name::new("b").unwrap(),
            Name::new("c").unwrap(),
        ];
        let mut r = RacingGame::new(names, 1, &f as &dyn Judge);
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
        let names = vec![
            Name::new("a").unwrap(),
            Name::new("b").unwrap(),
            Name::new("c").unwrap(),
        ];
        let mut r = RacingGame::new(names, count, &f as &dyn Judge);
        r.add_callback(&f);

        //when
        r.race();

        //then
        assert_eq!(f.nr_on_race_called.take(), 2);
    }

    #[test]
    fn when_race_then_on_raced2_callback_called() {
        //given
        let f = Fixture::new();
        let names = vec![
            Name::new("a").unwrap(),
            Name::new("b").unwrap(),
            Name::new("c").unwrap(),
        ];
        let mut r = RacingGame::new(names, 1, &f as &dyn Judge);
        r.add_callback(&f);

        //when
        r.race();

        //then
        assert_eq!(f.race_results.take().len(), 3);
    }
}
