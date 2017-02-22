pub enum Type {
    COMPUTER,
    USER
}

pub struct Player {
    pub name: String,
    pub score: usize,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
            score: 0,
        }
    }
    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}