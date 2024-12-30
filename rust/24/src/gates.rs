#[derive(Clone, PartialEq, Debug)]
pub enum Gate {
    AND,
    XOR,
    OR,
}

impl Gate {
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "AND" => Some(Gate::AND),
            "XOR" => Some(Gate::XOR),
            "OR" => Some(Gate::OR),
            _ => None,
        }
    }

    pub fn calculate(&self, left: bool, right: bool) -> bool {
        match self {
            Gate::AND => left && right,
            Gate::XOR => left ^ right,
            Gate::OR => left || right,
        }
    }
}
