use std::io;

pub fn read_user_input(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");

        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Пожалуйста, введите число."),
        }
    }
}
