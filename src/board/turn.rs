#[derive(Clone, Copy)]
pub enum Turn {
    WhiteMove,
    WhiteBreak,
    BlackMove,
    BlackBreak,
}

impl Turn {
    pub fn start() -> Turn {
        Turn::WhiteMove
    }

    pub fn next(self) -> Turn {
        match self {
            Turn::WhiteMove => Turn::WhiteBreak,
            Turn::WhiteBreak => Turn::BlackMove,
            Turn::BlackMove => Turn::BlackBreak,
            Turn::BlackBreak => Turn::WhiteMove,
        }
    }
}
