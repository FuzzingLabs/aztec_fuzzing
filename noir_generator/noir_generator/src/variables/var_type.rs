use crate::random;

#[derive(Debug, Clone, PartialEq)] // Ajout de PartialEq ici
pub(crate) enum VarType {
    Field,
    U8,
    U16,
    U32,
    U64,
    U127,
    I8,
    I16,
    I32,
    I64,
    I127,
    Bool,
    Str,
    // Array(Box<VarType>),
    // Slice(Box<VarType>),
    // Vec(Box<VarType>),
    // Tup(Box<VarType>),
}

impl Eq for VarType {}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Utilisez std::fmt::Display pour formater l'énumération en chaîne de caractères
        write!(f, "{:?}", self)
    }
}

pub fn random_type() -> VarType {
    let types: Vec<VarType> = vec![
        VarType::Field,
        VarType::U8,
        VarType::U16,
        VarType::U32,
        VarType::U64,
        VarType::U127,
        VarType::I8,
        VarType::I16,
        VarType::I32,
        VarType::I64,
        VarType::I127,
        VarType::Bool,
        VarType::Str,
        // VarType::Array(Box::new(random_type())),
        // VarType::Slice(Box::new(random_type())),
        // VarType::Vec(Box::new(random_type())),
        // VarType::Tup(Box::new(random_type())),
    ];


    random::choose_random_item_from_vec(&types)
}

pub fn basic_types() -> Vec<VarType> {
    vec![
        VarType::Field,
        VarType::U8,
        VarType::U16,
        VarType::U32,
        VarType::U64,
        VarType::U127,
        VarType::I8,
        VarType::I16,
        VarType::I32,
        VarType::I64,
        VarType::I127,
        VarType::Bool,
        VarType::Str,
    ]
}

pub fn supported_operations_by_type(var_type: VarType) -> Vec<&'static str> {
    match var_type {
        VarType::Field | VarType::U8 | VarType::U16 | VarType::U32 | VarType::U64 | VarType::U127 | VarType::I8 | VarType::I16 | VarType::I32 | VarType::I64 | VarType::I127 
            => vec!["+","-","*","/","^","&","|","<<",">>"],
        VarType::Bool => vec!["==","!=","|","&"],
        VarType::Str => vec!["+"],
        _ => vec![], // Handle unknown types
    }
}

pub fn supported_operations_for_assertion(var_type: VarType) -> Vec<&'static str> {
    match var_type {
        VarType::Field | VarType::U8 | VarType::U16 | VarType::U32 | VarType::U64 | VarType::U127 | VarType::I8 | VarType::I16 | VarType::I32 | VarType::I64 | VarType::I127
            => vec!["==", "!=", "<", ">", "<=", ">="],
        VarType::Bool => vec!["==", "!=", "|", "&"],
        VarType::Str => vec!["==", "!="],
        _ => vec![], // Handle unknown types
    }
}