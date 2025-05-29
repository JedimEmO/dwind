use dwind_design_tokens::prelude::*;

#[test]
fn test_parse_example_tokens() {
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

    let result = parse_tokens(json);
    assert!(result.is_ok(), "Failed to parse tokens: {:?}", result.err());

    let token_file = result.unwrap();
    
    // Test finding tokens
    let token_a = token_file.find_token("test.a").unwrap();
    assert_eq!(token_a.token_type, TokenType::Dimension);
    assert!(!token_a.has_references());

    let token_b = token_file.find_token("test.b").unwrap();
    assert_eq!(token_b.token_type, TokenType::Dimension);
    assert!(token_b.has_references());
    
    let refs = token_b.get_references();
    assert_eq!(refs, vec!["a"]);

    // Test validation
    let validation_report = validate_token_file(&token_file).unwrap();
    if !validation_report.is_valid() {
        println!("Validation errors: {}", validation_report.summary());
        for error in &validation_report.errors {
            println!("Error: {}", error);
        }
        for missing in &validation_report.missing_references {
            println!("Missing reference: {}", missing);
        }
    }
    assert!(validation_report.is_valid());
}

#[test]
fn test_complex_expressions() {
    let json = r##"
    {
        "math": {
            "base": {
                "$value": "10",
                "$type": "number",
                "$description": ""
            },
            "double": {
                "$value": "{base} * 2",
                "$type": "number",
                "$description": ""
            },
            "complex": {
                "$value": "({base} + 5) * 3",
                "$type": "number",
                "$description": ""
            },
            "multi_ref": {
                "$value": "{base} + {double}",
                "$type": "number",
                "$description": ""
            }
        },
        "$themes": [],
        "$metadata": {
            "tokenSetOrder": ["math"],
            "activeThemes": [],
            "activeSets": ["math"]
        }
    }
    "##;

    let token_file = parse_tokens(json).unwrap();
    let validation_report = validate_token_file(&token_file).unwrap();
    assert!(validation_report.is_valid());

    let complex_token = token_file.find_token("math.complex").unwrap();
    assert!(complex_token.has_references());
    
    let multi_ref_token = token_file.find_token("math.multi_ref").unwrap();
    let refs = multi_ref_token.get_references();
    assert_eq!(refs.len(), 2);
    assert!(refs.contains(&"base".to_string()));
    assert!(refs.contains(&"double".to_string()));
}

#[test]
fn test_nested_groups() {
    let json = r##"
    {
        "colors": {
            "primary": {
                "100": {
                    "$value": "#f0f0f0",
                    "$type": "color",
                    "$description": ""
                },
                "500": {
                    "$value": "#808080",
                    "$type": "color",
                    "$description": ""
                },
                "900": {
                    "$value": "#101010",
                    "$type": "color",
                    "$description": ""
                }
            },
            "secondary": {
                "base": {
                    "$value": "rgb(100, 150, 200)",
                    "$type": "color",
                    "$description": ""
                }
            }
        },
        "$themes": [],
        "$metadata": {
            "tokenSetOrder": ["colors"],
            "activeThemes": [],
            "activeSets": ["colors"]
        }
    }
    "##;

    let token_file = parse_tokens(json).unwrap();
    let validation_report = validate_token_file(&token_file).unwrap();
    assert!(validation_report.is_valid());

    // Test deep token access
    let primary_100 = token_file.find_token("colors.primary.100").unwrap();
    assert_eq!(primary_100.token_type, TokenType::Color);

    let secondary_base = token_file.find_token("colors.secondary.base").unwrap();
    assert_eq!(secondary_base.token_type, TokenType::Color);

    // Test getting all tokens
    let all_tokens = token_file.get_all_tokens();
    assert_eq!(all_tokens.len(), 4);
}

#[test]
fn test_validation_errors() {
    let json = r##"
    {
        "invalid": {
            "bad_dimension": {
                "$value": "not-a-dimension",
                "$type": "dimension",
                "$description": ""
            },
            "missing_ref": {
                "$value": "{nonexistent} * 2",
                "$type": "dimension",
                "$description": ""
            },
            "bad_number": {
                "$value": "abc123",
                "$type": "number",
                "$description": ""
            }
        },
        "$themes": [],
        "$metadata": {
            "tokenSetOrder": ["invalid"],
            "activeThemes": [],
            "activeSets": ["invalid"]
        }
    }
    "##;

    let token_file = parse_tokens(json).unwrap();
    let validation_report = validate_token_file(&token_file).unwrap();
    
    assert!(!validation_report.is_valid());
    assert!(!validation_report.errors.is_empty());
    assert!(!validation_report.missing_references.is_empty());
}

#[test]
fn test_circular_reference_detection() {
    let json = r##"
    {
        "spacing": {
            "base": {
                "$value": "8px",
                "$type": "dimension",
                "$description": ""
            },
            "circular_a": {
                "$value": "{circular_b}",
                "$type": "dimension",
                "$description": ""
            },
            "circular_b": {
                "$value": "{circular_c}",
                "$type": "dimension",
                "$description": ""
            },
            "circular_c": {
                "$value": "{circular_a}",
                "$type": "dimension",
                "$description": ""
            },
            "valid_ref": {
                "$value": "{base} * 2",
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
    "##;

    let token_file = parse_tokens(json).unwrap();
    let validation_report = validate_token_file(&token_file).unwrap();
    
    // Should detect circular references but not affect valid tokens
    assert!(!validation_report.is_valid());
    assert!(!validation_report.circular_references.is_empty());
    
    // The circular reference should involve all three circular tokens
    let circular_ref = &validation_report.circular_references[0];
    assert!(circular_ref.contains("circular_a"));
    assert!(circular_ref.contains("circular_b"));
    assert!(circular_ref.contains("circular_c"));
    
    // Valid tokens should still be accessible
    let base_token = token_file.find_token("spacing.base").unwrap();
    assert!(!base_token.has_references());
    
    let valid_ref_token = token_file.find_token("spacing.valid_ref").unwrap();
    assert!(valid_ref_token.has_references());
    assert_eq!(valid_ref_token.get_references(), vec!["base"]);
}