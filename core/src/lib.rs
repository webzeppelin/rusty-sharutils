use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::Path;

/// Validation error for option values
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Function type for validating option values
pub type OptionValidator = fn(&OsStr) -> Result<(), ValidationError>;

/// Defines a single command-line option with validation
pub struct OptionDefinition {
    pub flag: char,
    pub name: String,
    pub has_value: bool,
    pub default_value: Option<OsString>,  // Used when option is specified but without value
    pub validator: Option<OptionValidator>,
    pub help_text: String,
}

/// Contains the fully parsed and validated command line
pub struct ParsedCommand {
    pub executable_path: OsString,
    pub options: HashMap<String, Option<OsString>>,
    pub arguments: Vec<OsString>,
}

impl ParsedCommand {
    /// Returns true if the named option was specified on the command line
    pub fn is_option_set(&self, name: &str) -> bool {
        self.options.contains_key(name)
    }
    
    /// Returns the value associated with an option, or None if not set
    pub fn option_value(&self, name: &str) -> Option<&OsStr> {
        self.options.get(name).and_then(|v| v.as_deref())
    }
    
    /// Returns the value for an option or its default value
    pub fn option_value_or_default<'a>(&'a self, name: &str, default: &'a OsStr) -> &'a OsStr {
        self.option_value(name).unwrap_or(default)
    }
    
    /// Returns true if the option has an explicit value (not just present)
    pub fn has_option_value(&self, name: &str) -> bool {
        self.options.get(name).map_or(false, |v| v.is_some())
    }
}

/// Command line parsing errors
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

/// Returns the standard options that all commands must support
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

/// Parses command line arguments according to the provided option definitions
pub fn parse_command_line(
    option_definitions: &[OptionDefinition],
    args: impl Iterator<Item = OsString>
) -> Result<ParsedCommand, ParseError> {
    let mut args = args.collect::<Vec<_>>();
    
    if args.is_empty() {
        return Err(ParseError::UnknownOption("No executable path provided".to_string()));
    }
    
    let executable_path = args.remove(0);
    
    // Build lookup tables
    let mut by_flag: HashMap<char, &OptionDefinition> = HashMap::new();
    let mut by_name: HashMap<&str, &OptionDefinition> = HashMap::new();
    
    for def in option_definitions {
        if by_flag.insert(def.flag, def).is_some() {
            return Err(ParseError::DuplicateOption(format!("flag '{}'", def.flag)));
        }
        if by_name.insert(&def.name, def).is_some() {
            return Err(ParseError::DuplicateOption(def.name.clone()));
        }
    }
    
    let mut options: HashMap<String, Option<OsString>> = HashMap::new();
    let mut arguments: Vec<OsString> = Vec::new();
    let mut i = 0;
    
    while i < args.len() {
        let arg = &args[i];
        let arg_str = arg.to_string_lossy();
        
        if arg_str == "--" {
            // Everything after -- is arguments
            arguments.extend_from_slice(&args[i + 1..]);
            break;
        } else if arg_str.starts_with("--") {
            // Long option
            let (option_name, value) = if let Some(eq_pos) = arg_str.find('=') {
                (&arg_str[2..eq_pos], Some(OsString::from(&arg_str[eq_pos + 1..])))
            } else {
                (&arg_str[2..], None)
            };
            
            let def = by_name.get(option_name)
                .ok_or_else(|| ParseError::UnknownOption(format!("--{}", option_name)))?;
            
            if options.contains_key(&def.name) {
                return Err(ParseError::DuplicateOption(def.name.clone()));
            }
            
            let final_value = if def.has_value {
                if let Some(v) = value {
                    Some(v)
                } else if i + 1 < args.len() && !args[i + 1].to_string_lossy().starts_with('-') {
                    i += 1;
                    Some(args[i].clone())
                } else if let Some(default) = &def.default_value {
                    Some(default.clone())
                } else {
                    return Err(ParseError::MissingValue(def.name.clone()));
                }
            } else {
                if value.is_some() {
                    return Err(ParseError::ValidationError(ValidationError::new(
                        format!("Option '{}' does not accept a value", def.name)
                    )));
                }
                None
            };
            
            // Validate if there's a validator and a value
            if let (Some(validator), Some(val)) = (def.validator, &final_value) {
                validator(val).map_err(ParseError::ValidationError)?;
            }
            
            options.insert(def.name.clone(), final_value);
        } else if arg_str.starts_with('-') && arg_str.len() > 1 {
            // Short flag(s)
            let flags = &arg_str[1..];
            let flag_chars: Vec<char> = flags.chars().collect();
            
            for (j, &flag_char) in flag_chars.iter().enumerate() {
                let def = by_flag.get(&flag_char)
                    .ok_or_else(|| ParseError::UnknownOption(format!("-{}", flag_char)))?;
                
                if options.contains_key(&def.name) {
                    return Err(ParseError::DuplicateOption(def.name.clone()));
                }
                
                let is_last_flag = j == flag_chars.len() - 1;
                
                if def.has_value {
                    if !is_last_flag {
                        return Err(ParseError::InvalidFlagCombination(
                            format!("Flag '{}' requires a value but is not the last in combination '{}'", flag_char, flags)
                        ));
                    }
                    
                    let final_value = if i + 1 < args.len() && !args[i + 1].to_string_lossy().starts_with('-') {
                        i += 1;
                        Some(args[i].clone())
                    } else if let Some(default) = &def.default_value {
                        Some(default.clone())
                    } else {
                        return Err(ParseError::MissingValue(def.name.clone()));
                    };
                    
                    // Validate if there's a validator
                    if let (Some(validator), Some(val)) = (def.validator, &final_value) {
                        validator(val).map_err(ParseError::ValidationError)?;
                    }
                    
                    options.insert(def.name.clone(), final_value);
                } else {
                    options.insert(def.name.clone(), None);
                }
            }
        } else {
            // Regular argument - collect all remaining as arguments
            arguments.extend_from_slice(&args[i..]);
            break;
        }
        
        i += 1;
    }
    
    // Apply default values only for options that were explicitly specified
    // (Don't auto-add all options with defaults to the result)
    // The defaults are already applied above when options are processed
    
    Ok(ParsedCommand {
        executable_path,
        options,
        arguments,
    })
}

/// Generates formatted help text for the command
pub fn generate_help(
    command_name: &str,
    description: &str,
    usage_pattern: &str,
    option_definitions: &[OptionDefinition]
) -> String {
    let mut help = String::new();
    help.push_str(&format!("Usage: {} {}\n\n", command_name, usage_pattern));
    help.push_str(&format!("{}\n\n", description));
    help.push_str("Options:\n");
    
    for def in option_definitions {
        let short_flag = format!("-{}", def.flag);
        let long_flag = format!("--{}", def.name);
        let flags = format!("{}, {}", short_flag, long_flag);
        help.push_str(&format!("  {:<20} {}\n", flags, def.help_text));
    }
    
    help
}

// Common validators
pub fn validate_existing_file(value: &OsStr) -> Result<(), ValidationError> {
    let path = Path::new(value);
    if path.exists() {
        Ok(())
    } else {
        Err(ValidationError::new(format!("File does not exist: {}", path.display())))
    }
}

pub fn validate_positive_integer(value: &OsStr) -> Result<(), ValidationError> {
    let s = value.to_str()
        .ok_or_else(|| ValidationError::new("Invalid UTF-8 in number".to_string()))?;
    let n: u32 = s.parse()
        .map_err(|_| ValidationError::new("Not a valid positive integer".to_string()))?;
    if n == 0 {
        return Err(ValidationError::new("Value must be greater than zero".to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_options() {
        let options = standard_options();
        assert_eq!(options.len(), 2);
        assert_eq!(options[0].name, "help");
        assert_eq!(options[1].name, "version");
    }

    #[test]
    fn test_parse_simple_command() {
        let options = standard_options();
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("--help"),
        ];
        
        let result = parse_command_line(&options, args.into_iter()).unwrap();
        assert!(result.is_option_set("help"));
        assert!(!result.is_option_set("version"));
        assert_eq!(result.arguments.len(), 0);
    }

    #[test]
    fn test_parse_with_arguments() {
        let options = standard_options();
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("file1.txt"),
            OsString::from("file2.txt"),
        ];
        
        let result = parse_command_line(&options, args.into_iter()).unwrap();
        assert_eq!(result.arguments.len(), 2);
        assert_eq!(result.arguments[0], OsString::from("file1.txt"));
        assert_eq!(result.arguments[1], OsString::from("file2.txt"));
    }

    #[test]
    fn test_unknown_option_error() {
        let options = standard_options();
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("--unknown"),
        ];
        
        let result = parse_command_line(&options, args.into_iter());
        assert!(matches!(result, Err(ParseError::UnknownOption(_))));
    }

    #[test]
    fn test_combined_short_flags() {
        let mut options = standard_options();
        options.push(OptionDefinition {
            flag: 'm',
            name: "mode".to_string(),
            has_value: false,
            default_value: None,
            validator: None,
            help_text: "Test mode".to_string(),
        });
        
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("-hm"),
        ];
        
        let result = parse_command_line(&options, args.into_iter()).unwrap();
        assert!(result.is_option_set("help"));
        assert!(result.is_option_set("mode"));
    }

    #[test]
    fn test_long_option_with_value() {
        let mut options = standard_options();
        options.push(OptionDefinition {
            flag: 'f',
            name: "file".to_string(),
            has_value: true,
            default_value: None,
            validator: None,
            help_text: "File path".to_string(),
        });
        
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("--file=test.txt"),
        ];
        
        let result = parse_command_line(&options, args.into_iter()).unwrap();
        assert!(result.is_option_set("file"));
        assert_eq!(result.option_value("file").unwrap(), OsStr::new("test.txt"));
    }

    #[test]
    fn test_short_flag_with_value() {
        let mut options = standard_options();
        options.push(OptionDefinition {
            flag: 'f',
            name: "file".to_string(),
            has_value: true,
            default_value: None,
            validator: None,
            help_text: "File path".to_string(),
        });
        
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("-f"),
            OsString::from("test.txt"),
        ];
        
        let result = parse_command_line(&options, args.into_iter()).unwrap();
        assert!(result.is_option_set("file"));
        assert_eq!(result.option_value("file").unwrap(), OsStr::new("test.txt"));
    }

    #[test]
    fn test_missing_required_value() {
        let mut options = standard_options();
        options.push(OptionDefinition {
            flag: 'f',
            name: "file".to_string(),
            has_value: true,
            default_value: None,
            validator: None,
            help_text: "File path".to_string(),
        });
        
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("-f"),
        ];
        
        let result = parse_command_line(&options, args.into_iter());
        assert!(matches!(result, Err(ParseError::MissingValue(_))));
    }

    #[test]
    fn test_default_value_when_no_value_provided() {
        let mut options = standard_options();
        options.push(OptionDefinition {
            flag: 'o',
            name: "output".to_string(),
            has_value: true,
            default_value: Some(OsString::from("default.txt")),
            validator: None,
            help_text: "Output file".to_string(),
        });
        
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("-o"),
        ];
        
        let result = parse_command_line(&options, args.into_iter()).unwrap();
        assert!(result.is_option_set("output"));
        assert_eq!(result.option_value("output").unwrap(), OsStr::new("default.txt"));
    }

    #[test]
    fn test_duplicate_option_error() {
        let options = standard_options();
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("--help"),
            OsString::from("--help"),
        ];
        
        let result = parse_command_line(&options, args.into_iter());
        assert!(matches!(result, Err(ParseError::DuplicateOption(_))));
    }

    #[test]
    fn test_double_dash_separator() {
        let options = standard_options();
        let args = vec![
            OsString::from("test-cmd"),
            OsString::from("--help"),
            OsString::from("--"),
            OsString::from("--not-an-option"),
            OsString::from("file.txt"),
        ];
        
        let result = parse_command_line(&options, args.into_iter()).unwrap();
        assert!(result.is_option_set("help"));
        assert_eq!(result.arguments.len(), 2);
        assert_eq!(result.arguments[0], OsString::from("--not-an-option"));
        assert_eq!(result.arguments[1], OsString::from("file.txt"));
    }

    #[test]
    fn test_option_value_or_default() {
        let mut cmd = ParsedCommand {
            executable_path: OsString::from("test"),
            options: HashMap::new(),
            arguments: Vec::new(),
        };
        
        cmd.options.insert("test".to_string(), Some(OsString::from("value")));
        
        assert_eq!(
            cmd.option_value_or_default("test", OsStr::new("default")),
            OsStr::new("value")
        );
        assert_eq!(
            cmd.option_value_or_default("missing", OsStr::new("default")),
            OsStr::new("default")
        );
    }

    #[test]
    fn test_generate_help() {
        let options = standard_options();
        let help = generate_help("testcmd", "Test command", "[OPTIONS] files...", &options);
        
        assert!(help.contains("testcmd"));
        assert!(help.contains("Test command"));
        assert!(help.contains("--help"));
        assert!(help.contains("--version"));
    }

    #[test]
    fn test_validate_positive_integer() {
        assert!(validate_positive_integer(OsStr::new("42")).is_ok());
        assert!(validate_positive_integer(OsStr::new("1")).is_ok());
        assert!(validate_positive_integer(OsStr::new("0")).is_err());
        assert!(validate_positive_integer(OsStr::new("-1")).is_err());
        assert!(validate_positive_integer(OsStr::new("abc")).is_err());
    }

    #[test]
    fn test_validate_existing_file() {
        // Test with a file that should exist (current dir)
        assert!(validate_existing_file(OsStr::new(".")).is_ok());
        
        // Test with a file that shouldn't exist
        assert!(validate_existing_file(OsStr::new("/nonexistent/file/path")).is_err());
    }
}
