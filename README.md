TODO: 

* Insert zero-width spaces between characters at random or between all of them. Maybe zero-width space density? 0%/25%/50%/100%?
* Insert other random control chars (e.g. matched RtL/LtR chars)
* Randomly glyph characters.
* Figure out how to do multi-character confusables. (e.g. AA ->Ꜳ  )
* Digit confusables
* Figure out how not to have to build the hashmap in the map() function
* Whitespace/interpunct confusables
* Play around more with zero-width nonbreaking space (since it's not technically a whitespace char, according to unicode spec.)
* Finish discrete mode character set
* Figure out style guide and how best to enforce in pull requests
* Figure out a better verbose logging solution. Maybe a macro?


Options:

* -d - Discrete Mode
* -z - Insert word joiners (U+2060)
* -i="infile" - Pass input as a file
* -o="outfile" - Write result to a file
* -v - Verbose mode
* -h - Print Help
