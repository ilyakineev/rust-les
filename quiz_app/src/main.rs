mod quiz;
mod io_utils;

use std::fs;

fn main() {

    const FILE_NAME: &str = "questions.json";

    println!("Текущая директория: {:?}", std::env::current_dir().unwrap());

    let data = fs::read_to_string(FILE_NAME)
        .expect("Не удалось прочитать файл questions.json");

    let questions: Vec<quiz::Question> = serde_json::from_str(&data)
        .expect("Не удалось распарсить JSON");

    let mut quiz = quiz::Quiz::new(questions);

    quiz.run();
}
