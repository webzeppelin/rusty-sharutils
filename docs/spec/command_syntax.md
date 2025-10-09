# Command Line Parsing Framework Specification

This specification defines a shared command line parsing framework for all four rusty-sharutils commands (`shar`, `unshar`, `uuencode`, `uudecode`). The framework provides type-safe parsing with validation while using only the Rust standard library.

## Overview

Commands follow GNU/Linux conventions:
- Command path, followed by options, followed by arguments
- Long options: `--option` or `--option=value`
- Short flags: `-f` or `-f value`
- Flag combining: `-abc` (equivalent to `-a -b -c`)
- Only the last flag in a combination can take a value: `-abc value`

## Data Structures

### OptionDefinition

Defines a single command-line option with validation.

```rust
pub struct OptionDefinition {
    pub flag: char,
    pub name: String,
    pub has_value: bool,
    pub default_value: Option<OsString>,
    pub validator: Option<OptionValidator>,
    pub help_text: String,
}
```

**Field Requirements:**
- `flag`: Single ASCII letter (a-z, A-Z)
- `name`: Lowercase letters and hyphens only, no whitespace
- `has_value`: If true, option accepts/requires a value
- `default_value`: Used when option specified without value (only valid if `has_value` is true)
- `validator`: Optional function to validate option values
- `help_text`: Description for help output

### ParsedCommand

Contains the fully parsed and validated command line.

```rust
pub struct ParsedCommand {
    pub executable_path: OsString,
    pub options: HashMap<String, Option<OsString>>,
    pub arguments: Vec<OsString>,
}

impl ParsedCommand {
    /// Returns true if the named option was specified on the command line
    pub fn is_option_set(&self, name: &str) -> bool;
    
    /// Returns the value associated with an option, or None if not set
    pub fn option_value(&self, name: &str) -> Option<&OsStr>;
    
    /// Returns the value for an option or its default value
    pub fn option_value_or_default(&self, name: &str, default: &OsStr) -> &OsStr;
    
    /// Returns true if the option has an explicit value (not just present)
    pub fn has_option_value(&self, name: &str) -> bool;
}
```

### Error Types

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    ValidationError(ValidationError),
    UnknownOption(String),
    MissingValue(String),
    InvalidFlagCombination(String),
    DuplicateOption(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ValidationError(e) => write!(f, "Validation error: {}", e),
            ParseError::UnknownOption(opt) => write!(f, "Unknown option: {}", opt),
            ParseError::MissingValue(opt) => write!(f, "Option '{}' requires a value", opt),
            ParseError::InvalidFlagCombination(flags) => write!(f, "Invalid flag combination: {}", flags),
            ParseError::DuplicateOption(opt) => write!(f, "Option '{}' specified multiple times", opt),
        }
    }
}

impl std::error::Error for ParseError {}
```

## Core Parsing Function

```rust
/// Parses command line arguments according to the provided option definitions
pub fn parse_command_line(
    option_definitions: &[OptionDefinition],
    args: impl Iterator<Item = OsString>
) -> Result<ParsedCommand, ParseError>;
```

## Parsing Algorithm Specification

### 1. Initialization
- Extract executable path from first argument
- Build lookup tables for options by flag and name
- Validate option definitions for conflicts

### 2. Argument Processing
Process remaining arguments in order:

**Long Options (`--name` or `--name=value`)**
- Strip `--` prefix
- Split on `=` if present
- Look up option by name
- Validate value if provided/required
- Store in options map

**Short Flags (`-f` or `-abc`)**
- Strip `-` prefix  
- For single flag: process normally
- For multiple flags: process each except last as boolean options
- Last flag can accept value from next argument
- Validate no value-requiring flags in middle of combination

**Arguments**
- All non-option arguments after options are complete
- Options processing stops at `--` or first non-option argument

### 3. Post-Processing
- Apply default values for unspecified options
- Validate all required values are present
- Run validators on all provided values

### 4. Standard Options
All commands must support these built-in options:

```rust
pub fn standard_options() -> Vec<OptionDefinition> {
    vec![
        OptionDefinition {
            flag: 'h',
            name: "help".to_string(),
            has_value: false,
            default_value: None,
            validator: None,
            help_text: "Display this help message and exit".to_string(),
        },
        OptionDefinition {
            flag: 'V',
            name: "version".to_string(),
            has_value: false,
            default_value: None,
            validator: None,
            help_text: "Display version information and exit".to_string(),
        },
    ]
}
```

## Validation Framework

### Custom Validators
Commands can provide custom validators for specific options:

```rust
// Example: validate file exists
fn validate_existing_file(value: &OsStr) -> Result<(), ValidationError> {
    let path = Path::new(value);
    if path.exists() {
        Ok(())
    } else {
        Err(ValidationError::new(format!("File does not exist: {}", path.display())))
    }
}

// Example: validate positive integer
fn validate_positive_integer(value: &OsStr) -> Result<(), ValidationError> {
    let s = value.to_str()
        .ok_or_else(|| ValidationError::new("Invalid UTF-8 in number"))?;
    let n: u32 = s.parse()
        .map_err(|_| ValidationError::new("Not a valid positive integer"))?;
    if n == 0 {
        return Err(ValidationError::new("Value must be greater than zero"));
    }
    Ok(())
}
```

## Help Generation

The framework must automatically generate help text:

```rust
/// Generates formatted help text for the command
pub fn generate_help(
    command_name: &str,
    description: &str,
    usage_pattern: &str,
    option_definitions: &[OptionDefinition]
) -> String;
```

Example output:
```
Usage: uuencode [OPTIONS] [input-file] output-name

Convert files to uuencoded format for safe transmission

Options:
  -m, --base64           Use base64 encoding instead of uuencoding
  -e, --encode-filename  Encode the output filename
  -h, --help            Display this help message and exit
  -V, --version         Display version information and exit
```

## Integration Pattern

Each command integrates the framework as follows:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = standard_options();
    options.extend(command_specific_options());
    
    let parsed = parse_command_line(&options, std::env::args_os().skip(1))?;
    
    if parsed.is_option_set("help") {
        println!("{}", generate_help("command", "description", "usage", &options));
        return Ok(());
    }
    
    if parsed.is_option_set("version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    
    // Command-specific logic using parsed.options and parsed.arguments
    Ok(())
}
```

## Error Handling Requirements

- All parsing errors must be recoverable and user-friendly
- Invalid options should suggest similar valid options when possible
- Validation errors should clearly indicate what input was invalid and why
- Help should be automatically displayed for parsing errors

## Testing Requirements

The framework must support:
- Unit tests for individual parsing scenarios
- Property-based testing for option combinations
- Integration tests with real command line argument arrays
- Error case validation
- Round-trip testing (parse then reconstruct command line)

## Cross-Platform Considerations

- Use `OsString`/`OsStr` for all user-provided values
- Handle platform-specific path separators in file validation
- Support Unicode in option values where the platform allows
- Consistent behavior across Windows, macOS, and Linux
