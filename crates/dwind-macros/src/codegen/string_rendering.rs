pub fn class_name_to_struct_identifier(input: &String) -> String {
    input.to_uppercase().replace('-', "_")
}

pub fn class_name_to_raw_identifier(input: &String) -> String {
    format!(
        "{}_RAW",
        class_name_to_struct_identifier(&sanitize_class_prefix(input))
    )
}

pub fn sanitize_class_prefix(input: &String) -> String {
    input.replace('#', "HB").replace('%', "PP")
}
