use super::declaratons::Declaration; 
use super::types::TypeMark;


pub struct ProcedureHeader {
    pub identifier: String,
    pub type_mark: TypeMark,
    pub param_list: Option<ParamList>
}

pub struct ProcedureBody {
    pub declarations: Vec<Declaration>,
    pub statements: ()

}

pub struct ParamList { 
    
}

