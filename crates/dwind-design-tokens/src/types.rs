use crate::expressions::Expr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported token types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TokenType {
    Dimension,
    Number,
    BorderRadius,
    Color,
}

/// Color value with structured components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorValue {
    pub color_space: String,
    pub components: [f32; 3],
    pub alpha: f32,
    pub hex: String,
}

/// Token value variants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenValue {
    /// Structured color object
    Color(ColorValue),
    /// Literal string value (e.g., "5px", "rgb(255, 0, 0)")
    Literal(String),
    /// Expression that needs to be evaluated
    Expression(Expr),
}

impl TokenValue {
    /// Check if this value contains any token references
    pub fn has_references(&self) -> bool {
        match self {
            TokenValue::Expression(expr) => expr.has_references(),
            _ => false,
        }
    }

    /// Get all token references in this value
    pub fn get_references(&self) -> Vec<String> {
        match self {
            TokenValue::Expression(expr) => expr.get_references(),
            _ => vec![],
        }
    }

    /// Check if this is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self, TokenValue::Literal(_) | TokenValue::Color(_))
    }
}

/// A design token with its metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignToken {
    #[serde(rename = "$value")]
    pub value: TokenValue,

    #[serde(rename = "$type")]
    pub token_type: TokenType,

    #[serde(rename = "$description")]
    pub description: Option<String>,
}

impl DesignToken {
    /// Create a new design token
    pub fn new(value: TokenValue, token_type: TokenType, description: Option<String>) -> Self {
        Self {
            value,
            token_type,
            description,
        }
    }

    /// Check if this token has any references to other tokens
    pub fn has_references(&self) -> bool {
        self.value.has_references()
    }

    /// Get all token references in this token
    pub fn get_references(&self) -> Vec<String> {
        self.value.get_references()
    }
}

/// A node in the token tree - either a token or a group
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenNode {
    /// A leaf token
    Token(DesignToken),
    /// A group containing other tokens or groups
    Group(HashMap<String, TokenNode>),
}

impl TokenNode {
    /// Check if this node is a token
    pub fn is_token(&self) -> bool {
        matches!(self, TokenNode::Token(_))
    }

    /// Check if this node is a group
    pub fn is_group(&self) -> bool {
        matches!(self, TokenNode::Group(_))
    }

    /// Get the token if this node is a token
    pub fn as_token(&self) -> Option<&DesignToken> {
        match self {
            TokenNode::Token(token) => Some(token),
            _ => None,
        }
    }

    /// Get the group if this node is a group
    pub fn as_group(&self) -> Option<&HashMap<String, TokenNode>> {
        match self {
            TokenNode::Group(group) => Some(group),
            _ => None,
        }
    }

    /// Recursively collect all tokens in this node and its children
    pub fn collect_tokens(&self) -> Vec<(String, &DesignToken)> {
        self.collect_tokens_with_prefix("")
    }

    /// Recursively collect all tokens with a path prefix
    fn collect_tokens_with_prefix(&self, prefix: &str) -> Vec<(String, &DesignToken)> {
        match self {
            TokenNode::Token(token) => vec![(prefix.to_string(), token)],
            TokenNode::Group(group) => {
                let mut tokens = Vec::new();
                for (key, node) in group {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    tokens.extend(node.collect_tokens_with_prefix(&path));
                }
                tokens
            }
        }
    }

    /// Find a token by path (e.g., "group.subgroup.token")
    pub fn find_token(&self, path: &str) -> Option<&DesignToken> {
        let parts: Vec<&str> = path.split('.').collect();
        self.find_token_by_parts(&parts)
    }

    fn find_token_by_parts(&self, parts: &[&str]) -> Option<&DesignToken> {
        if parts.is_empty() {
            return None;
        }

        match self {
            TokenNode::Token(token) => {
                // If we have a token and no more parts, return it
                if parts.len() == 1 {
                    Some(token)
                } else {
                    None // Can't navigate deeper into a token
                }
            }
            TokenNode::Group(group) => {
                if parts.len() == 1 {
                    // Look for a direct token in this group
                    if let Some(TokenNode::Token(token)) = group.get(parts[0]) {
                        Some(token)
                    } else {
                        None
                    }
                } else {
                    // Navigate deeper into the tree
                    let key = parts[0];
                    let remaining = &parts[1..];
                    group.get(key)?.find_token_by_parts(remaining)
                }
            }
        }
    }
}

/// Metadata about the token file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub token_set_order: Vec<String>,
    pub active_themes: Vec<String>,
    pub active_sets: Vec<String>,
}

/// The root design token file structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignTokenFile {
    /// Token sets (flattened into the root)
    #[serde(flatten)]
    pub sets: HashMap<String, TokenNode>,

    /// Theme definitions (not implemented in Phase 1)
    #[serde(rename = "$themes")]
    pub themes: Vec<serde_json::Value>,

    /// File metadata
    #[serde(rename = "$metadata")]
    pub metadata: Metadata,
}

impl DesignTokenFile {
    /// Get all tokens from all sets
    pub fn get_all_tokens(&self) -> Vec<(String, &DesignToken)> {
        let mut all_tokens = Vec::new();
        for (set_name, node) in &self.sets {
            let tokens = node.collect_tokens();
            for (token_path, token) in tokens {
                let full_path = if token_path.is_empty() {
                    set_name.clone()
                } else {
                    format!("{}.{}", set_name, token_path)
                };
                all_tokens.push((full_path, token));
            }
        }
        all_tokens
    }

    /// Find a token by its full path (e.g., "set.group.token")
    pub fn find_token(&self, path: &str) -> Option<&DesignToken> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        let set_name = parts[0];
        let remaining_path = parts[1..].join(".");

        let set = self.sets.get(set_name)?;
        if remaining_path.is_empty() {
            set.as_token()
        } else {
            set.find_token(&remaining_path)
        }
    }

    /// Get all token references in the file
    pub fn get_all_references(&self) -> HashMap<String, Vec<String>> {
        let mut references = HashMap::new();
        let all_tokens = self.get_all_tokens();

        for (path, token) in all_tokens {
            let token_refs = token.get_references();
            if !token_refs.is_empty() {
                references.insert(path, token_refs);
            }
        }

        references
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::Expr;

    #[test]
    fn test_token_value_has_references() {
        let literal = TokenValue::Literal("5px".to_string());
        assert!(!literal.has_references());

        let expr = TokenValue::Expression(Expr::parse("{a} + 3").unwrap());
        assert!(expr.has_references());
    }

    #[test]
    fn test_design_token_creation() {
        let token = DesignToken::new(
            TokenValue::Literal("5px".to_string()),
            TokenType::Dimension,
            Some("Test dimension".to_string()),
        );

        assert_eq!(token.token_type, TokenType::Dimension);
        assert_eq!(token.description, Some("Test dimension".to_string()));
        assert!(!token.has_references());
    }

    #[test]
    fn test_token_node_find_token() {
        let mut group = HashMap::new();
        group.insert(
            "test".to_string(),
            TokenNode::Token(DesignToken::new(
                TokenValue::Literal("5px".to_string()),
                TokenType::Dimension,
                None,
            )),
        );

        let node = TokenNode::Group(group);
        let found = node.find_token("test");
        assert!(found.is_some());
        assert_eq!(found.unwrap().token_type, TokenType::Dimension);
    }

    #[test]
    fn test_collect_tokens() {
        let mut inner_group = HashMap::new();
        inner_group.insert(
            "inner".to_string(),
            TokenNode::Token(DesignToken::new(
                TokenValue::Literal("10px".to_string()),
                TokenType::Dimension,
                None,
            )),
        );

        let mut outer_group = HashMap::new();
        outer_group.insert("group".to_string(), TokenNode::Group(inner_group));

        let node = TokenNode::Group(outer_group);
        let tokens = node.collect_tokens();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, "group.inner");
    }
}
