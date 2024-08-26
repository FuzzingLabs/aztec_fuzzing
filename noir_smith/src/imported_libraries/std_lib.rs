use crate::{
    functions::{function::Function, list_functions::ListFunctions},
    variables::{
        bloc_data::BlocData, list_structs::ListStructs, struct_type::StructType, var_type::VarType,
        variable::Variable,
    },
};

pub fn import_statement() -> String {
    "use dep::std;\n".to_string()
}

/// Add all structures and functions present in the standard library to the generator's lists.
pub fn add_structures_and_functions(
    list_structs: &mut ListStructs,
    list_functions: &mut ListFunctions,
) {
    let pedersen_point = StructType::new(
        vec![
            (VarType::Field, "x".to_string()),
            (VarType::Field, "y".to_string()),
        ],
        "std::hash::PedersenPoint".to_string(),
    );
    list_structs.add_struct(pedersen_point.clone());

    let big_int = StructType::new(
        vec![
            (VarType::UInt(32), "pointer".to_string()),
            (VarType::UInt(32), "modulus".to_string()),
        ],
        "std::bigint::BigInt".to_string(),
    );
    list_structs.add_struct(big_int.clone());

    let mut bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "input".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), usize::max_value()),
    ));
    bloc_data.add_variable(Variable::new(
        "inv".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), 16),
    ));
    bloc_data.add_variable(Variable::new(
        "key".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), 16),
    ));
    list_functions.add_function(Function::new(
        "std::aes128::aes128_encrypt".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Slice(Box::new(VarType::UInt(8)), 1)),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "input".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::sha256::sha256_var".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Array(Box::new(VarType::UInt(8)), 32)),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "input".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::hash::blake2s".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Array(Box::new(VarType::UInt(8)), 32)),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "input".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::hash::blake3".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Array(Box::new(VarType::UInt(8)), 32)),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "input".to_string(),
        false,
        &VarType::Array(Box::new(VarType::Field), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::hash::pedersen_hash".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Field),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "input".to_string(),
        false,
        &VarType::Array(Box::new(VarType::Field), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::hash::pedersen_commitment".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Strct(pedersen_point.clone())),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "input".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), usize::max_value()),
    ));
    bloc_data.add_variable(Variable::new(
        "message_size".to_string(),
        false,
        &VarType::UInt(32),
    ));
    list_functions.add_function(Function::new(
        "std::hash::keccak256".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Array(Box::new(VarType::UInt(8)), 32)),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "input".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), 1),
    ));
    list_functions.add_function(Function::new(
        "std::hash::poseidon::bn254::hash_2".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Field),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "array".to_string(),
        false,
        &VarType::Array(Box::new(VarType::Field), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::hash::mimc::mimc_bn254".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Field),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "points".to_string(),
        false,
        &VarType::Array(Box::new(VarType::Field), usize::max_value()),
    ));
    bloc_data.add_variable(Variable::new(
        "scalars".to_string(),
        false,
        &VarType::Array(Box::new(VarType::Field), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::embedded_curve_ops::multi_scalar_mul".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Array(Box::new(VarType::Field), 2)),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "scalar_low".to_string(),
        false,
        &VarType::Field,
    ));
    bloc_data.add_variable(Variable::new(
        "scalar_high".to_string(),
        false,
        &VarType::Field,
    ));
    list_functions.add_function(Function::new(
        "std::embedded_curve_ops::fixed_base_scalar_mul".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Array(Box::new(VarType::Field), 2)),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "public_key_x".to_string(),
        false,
        &VarType::Field,
    ));
    bloc_data.add_variable(Variable::new(
        "public_key_y".to_string(),
        false,
        &VarType::Field,
    ));
    bloc_data.add_variable(Variable::new(
        "signature".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), 64),
    ));
    bloc_data.add_variable(Variable::new(
        "message".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::schnorr::verify_signature".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Bool),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "public_key_x".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), 32),
    ));
    bloc_data.add_variable(Variable::new(
        "public_key_y".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), 32),
    ));
    bloc_data.add_variable(Variable::new(
        "signature".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), 64),
    ));
    bloc_data.add_variable(Variable::new(
        "message_hash".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), usize::max_value()),
    ));
    list_functions.add_function(Function::new(
        "std::ecdsa_secp256k1::verify_signature".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Bool),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "public_key_x".to_string(),
        false,
        &VarType::Field,
    ));
    bloc_data.add_variable(Variable::new(
        "public_key_y".to_string(),
        false,
        &VarType::Field,
    ));
    bloc_data.add_variable(Variable::new(
        "signature_s".to_string(),
        false,
        &VarType::Field,
    ));
    bloc_data.add_variable(Variable::new(
        "signature_r8_x".to_string(),
        false,
        &VarType::Field,
    ));
    bloc_data.add_variable(Variable::new(
        "signature_r8_y".to_string(),
        false,
        &VarType::Field,
    ));
    bloc_data.add_variable(Variable::new("message".to_string(), false, &VarType::Field));
    list_functions.add_function(Function::new(
        "std::eddsa::eddsa_poseidon_verify".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Bool),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new("secret".to_string(), false, &VarType::Field));
    list_functions.add_function(Function::new(
        "std::eddsa::eddsa_to_pub".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Tup(vec![
            Box::new(VarType::Field),
            Box::new(VarType::Field),
        ])),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new(
        "array".to_string(),
        false,
        &VarType::Array(Box::new(VarType::UInt(8)), 32),
    ));
    list_functions.add_function(Function::new(
        "std::bigint::Secpk1Fq::from_le_bytes_32".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Strct(big_int)),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new("x".to_string(), false, &VarType::Field));
    list_functions.add_function(Function::new(
        "std::field::bn254::decompose".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Tup(vec![
            Box::new(VarType::Field),
            Box::new(VarType::Field),
        ])),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new("a".to_string(), false, &VarType::Field));
    bloc_data.add_variable(Variable::new("b".to_string(), false, &VarType::Field));
    list_functions.add_function(Function::new(
        "std::field::bn254::assert_gt".to_string(),
        true,
        bloc_data.clone(),
        None,
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new("a".to_string(), false, &VarType::Field));
    bloc_data.add_variable(Variable::new("b".to_string(), false, &VarType::Field));
    list_functions.add_function(Function::new(
        "std::field::bn254::assert_lt".to_string(),
        true,
        bloc_data.clone(),
        None,
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new("a".to_string(), false, &VarType::Field));
    bloc_data.add_variable(Variable::new("b".to_string(), false, &VarType::Field));
    list_functions.add_function(Function::new(
        "std::field::bn254::gt".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Bool),
    ));

    bloc_data = BlocData::new();
    bloc_data.add_variable(Variable::new("a".to_string(), false, &VarType::Field));
    bloc_data.add_variable(Variable::new("b".to_string(), false, &VarType::Field));
    list_functions.add_function(Function::new(
        "std::field::bn254::lt".to_string(),
        true,
        bloc_data.clone(),
        Some(VarType::Bool),
    ));
}
