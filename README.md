# INTRODUCTION

samesame is a lightweight utility for replacing ASCII characters with homograph (look-alike) characters.

samesame is under active development and is public domain software.

In other words: do whatever you want with it, just don't be a dick.

See the LICENSE.txt file for slightly more info.

# BUILD

cargo build

Artifacts will be in the ./target/debug folder by default

# RUN

samesame "TEXT"

or

$cmd | samesame

# Options:

* -d - Discreet Mode
* -z - Insert word joiners (U+2060)
* -i="infile" - Pass input as a file
* -o="outfile" - Write result to a file
* -v - Verbose mode
* -h - Print Help

# TODO: 

* Migrate these from the README to actual issues tracking in GitHub.
* Finish discreet mode character set
* Implement discreet mode hinting system
* Create more robust build scheme which adds to path and such
* See about rolling a package for apt, yum, etc.
* Insert random control chars (e.g. matched RtL/LtR chars)
* Randomly glyph characters.
* Figure best way to do multi-character confusables.
* Digit confusables
* Refactor to not to have to build the hashmap in the map() function
* Whitespace/interpunct confusables
* Figure out style guide and how best to enforce in pull requests
* Figure out a better verbose logging solution. Maybe a macro?
