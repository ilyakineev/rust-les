use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Question {
    pub text: String,
    pub options: Vec<String>,
    pub correct: usize,
}

pub struct Quiz {
    pub questions: Vec<Question>,
    pub score: usize,
}

impl Quiz {
    pub fn new(questions: Vec<Question>) -> Self {
        Quiz { questions, score: 0 }
    }

    pub fn run(&mut self) {
        for (i, q) in self.questions.iter().enumerate() {
            println!("\nВопрос {}: {}", i + 1, q.text);
            for (j, opt) in q.options.iter().enumerate() {
                println!("  {}. {}", j + 1, opt);
            }

            let answer = super::io_utils::read_user_input("Ваш ответ: ");

            if answer == q.correct {
                println!("✅ Правильно!");
                self.score += 1;
            } else {
                println!("❌ Неправильно! Правильный ответ: {}", q.correct);
            }
        }

        println!("\nВы ответили правильно на {}/{} вопросов.", self.score, self.questions.len());
    }
}
