#[derive(Debug, Clone, PartialEq)]
pub enum BasicOperations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FieldTypes {
    Field,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    USize,
    ISize,
    F32,
    F64,
    Bool,
    Char,
    Str,
}

#[derive(Debug)]
pub enum FieldTransformations<'a> {
    to_le_bits(&'a str),
    to_be_bits(&'a str),
    to_le_bytes(&'a str),
    to_be_bytes(&'a str),
    to_le_radix(&'a str),
    to_be_radix(&'a str),
    pow_32(&'a str),
    sgn0(&'a str),
}

#[derive(Debug)]
pub struct PossibleOperations<'a> {
    pub field_type: FieldTypes,
    pub operations: Vec<BasicOperations>,
    pub transformations: Vec<FieldTransformations<'a>>,
}

pub struct OperationsManager<'a> {
    pub possible_operations: Vec<PossibleOperations<'a>>,
}

impl OperationsManager<'_> {
    pub fn new() -> Self {
        OperationsManager {
            possible_operations: Vec::new(),
        }
    }

    pub fn get_possible_operations_for_type(&self, field_type: FieldTypes) -> Option<&PossibleOperations> {
        self.possible_operations.iter().find(|ops| ops.field_type == field_type)
    }

    pub fn display_possible_operations_for_type(&self, field_type: &FieldTypes) {
        if let Some(ops) = self.get_possible_operations_for_type(*field_type) {
            println!("Operations for {:?} type: {:?}", field_type, ops);
        } else {
            println!("No operations found for {:?} type.", field_type);
        }
    }

    pub fn generate_basic_operations() -> Vec<BasicOperations> {
        vec![
            BasicOperations::Addition,
            BasicOperations::Subtraction,
            BasicOperations::Multiplication,
            BasicOperations::Division,
        ]
    }

    pub fn generate_possible_operations() -> Vec<PossibleOperations<'static>> {
        let basic_operations = OperationsManager::generate_basic_operations();

        vec![
            PossibleOperations {
                field_type: FieldTypes::Field,
                operations: basic_operations.clone(),
                transformations: vec![
                    FieldTransformations::to_le_bits("<N>(_x : Field, _bit_size: u32) -> [u1; N]"),
                    FieldTransformations::to_be_bits("<N>(_x : Field, _bit_size: u32) -> [u1; N]"),
                    FieldTransformations::to_le_bytes("(_x : Field, byte_size: u32) -> [u8]"),
                    FieldTransformations::to_be_bytes("(_x : Field, byte_size: u32) -> [u8]"),
                    FieldTransformations::to_le_radix("(_x : Field, _radix: u32, _result_len: u32) -> [u8]"),
                    FieldTransformations::to_be_radix("(_x : Field, _radix: u32, _result_len: u32) -> [u8]"),
                    FieldTransformations::pow_32("(self, exponent: Field) -> Field"),
                    FieldTransformations::sgn0("(self) -> u1"),
                ],
            },
            PossibleOperations { field_type: FieldTypes::U8, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::U16, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::U32, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::U64, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::U128, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::I8, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::I16, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::I32, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::I64, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::I128, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::USize, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::ISize, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::F32, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::F64, operations: basic_operations.clone(), transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::Bool, operations: vec![], transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::Char, operations: vec![], transformations: vec![] },
            PossibleOperations { field_type: FieldTypes::Str, operations: vec![], transformations: vec![] },
        ]
    }
}
