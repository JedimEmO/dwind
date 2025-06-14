include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub mod tokens {
    include!(concat!(env!("OUT_DIR"), "/design_tokens_generated.rs"));
}

#[cfg(test)]
mod tests {
    use crate::tokens::TEXT_TEST_PRIMARY_RAW;

    #[test]
    fn sanity_check() {
        let rust_file =
            dominator_css_bindgen::css::generate_rust_bindings_from_file("resources/simple.css")
                .expect("failed to generate rust bindings");

        println!("{rust_file}");

        println!("{}",TEXT_TEST_PRIMARY_RAW);
        // just check that it exists
        //let multiline = &super::WITHPSEUDO_HOVER_ACTIVE;
    }
}
