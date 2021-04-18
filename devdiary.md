# Development

I'll post small updates on what is happening with the engine

## 21-03-30

Started the project! I did have a idea of what Rust is and that helped to get me even this far. In the span of about 8 hours I managed to read up quite a bit on how a chess engine works and what is actually needed to have a workign engine. A bug free core is mentioned multiple times soooo... Tests? I also managed to create the first iteration of the board representation. At first I really wanted to create a bitboard, but that proved to be a bit too much to take in this early on. I decided to make this first go a bit easier for myself and pivoted to using just multi-dimensional array. Once I did that it took me maybe a couple of hours to get to the point where I had the program spewing the board into the terminal! Next up I should come up with a way to initialize the board with a custom state. Gotta look into PGN and FEN.. Maybe I'll start with just a string or array of strings? I would like to see how IO works with rust so maybe a text file with 8x8 char matrix?

## 21-03-30

Didn't have that much time today. Bascially just spent my time refactoring the whole thing a bit and trying to come up with a way to represent board history neatly. Initial idea was to have the whole Game object be a doubly linked list that one could easily traverse but that turned out to be easier to thin that to implement. I've left the codebase in shatters for now. Maybe I'll figure something out over easter.

Maybe I'll just have to KISS and revert back to what I had before, create a FEN/PGN parser and go from there towards the puzzle solver idea I had yesterday.

## 21-04-(02-03)

Whooa boi. Did a whole lot of refactoring last night and this morning. Read and watched some tutorials about modules and testing and decided to give them a go. At the moment I feel like I made good decisions but I'll take another look into this in the evening. Then I'll know for sure. I know that the regex tests are failing but I'll address that in the evening.

Special thanks to [@murtsi](https://github.com/katis) for telling me about the vector/index implementation of linked lists. I did expand on the idea a bit but time will tell whether my contributions were any good :D

### Evening update

Didn't spend that much time on this. Fixed the failing test - or rather fixed my code - and started to work on FEN parsing.

## 21-04-05

Once again... I refactored the whole thing :D

This time around I think I'm quite satisfied with the results. I moved some code around and created much more sense-making structure for the whole codebase. I also created a struct to represent pieces, since playing with strings turned out to be a major pain in my ass... I think that it didn't help that I had `String`, `&str` and `char`types being used nigh interchangeably. Needles to say that that caused more than a couple issues. All that should now be things of the past. I also managed to get FEN-parsing to work when reading the board state in! Yay! Next up I'll try and get the array-based engine to ouput its board state as FEN!

I'll still need to refactor the history/state representation a bit. The engine should have a History-field that holds its state and not the other way around... I think this confusion comes from the fact that at one point I thought I could represent the board as a single entity, rather than it being part of the whole engine's state.

#### Just popping this in here

[UCI](http://wbec-ridderkerk.nl/html/UCIProtocol.html) seems like something that would be nice to implement in the not-so-near future. Albeit one needs a functional engine before there's any point in trying to implement the UCI... But that would mean that the engine could be plopped into existing UIs...

## 21-04- (06-10)

What a week. Didn't have time to dig deep into this but managed to do small things here and there. Surprise to no one, I refactored the whole thing even more. Now the FEN parsing is a module of it's own. I also moved aroudn the existing tests to places where they made more sense. I also managed to get the program to parse it's own state to FEN notation. Yay! I'll try and work on the `State` and `History` modules a bit so I could have the FEN parsing parse the whole game state rather than just the board state. Once those two are in a reasonable state, I can start working on the PGN parsing. Maybe once THAT is done I can move on with the actual engine stuff :D
