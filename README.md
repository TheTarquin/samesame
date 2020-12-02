# INTRODUCTION

samesame is a lightweight utility for replacing ASCII characters with homograph (look-alike) characters.

samesame is under active development and is public domain software.

In other words: do whatever you want with it, just don't be a dick.

See the LICENSE.txt file for slightly more info.

# BUILD

`cargo build`

Artifacts will be in the ./target/debug folder by default

# RUN

`samesame "TEXT"`

or

`$cmd | samesame`

# Options:

```
-d - Discreet Mode
-z - Insert word joiners (U+2060)
-i="infile" - Pass input as a file
-o="outfile" - Write result to a file
-v - Verbose mode
-h - Print Help
-w - Swap ' ' for various unicode whitespaces
-n - Swap numbers out as well as letters
-l - Swap out character pairs for equivalent ligatures
```
