use std::ffi::OsString;
use std::process;
use sharutils_core::{
    OptionDefinition, ParsedCommand, standard_options, parse_command_line, 
    generate_help, validate_version_mode, validate_file_path
};

/// Returns uuencode-specific command line options
fn uuencode_options() -> Vec<OptionDefinition> {
    vec![
        OptionDefinition {
            flag: 'm',
            name: "base64".to_string(),
            has_value: false,
            default_value: None,
            validator: None,
            help_text: "Convert using base64 instead of traditional uuencoding".to_string(),
        },
        OptionDefinition {
            flag: 'e',
            name: "encode-file-name".to_string(),
            has_value: false,
            default_value: None,
            validator: None,
            help_text: "Encode the output file name".to_string(),
        },
        OptionDefinition {
            flag: 'v',
            name: "version".to_string(),
            has_value: true,
            default_value: Some(OsString::from("copyright")),  // Only when -v is specified without value
            validator: Some(validate_version_mode),
            help_text: "Output version information and exit [=MODE]".to_string(),
        },
        OptionDefinition {
            flag: '!',
            name: "more-help".to_string(),
            has_value: false,
            default_value: None,
            validator: None,
            help_text: "Extended usage information passed through pager".to_string(),
        },
        OptionDefinition {
            flag: 'R',
            name: "save-opts".to_string(),
            has_value: true,
            default_value: None,  // No automatic default - only when explicitly specified
            validator: Some(validate_file_path),
            help_text: "Save the option state to a config file [=FILE]".to_string(),
        },
        OptionDefinition {
            flag: 'r',
            name: "load-opts".to_string(),
            has_value: true,
            default_value: None,
            validator: Some(validate_file_path),
            help_text: "Load options from the config file FILE".to_string(),
        },
    ]
}

/// Debug print the parsed command structure
fn debug_print_parsed_command(parsed: &ParsedCommand) {
    eprintln!("DEBUG: Parsed Command Structure:");
    eprintln!("  Executable: {:?}", parsed.executable_path);
    eprintln!("  Options:");
    for (name, value) in &parsed.options {
        match value {
            Some(v) => eprintln!("    --{} = {:?}", name, v),
            None => eprintln!("    --{}", name),
        }
    }
    eprintln!("  Arguments:");
    for (i, arg) in parsed.arguments.iter().enumerate() {
        eprintln!("    [{}]: {:?}", i, arg);
    }
    eprintln!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Combine standard options with uuencode-specific options
    let mut options = standard_options();
    
    // Remove the standard version option since uuencode has special version handling
    options.retain(|opt| opt.name != "version");
    
    // Add uuencode-specific options including custom version option
    options.extend(uuencode_options());
    
    // Parse command line arguments
    let args: Vec<OsString> = std::env::args_os().collect();
    let parsed = match parse_command_line(&options, args.into_iter()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nUse --help for usage information.");
            process::exit(1);
        }
    };
    
    // Debug output the parsed command
    debug_print_parsed_command(&parsed);
    
    // Handle special options that cause immediate exit
    if parsed.is_option_set("help") {
        let help_text = generate_help(
            "uuencode",
            "Encode a file into email-friendly text",
            "[OPTIONS] [input-file] output-name",
            &options
        );
        println!("{}", help_text);
        return Ok(());
    }
    
    if parsed.is_option_set("more-help") {
        let help_text = generate_help(
            "uuencode",
            "Encode a file into email-friendly text",
            "[OPTIONS] [input-file] output-name",
            &options
        );
        
        // Try to use a pager, fall back to direct output
        match std::process::Command::new("less")
            .arg("-F")  // exit if content fits on one screen
            .arg("-R")  // allow ANSI color codes
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                use std::io::Write;
                if let Some(stdin) = child.stdin.take() {
                    let mut stdin = stdin;
                    let _ = stdin.write_all(help_text.as_bytes());
                }
                let _ = child.wait();
            }
            Err(_) => {
                // Fall back to direct output if pager fails
                println!("{}", help_text);
            }
        }
        return Ok(());
    }
    
    if parsed.is_option_set("version") {
        let version_mode = parsed.option_value("version")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "copyright".into());  // Default to copyright when no value provided
        
        match version_mode.to_lowercase().chars().next().unwrap_or('c') {
            'v' => {
                // Just version
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            },
            'c' => {
                // Copyright info (default)
                println!("{} {} (rusty-sharutils)", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                println!("Copyright (C) 2025 rusty-sharutils contributors");
                println!("This is free software; see the source for copying conditions.");
                println!("There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A");
                println!("PARTICULAR PURPOSE.");
            },
            _ => {
                // Verbose - full licensing terms
                println!("{} {} (rusty-sharutils)", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                println!("Copyright (C) 2025 rusty-sharutils contributors");
                println!();
                println!("This program is free software: you can redistribute it and/or modify");
                println!("it under the terms of the GNU General Public License as published by");
                println!("the Free Software Foundation, either version 3 of the License, or");
                println!("(at your option) any later version.");
                println!();
                println!("This program is distributed in the hope that it will be useful,");
                println!("but WITHOUT ANY WARRANTY; without even the implied warranty of");
                println!("MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the");
                println!("GNU General Public License for more details.");
                println!();
                println!("You should have received a copy of the GNU General Public License");
                println!("along with this program.  If not, see <https://www.gnu.org/licenses/>.");
            }
        }
        return Ok(());
    }
    
    // Validate argument count - uuencode requires at least output-name
    if parsed.arguments.is_empty() {
        eprintln!("Error: Missing required output-name argument");
        eprintln!("Usage: uuencode [OPTIONS] [input-file] output-name");
        process::exit(1);
    }
    
    if parsed.arguments.len() > 2 {
        eprintln!("Error: Too many arguments provided");
        eprintln!("Usage: uuencode [OPTIONS] [input-file] output-name");
        process::exit(1);
    }
    
    // At this point we would implement the actual uuencode functionality
    // For now, just indicate that parsing was successful
    println!("uuencode: Command line parsing completed successfully!");
    println!("This is where the actual encoding logic would be implemented.");
    
    // Print the configuration for verification
    if parsed.is_option_set("base64") {
        println!("Configuration: Using base64 encoding");
    } else {
        println!("Configuration: Using traditional uuencoding");
    }
    
    if parsed.is_option_set("encode-file-name") {
        println!("Configuration: Output filename will be encoded");
    }
    
    match parsed.arguments.len() {
        1 => {
            println!("Input: Reading from standard input");
            println!("Output name: {:?}", parsed.arguments[0]);
        },
        2 => {
            println!("Input file: {:?}", parsed.arguments[0]);
            println!("Output name: {:?}", parsed.arguments[1]);
        },
        _ => unreachable!()
    }
    
    Ok(())
}
