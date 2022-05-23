use crate::Name;

pub struct Input;
impl Input {
    pub fn car_names() -> std::io::Result<Vec<String>> {
        println!("경주할 자동차 이름을 입력하세요(이름은 쉼표(,)를 기준으로 구분).");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer
            .trim()
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
    }

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
