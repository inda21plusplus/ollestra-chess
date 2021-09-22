#[derive(Debug)]
pub(crate) struct Player {
    name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name }
    }
}
