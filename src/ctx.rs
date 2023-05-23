#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: usize,
}

impl Ctx {
    pub fn new(id: usize) -> Self {
        Self { user_id: id }
    }
    pub fn user_id(&self) -> usize {
        self.user_id
    }
}
