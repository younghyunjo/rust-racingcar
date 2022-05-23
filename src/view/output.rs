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
