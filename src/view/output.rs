use crate::domain::position::Position;

pub struct Output;

impl Output {
    pub fn print_title() {
        println!("실행 결과");
    }

    pub fn print_positions(positions: Vec<Position>) {
        let marks = Output::to_marks(positions);
        Output::print_marks(marks);
    }

    fn print_marks(marks: Vec<String>) {
        for mark in marks {
            println!("{}", mark);
        }
        println!();
    }

    fn to_marks(positions: Vec<Position>) -> Vec<String> {
        positions
            .iter()
            .map(|p| p.position())
            .map(|n| std::iter::repeat('-').take(n as usize).collect::<String>())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::position::Position;
    use crate::view::output::Output;

    #[test]
    fn test_print_position() {
        Output::print_title();
        let positions = vec![Position::from(1), Position::from(3)];
        Output::print_positions(positions);
    }
}
