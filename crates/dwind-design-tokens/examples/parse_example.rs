use dwind_design_tokens::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The example JSON from the task
    let json = r##"
    {
      "test": {
        "a": {
          "$value": "5px",
          "$type": "dimension",
          "$description": ""
        },
        "b": {
          "$value": "{a}*3",
          "$type": "dimension",
          "$description": ""
        },
        "primary": {
          "$value": "rgb(193, 51, 51)",
          "$type": "color",
          "$description": ""
        },
        "medium": {
          "$value": "12px",
          "$type": "borderRadius",
          "$description": ""
        },
        "rounded-md": {
          "$value": "{medium}+12",
          "$type": "borderRadius",
          "$description": ""
        }
      },
      "$themes": [],
      "$metadata": {
        "tokenSetOrder": [
          "test"
        ],
        "activeThemes": [],
        "activeSets": [
          "test"
        ]
      }
    }
    "##;

    println!("Parsing design tokens...");

    // Parse the tokens
    let token_file = parse_tokens(json)?;
    println!(
        "✅ Successfully parsed {} token sets",
        token_file.sets.len()
    );

    // Validate the tokens
    let validation_report = validate_token_file(&token_file)?;
    println!("✅ Validation completed: {}", validation_report.summary());

    if !validation_report.is_valid() {
        println!("❌ Validation failed!");
        for error in &validation_report.errors {
            println!("  Error: {}", error);
        }
        for missing in &validation_report.missing_references {
            println!("  Missing reference: {}", missing);
        }
        return Ok(());
    }

    // Display all tokens
    println!("\n📋 All tokens:");
    let all_tokens = token_file.get_all_tokens();
    for (path, token) in &all_tokens {
        println!("  {} ({:?}): {:?}", path, token.token_type, token.value);
        if token.has_references() {
            let refs = token.get_references();
            println!("    References: {:?}", refs);
        }
    }

    // Test specific token access
    println!("\n🔍 Testing token access:");

    if let Some(token_a) = token_file.find_token("test.a") {
        println!("  test.a: {:?}", token_a.value);
    }

    if let Some(token_b) = token_file.find_token("test.b") {
        println!("  test.b: {:?}", token_b.value);
        if let TokenValue::Expression(expr) = &token_b.value {
            println!("    Expression AST: {:#?}", expr);
        }
    }

    if let Some(rounded_md) = token_file.find_token("test.rounded-md") {
        println!("  test.rounded-md: {:?}", rounded_md.value);
        if let TokenValue::Expression(expr) = &rounded_md.value {
            println!("    Expression AST: {:#?}", expr);
        }
    }

    // Show all references
    println!("\n🔗 Token references:");
    let all_references = token_file.get_all_references();
    for (token_path, refs) in &all_references {
        println!("  {} references: {:?}", token_path, refs);
    }

    println!("\n✅ Design token parsing completed successfully!");
    Ok(())
}
