use crate::agent::{Agent, Ai, Human};

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlayerType {
    Ai,
    #[default]
    Human,
}

impl PlayerType {
    pub fn is_human(&self) -> bool {
        *self == PlayerType::Human
    }

    pub fn is_ai(&self) -> bool {
        *self == PlayerType::Ai
    }

    pub fn into_agent(self) -> Box<dyn Agent> {
        match self {
            PlayerType::Ai => Box::new(Ai()),
            PlayerType::Human => Box::new(Human()),
        }
    }
}
