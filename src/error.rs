use fixed::types::I64F64;
#[derive(Debug)]
pub enum GeohashError {
    InvalidHashCharacter(char),
    InvalidCoordinateRange(I64F64, I64F64),
}
