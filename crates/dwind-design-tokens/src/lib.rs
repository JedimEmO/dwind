//! # DWIND Design Tokens
//!
//! A library for parsing and working with design token files in the DWIND ecosystem.
//! This crate provides data structures and parsing capabilities for design tokens
//! following the Design Tokens Community Group specification.
//!
//! ## Features
//!
//! - Parse design token JSON files into structured Rust types
//! - Support for nested token groups and hierarchical organization
//! - Expression parsing with nom for token references and arithmetic
//! - Comprehensive validation with detailed error reporting
//! - Support for multiple token types: dimension, number, borderRadius, color
//! - Robust error handling with thiserror
//!
//! ## Usage
//!
//! ```rust
//! use dwind_design_tokens::{parse_tokens, validate_token_file};
//!
//! let json = r#"
//! {
//!   "spacing": {
//!     "small": {
//!       "$value": "8px",
//!       "$type": "dimension",
//!       "$description": "Small spacing unit"
//!     },
//!     "medium": {
//!       "$value": "{small} * 2",
//!       "$type": "dimension",
//!       "$description": "Medium spacing unit"
//!     }
//!   },
//!   "$themes": [],
//!   "$metadata": {
//!     "tokenSetOrder": ["spacing"],
//!     "activeThemes": [],
//!     "activeSets": ["spacing"]
//!   }
//! }
//! "#;
//!
//! // Parse the token file
//! let token_file = parse_tokens(json).unwrap();
//!
//! // Validate the tokens
//! let validation_report = validate_token_file(&token_file).unwrap();
//! assert!(validation_report.is_valid());
//!
//! // Access tokens
//! let small_token = token_file.find_token("spacing.small").unwrap();
//! println!("Small spacing: {:?}", small_token.value);
//! ```

pub mod error;
pub mod expressions;
pub mod parser;
pub mod types;
pub mod validation;

// Re-export commonly used types and functions
pub use error::{TokenError, TokenResult};
pub use expressions::{BinaryOperator, Expr};
pub use parser::{parse_tokens, parse_tokens_from_file};
pub use types::{
    ColorValue, DesignToken, DesignTokenFile, Metadata, TokenNode, TokenType, TokenValue,
};
pub use validation::{validate_token_file, ValidationReport};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        parse_tokens, parse_tokens_from_file, validate_token_file, BinaryOperator, ColorValue,
        DesignToken, DesignTokenFile, Expr, Metadata, TokenError, TokenNode, TokenResult,
        TokenType, TokenValue, ValidationReport,
    };
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_integration() {
        let json = r##"
        {
            "colors": {
                "primary": {
                    "$value": "rgb(193, 51, 51)",
                    "$type": "color",
                    "$description": "Primary brand color"
                },
                "secondary": {
                    "$value": "#6c757d",
                    "$type": "color",
                    "$description": "Secondary color"
                }
            },
            "spacing": {
                "base": {
                    "$value": "8px",
                    "$type": "dimension",
                    "$description": "Base spacing unit"
                },
                "small": {
                    "$value": "{base} / 2",
                    "$type": "dimension",
                    "$description": "Small spacing"
                },
                "medium": {
                    "$value": "{base} * 2",
                    "$type": "dimension",
                    "$description": "Medium spacing"
                },
                "large": {
                    "$value": "{medium} + {base}",
                    "$type": "dimension",
                    "$description": "Large spacing"
                }
            },
            "borderRadius": {
                "small": {
                    "$value": "4px",
                    "$type": "borderRadius",
                    "$description": "Small border radius"
                },
                "medium": {
                    "$value": "{small} * 2",
                    "$type": "borderRadius",
                    "$description": "Medium border radius"
                }
            },
            "$themes": [],
            "$metadata": {
                "tokenSetOrder": ["colors", "spacing", "borderRadius"],
                "activeThemes": [],
                "activeSets": ["colors", "spacing", "borderRadius"]
            }
        }
        "##;

        // Parse the tokens
        let token_file = parse_tokens(json).expect("Failed to parse tokens");

        // Validate the tokens
        let validation_report = validate_token_file(&token_file).expect("Validation failed");
        
        if !validation_report.is_valid() {
            println!("Validation issues: {}", validation_report.summary());
            for error in &validation_report.errors {
                println!("Error: {}", error);
            }
            for missing in &validation_report.missing_references {
                println!("Missing reference: {}", missing);
            }
        }
        
        assert!(validation_report.is_valid(), "Validation should pass");

        // Test token access
        let primary_color = token_file.find_token("colors.primary").unwrap();
        assert_eq!(primary_color.token_type, TokenType::Color);

        let base_spacing = token_file.find_token("spacing.base").unwrap();
        assert_eq!(base_spacing.token_type, TokenType::Dimension);
        assert!(!base_spacing.has_references());

        let medium_spacing = token_file.find_token("spacing.medium").unwrap();
        assert!(medium_spacing.has_references());
        let refs = medium_spacing.get_references();
        assert_eq!(refs, vec!["base"]);

        // Test getting all tokens
        let all_tokens = token_file.get_all_tokens();
        assert_eq!(all_tokens.len(), 8); // 2 colors + 4 spacing + 2 border radius = 8 total tokens

        // Test getting all references
        let all_references = token_file.get_all_references();
        assert!(all_references.len() >= 3); // At least small, medium spacing and medium border radius have references
    }

    #[test]
    fn test_expression_parsing_integration() {
        let json = r##"
        {
            "test": {
                "a": {
                    "$value": "5",
                    "$type": "number",
                    "$description": ""
                },
                "b": {
                    "$value": "{a} * 3 + 2",
                    "$type": "number",
                    "$description": ""
                },
                "c": {
                    "$value": "({a} + 1) * 2",
                    "$type": "number",
                    "$description": ""
                }
            },
            "$themes": [],
            "$metadata": {
                "tokenSetOrder": ["test"],
                "activeThemes": [],
                "activeSets": ["test"]
            }
        }
        "##;

        let token_file = parse_tokens(json).expect("Failed to parse tokens");
        let validation_report = validate_token_file(&token_file).expect("Validation failed");
        assert!(validation_report.is_valid());

        let token_b = token_file.find_token("test.b").unwrap();
        let token_c = token_file.find_token("test.c").unwrap();

        assert!(token_b.has_references());
        assert!(token_c.has_references());

        // Check that expressions were parsed correctly
        if let TokenValue::Expression(expr) = &token_b.value {
            assert!(expr.has_references());
            let refs = expr.get_references();
            assert_eq!(refs, vec!["a"]);
        } else {
            panic!("Expected expression for token b");
        }
    }

    #[test]
    fn test_validation_errors() {
        let json = r##"
        {
            "test": {
                "invalid_dimension": {
                    "$value": "not-a-dimension",
                    "$type": "dimension",
                    "$description": ""
                },
                "missing_ref": {
                    "$value": "{nonexistent} * 2",
                    "$type": "dimension",
                    "$description": ""
                },
                "invalid_number": {
                    "$value": "abc",
                    "$type": "number",
                    "$description": ""
                }
            },
            "$themes": [],
            "$metadata": {
                "tokenSetOrder": ["test"],
                "activeThemes": [],
                "activeSets": ["test"]
            }
        }
        "##;

        let token_file = parse_tokens(json).expect("Failed to parse tokens");
        let validation_report = validate_token_file(&token_file).expect("Validation failed");
        
        assert!(!validation_report.is_valid());
        assert!(!validation_report.errors.is_empty());
        assert!(!validation_report.missing_references.is_empty());
    }
}