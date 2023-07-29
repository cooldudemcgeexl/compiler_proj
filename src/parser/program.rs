use super::declaratons::Declaration;

pub struct ProgramStruct {
    program_header: ProgramHeader,
    program_body: ProgramBody
}

pub struct ProgramHeader {
    header_identifier: String
}

pub struct ProgramBody {
    declarations: Vec<Declaration>,
    statements: (),
}