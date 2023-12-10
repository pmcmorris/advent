# Day 1

This was pretty straight forward in that step one was just iterating over chars
in a string.  But Rust does everything in utf-8 by default so that added a bit
of complexity in that we're iterating by char rather than u8. This makes it more
painful when trying to go backwards (even though you can in utf-8).  Part two
also added strings, so searching by characters didn't make sense anymore.  This
probably would have been a lot faster to use a multi-string search like
Aho-Corasick but I decided to just keep it simple for now. Again going backwards
had some additional complexities.  But rather than deal with them, I just did a
full pass over the whole string keeping track of first and last matches.  Not as
efficient, but simpler to code up.

Additionally, the code could probably have been cleaned up to share the
summation between part one and part two with generics or something.  But this
doesn't need a lot of polish if the goal is simply to get a correct answer.
