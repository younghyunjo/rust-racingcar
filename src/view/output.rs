use crate::domain::winners::Winners;
use crate::RaceResult;

pub struct Output;

impl Output {
    pub fn print_title() {
        println!("실행 결과");
    }

    pub fn print_results(results: Vec<RaceResult>) {
        for r in results {
            Output::print_result(r);
        }
        println!();
    }

    pub fn print_winners(winners: Winners) {
        let names: Vec<String> = winners.names().iter().map(|n| n.name()).collect();
        print!("{}", names.join(","));
        println!("가 최종 우승했습니다.");
    }

    fn print_result(result: RaceResult) {
        print!("{} :", result.name().name());
        print!(
            "{}",
            std::iter::repeat('-')
                .take(result.position().position() as usize)
                .collect::<String>()
        );
        println!();
    }
}
