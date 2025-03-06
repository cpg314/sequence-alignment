A toy implementation of Needleman-Wunsch to demo Rust, not meant to be used for any other purpose.

```
$ alignment -h
Usage: alignment [OPTIONS] <COMMAND>

Commands:
    align  Align the first two sequences in a FASTA file
    serve  Launch alignment HTTP service
    help   Print this message or the help of the given subcommand(s)

Options:
        --mismatch-penalty <MISMATCH_PENALTY>  [default: -2]
        --gap-penalty <GAP_PENALTY>            [default: -1]
    -h, --help                                 Print help
```
