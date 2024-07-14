include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        let rust_file = dominator_css_bindgen::css::generate_rust_bindings_from_file("resources/simple.css")
            .expect("failed to generate rust bindings");

        println!("{rust_file}");

        // just check that it exists
        //let multiline = &super::WITHPSEUDO_HOVER_ACTIVE;
    }
}
