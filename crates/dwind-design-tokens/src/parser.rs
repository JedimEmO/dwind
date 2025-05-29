use crate::error::{TokenError, TokenResult};
use crate::expressions::Expr;
use crate::types::{DesignTokenFile, TokenValue};
use serde_json;
use std::path::Path;

/// Parse a design token file from JSON string
pub fn parse_tokens(json: &str) -> TokenResult<DesignTokenFile> {
    let mut file: DesignTokenFile = serde_json::from_str(json)?;
    
    // Post-process to convert expression strings to Expression variants
    process_token_file(&mut file)?;
    
    Ok(file)
}

/// Parse a design token file from a file path
pub fn parse_tokens_from_file<P: AsRef<Path>>(path: P) -> TokenResult<DesignTokenFile> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| TokenError::InvalidValue(format!("Failed to read file: {}", e)))?;
    parse_tokens(&content)
}

/// Process the token file to convert expression strings to Expression variants
fn process_token_file(file: &mut DesignTokenFile) -> TokenResult<()> {
    for (_, node) in file.sets.iter_mut() {
        process_token_node(node)?;
    }
    Ok(())
}

/// Recursively process token nodes to detect and parse expressions
fn process_token_node(node: &mut crate::types::TokenNode) -> TokenResult<()> {
    use crate::types::TokenNode;
    
    match node {
        TokenNode::Token(token) => {
            // Check if the value is a string that looks like an expression
            if let TokenValue::Literal(ref value_str) = token.value {
                if is_expression(value_str) {
                    // Parse the expression
                    let expr = Expr::parse(value_str)?;
                    token.value = TokenValue::Expression(expr);
                }
            }
        }
        TokenNode::Group(group) => {
            for (_, child_node) in group.iter_mut() {
                process_token_node(child_node)?;
            }
        }
    }
    
    Ok(())
}

/// Determine if a string value should be treated as an expression
fn is_expression(value: &str) -> bool {
    // Check for token references (contains {})
    if value.contains('{') && value.contains('}') {
        return true;
    }
    
    // Check for arithmetic operators with potential references
    // This is a simple heuristic - we could make it more sophisticated
    let has_operators = value.contains('+') || value.contains('-') || value.contains('*') || value.contains('/');
    let has_numbers = value.chars().any(|c| c.is_ascii_digit());
    
    // If it has operators and numbers, it might be an expression
    // But we need to be careful not to catch CSS values like "rgb(255, 0, 0)"
    if has_operators && has_numbers && !value.starts_with("rgb") && !value.starts_with("hsl") {
        return true;
    }
    
    false
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{TokenType, DesignToken};

    #[test]
    fn test_is_expression() {
        assert!(is_expression("{a}*3"));
        assert!(is_expression("{medium}+12"));
        assert!(is_expression("2 + 3"));
        assert!(!is_expression("5px"));
        assert!(!is_expression("rgb(255, 0, 0)"));
        assert!(!is_expression("solid"));
    }

    #[test]
    fn test_parse_simple_tokens() {
        let json = r#"
        {
            "test": {
                "a": {
                    "$value": "5px",
                    "$type": "dimension",
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
        "#;

        let result = parse_tokens(json);
        assert!(result.is_ok());
        
        let file = result.unwrap();
        let token = file.find_token("test.a").unwrap();
        assert_eq!(token.token_type, TokenType::Dimension);
        
        if let TokenValue::Literal(value) = &token.value {
            assert_eq!(value, "5px");
        } else {
            panic!("Expected literal value");
        }
    }

    #[test]
    fn test_parse_expression_tokens() {
        let json = r#"
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

        let result = parse_tokens(json);
        assert!(result.is_ok());
        
        let file = result.unwrap();
        let token_a = file.find_token("test.a").unwrap();
        let token_b = file.find_token("test.b").unwrap();
        
        assert!(!token_a.has_references());
        assert!(token_b.has_references());
        
        let refs = token_b.get_references();
        assert_eq!(refs, vec!["a"]);
    }

    #[test]
    fn test_parse_nested_groups() {
        let json = r#"
        {
            "colors": {
                "primary": {
                    "500": {
                        "$value": "rgb(193, 51, 51)",
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
        "#;

        let result = parse_tokens(json);
        assert!(result.is_ok());
        
        let file = result.unwrap();
        let token = file.find_token("colors.primary.500").unwrap();
        assert_eq!(token.token_type, TokenType::Color);
    }

    #[test]
    fn test_get_all_references() {
        let json = r#"
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
                "c": {
                    "$value": "{a} + {b}",
                    "$type": "dimension",
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
        "#;

        let result = parse_tokens(json);
        assert!(result.is_ok());
        
        let file = result.unwrap();
        let references = file.get_all_references();
        
        assert_eq!(references.len(), 2);
        assert_eq!(references.get("test.b").unwrap(), &vec!["a"]);
        assert_eq!(references.get("test.c").unwrap(), &vec!["a", "b"]);
    }
}