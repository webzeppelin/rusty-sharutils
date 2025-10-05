# uuencode — encode a file into email-friendly text

**Program:** `uuencode` (GNU sharutils)  
**Purpose:** Create an ASCII representation of a file that can be safely sent over channels (e.g., email) that may corrupt binary data.

---

## Synopsis

```
uuencode [ -<flag> | --<name> ]... [<in-file>] <output-name>
```

- If `<in-file>` is supplied, `uuencode` reads from that file; otherwise it reads from standard input.
- The encoded output is written to standard output and begins with a header for use by `uudecode`, providing the suggested `<output-name>` and access mode.
- If `<output-name>` is `/dev/stdout`, then `uudecode` will emit the decoded file to standard output.

---

## Description

`uuencode` converts a file into a 7-bit ASCII form suitable for transmission through media (notably email) that cannot reliably carry binary data and may, for example, insert a character when the six-character sequence `\nFrom ` is seen.

**Note:** `uuencode` uses buffered input and assumes it is not being hand-typed at a TTY. At a TTY, you may need to press **Ctrl-D** several times to terminate input.

This documentation reflects the AutoGen-generated “invoke man” content for `uuencode` (GNU sharutils), released under the GNU General Public License, version 3 or later.

---

## Help / Usage (`--help`, `--more-help`)

The same usage text is printed for `--help` and `--more-help`.  
`--more-help` passes the text through a pager (disabled on platforms without a working `fork(2)`). The pager is selected via the `PAGER` environment variable (default: `more`). Both exit with status code **0**.

```
uuencode (GNU sharutils) - encode a file into email friendly text
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

Options are specified by doubled hyphens and their name or by a single
hyphen and the flag character.

The following option preset mechanisms are supported:
 - reading file $HOME/.sharrc
```

Please send bug reports to: `<bug-gnu-utils@gnu.org>`.

---

## Options

### `-m`, `--base64` — convert using base64
By default, `uuencode` uses the traditional uuencoding conversion, which is slower and less compact than base64.  
Expansion factors:
- **UU encoding:** input expands by ~37% (3 bytes → 4 chars + control info)
- **Base64 encoding:** input expands by ~35% (3 bytes → 4 chars + control info)

### `-e`, `--encode-file-name` — encode the output file name
Transmissions may mishandle certain filename characters. This option base64-encodes the `output-name` in the header.  
*(Traditional uuencoding of the filename itself is not supported.)*

### `-v`, `--version[=MODE]` — print version and exit
Print program version to standard output (exit **0**). The optional `MODE` controls the amount of licensing info shown (only the first letter is examined):

- `version` — print only the version  
- `copyright` — print copyright/licensing name *(default)*  
- `verbose` — print full copyright usage licensing terms

### `-h`, `--help`
Display extended usage information and exit **0**.

### `-!`, `--more-help`
Display usage via a pager and exit **0** (see **Help / Usage** above).

### `-R`, `--save-opts[=FILE]`
Save current option state to a config file `FILE`.

### `-r`, `--load-opts=FILE`
Load options from config file `FILE`.  
- Disabled with `--no-load-opts`.  
- May appear multiple times.

---

## Configuration / Presetting

Any option not marked “not presettable” may be preset from configuration (“rc”/“ini”) files.

- `libopts` searches in `$HOME` for configuration (option) data. The `HOME` environment variable is expanded when the program runs.
- If `$HOME` is a **file**, it is processed directly.
- If `$HOME` is a **directory**, `~/.sharrc` is searched for within that directory.

**Formats:**
- Basic: `option-name` followed by a value on the same line. Separator may be **space**, `:`, or `=`.
- Values may continue across lines by escaping the newline with a backslash.
- Multiple programs may share one initialization file. Common options appear at the top, followed by program-specific segments separated by either:

```ini
[UUENCODE]
```

or

```
<?program uuencode>
```

*(Do not mix these styles in one file.)*

**XML-style compound values:**

```xml
<option-name>
   <sub-opt>...&lt;...&gt;...</sub-opt>
</option-name>
```

This yields an `option-name.sub-opt` string value of:

```
"...<...>..."
```

`AutoOpts` does not track suboptions; treat them as hierarchical values. It provides a search facility over name/value pairs (see `optionFindValue`).

---

## Exit Status

One of the following values is returned:

| Code | Name             | Meaning                                                                 |
|-----:|------------------|-------------------------------------------------------------------------|
| 0    | `EXIT_SUCCESS`   | Successful program execution.                                           |
| 1    | `EXIT_FAILURE`   | Operation failed or command syntax was not valid.                       |
| 66   | `EX_NOINPUT`     | A specified configuration file could not be loaded.                     |
| 70   | `EX_SOFTWARE`    | `libopts` encountered an internal operational error. Report to `autogen-users@lists.sourceforge.net`. |

---

## Bugs

Please include `sharutils` in the email subject when reporting bugs; it helps triage the message.

---

## Standards

This implementation is compliant with **P1003.2b/D11**.

---

## History

The `uuencode` command first appeared in **BSD 4.0**.

---

## See Also

- `uudecode(1)`
- `uuencode(5)`
