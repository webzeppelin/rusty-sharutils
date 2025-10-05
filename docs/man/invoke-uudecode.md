# uudecode — decode an encoded file

**Program:** `uudecode` (GNU sharutils)  
**Purpose:** Transform uuencoded files back into their original form.

---

## Synopsis

```
uudecode [ -<flag> [<val>] | --<name>[{=| @}<val>] ]... [<file>...]
```

- If no `<file>` arguments are provided, standard input is decoded.
- Input may be one or more encoded files given on the command line, or data read from standard input.
- The output file name is taken from the encoded header, but can be overridden with `-o`/`--output-file`.
- The output file’s mode is derived from the original file, except that **setuid** and **execute** bits are *not* retained.
- If the output file is specified as `/dev/stdout` or `-`, the decoded data are written to standard output. When decoding multiple inputs, if a subsequent file also specifies standard output, it will write to the *same* stream as the previous output — **don’t do that**.

---

## Description

`uudecode` processes text produced by uuencoding tools and reconstructs the original binary data.

- It ignores any leading or trailing lines and starts decoding at the first line beginning with `begin`, continuing until the end-of-encoding marker is found.
- The header line indicates which of the two supported encoding schemes was used and whether the output file name itself has been base64-encoded. (See `uuencode(5)`.)

This documentation reflects the AutoGen-generated “invoke man” content for `uudecode` (GNU sharutils), released under the GNU General Public License, version 3 or later.

---

## Help / Usage (`--help`, `--more-help`)

The same usage text is printed for `--help` and `--more-help`.  
`--more-help` passes the text through a pager (disabled on platforms without a working `fork(2)`). The pager program is chosen via the `PAGER` environment variable (default: `more`). Both exit with status code **0**.

```
uudecode (GNU sharutils) - decode an encoded file
Usage:  uudecode [ -<flag> [<val>] | --<name>[{=| @}<val>] ]... [<file>...]

   -o, --output-file=str      direct output to file
   -c, --ignore-chmod         ignore fchmod(3P) errors
   -v, --version[=MODE]       output version information and exit
   -h, --help                 display extended usage information and exit
   -!, --more-help            extended usage information passed thru pager
   -R, --save-opts[=FILE]     save the option state to a config file FILE
   -r, --load-opts=FILE       load options from the config file FILE
                                - disabled with '--no-load-opts'
                                - may appear multiple times

Options are specified by doubled hyphens and their name or by a single
hyphen and the flag character.
If no 'file'(s) are provided, then standard input is decoded.

The following option preset mechanisms are supported:
 - reading file $HOME/.sharrc

'uudecode' transforms uuencoded files into their original form.

The encoded file(s) may be specified on the command line, or one may be
read from standard input.  The output file name is specified in the encoded
file, but may be overridden with the '-o' option.  It will have the mode of
the original file, except that setuid and execute bits are not retained.  If
the output file is specified to be '/dev/stdout' or '-', the result will be
written to standard output.  If there are multiple input files and the
second or subsquent file specifies standard output, the decoded data will
be written to the same file as the previous output.  Don't do that.

'uudecode' ignores any leading and trailing lines.  It looks for a line
that starts with 'begin' and proceeds until the end-of-encoding marker is
found.  The program determines from the header line of the encoded file
which of the two supported encoding schemes was used and whether or not the
output file name has been encoded with base64 encoding.  See 'uuencode(5)'.

Please send bug reports to:  <bug-gnu-utils@gnu.org>
```

---

## Options

### `-o`, `--output-file=<file>` — direct output to file
Takes a string argument `file`. If specified, decoded data are written to this file.

- When multiple inputs are given on the command line, this option **cannot** be used; all decoded data must go to the filename encoded within each data stream.

### `-c`, `--ignore-chmod` — ignore `fchmod(3P)` errors
By default, if the output file permissions cannot be set to those specified in the encoded data, the file is not written and execution stops. With this option, that error is **ignored**: the file data are written but the mode may be incorrect.

- `fchmod()` errors are also ignored if the environment variable `POSIXLY_CORRECT` is set. See: <http://austingroupbugs.net/view.php?id=635>.
- A warning is always emitted when `fchmod()` fails.

### `-v`, `--version[=MODE]` — print version and exit
Print program version to standard output (exit **0**). The optional `MODE` controls licensing detail (only the first letter is examined):

- `version` — print only the version
- `copyright` — print license name *(default)*
- `verbose` — print full copyright usage licensing terms

### `-h`, `--help`
Display extended usage information and exit **0**.

### `-!`, `--more-help`
Display usage via a pager and exit **0** (see **Help / Usage** above).

### `-R`, `--save-opts[=FILE]`
Save current option state to configuration file `FILE`.

### `-r`, `--load-opts=FILE`
Load options from configuration file `FILE`.

- Disabled with `--no-load-opts`.
- May appear multiple times.

---

## Configuration / Presetting

Any option not marked “not presettable” may be preset from configuration (“rc”/“ini”) files.

- `libopts` searches in `$HOME` for configuration data. The `HOME` environment variable is expanded when the program runs.
- If `$HOME` is a **file**, it is processed directly.
- If `$HOME` is a **directory**, a file named `~/.sharrc` is searched for within that directory.

**Formats:**
- Basic: option name followed by a value on the same line. Separator may be **space**, `:`, or `=`.
- Values may continue across multiple lines by escaping the newline with a backslash.
- Multiple programs can share one initialization file. Common options are collected at the top; program-specific segments follow and are separated by either:

```ini
[UUDECODE]
```

or

```
<?program uudecode>
```

*(Do not mix these styles in the same file.)*

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

`AutoOpts` does not track suboptions; treat them as hierarchical values. It provides a means to search the associated name/value list (see: `optionFindValue`).

**Related help/config options:** see `--version`, `--help`, `--more-help`, `--save-opts`, `--load-opts` above.

---

## Exit Status

One of the following values is returned:

| Code | Name               | Meaning                                                                                                   |
|-----:|--------------------|-----------------------------------------------------------------------------------------------------------|
| 0    | `EXIT_SUCCESS`     | Successful program execution.                                                                             |
| 1    | `EXIT_OPTION_ERROR`| The command options were misconfigured.                                                                    |
| 2    | `EXIT_INVALID`     | *(warning)* One or more input files contained no valid data.                                              |
| 4    | `EXIT_NO_INPUT`    | *(warning)* The specified input file was not found.                                                        |
| 8    | `EXIT_NO_OUTPUT`   | The specified output file could not be created *(error)*; or an output file could not be written or its access mode changed *(warnings)*. The accompanying messages distinguish. |
| 9    | `EXIT_NO_MEM`      | No process memory available.                                                                              |
| 66   | `EX_NOINPUT`       | A specified configuration file could not be loaded.                                                       |
| 70   | `EX_SOFTWARE`      | `libopts` had an internal operational error. Report to `autogen-users@lists.sourceforge.net`.             |

---

## Bugs

Please include `sharutils` in the subject line of emailed bug reports; it helps identify the message.

If more than one encoded `name` is the same across inputs, or if a second (or later) input specifies standard output as the destination, the result is likely **not** what you expect: standard output will be appended to, and named output files may be replaced.

---

## Standards

This implementation is compliant with **P1003.2b/D11**.

---

## See Also

- `uuencode(1)`
- `uuencode(5)`