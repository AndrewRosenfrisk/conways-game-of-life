# Conway's Game of Life

## About

Conway’s Game of Life is a cellular automata simulation that follows simple rules to create interesting patterns. 
It was invented by mathematician John Conway in 1970 and popularized by Martin Gardner’s “Mathematical Games” 
column in Scientific American. Today, it’s a favorite among programmers and computer scientists, though it’s 
more an interesting visualization than a true “game.” The two-dimensional board has a grid of “cells,” each of 
which follows three simple rules:

* Living cells with two or three neighbors stay alive in the next step of the simulation.
* Dead cells with exactly three neighbors become alive in the next step of the simulation.
* Any other cell dies or stays dead in the next step of the simulation.

The living or dead state of the cells in the next step of the simulation depends entirely on their current state. 
The cells don’t “remember” any older states. There is a large body of research regarding the patterns that these 
simple rules produce. Tragically, Professor Conway passed away of complications from COVID-19 in April 2020. More 
information about Conway’s Game of Life can be found at https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life, 
and more information about Martin Gardner at https://en.wikipedia.org/wiki/Martin_Gardner.

## Running the project
* Install Rust: [rustup.rs](https://rustup.rs/)
* Clone the repository locally:
  * `git clone https:://github.com/AndrewRosenfrisk/conways-game-of-life`
  * `cd conways-game-of-life`
* Build the project with `cargo build`
* Run the project with `cargo run`

Based on the project detailed in the "[Big Book of Small Python Projects](https://inventwithpython.com/bigbookpython/project13.html)"
