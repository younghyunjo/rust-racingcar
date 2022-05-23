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
    fn on_raced(&self, result: Vec<RaceResult>) {
        let on_race_called = self.on_race_called.replace(true);
        if on_race_called == false {
            Output::print_title();
        }
        Output::print_results(result);
    }
}

fn main() -> std::io::Result<()> {
    let random_judge = RandomJudge::new();
    let callbakcs = Callbacks {
        on_race_called: RefCell::new(false),
    };

    let names = Input::car_names()?
        .into_iter()
        .map(|n| Name::new(n.as_str()).unwrap())
        .collect();

    let count = Input::count()?;

    let mut racing_game = RacingGame::new(names, count, &random_judge);
    racing_game.add_callback(&callbakcs);
    racing_game.race();

    Ok(())
}
