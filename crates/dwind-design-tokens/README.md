# DWIND Design Tokens

A Rust library for parsing and working with design token files in the DWIND ecosystem. This crate provides data structures and parsing capabilities for design tokens following the Design Tokens Community Group specification.

## Features

- **JSON Parsing**: Parse design token JSON files into structured Rust types
- **Nested Groups**: Support for hierarchical token organization
- **Expression Parsing**: Parse token references and arithmetic expressions using nom
- **Enhanced Validation**: Comprehensive validation with detailed error reporting and circular reference detection
- **Multiple Token Types**: Support for dimension, number, borderRadius, and color tokens
- **Error Handling**: Robust error handling with thiserror

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dwind-design-tokens = "0.1.0"
```

### Basic Example

```rust
use dwind_design_tokens::prelude::*;

let json = r#"
{
  "spacing": {
    "small": {
      "$value": "8px",
      "$type": "dimension",
      "$description": "Small spacing unit"
    },
    "medium": {
      "$value": "{small} * 2",
      "$type": "dimension",
      "$description": "Medium spacing unit"
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

// Parse the token file
let token_file = parse_tokens(json)?;

// Validate the tokens
let validation_report = validate_token_file(&token_file)?;
assert!(validation_report.is_valid());

// Access tokens
let small_token = token_file.find_token("spacing.small").unwrap();
println!("Small spacing: {:?}", small_token.value);

// Check for references
let medium_token = token_file.find_token("spacing.medium").unwrap();
assert!(medium_token.has_references());
let refs = medium_token.get_references();
assert_eq!(refs, vec!["small"]);
```

### Loading from File

```rust
use dwind_design_tokens::prelude::*;

let token_file = parse_tokens_from_file("tokens.json")?;
let validation_report = validate_token_file(&token_file)?;

if !validation_report.is_valid() {
    println!("Validation errors: {}", validation_report.summary());
}
```

## Supported Token Types

- **Dimension**: Values with units (e.g., "8px", "1.5rem")
- **Number**: Numeric values (e.g., "42", "3.14")
- **BorderRadius**: Border radius values (e.g., "4px", "50%")
- **Color**: Color values (e.g., "#ff0000", "rgb(255, 0, 0)")

## Expression Support

The library supports expressions in token values:

- **Token References**: `{token_name}`, `{group.token_name}`
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Parentheses**: `({a} + {b}) * 2`

Examples:
- `{base} * 2`
- `{medium} + {small}`
- `({width} - {padding}) / 2`

## Validation

The validation system provides comprehensive checking including **enhanced circular reference detection**:

- **Missing References**: Tokens that reference non-existent tokens
- **Enhanced Circular Reference Detection**: Detects all types of circular dependencies:
  - Direct self-references: `{a: "{a}"}`
  - Simple circular references: `{a: "{b}", b: "{a}"}`
  - Complex circular chains: `{a: "{b}", b: "{c}", c: "{d}", d: "{a}"}`
  - Circular references in expressions: `{a: "{b} + 5", b: "{a} * 2"}`
- **Type Mismatches**: Values that don't match their declared type
- **Invalid Values**: Malformed dimension, number, or color values

### Circular Reference Detection Example

```rust
use dwind_design_tokens::prelude::*;

let circular_json = r#"
{
  "spacing": {
    "a": {"$value": "{b}", "$type": "dimension"},
    "b": {"$value": "{c}", "$type": "dimension"},
    "c": {"$value": "{a}", "$type": "dimension"}
  },
  "$themes": [],
  "$metadata": {"tokenSetOrder": ["spacing"], "activeThemes": [], "activeSets": ["spacing"]}
}
"#;

let token_file = parse_tokens(circular_json)?;
let report = validate_token_file(&token_file)?;

if !report.is_valid() {
    for circular_ref in &report.circular_references {
        println!("Detected: {}", circular_ref);
        // Output: "Circular reference detected: spacing.a -> spacing.b -> spacing.c -> spacing.a. This creates an infinite dependency loop."
    }
}
```

## Data Structures

### Core Types

- `DesignTokenFile`: Root structure containing all token sets
- `TokenNode`: Either a token or a group of tokens
- `DesignToken`: Individual token with value, type, and description
- `TokenValue`: Literal value, expression, or structured color
- `Expr`: Expression AST for token references and arithmetic

### Error Handling

All operations return `TokenResult<T>` which is an alias for `Result<T, TokenError>`. The `TokenError` enum provides detailed error information for different failure scenarios.

## Phase 1 Implementation

This is Phase 1 of the design token support, focusing on:

- ✅ JSON parsing and data structures
- ✅ Expression AST creation with nom
- ✅ Validation and error handling
- ✅ **Enhanced circular reference detection**
- ❌ Expression evaluation (planned for Phase 2)
- ❌ Circular reference resolution (planned for Phase 2)

## Integration

This crate is designed to be used by other parts of the DWIND ecosystem:

- `dwind-build`: Build-time token processing
- Future code generation tools
- Runtime token resolution systems

## License

MIT