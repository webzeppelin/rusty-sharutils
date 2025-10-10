use std::ffi::OsString;
use std::process;
use sharutils_core::{
    OptionDefinition, standard_options, parse_command_line, 
    generate_help, validate_version_mode, validate_file_path,
    handle_version_output, handle_more_help, debug_print_parsed_command,
    print_config_file_options
};

/// Returns uudecode-specific command line options
fn uudecode_options() -> Vec<OptionDefinition> {
    vec![
        OptionDefinition {
            flag: 'o',
            name: "output-file".to_string(),
            has_value: true,
            default_value: None,
            validator: Some(validate_file_path),
            help_text: "Direct output to file".to_string(),
        },
        OptionDefinition {
            flag: 'c',
            name: "ignore-chmod".to_string(),
            has_value: false,
            default_value: None,
            validator: None,
            help_text: "Ignore fchmod(3P) errors".to_string(),
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
    // Combine standard options with uudecode-specific options
    let mut options = standard_options();
    
    // Remove the standard version option since uudecode has special version handling
    options.retain(|opt| opt.name != "version");
    
    // Add uudecode-specific options including custom version option
    options.extend(uudecode_options());
    
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
            "uudecode",
            "Decode an encoded file",
            "[OPTIONS] [input-file...]",
            &options
        );
        println!("{}", help_text);
        return Ok(());
    }
    
    if parsed.is_option_set("more-help") {
        handle_more_help(
            "uudecode",
            "Decode an encoded file",
            "[OPTIONS] [input-file...]",
            &options
        );
        return Ok(());
    }
    
    if parsed.is_option_set("version") {
        handle_version_output(parsed.option_value("version"), "uudecode");
        return Ok(());
    }
    
    // Validate output-file option usage
    if parsed.is_option_set("output-file") && parsed.arguments.len() > 1 {
        eprintln!("Error: --output-file cannot be used when multiple input files are provided");
        eprintln!("When decoding multiple files, each must specify its own output filename in the encoded data");
        process::exit(1);
    }
    
    // At this point we would implement the actual uudecode functionality
    // For now, just indicate that parsing was successful
    println!("uudecode: Command line parsing completed successfully!");
    println!("This is where the actual decoding logic would be implemented.");
    
    // Print the configuration for verification
    if parsed.is_option_set("ignore-chmod") {
        println!("Configuration: Will ignore fchmod() errors");
    } else {
        println!("Configuration: Will respect file permission errors");
    }
    
    if let Some(output_file) = parsed.option_value("output-file") {
        println!("Configuration: Output will be written to: {:?}", output_file);
    } else {
        println!("Configuration: Output filename will be taken from encoded data");
    }
    
    // Print input file information
    if parsed.arguments.is_empty() {
        println!("Input: Reading from standard input");
    } else {
        println!("Input files ({}):", parsed.arguments.len());
        for (i, file) in parsed.arguments.iter().enumerate() {
            println!("  [{}]: {:?}", i + 1, file);
        }
    }
    
    // Handle save-opts and load-opts if specified
    print_config_file_options(&parsed);
    
    Ok(())
}
