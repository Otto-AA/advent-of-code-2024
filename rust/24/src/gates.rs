#[derive(Clone, PartialEq, Debug)]
pub enum Gate {
    And,
    Xor,
    Or,
}

impl Gate {
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "AND" => Some(Gate::And),
            "XOR" => Some(Gate::Xor),
            "OR" => Some(Gate::Or),
            _ => None,
        }
    }

    pub fn calculate(&self, left: bool, right: bool) -> bool {
        match self {
            Gate::And => left && right,
            Gate::Xor => left ^ right,
            Gate::Or => left || right,
        }
    }
}
