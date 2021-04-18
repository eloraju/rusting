# Learning rust and chess

## Starting off

This repo is part of Wunderdog's intrest groups and in this case rust intrest group.
The goal is to try and learn a new language - rust - while working on something
that's interesting and beneficial. If you haven't picked it up yet, I've gone ahead
and decided I'd like to learn Rust.

You can follow my development and thought process in my [development diary][1]

I'll also try to gather stuff that I found useful in understanding Rust in [this file][2].

## What's in the box?

Hopefully something that resembles a chess engine...

Maybe creating a terminal program that you can solve puzzles with could actually be
the thing to strive towards.

## Running

`cargo run` to run the application

`cargo test` to run tests

`cargo doc --open` to create and open documentation in your default browser

Nothing else to it. At this point.

## How to build a chess engine

🤷

[Chess programming wiki][3] has been a decent source so far but. But since I'm a
dumdum I might have to search for 'How to engine - For dummies'...

I need to be able to 

1. Keep track of the board state
2. Analyze the state and generate moves
3. Handle more nieche stuff like en passant and castling
4. ????
5. Become the next Murphy

Sillyness aside, I should be able to build this thing more or less incrementally.
Starting with initializing a board and coming up with a way to print out the board
to the terminal. After that's done I might move on to inputting moves
(no validation yet). My initial idea was to have the engine run as a daemon and
then using commands like `rce Ne5 (move knight to E5)` trying to move a piece on
the board. That would be extremely cool since with those two features and a way of
importing chess puzzles, I could create a terminal chess puzzle program! I don't
have to validate the moves, just check whether the move is the one the puzzle is
looking for or not. 

[1]:devdiary.md
[2]:learning.md 
[3]:https://www.chessprogramming.org/Main_Page
