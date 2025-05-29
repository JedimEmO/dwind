use crate::error::{TokenError, TokenResult};
use crate::types::{DesignTokenFile, TokenNode, TokenType, TokenValue};
use std::collections::HashSet;

/// Validation context for tracking state during validation
#[derive(Debug)]
pub struct ValidationContext {
    /// All available token paths in the file
    available_tokens: HashSet<String>,
    /// Tokens currently being validated (for circular reference detection)
    validation_stack: Vec<String>,
}

impl ValidationContext {
    fn new(file: &DesignTokenFile) -> Self {
        let available_tokens = file
            .get_all_tokens()
            .into_iter()
            .map(|(path, _)| path)
            .collect();

        Self {
            available_tokens,
            validation_stack: Vec::new(),
        }
    }

    fn push_token(&mut self, path: &str) -> TokenResult<()> {
        if self.validation_stack.contains(&path.to_string()) {
            let cycle_start = self.validation_stack.iter()
                .position(|p| p == path)
                .unwrap();
            let cycle = &self.validation_stack[cycle_start..];
            let cycle_chain = cycle.iter()
                .chain(std::iter::once(&path.to_string()))
                .cloned()
                .collect::<Vec<_>>()
                .join(" -> ");
                
            return Err(TokenError::CircularReference(format!(
                "Circular reference detected: {}. This creates an infinite dependency loop.",
                cycle_chain
            )));
        }
        self.validation_stack.push(path.to_string());
        Ok(())
    }

    fn pop_token(&mut self) {
        self.validation_stack.pop();
    }

    fn token_exists(&self, path: &str) -> bool {
        self.available_tokens.contains(path)
    }

    /// Check if we're already validating a token (indicates potential cycle)
    fn is_validating(&self, path: &str) -> bool {
        self.validation_stack.contains(&path.to_string())
    }

    /// Get the current validation path for error reporting
    fn get_validation_path(&self) -> String {
        self.validation_stack.join(" -> ")
    }

    /// Validate a token's references recursively for circular dependencies
    fn validate_token_references_recursively(
        &mut self,
        token_path: &str,
        file: &DesignTokenFile,
        report: &mut ValidationReport,
    ) -> TokenResult<()> {
        // Push current token to validation stack (detects cycles)
        if let Err(e) = self.push_token(token_path) {
            // If we get a circular reference error, add it to the report instead of failing
            if let TokenError::CircularReference(msg) = e {
                report.add_circular_reference(msg);
                return Ok(()); // Continue validation of other tokens
            } else {
                return Err(e);
            }
        }

        // Get the token and its references
        if let Some(token) = file.find_token(token_path) {
            let references = token.get_references();

            for reference in references {
                // Resolve the reference path
                let resolved_ref = resolve_reference(&reference, token_path);

                // Check if referenced token exists
                if !self.token_exists(&resolved_ref) {
                    report.add_missing_reference(format!(
                        "Token '{}' references missing token '{}' (resolved as '{}')",
                        token_path, reference, resolved_ref
                    ));
                    continue;
                }

                // If the referenced token has its own references, validate them recursively
                if let Some(referenced_token) = file.find_token(&resolved_ref) {
                    if referenced_token.has_references() {
                        // Recursive call - this will detect cycles via the validation stack
                        self.validate_token_references_recursively(&resolved_ref, file, report)?;
                    }
                }
            }
        }

        // Pop current token from validation stack
        self.pop_token();
        Ok(())
    }
}

/// Validate a design token file
pub fn validate_token_file(file: &DesignTokenFile) -> TokenResult<ValidationReport> {
    let mut context = ValidationContext::new(file);
    let mut report = ValidationReport::new();

    // Validate each token set
    for (set_name, node) in &file.sets {
        validate_token_node(node, set_name, &mut context, &mut report)?;
    }

    // Validate references
    validate_all_references(file, &mut context, &mut report)?;

    Ok(report)
}

/// Validation report containing warnings and errors
#[derive(Debug, Default)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub circular_references: Vec<String>,
    pub missing_references: Vec<String>,
    pub type_mismatches: Vec<String>,
}

impl ValidationReport {
    fn new() -> Self {
        Self::default()
    }

    fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    fn add_circular_reference(&mut self, reference: String) {
        self.circular_references.push(reference);
    }

    fn add_missing_reference(&mut self, reference: String) {
        self.missing_references.push(reference);
    }

    fn add_type_mismatch(&mut self, mismatch: String) {
        self.type_mismatches.push(mismatch);
    }

    /// Check if the validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty() && self.circular_references.is_empty() && self.missing_references.is_empty()
    }

    /// Get a summary of the validation results
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        
        if !self.errors.is_empty() {
            parts.push(format!("{} errors", self.errors.len()));
        }
        if !self.warnings.is_empty() {
            parts.push(format!("{} warnings", self.warnings.len()));
        }
        if !self.circular_references.is_empty() {
            parts.push(format!("{} circular references", self.circular_references.len()));
        }
        if !self.missing_references.is_empty() {
            parts.push(format!("{} missing references", self.missing_references.len()));
        }
        if !self.type_mismatches.is_empty() {
            parts.push(format!("{} type mismatches", self.type_mismatches.len()));
        }

        if parts.is_empty() {
            "Validation passed".to_string()
        } else {
            format!("Validation issues: {}", parts.join(", "))
        }
    }
}

/// Validate a token node recursively
fn validate_token_node(
    node: &TokenNode,
    path: &str,
    context: &mut ValidationContext,
    report: &mut ValidationReport,
) -> TokenResult<()> {
    match node {
        TokenNode::Token(token) => {
            validate_design_token(token, path, context, report)?;
        }
        TokenNode::Group(group) => {
            for (key, child_node) in group {
                let child_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };
                validate_token_node(child_node, &child_path, context, report)?;
            }
        }
    }
    Ok(())
}

/// Validate a single design token
fn validate_design_token(
    token: &crate::types::DesignToken,
    path: &str,
    _context: &mut ValidationContext,
    report: &mut ValidationReport,
) -> TokenResult<()> {
    // Validate token value based on type
    match (&token.value, &token.token_type) {
        (TokenValue::Literal(value), TokenType::Dimension) => {
            if !is_valid_dimension(value) {
                report.add_error(format!("Invalid dimension value '{}' at {}", value, path));
            }
        }
        (TokenValue::Literal(value), TokenType::Number) => {
            if !is_valid_number(value) {
                report.add_error(format!("Invalid number value '{}' at {}", value, path));
            }
        }
        (TokenValue::Literal(value), TokenType::BorderRadius) => {
            if !is_valid_dimension(value) {
                report.add_error(format!("Invalid border radius value '{}' at {}", value, path));
            }
        }
        (TokenValue::Literal(value), TokenType::Color) => {
            if !is_valid_color(value) {
                report.add_error(format!("Invalid color value '{}' at {}", value, path));
            }
        }
        (TokenValue::Color(_), TokenType::Color) => {
            // Structured color is always valid for color type
        }
        (TokenValue::Expression(_), _) => {
            // Expression validation will be done in the reference validation phase
        }
        (value_type, token_type) => {
            report.add_type_mismatch(format!(
                "Type mismatch at {}: {:?} value for {:?} token",
                path, value_type, token_type
            ));
        }
    }

    Ok(())
}

/// Validate all token references in the file for both missing references and circular dependencies
fn validate_all_references(
    file: &DesignTokenFile,
    context: &mut ValidationContext,
    report: &mut ValidationReport,
) -> TokenResult<()> {
    let all_tokens = file.get_all_tokens();

    // Validate each token that has references
    for (token_path, token) in all_tokens {
        if token.has_references() {
            // Clear the validation stack for each top-level validation
            context.validation_stack.clear();
            
            // Perform recursive validation for circular dependencies
            context.validate_token_references_recursively(&token_path, file, report)?;
        }
    }

    Ok(())
}

/// Resolve a reference relative to the current token path
fn resolve_reference(reference: &str, current_path: &str) -> String {
    if reference.contains('.') {
        // Absolute reference
        reference.to_string()
    } else {
        // Relative reference - resolve within the same group
        let path_parts: Vec<&str> = current_path.split('.').collect();
        if path_parts.len() > 1 {
            let parent_path = path_parts[..path_parts.len() - 1].join(".");
            format!("{}.{}", parent_path, reference)
        } else {
            reference.to_string()
        }
    }
}

/// Validate if a string is a valid dimension value
fn is_valid_dimension(value: &str) -> bool {
    // Simple validation - should end with a unit or be a number
    if value.parse::<f64>().is_ok() {
        return true;
    }

    let units = ["px", "em", "rem", "%", "vh", "vw", "pt", "pc", "in", "cm", "mm"];
    units.iter().any(|unit| value.ends_with(unit))
}

/// Validate if a string is a valid number
fn is_valid_number(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

/// Validate if a string is a valid color value
fn is_valid_color(value: &str) -> bool {
    // Simple validation for common color formats
    if value.starts_with('#') && (value.len() == 4 || value.len() == 7 || value.len() == 9) {
        return value[1..].chars().all(|c| c.is_ascii_hexdigit());
    }

    if value.starts_with("rgb(") && value.ends_with(')') {
        return true; // More detailed validation could be added
    }

    if value.starts_with("rgba(") && value.ends_with(')') {
        return true;
    }

    if value.starts_with("hsl(") && value.ends_with(')') {
        return true;
    }

    if value.starts_with("hsla(") && value.ends_with(')') {
        return true;
    }

    // Named colors (basic set)
    let named_colors = [
        "red", "green", "blue", "white", "black", "transparent",
        "yellow", "orange", "purple", "pink", "gray", "grey",
    ];
    named_colors.contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_tokens;

    #[test]
    fn test_validate_simple_tokens() {
        let json = r#"
        {
            "test": {
                "a": {
                    "$value": "5px",
                    "$type": "dimension",
                    "$description": ""
                },
                "b": {
                    "$value": "red",
                    "$type": "color",
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

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_invalid_values() {
        let json = r#"
        {
            "test": {
                "a": {
                    "$value": "invalid-dimension",
                    "$type": "dimension",
                    "$description": ""
                },
                "b": {
                    "$value": "not-a-number",
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
        "#;

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(!report.is_valid());
        assert_eq!(report.errors.len(), 2);
    }

    #[test]
    fn test_validate_missing_references() {
        let json = r#"
        {
            "test": {
                "a": {
                    "$value": "{missing}*3",
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

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(!report.is_valid());
        assert_eq!(report.missing_references.len(), 1);
    }

    #[test]
    fn test_is_valid_dimension() {
        assert!(is_valid_dimension("5px"));
        assert!(is_valid_dimension("1.5em"));
        assert!(is_valid_dimension("100%"));
        assert!(is_valid_dimension("42"));
        assert!(!is_valid_dimension("invalid"));
    }

    #[test]
    fn test_is_valid_color() {
        assert!(is_valid_color("#ff0000"));
        assert!(is_valid_color("#f00"));
        assert!(is_valid_color("rgb(255, 0, 0)"));
        assert!(is_valid_color("red"));
        assert!(!is_valid_color("invalid-color"));
    }

    #[test]
    fn test_direct_self_reference() {
        let json = r#"
        {
            "test": {
                "a": {
                    "$value": "{a}",
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

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(!report.is_valid());
        assert!(!report.circular_references.is_empty());
        assert!(report.circular_references[0].contains("test.a -> test.a"));
    }

    #[test]
    fn test_simple_circular_reference() {
        let json = r#"
        {
            "test": {
                "a": {
                    "$value": "{b}",
                    "$type": "dimension",
                    "$description": ""
                },
                "b": {
                    "$value": "{a}",
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

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(!report.is_valid());
        assert!(!report.circular_references.is_empty());
        // Should detect the circular reference between a and b
        let circular_ref = &report.circular_references[0];
        assert!(circular_ref.contains("test.a") && circular_ref.contains("test.b"));
    }

    #[test]
    fn test_complex_circular_chain() {
        let json = r#"
        {
            "test": {
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
                    "$value": "{d}",
                    "$type": "dimension",
                    "$description": ""
                },
                "d": {
                    "$value": "{a}",
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

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(!report.is_valid());
        assert!(!report.circular_references.is_empty());
        // Should detect the circular reference in the chain a -> b -> c -> d -> a
        let circular_ref = &report.circular_references[0];
        assert!(circular_ref.contains("test.a") &&
                circular_ref.contains("test.b") &&
                circular_ref.contains("test.c") &&
                circular_ref.contains("test.d"));
    }

    #[test]
    fn test_mixed_circular_and_valid_references() {
        let json = r#"
        {
            "test": {
                "valid": {
                    "$value": "5px",
                    "$type": "dimension",
                    "$description": ""
                },
                "also_valid": {
                    "$value": "{valid} * 2",
                    "$type": "dimension",
                    "$description": ""
                },
                "circular_a": {
                    "$value": "{circular_b}",
                    "$type": "dimension",
                    "$description": ""
                },
                "circular_b": {
                    "$value": "{circular_a}",
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

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(!report.is_valid());
        assert!(!report.circular_references.is_empty());
        // Should only detect circular references, not affect valid ones
        let circular_ref = &report.circular_references[0];
        assert!(circular_ref.contains("circular_a") && circular_ref.contains("circular_b"));
        assert!(!circular_ref.contains("valid") && !circular_ref.contains("also_valid"));
    }

    #[test]
    fn test_expression_circular_references() {
        let json = r#"
        {
            "test": {
                "a": {
                    "$value": "{b} + 5",
                    "$type": "dimension",
                    "$description": ""
                },
                "b": {
                    "$value": "{a} * 2",
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

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(!report.is_valid());
        assert!(!report.circular_references.is_empty());
        // Should detect circular references even within mathematical expressions
        let circular_ref = &report.circular_references[0];
        assert!(circular_ref.contains("test.a") && circular_ref.contains("test.b"));
    }

    #[test]
    fn test_no_false_positives_for_valid_references() {
        let json = r#"
        {
            "test": {
                "base": {
                    "$value": "10px",
                    "$type": "dimension",
                    "$description": ""
                },
                "double": {
                    "$value": "{base} * 2",
                    "$type": "dimension",
                    "$description": ""
                },
                "triple": {
                    "$value": "{base} * 3",
                    "$type": "dimension",
                    "$description": ""
                },
                "combined": {
                    "$value": "{double} + {triple}",
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

        let file = parse_tokens(json).unwrap();
        let report = validate_token_file(&file).unwrap();
        assert!(report.is_valid());
        assert!(report.circular_references.is_empty());
    }
}