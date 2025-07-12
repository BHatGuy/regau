# regau

regau, from German Regulärer Ausdruck (regular expression), is a regular expression engine, that I wrote just for fun. 

One additional (except matching) feature that I especially have in mind is to visualize of regexes as graphs.

## Plan

The strategy of just start to implement did not work ^^

My plan now is:

 - parse the regex into a NFA
 - convert NFA to DFA
 - run DFA

