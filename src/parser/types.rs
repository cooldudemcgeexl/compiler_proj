pub enum TypeMark {
    Integer,
    Float,
    String,
    Bool,
}

pub struct ArrayBound {
    pub number: Number,
}

pub struct Number {
    pub literal_string: String,
}

pub struct StringNode {
    pub literal_string: String,
}

pub struct Identifier {
    pub identifier_string: String,
}
