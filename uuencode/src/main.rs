use std::ffi::OsString;
use std::process;
use sharutils_core::{
    OptionDefinition, standard_options, parse_command_line, 
    generate_help, validate_version_mode, validate_file_path,
    handle_version_output, handle_more_help, debug_print_parsed_command,
    print_config_file_options
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
        handle_more_help(
            "uuencode",
            "Encode a file into email-friendly text",
            "[OPTIONS] [input-file] output-name",
            &options
        );
        return Ok(());
    }
    
    if parsed.is_option_set("version") {
        handle_version_output(parsed.option_value("version"), "uuencode");
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
    
    // Handle save-opts and load-opts if specified
    print_config_file_options(&parsed);
    
    Ok(())
}
