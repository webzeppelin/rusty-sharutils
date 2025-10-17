use std::ffi::OsString;
use std::process;
use sharutils_core::{
    OptionDefinition, standard_options, parse_command_line, 
    generate_help, validate_version_mode, validate_file_path,
    handle_version_output, handle_more_help, print_config_file_options
};
#[cfg(debug_assertions)]
use sharutils_core::debug_print_parsed_command;

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
    
    // Debug output the parsed command (only in debug builds)
    #[cfg(debug_assertions)]
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
    
    // Handle save-opts and load-opts if specified
    print_config_file_options(&parsed);

    // Parse options for encoding behavior  
    let use_base64 = parsed.is_option_set("base64");
    let encode_filename = parsed.is_option_set("encode-file-name");
    
    // Determine input source and output filename
    let (input_file, output_name) = match parsed.arguments.len() {
        1 => {
            // Read from stdin, output name is first argument
            (None, &parsed.arguments[0])
        },
        2 => {
            // Read from file, output name is second argument  
            (Some(&parsed.arguments[0]), &parsed.arguments[1])
        },
        _ => unreachable!()
    };
    
    // Get file mode (permissions) - default to 644 for stdin
    let file_mode = if let Some(input_path) = input_file {
        // Try to get actual file permissions
        match std::fs::metadata(input_path) {
            Ok(metadata) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    metadata.permissions().mode() & 0o777
                }
                #[cfg(not(unix))]
                {
                    0o644 // Default mode for non-Unix systems
                }
            }
            Err(e) => {
                eprintln!("Error accessing input file {:?}: {}", input_path, e);
                process::exit(1);
            }
        }
    } else {
        0o644 // Default mode for stdin
    };
    
    // Open input source
    let mut input: Box<dyn std::io::Read> = if let Some(input_path) = input_file {
        match std::fs::File::open(input_path) {
            Ok(file) => Box::new(file),
            Err(e) => {
                eprintln!("Error opening input file {:?}: {}", input_path, e);
                process::exit(1);
            }
        }
    } else {
        Box::new(std::io::stdin())
    };
    
    let mut output = std::io::stdout();
    
    // Write header
    let output_name_str = output_name.to_string_lossy();
    if let Err(e) = sharutils_core::write_uuencode_header(
        &mut output, 
        file_mode, 
        &output_name_str, 
        use_base64,
        encode_filename
    ) {
        eprintln!("Error writing header: {}", e);
        process::exit(1);
    }
    
    // Encode the data
    if let Err(e) = sharutils_core::encode(&mut input, &mut output, use_base64) {
        eprintln!("Error during encoding: {}", e);
        process::exit(1);
    }
    
    // Write trailer
    if let Err(e) = sharutils_core::write_uuencode_trailer(&mut output, use_base64) {
        eprintln!("Error writing trailer: {}", e);
        process::exit(1);
    }
    
    Ok(())
}
