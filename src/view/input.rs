pub struct Input;
impl Input {
    pub fn number_of_car() -> std::io::Result<u32> {
        println!("자동차 대수는 몇 대 인가요?");

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        let nr_cars = buffer.trim().parse::<u32>().unwrap();

        Ok(nr_cars)
    }

    pub fn count() -> std::io::Result<u32> {
        println!("시도할 회수는 몇 회 인가요?");

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        let count = buffer.trim().parse::<u32>().unwrap();

        Ok(count)
    }
}
