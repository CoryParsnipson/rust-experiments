pub type OperandList = Vec<Operand>;

#[derive(Debug)]
pub struct Operand {
    value: String,
}

impl Operand {
    pub fn new(value: &str) -> Operand {
        Operand { value: value.into() }
    }
}
