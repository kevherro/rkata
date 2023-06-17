use std::collections::VecDeque;

pub(crate) struct Pot {
    size: u32,
}

impl Pot {
    #[allow(unused)]
    pub fn new(small_blind: u32, big_blind: u32, ante: u32) -> Pot {
        Pot {
            size: small_blind + big_blind + ante,
        }
    }

    #[allow(unused)]
    pub fn size(&self) -> u32 {
        self.size
    }

    #[allow(unused)]
    pub fn update_size(&mut self, _actions: VecDeque<PlayerAction>) {
        self.size += 100
    }
}

pub type PlayerAction = (usize, Action);

pub enum Action {
    Open(u32),
    Call,
    Raise(u32),
    ReRaise(u32),
}
