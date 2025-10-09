# Command Syntax for Sharutils Commands

Commands in GNU Sharutils follow the same standard used by many other Gnu and Linux commands. There is the command path itself, followed by options, followed finally by a list of arguments, which in the case of Sharutils are typically filenames to operate on. Options are specified by doubled hyphens and their name or by a single hyphen and the flag character.

Options can have values associated with them, and the values are specified differently when you are using a flag and when you are using the option name. When using the flag to specify an option with a value, the option is separated from the flag by whitespace. When using the name to specify the option, the value is separated from the name by an "=" character. For example, if we have an option with a flag of "f" and a name of "file" that requires a value, we coould specify it either as "-f FILENAME" or as "--file=FILENAME".

For example, the syntax for the uuencode command from its manpage is:

```
Usage:  uuencode [ -<flag> | --<name> ]... [<in-file>] <output-name>

   -m, --base64               convert using base64
   -e, --encode-file-name     encode the output file name
   -v, --version[=MODE]       output version information and exit
   -h, --help                 display extended usage information and exit
   -!, --more-help            extended usage information passed thru pager
   -R, --save-opts[=FILE]     save the option state to a config file FILE
   -r, --load-opts=FILE       load options from the config file FILE
                                - disabled with '--no-load-opts'
                                - may appear multiple times
```

For this Rust port of the GNU Sharutils, our goal is not to duplicate the full functionality of getopts and the other libraries used in the C source, but to provide a pragmatic implementation that supports the primary, documented command line capabilities.

We do want to support one documented convenience, though, and this is the ability to combine flags. This convenient notation allows a series of flags to be defined with a single dash proceeding it. For example, a command with the options "-t -e -a" could be specified on the command line as "-tea". In this notation, only the last flag in the list of flags can have an option value (e.g., "-tea <a_option_value>")

When the "main" function of our command line program is invoked, it will use `std::env::args_os()` to discover the command path and other command line arguments, and then parse them into a well-defined structure that our command code can use to discover what options should be used in processing.

## Option Validation
Options may also be assigned a validator that validates the string value provided on the command line for that option.  It uses a type of...

```
pub type OptionValidator = fn(&OsStr) -> Result<(), ValidationError>;
```
*(Note: This type has already been added to the core code)*

With the `ValidationError` newtype defined as...

```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub Cow<'static, str>);

impl ValidationError {
    pub fn new<S: Into<Cow<'static, str>>>(msg: S) -> Self {
        ValidationError(msg.into())
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ValidationError {}
```
*(Note: This struct has already been added to the core code)*

## Option and Command Data Model
We will need a couple structures to support parsing of the command line.

The first is a structure that holds the definition for a command option. 

### struct: OptionDefinition

| Field name     | Type                      | Description                                                                                                      |
|----------------|---------------------------|------------------------------------------------------------------------------------------------------------------|
| `flag`         | `char`                    | The flag character to use to represent the option.                                                               |
| `name`         | `String`                  | The name of the option (lowercase letters only, no whitespace allowed).                                          |
| `hasValue`     | `bool`                    | True if the option can have a value associated with it.                                                          |
| `defaultValue` | `Option<OsString>`        | The value to use if no value is specified. If None, then the value is required.                                  |
| `validator`    | `Option<OptionValidator>` | An optional validator that will be used to validate a value. If not provided, then all input is considered valid.|

The next is a structure that holds the information from a fully parsed command line.

### struct: Command

| Field name     | Type                          | Description                                                                 |
|----------------|-------------------------------|-----------------------------------------------------------------------------|
| `path`         | `OsStr`                       | The path to the command executable.                                         |
| `options`      | `Map<String,Option<OsStr>>`   | A map of option names to option values provided.                            |
| `args`         | `Vec<OsStr>`                  | The arguments from the command line that follow the options (0 or more).    |


Each individual command in this library will use shared components from the core library to parse the command line into a `Command` object. This entails:

- Constructing a `Vec<OptionDefinition>` containing definitions of each option supported by the command (including validators where appripriate).
- Passing this Vec to a parsing function that has the following signature: `fn ParseCommandLine(options: Vec<OptionDefinition>) -> Result<Command, ValidationError>`

With the command line parsed and validated, the command can now interogate the command using the struct fields and the following methods associated with the `Command` struct.

```
// returns true if option was specified, false otherwise
optionSet(name: &str) -> bool

// tests whether there is a value associated with an option
hasOptionValue(name: &str) -> bool

// returns the value associated with an option
optionValue(name: &str) -> Option<OsStr>
```
