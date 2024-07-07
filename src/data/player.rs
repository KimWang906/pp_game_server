pub struct Player {
    id: u32,
    name: String,
    is_ready: bool,
    score: u32,
}

impl Player {
    pub fn new(id: u32, name: String, is_ready: bool, score: u32) -> Self {
        Self {
            id,
            name,
            is_ready,
            score,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready
    }

    pub fn set_ready(&mut self, is_ready: bool) {
        self.is_ready = is_ready;
    }
}
