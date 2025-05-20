use std::fs;
use std::io;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Question {
    text: String,
    options: Vec<String>,
    correct: usize,
}

fn main() {
    // Читаем JSON файл
    let data = fs::read_to_string("questions.json")
        .expect("Не удалось прочитать файл questions.json");

    let questions: Vec<Question> = serde_json::from_str(&data)
        .expect("Не удалось распарсить JSON");

    let mut score = 0;

    for (i, q) in questions.iter().enumerate() {
        println!("\nВопрос {}: {}", i + 1, q.text);
        for (j, opt) in q.options.iter().enumerate() {
            println!("  {}. {}", j + 1, opt);
        }

        println!("Ваш ответ: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");
        let answer = input.trim().parse::<usize>().unwrap_or(0);

        if answer == q.correct {
            println!("✅ Правильно!");
            score += 1;
        } else {
            println!("❌ Неправильно! Правильный ответ: {}", q.correct);
        }
    }

    println!("\nВы ответили правильно на {}/{} вопросов.", score, questions.len());
}
