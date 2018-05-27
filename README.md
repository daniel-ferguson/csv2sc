csv2sc
===========

A simple command line tool for converting CSV & TSV files into the format used
by `sc`, a linux terminal spreadsheet program.

It creates cells of the following formats:

 * integer
 * float
 * left-aligned string

## Usage

Currently it only operates on `STDIN`, and prints to `STDOUT`.

To convert a CSV file:
```
csv2sc < file.csv > file.sc
```

To convert a TSV file:

```
csv2sc --tsv < file.tsv > file.sc
```

To stream a file straight to sc:

```
csv2sc < file.csv | sc
```

---

```
csv2sc 0.1
Converts CSV or TSV input into sc spreadsheet format

USAGE:
    csv2sc [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -t, --tsv        Use tabs rather than commas as field delimiter
    -V, --version    Prints version information
```
