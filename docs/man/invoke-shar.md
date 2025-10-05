# shar — create a shell archive

**Program:** `shar` (GNU sharutils)  
**Purpose:** Create “shell archives” (shar files) in text format that can be emailed and later unpacked by executing them with `/bin/sh`.

---

## Synopsis

```
shar [ -<flag> [<val>] | --<name>[{=| @}<val>] ]... [<file>...]
```

- If no `file`s are specified, the list of input files is read from **standard input**. Standard input **must not** be a terminal.
- The resulting archive is written to standard output unless `-o`/`--output-prefix` is given.
- Features provide flexibility for compression, encoding, headers, transmission defenses, and different shar “flavors.” Archives can be simple (`--vanilla-operation`) or essentially a mailable `tar`-like archive.
- Options may be specified in any order until a `file` argument is recognized. With `--intermix-type`, additional compression/encoding options may appear between file arguments.
- Support for `uuencode`d files exists but is **deprecated**. Prefer MIME-encoded email. If you **do** uuencode, base64 is preferred.

This documentation reflects the AutoGen-generated “invoke man” content for `shar` (GNU sharutils), released under the GNU General Public License, version 3 or later.

---

## Help / Usage (`--help`, `--more-help`)

The same usage text is printed for `--help` and `--more-help`.  
`--more-help` passes the text through a pager (disabled on platforms without a working `fork(2)`). Pager is chosen via `PAGER` (default: `more`). Both exit with status **0**.

```
shar (GNU sharutils) - create a shell archive
Usage:  shar [ -<flag> [<val>] | --<name>[{=| @}<val>] ]... [<file>...]

Specify compression:
   -p, --intermix-type        specify compression for input files
                                - prohibits the option 'vanilla-operation'
   -C, --compactor=PROG       specify compaction (compression) program PROG
                                - prohibits the option 'vanilla-operation'
                                - may appear multiple times
                                - it must be known to shar: xz gzip bzip2
   -g, --level-of-compression=LEVEL
                              pass LEVEL for compression
                                - it must be in the range: 1 to 9

Specify file encoding methodology:
   -M, --mixed-uuencode       decide uuencoding for each file
   -B, --uuencode             treat all files as binary
                                - an alternate for mixed-uuencode
   -T, --text-files           treat all files as text
                                - an alternate for mixed-uuencode

Specifying file selection and output modes:
   -o, --output-prefix=str    print output to file PREFIX.nn
   -l, --whole-size-limit=SIZE
                              split archive, not files, to SIZE
                                - requires the option 'output-prefix'
                                - is scalable with a suffix: k/K/m/M/g/G/t/T
                                - it must lie in one of the ranges:
                                  8 to 1023, or 8192 to 4194304
   -L, --split-size-limit=SIZE
                              split archive or files to SIZE
                                - requires the option 'output-prefix'
                                - is scalable with a suffix: k/K/m/M/g/G/t/T
                                - it must lie in one of the ranges:
                                  8 to 1023, or 8192 to 4194304
                                - an alternate for 'whole-size-limit'
   -I, --input-file-list=FILE read file list from FILE

Controlling the shar headers:
   -n, --archive-name=NAME    use NAME to document the archive
   -s, --submitter=NAME       override the submitter name with NAME
   -a, --net-headers          output Submitted-by: & Archive-name: headers
                                - requires the option 'archive-name'
   -c, --cut-mark             start the shar with a cut line
   -t, --translate            translate messages in the script

Protecting against transmission issues:
       --no-character-count   do not use `wc -c' to check size
   -D, --no-md5-digest        do not use md5sum digest to verify
   -F, --force-prefix         apply the prefix character on every line
   -d, --here-delimiter=DELIM use DELIM to delimit the files

Producing different kinds of shars:
   -V, --vanilla-operation    produce very simple shars
   -P, --no-piping            use temporary files between programs
   -x, --no-check-existing    blindly overwrite existing files
   -X, --query-user           ask user before overwriting files
                                - prohibits the option 'vanilla-operation'
   -m, --no-timestamp         do not restore modification times
   -Q, --quiet-unshar         avoid verbose messages at unshar time
   -f, --basename             restore in one directory, despite hierarchy

Internationalization options:
       --no-i18n              do not internationalize
       --print-text-domain-dir  print directory with shar messages

User feedback/entertainment:
   -q, --quiet                do not output verbose messages
       --silent               an alias for the 'quiet' option

Version, usage and configuration options:
   -v, --version[=MODE]       output version information and exit
   -h, --help                 display extended usage information and exit
   -!, --more-help            extended usage information passed thru pager
   -R, --save-opts[=FILE]     save the option state to a config file FILE
   -r, --load-opts=FILE       load options from the config file FILE
                                - disabled with '--no-load-opts'
                                - may appear multiple times

Options are specified by doubled hyphens and their name or by a single
hyphen and the flag character.
If no 'file's are specified, the list of input files is read from a
standard input.  Standard input must not be a terminal.

The following option preset mechanisms are supported:
 - reading file $HOME/.sharrc

'shar' creates "shell archives" (or shar files) which are in text format
and can be emailed.  These files may be unpacked later by executing them
with '/bin/sh'.  The resulting archive is sent to standard out unless the
'-o' option is given.  A wide range of features provide extensive
flexibility in manufacturing shars and in specifying 'shar' "smartness".
Archives may be fairly simple ('--vanilla-operation') or essentially a
mailable 'tar' archive.

Options may be specified in any order until a 'file' argument is
recognized.  If the '--intermix-type' option has been specified, more
compression and encoding options will be recognized between the 'file'
arguments.

Though this program supports 'uuencode'-d files, they are deprecated.  If
you are emailing files, please consider mime-encoded files.  If you do
'uuencode', base64 is the preferred encoding method.

Please send bug reports to:  <bug-gnu-utils@gnu.org>
```

---

## Compression options

### `-p`, `--intermix-type` — specify compression for input files
Allow positional parameter options so compression/encoding options can be intermixed with file names. Files following these options are processed as specified.

*Usage constraints:* must **not** be used with `--vanilla-operation`.

### `-C`, `--compactor=PROGRAM` — specify compaction (compression) program
- May appear multiple times.  
- Must **not** be used with `--vanilla-operation`.

Supported compactors include `xz`, `gzip`, and `bzip2`. (Historical options like `--gzip` are deprecated.) To use `xz`, specify `-C xz` or `--compactor=xz`.

Specify compactor `none` to disable compression. Compressed files are **always** uuencoded; recipients need `uudecode` to unpack. Using `compress` is deprecated.

### `-g`, `--level-of-compression=LEVEL` — pass compression level
Provide a numeric `LEVEL` (commonly 1–9). Used by `gzip`, `bzip2`, and `xz` (not by `compress`). Default is `9`.

### `-j` — **bzip2** and **uuencode** files *(deprecated)*
May appear multiple times. Compress with `bzip2` and uuencode prior to packing. Recipients need `uudecode` and `bzip2`.

### `-z` — **gzip** and **uuencode** files *(deprecated)*
May appear multiple times. Compress with `gzip` and uuencode prior to packing. Recipients need `uudecode` and `gzip`.

### `-Z` — **compress** and **uuencode** files *(deprecated)*
May appear multiple times; must be compiled with `HAVE_COMPRESS`. Recipients need `uudecode` and `compress`.

### `--level-for-gzip` — alias for `--level-of-compression`

### `-b`, `--bits-per-code=BITS` — pass bits (default 12) to `compress` *(deprecated)*
Requires build-time `HAVE_COMPRESS`.

---

## Encoding options

Files can be stored as plain text or uuencoded. By default, `shar` examines each file and decides automatically. In intermixed mode, this can change mid-processing.

### `-M`, `--mixed-uuencode` — decide uuencoding per file *(default)*
Automatically detect text vs. binary; uuencode binary files before packing.

**Text file heuristic:** Consider a file text only if **all** hold:
1. Contains no ASCII control chars except **BS**, **HT**, **LF**, **FF**.  
2. Contains no bytes with the high bit set.  
3. No line begins with `from ` (case-insensitive).  
4. Is empty or ends with a newline (`LF`).  
5. No line exceeds 200 characters (lines are `LF`-separated).

### `-B`, `--uuencode` — treat all files as binary
Member of the mixed-uuencode option class. Always uuencode (larger archives). Recipients need `uudecode`. Compressed files are always encoded.

### `-T`, `--text-files` — treat all files as text
Member of the mixed-uuencode option class. Be cautious with non-ASCII bytes or fragile mailers; for FTP/SSH/SCP transfers, such files are typically fine.

---

## In/out options

### `-o`, `--output-prefix=PREFIX` — print output to `PREFIX.nn`
Write archive parts to files `prefix.01` … `prefix.nn` instead of stdout. Required with `--whole-size-limit` or `--split-size-limit`.

- If `prefix` contains `%`, it is treated as a `sprintf` format that prints a single decimal number. Otherwise `.%02d` is appended internally.

### `-l`, `--whole-size-limit=SIZE` — split archive (not files) to *SIZE*
Member of the **whole-size-limit** option class. Requires `--output-prefix`.

Limit output file size to `SIZE` bytes but **do not** split input files. Values `<1024` are multiplied by 1024. Suffixes: `k`, `K`, `m`, `M` → ×1000/1024/1,000,000/1,048,576. Max is **4M (4194304)**. Recipient may unpack parts in **any order**.

### `-L`, `--split-size-limit=SIZE` — split archive or files to *SIZE*
Member of the **whole-size-limit** option class. Requires `--output-prefix`.

Split at `SIZE`, possibly splitting files. Same value rules as above. Parts must be unpacked in the **correct order**. If collecting parts into one mailbox/file for `unshar`, ensure they are ordered for batch unpacking (see **unshar**).

### `-I`, `--input-file-list=FILE` — read file list from FILE
Reopens `FILE` as standard input. If no files are found on the command line, filenames are read from stdin. Prohibits listing inputs on the command line.

- Input format: one filename per line (like `find … -print`).  
- With `--intermix-type`, compression options may appear on their own lines; no filename may start with `-`.

**Example:**

```sh
{
  echo --compact xz
  find . -type f -print | sort
} | shar -S -p -L50K -o /somewhere/big
```

### `-S`, `--stdin-file-list` — read file list from standard input *(deprecated)*
Equivalent to `--input-file-list=-`.

---

## Header options

### `-n`, `--archive-name=NAME` — document the archive name
Embed `NAME` in headers; used with `--net-headers`.

### `-s`, `--submitter=WHO@WHERE` — override submitter name
By default, `shar` determines the submitter from the system. Use this to specify another identity.

### `-a`, `--net-headers` — output `Submitted-by:` & `Archive-name:` headers
*Usage constraint:* requires `--archive-name`.

Adds headers:

```
Submitted-by: who@where
Archive-name: name/part##
```

- `who@where` is derived or set via `--submitter`.
- `name` is set via `--archive-name`.
- If `name` contains `/`, the `/part##` suffix is omitted.

**Examples:**

```
-n xyzzy           →
  xyzzy/part01
  xyzzy/part02

-n xyzzy/patch    →
  xyzzy/patch01
  xyzzy/patch02

-n xyzzy/patch01. →
  xyzzy/patch01.01
  xyzzy/patch01.02
```

### `-c`, `--cut-mark` — start archive with a “Cut here” line
Adds a “Cut here” line at the start of each output file.

### `-t`, `--translate` — translate messages in the script
If `LANG` is set, messages printed by `shar` will be in that language. By default, generated script messages are in English; this option localizes the script’s messages according to `LANG` at generation time.

---

## Transmission defenses

### `--no-character-count` — do not use `wc -c` to check size
Skips size verification after unpacking (default is to check).

### `-D`, `--no-md5-digest` — do not use `md5sum` to verify
Skips MD5 verification (default is to check).

### `-F`, `--force-prefix` — apply prefix on every line
Prepend the prefix character to every line, even when not required. May slightly increase archive size, especially with `--uuencode` or compression.

### `-d`, `--here-delimiter=DELIM` — use custom delimiter
Use `DELIM` instead of `SHAR_EOF` to delimit files. The delimiter is always wrapped with underscores. For personalization of shar files.

---

## Shar flavors

### `-V`, `--vanilla-operation` — produce very simple shars
Relies only on `echo`, `test`, and `sed` at unpack time. Changes default mode from mixed (`--mixed-uuencode`) to text (`--text-files`). Warns if options require decompression/decoding.

### `-P`, `--no-piping` — use temporary files between programs
Use temp files instead of pipes in the shar. Required for systems that lack pipe support.

### `-x`, `--no-check-existing` — blindly overwrite existing files
Generates archives that overwrite existing files without checking. If neither this nor `--query-user` is used, unpacking will not overwrite files. Passing `-c` to the script during unpacking forces unconditional overwrites.

Example:

```
sh shar-archive-file -c
```

### `-X`, `--query-user` — ask before overwriting files
*Usage constraint:* must **not** be used with `--vanilla-operation`.

Interactively prompt before overwriting during unpack. **Do not** use for net-submitted shars. This can break many `unshar` procedures (competition for stdin). `shar` attempts to use `/dev/tty` for replies if available, but this may still require running `/bin/sh` directly. In vanilla mode, `/dev/tty` is not attempted.

### `-m`, `--no-timestamp` — do not restore modification times
Avoid generating `touch` commands to restore modification dates. Without this, `shar` attempts to restore timestamps so build tools (e.g., `make`) see correct file ages.

### `-Q`, `--quiet-unshar` — avoid verbose messages at unshar time
Suppresses comments in the unpack script.

### `-f`, `--basename` — restore in one directory, ignoring hierarchy
Use only base filenames when restoring. Useful when building a shar from several directories. If a directory is passed to `shar`, its substructure will still be restored regardless of this option.

---

## Internationalization options

### `--no-i18n` — do not internationalize
Produce archives that always print English messages at unpack time.

### `--print-text-domain-dir` — print directory with shar messages
Print the directory where `shar` looks for message catalogs and exit.

---

## Feedback options

### `-q`, `--quiet` — do not output verbose messages
Omit progress messages.

### `--silent` — alias for `--quiet`

---

## Presetting / Configuration

Any option not marked “not presettable” can be preset via configuration (`rc`/`ini`) files.

- `libopts` searches `$HOME` for option data. `HOME` is expanded at runtime.
- If `$HOME` is a **file**, that file is processed. If a **directory**, `~/.sharrc` is searched within it.

**Formats:**
- `option-name` followed by a value on the same line. Separator may be space, `:`, or `=`.
- Values can continue across lines with a trailing backslash.
- Multiple programs may share one file. Common options first, then program-specific segments, separated by either:

```ini
[SHAR]
```

or

```
<?program shar>
```

*(Do not mix these styles.)*

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

`AutoOpts` does not track suboptions; treat values as hierarchical. It provides search over the associated name/value list (see `optionFindValue`).

**Related help/config options:** see `--version`, `--help`, `--more-help`, `--save-opts`, `--load-opts` in the usage block.

---

## Exit status

One of the following values is returned:

| Code | Name                    | Meaning                                                       |
|-----:|-------------------------|---------------------------------------------------------------|
| 0    | `EXIT_SUCCESS`          | Successful program execution.                                |
| 1    | `EXIT_OPTION_ERROR`     | The command options were misconfigured.                      |
| 2    | `EXIT_FILE_NOT_FOUND`   | A specified input could not be found.                        |
| 3    | `EXIT_CANNOT_OPENDIR`   | Opening/closing a specified directory failed.                |
| 4    | `EXIT_FAILED`           | Resource limit / miscellaneous `shar` command failure.       |
| 63   | `EXIT_BUG`              | Internal `shar` command bug — please report it.              |
| 66   | `EX_NOINPUT`            | A specified configuration file could not be loaded.          |
| 70   | `EX_SOFTWARE`           | `libopts` had an internal operational error; please report.  |

---

## Authors

The `shar` and `unshar` programs are the collective work of many authors. Contributors reported problems, suggested improvements, and submitted code. See the `THANKS` file in the sharutils distribution.

---

## Bugs

Please include `sharutils` in the subject line of emailed bug reports; it helps to spot the message.

---

## Examples

Create a shell archive of all C sources:

```sh
shar *.c > cprog.shar
```

A silent shell archive of all `.c` and `.h` files:

```sh
shar -Q *.[ch] > cprog.shar
```

A shell archive of all uuencoded `.arc` files, saved to numbered files starting at `arc.sh.01`:

```sh
shar -B -l28 -oarc.sh *.arc
```

Restore using only base filenames (ignore hierarchy):

```sh
shar -f /lcl/src/u*.c > u.sh
```

---

## Warnings

- No attempt is made to restore protection and modification dates for **directories**, even if done for files; unpacked directory metadata may differ from originals.
- If a directory is passed to `shar`, it may be scanned multiple times to conserve memory—do **not** modify contents while `shar` runs.
- Ensure output files are not included in inputs, or `shar` may loop until disk fills. Be especially careful when passing directories that the output files are not inside them.
- Compression and encoding options can significantly slow the archiving process.
- `--query-user` can break many `unshar` procedures; use only among agreeable parties. It is **not** for net-distributed archives. Compression in net shars is discouraged; omitting `--no-timestamp` or `--force-prefix` may also cause complaints — consider adding these to `~/.sharrc`.

---

## See also

- `unshar(1)`
