# Learning rust and chess

## Starting off

This repo is part of Wunderdog's intrest groups and in this case rust intrest group. The goal is to try and learn a new language - rust - while working on something that's interesting and beneficial. If you haven't picked it up yet, I've gone ahead and decided I'd like to learn Rust.

## What's in the box?

Hopefully something that resembles a chess engine...

Maybe creating a terminal program that you can solve puzzles with could actually be the thing to strive towards.

### How to build a chess engine

🤷

[Chess programming wiki](https://www.chessprogramming.org/Main_Page) has been a decent source so far but. But since I'm a dumdum I might have to search for 'How to engine - For dummies'...

I need to be able to 

1. Keep track of the board state
2. Analyze the state and generate moves
3. Handle more nieche stuff like en passant and castling
4. ????
5. Become the next Murphy

Sillyness aside, I should be able to build this thing more or less incrementally. Starting with initializing a board and coming up with a way to print out the board to the terminal. After that's done I might move on to inputting moves (no validation yet). My initial idea was to have the engine run as a daemon and then using commands like `rce Ne5 (move knight to E5)` trying to move a piece on the board. That would be extremely cool since with those two features and a way of importing chess puzzles, I could create a terminal chess puzzle program! I don't have to validate the moves, just check whether the move is the one the puzzle is looking for or not. 


## Development

I'll post small updates on what is happening with the engine

### 21-03-30

Started the project! I did have a idea of what Rust is and that helped to get me even this far. In the span of about 8 hours I managed to read up quite a bit on how a chess engine works and what is actually needed to have a workign engine. A bug free core is mentioned multiple times soooo... Tests? I also managed to create the first iteration of the board representation. At first I really wanted to create a bitboard, but that proved to be a bit too much to take in this early on. I decided to make this first go a bit easier for myself and pivoted to using just multi-dimensional array. Once I did that it took me maybe a couple of hours to get to the point where I had the program spewing the board into the terminal! Next up I should come up with a way to initialize the board with a custom state. Gotta look into PGN and FEN.. Maybe I'll start with just a string or array of strings? I would like to see how IO works with rust so maybe a text file with 8x8 char matrix?

### 21-03-30

Didn't have that much time today. Bascially just spent my time refactoring the whole thing a bit and trying to come up with a way to represent board history neatly. Initial idea was to have the whole Game object be a doubly linked list that one could easily traverse but that turned out to be easier to thin that to implement. I've left the codebase in shatters for now. Maybe I'll figure something out over easter.

Maybe I'll just have to KISS and revert back to what I had before, create a FEN/PGN parser and go from there towards the puzzle solver idea I had yesterday.

### 21-04-(02-03)

Whooa boi. Did a whole lot of refactoring last night and this morning. Read and watched some tutorials about modules and testing and decided to give them a go. At the moment I feel like I made good decisions but I'll take another look into this in the evening. Then I'll know for sure. I know that the regex tests are failing but I'll address that in the evening.

Special thanks to [@murtsi](https://github.com/katis) for telling me about the vector/index implementation of linked lists. I did expand on the idea a bit but time will tell whether my contributions were any good :D

#### Evening update

Didn't spend that much time on this. Fixed the failing test - or rather fixed my code - and started to work on FEN parsing.

### 21-04-05

Once again... I refactored the whole thing :D

This time around I think I'm quite satisfied with the results. I moved some code around and created much more sense-making structure for the whole codebase. I also created a struct to represent pieces, since playing with strings turned out to be a major pain in my ass... I think that it didn't help that I had `String`, `&str` and `char`types being used nigh interchangeably. Needles to say that that caused more than a couple issues. All that should now be things of the past. I also managed to get FEN-parsing to work when reading the board state in! Yay! Next up I'll try and get the array-based engine to ouput its board state as FEN!

I'll still need to refactor the history/state representation a bit. The engine should have a History-field that holds its state and not the other way around... I think this confusion comes from the fact that at one point I thought I could represent the board as a single entity, rather than it being part of the whole engine's state.
