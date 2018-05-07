TODO: 

* Insert zero-width spaces between characters at random or between all of them. Maybe zero-width space density? 0%/25%/50%/100%?
* Insert random, invisible joiner characters.
* Randomly glyph characters.
* Randomly switch spaces with other, reasonably equivalent white space characters or interpuncts.
* Swap random characters for confusables. (Again, configurable density?)
* Figure out how to do multi-character confusables. (e.g. AA ->Ꜳ  )
* Digit confusables
* Figure out how not to have to build the hashmap in the map() function
* Whitespace confusables
* Zero-width nonbreaking space (since it's not technically a whitespace char, according to unicode spec.)
* Create a special "discrete mode" that emphasizes characters with minimal visual difference in most contexts.
