# unshar — unpack a shar archive

**Program:** `unshar` (GNU sharutils)  
**Purpose:** Scan input for shell archives (shars) and unpack them by invoking the system shell.

---

## Synopsis

```
unshar [ -<flag> [<val>] | --<name>[{=| @}<val>] ]... [<file>...]
```

- If no files are given, **standard input** is processed.  
- Each archive discovered in the input is passed to the shell to be unpacked.

This documentation reflects the AutoGen-generated “invoke man” content for `unshar` (GNU sharutils), released under the GNU General Public License, version 3 or later.

---

## Help / Usage (`--help`, `--more-help`)

The same usage text is printed for `--help` and `--more-help`.  
`--more-help` passes the text through a pager (disabled on platforms without a working `fork(2)`). The pager is chosen via the `PAGER` environment variable (default: `more`). Both exit with status code **0**.

```
unshar (GNU sharutils) - unpack a shar archive
Usage:  unshar [ -<flag> [<val>] | --<name>[{=| @}<val>] ]... [<file>...]

   -d, --directory=DIR        unpack into the directory DIR
   -c, --overwrite            overwrite any pre-existing files
   -f, --force                an alias for the 'overwrite' option
   -E, --split-at=SPLIT-PAT   split input on SPLIT-PAT lines
   -e, --exit-0               split input on "exit 0" lines
                                - prohibits the option 'split-at'
   -D, --debug                debug the shell code
   -v, --version[=MODE]       output version information and exit
   -h, --help                 display extended usage information and exit
   -!, --more-help            extended usage information passed thru pager
   -R, --save-opts[=FILE]     save the option state to the config file FILE
   -r, --load-opts=FILE       load options from the config file FILE
                                - disabled as '--no-load-opts'
                                - may appear multiple times

Options are specified by doubled hyphens and their name or by a single
hyphen and the flag character.

If no arguments are provided, input arguments are read from stdin,
one per line; blank and '#'-prefixed lines are comments.
'standin' may not be a terminal (tty).

The following option preset mechanisms are supported:
 - reading file $HOME/.sharrc

'unshar' scans the input files (typically email messages) looking for the
start of a shell archive.  If no files are given, then standard input is
processed instead.  It then passes each archive discovered through an
invocation of the shell program to unpack it.

Please send bug reports to:  <bug-gnu-utils@gnu.org>
```

---

## Options

### `-d`, `--directory=DIR` — unpack into directory `DIR`
Takes a string argument `DIR`. Input filenames are interpreted relative to the directory where the program was started. This option inserts a `cd <dir>` command at the start of the `shar` text fed to the shell.

### `-c`, `--overwrite` — overwrite any pre-existing files
Passes a `-c` flag to the archive script so that existing files are overwritten.

### `-f`, `--force` — alias for `--overwrite`
Equivalent to `--overwrite`.

### `-E`, `--split-at=SPLIT-PAT` — split input on matching lines
Takes a string argument `SPLIT-PAT`. Isolates each archive placed in the same file and unpacks each in turn, relying on an identifiable terminator line in typical shars.

**Example:** Since many signatures begin with `--` on a line by itself, `--split-at=--` will often skip signatures and the following mail headers.

### `-e`, `--exit-0` — split input on `"exit 0"` lines
*Usage constraint:* must **not** be combined with `--split-at`.

Most shell archives end with a line that is exactly `exit 0`. This option is equivalent to `--split-at="exit 0"`.

### `-D`, `--debug` — debug the shell code
Emit `set -x` into the script interpreted by the shell to trace execution.

### `-v`, `--version[=MODE]` — print version and exit
Print program version to stdout (exit **0**). Optional `MODE` selects license verbosity (first letter examined): `version`, `copyright` *(default)*, or `verbose`.

### `-h`, `--help` — show usage and exit

### `-!`, `--more-help` — show usage via pager and exit

### `-R`, `--save-opts[=FILE]` — save current options to config file `FILE`

### `-r`, `--load-opts=FILE` — load options from config file `FILE`
Disabled with `--no-load-opts`. May appear multiple times.

---

## Presetting / Configuration

Any option not marked “not presettable” may be preset from configuration (`rc`/`ini`) files.

- `libopts` searches `$HOME` for configuration data (expanding `HOME` at runtime).  
- If `$HOME` is a **file**, it is processed directly. If a **directory**, `~/.sharrc` is sought within it.

**Formats:**
- Basic: `option-name` followed by a value on the same line (separator may be space, `:`, or `=`).  
- Values may span multiple lines by escaping the newline with `\`.  
- Multiple programs may share a single file. Common options first; program-specific segments follow and are separated by either:

```ini
[UNSHAR]
```

or

```
<?program unshar>
```

*(Do not mix these styles in one file.)*

**XML-style compound values:**

```xml
<option-name>
   <sub-opt>...&lt;...&gt;...</sub-opt>
</option-name>
```

yields an `option-name.sub-opt` string value of:

```
"...<...>..."
```

`AutoOpts` does not track suboptions; treat them as hierarchical values. It provides a search facility over the associated name/value list (`optionFindValue`).

---

## Exit status

One of the following values is returned:

| Code | Name                    | Meaning                                                |
|-----:|-------------------------|--------------------------------------------------------|
| 0    | `EXIT_SUCCESS`          | Successful program execution.                          |
| 1    | `EXIT_FAILURE`          | Error in command usage.                                |
| 2    | `EXIT_POPEN_PROBLEM`    | Cannot spawn or write to a shell process.              |
| 3    | `EXIT_CANNOT_CREATE`    | Cannot create output file.                             |
| 4    | `EXIT_BAD_DIRECTORY`    | Working directory structure is invalid.                |
| 5    | `EXIT_NOMEM`            | Memory allocation failure.                             |
| 6    | `EXIT_INVALID`          | Invalid input; no shar file found.                     |
| 66   | `EX_NOINPUT`            | A specified configuration file could not be loaded.    |
| 70   | `EX_SOFTWARE`           | `libopts` internal operational error; please report.   |

---

## Authors

The `shar` and `unshar` programs are the collective work of many authors. Contributors reported problems, suggested improvements, and submitted code. See the `THANKS` file in the sharutils distribution.

---

## Bugs

Please include `sharutils` in the subject line for emailed bug reports; it helps spot the message.

---

## See also

- `shar(1)`