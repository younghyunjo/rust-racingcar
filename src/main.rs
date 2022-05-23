mod domain;
mod view;

use crate::domain::position::Position;
use crate::domain::racing_game::RacingGame;
use crate::domain::racing_game_callbacks::RacingGameCallback;
use crate::domain::random_judge::RandomJudge;

use crate::domain::name::Name;
use crate::domain::race_result::RaceResult;
use crate::view::input::Input;
use crate::view::output::Output;
use std::cell::RefCell;

struct Callbacks {
    on_race_called: RefCell<bool>,
}

impl RacingGameCallback for Callbacks {
    fn on_raced(&self, positions: Vec<Position>) {
        // if self.on_race_called.borrow().eq(&false) {
        //     self.on_race_called.replace(true);
        //     Output::print_title();
        // }
        //
        // Output::print_positions(positions);
    }

    fn on_raced2(&self, result: Vec<RaceResult>) {
        Output::print_results(result);
    }
}

fn main() -> std::io::Result<()> {
    let random_judge = RandomJudge::new();
    let callbakcs = Callbacks {
        on_race_called: RefCell::new(false),
    };

    let count = Input::count()?;

    let names = vec![
        Name::new("a").unwrap(),
        Name::new("b").unwrap(),
        Name::new("c").unwrap(),
    ];
    let mut racing_game = RacingGame::new(names, count, &random_judge);
    racing_game.add_callback(&callbakcs);
    racing_game.race();

    Ok(())
}
