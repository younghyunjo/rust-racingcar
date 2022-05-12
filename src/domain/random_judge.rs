use crate::domain::judge::Judge;
use rand::Rng;

pub struct RandomJudge;

impl RandomJudge {
    pub fn new() -> Self {
        RandomJudge
    }
}
impl Judge for RandomJudge {
    fn judge(&self) -> bool {
        let mut rnd = rand::thread_rng();
        rnd.gen_range(0..10) > 4
    }
}
