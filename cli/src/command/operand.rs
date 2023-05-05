pub type OperandList = Vec<Operand>;

#[derive(Debug)]
pub struct MissingOperandError(pub OperandList, pub usize);

impl std::fmt::Display for MissingOperandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "not enough operands provided. Received {:?}, expected {}",
            self.0.len(),
            self.1,
        )
    }
}

impl std::error::Error for MissingOperandError {}

#[derive(Clone, Debug)]
pub struct Operand {
    value: String,
}

impl Operand {
    pub fn new(value: &str) -> Operand {
        Operand { value: value.into() }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn value_as<T>(&self) -> Result<T, <T as std::str::FromStr>::Err>
        where T: std::str::FromStr + std::clone::Clone
    {
        self.value.clone().parse()
    }
}
