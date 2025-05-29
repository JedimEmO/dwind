//! Demonstration of the enhanced circular reference detection system
//! 
//! This example shows how the validation system now detects various types
//! of circular references in design token files.

use dwind_design_tokens::prelude::*;

fn main() {
    println!("=== Enhanced Circular Reference Detection Demo ===\n");

    // Example 1: Direct self-reference
    println!("1. Testing direct self-reference:");
    let self_ref_json = r#"
    {
        "test": {
            "self_ref": {
                "$value": "{self_ref}",
                "$type": "dimension",
                "$description": "This token references itself"
            }
        },
        "$themes": [],
        "$metadata": {
            "tokenSetOrder": ["test"],
            "activeThemes": [],
            "activeSets": ["test"]
        }
    }
    "#;

    match parse_tokens(self_ref_json) {
        Ok(file) => {
            match validate_token_file(&file) {
                Ok(report) => {
                    if !report.is_valid() {
                        println!("✓ Detected circular reference:");
                        for circular_ref in &report.circular_references {
                            println!("  - {}", circular_ref);
                        }
                    }
                }
                Err(e) => println!("✗ Validation error: {}", e),
            }
        }
        Err(e) => println!("✗ Parse error: {}", e),
    }

    println!();

    // Example 2: Complex circular chain
    println!("2. Testing complex circular chain (A → B → C → A):");
    let chain_json = r#"
    {
        "spacing": {
            "a": {
                "$value": "{b}",
                "$type": "dimension",
                "$description": ""
            },
            "b": {
                "$value": "{c}",
                "$type": "dimension",
                "$description": ""
            },
            "c": {
                "$value": "{a}",
                "$type": "dimension",
                "$description": ""
            }
        },
        "$themes": [],
        "$metadata": {
            "tokenSetOrder": ["spacing"],
            "activeThemes": [],
            "activeSets": ["spacing"]
        }
    }
    "#;

    match parse_tokens(chain_json) {
        Ok(file) => {
            match validate_token_file(&file) {
                Ok(report) => {
                    if !report.is_valid() {
                        println!("✓ Detected circular reference chain:");
                        for circular_ref in &report.circular_references {
                            println!("  - {}", circular_ref);
                        }
                    }
                }
                Err(e) => println!("✗ Validation error: {}", e),
            }
        }
        Err(e) => println!("✗ Parse error: {}", e),
    }

    println!();

    // Example 3: Valid references (should pass)
    println!("3. Testing valid references (should pass):");
    let valid_json = r#"
    {
        "spacing": {
            "base": {
                "$value": "8px",
                "$type": "dimension",
                "$description": ""
            },
            "small": {
                "$value": "{base}",
                "$type": "dimension",
                "$description": ""
            },
            "medium": {
                "$value": "{base} * 2",
                "$type": "dimension",
                "$description": ""
            },
            "large": {
                "$value": "{medium} + {small}",
                "$type": "dimension",
                "$description": ""
            }
        },
        "$themes": [],
        "$metadata": {
            "tokenSetOrder": ["spacing"],
            "activeThemes": [],
            "activeSets": ["spacing"]
        }
    }
    "#;

    match parse_tokens(valid_json) {
        Ok(file) => {
            match validate_token_file(&file) {
                Ok(report) => {
                    if report.is_valid() {
                        println!("✓ All references are valid - no circular dependencies detected");
                        println!("  Summary: {}", report.summary());
                    } else {
                        println!("✗ Unexpected validation issues:");
                        for error in &report.errors {
                            println!("  - Error: {}", error);
                        }
                        for circular_ref in &report.circular_references {
                            println!("  - Circular: {}", circular_ref);
                        }
                    }
                }
                Err(e) => println!("✗ Validation error: {}", e),
            }
        }
        Err(e) => println!("✗ Parse error: {}", e),
    }

    println!("\n=== Demo Complete ===");
    println!("The enhanced validation system now detects:");
    println!("• Direct self-references (A → A)");
    println!("• Simple circular references (A → B → A)");
    println!("• Complex circular chains (A → B → C → D → A)");
    println!("• Circular references within mathematical expressions");
    println!("• Mixed scenarios with both valid and circular references");
}