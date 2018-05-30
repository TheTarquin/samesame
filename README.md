TODO: 

* Insert zero-width spaces between characters at random or between all of them. Maybe zero-width space density? 0%/25%/50%/100%?
* Insert random, invisible joiner characters.
* Insert other random control chars (e.g. matched RtL/LtR chars)
* Randomly glyph characters.
* Swap random characters for confusables. (Again, configurable density?)
* Figure out how to do multi-character confusables. (e.g. AA ->Ꜳ  )
* Digit confusables
* Figure out how not to have to build the hashmap in the map() function
* Whitespace/interpunct confusables
* Play around more with zero-width nonbreaking space (since it's not technically a whitespace char, according to unicode spec.)
* Create a special "discrete mode" that emphasizes characters with minimal visual difference in most contexts.
* Handle multiline input

Options:

* -d - Discrete Mode
* -i="input" - Pass input string as an option (for script friendliness)
* -v - Verbose mode
* -h - Print Help
