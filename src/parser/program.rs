use super::{declaratons::Declaration, statement::Statement};

pub struct ProgramStruct {
    program_header: ProgramHeader,
    program_body: ProgramBody,
}

pub struct ProgramHeader {
    header_identifier: String,
}

pub struct ProgramBody {
    declarations: Vec<Declaration>,
    statements: Vec<Statement>,
}
