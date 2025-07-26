pub fn class_name_to_struct_identifier(input: &String) -> String {
    input
        .to_uppercase()
        .replace('-', "_")
        .replace('.', "_")
        .replace('/', "_OF_")
}

pub fn class_name_to_raw_identifier(input: &String) -> String {
    format!(
        "{}_RAW",
        class_name_to_struct_identifier(&sanitize_class_prefix(
            &input.replace("/", "_").replace(".", "_")
        ))
    )
}

pub fn sanitize_class_prefix(input: &String) -> String {
    input.replace('#', "HB").replace('%', "PP")
}
