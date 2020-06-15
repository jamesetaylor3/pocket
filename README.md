# pocket

pocket is a command line program that will solve 2x2 pocket cubes using graph theory algorithms. It's still heavily in development!

### Build

You will have to install rustup, rustc and cargo to perform this. Navigate to project and build using this command. The target will be in the target directory. Would strongly recommend building using release instead of dev. The time you will save in runtime due to compiler optimizations will vastly outweigh mild increase in compile time when it comes to this bfs.

`cargo build --release`

### Usage

To solve the rubiks cube at a path "cube.txt", use the file tag.

`pocket file cube.txt`

The contents of the text file must be 23 characters, each character one of (W, G, R, B, Y, O) for (white, green, red, blue, yellow, orange). Don't worry about whitespace or capitalizing the letters. The sequence will project a cube as follows, where each block is an unfolded face of the cube. Each number represents the index of the list of characters entered into the text file.

```
        |  0  1 |
        |  3  2 |

|  4  5 |  8  9 | 12 13 | 16 17 |
|  7  6 | 11 10 | 15 14 | 19 18 |

        | 20 21 |
        | 23 22 |
```
The cubes faces can be noted as follows. U: Up, L: Left, F: Front, R: Right, B: Back, D: Down
```
    | U |
| L | F | R | B |
    | D |
```

To run a random scrambled cube, use the scrable tag.

`pocket scramble 20 5`

The first number is the number of random turns to apply to a solved cube, and the second number is the number of cubes to run this iteratively on.

### Testing

There are a couple unit tests at the bottom of src/cube/object that test the cube movement functionality. Run the following command to execute the unit tests

`cargo test`

They all pass ofc :) I made them!

### Couple things to note
* This is so slow. If the solved state is over 8 moves away (max 12), the BFS takes a ridiculous amount of time. It grows with combinatorial time. Unfortunately, this accounts for the vast majority of pocket cube states. Yikes.
* You can't run multiple scrambles. The program will only do one scramble for now.
* No validation for the cubes that are read from file YET! If the program is taking forever either you inputted an invalid cube scheme or BFS is taking forever for your cube. I would recommend flipping a coin every couple minutes to decided to quit the program or not.

### Up next
* Validation for cubes read from file.
* Scamble the way I envisioned it (with stats)
* Create a feature that solves the entire rubik's cube and store it as a database in a file. Each time thereafter, pocket will load the file to use the already solved graph. Hopefully will save a lot of time! Creating that database would take forever and also take up much space--not to mention the time loading it into memory from disk. Ah