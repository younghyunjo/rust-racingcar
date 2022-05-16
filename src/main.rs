mod domain;
mod view;

use crate::domain::position::Position;
use crate::domain::racing_game::RacingGame;
use crate::domain::racing_game_callbacks::RacingGameCallback;
use crate::domain::random_judge::RandomJudge;
use crate::view::input::Input;
use crate::view::output::Output;
use std::cell::RefCell;

struct Callbacks {
    on_race_called: RefCell<bool>,
}

impl RacingGameCallback for Callbacks {
    fn on_raced(&self, positions: Vec<Position>) {
        if self.on_race_called.borrow().eq(&false) {
            self.on_race_called.replace(true);
            Output::print_title();
        }

        Output::print_positions(positions);
    }
}

fn main() -> std::io::Result<()> {
    let random_judge = RandomJudge::new();
    let callbakcs = Callbacks {
        on_race_called: RefCell::new(false),
    };

    let nr_cars = Input::number_of_car()?;
    let count = Input::count()?;

    let mut racing_game = RacingGame::new(nr_cars, count, &random_judge);
    racing_game.add_callback(&callbakcs);
    racing_game.race();

    Ok(())
}
