## Knight's Tour in Rust

This program implements the backtracking algorithm to solve the Knight's Tour problem on a chessboard.

### How to Run

-   If you dont have Rust installed, you can choose to install it or just run the executable file `knight_move`

    -   To run the executable file, open terminal and navigate to the directory where the file is located and run `./knight_move`
    -   You have to provide the board size as an argument. For example, to run the program for a 5x8 board, run `./knight_move 5 8`

-   If you have Rust Installed:
    -   Open terminal and navigate to the directory where the code is located.
    -   Run `cargo build` to compile the code.
    -   Run `cargo run -- <board_size>` to run the program, replacing `<board_size>` with the desired size of the chessboard. Example: `cargo run -- 5 8`

### The Knight's Tour Problem

The Knight's Tour problem is like a brainteaser for chess lovers. Imagine a knight on a chessboard, and you want it to visit every square exactly once, but only using its unique "L" shaped move. This program helps you find a solution, so you don't have to spend hours puzzling over it yourself.

### Backtracking Algorithm

This program uses backtracking, a kind of like a maze solving strategy. It explores different paths (knight moves) and backtracks when a dead end is reached.

Here's a breakdown of the algorithm:

1. **Initialize the board:** A chessboard is represented as a grid where each square is empty or has a number showing the knight's visit order.
2. **Define knight's moves:** An array stores the possible ways a knight can move from any square.
3. **Recursive function:** The `solve_knight_tour` function attempts to solve the puzzle. It takes the board, knight move options, current move count, and knight's position as arguments.
    - **Base Case:** If the knight has visited every square, a solution is found! The function returns true.
    - **Iterate through possible moves:** It tries each of the eight possible knight moves.
        - **Validate move:** It checks if the move is within board boundaries and lands on an unvisited square.
        - **Place move:** If valid, the move number is placed on the board, and the function calls itself recursively with the updated board and knight's new position.
        - **Backtrack:** If the recursive call doesn't find a solution from that move, the move number is removed from the board (backtracking).
4. **Main function:**
    - Sets the board size.
    - Defines the knight's possible x and y moves.
    - Places the knight on the starting square (usually (0, 0)) with move number 0.
    - Calls the `solve_knight_tour` function.
    - If the function returns false, no solution exists, and a message is displayed.
    - Otherwise, the final board configuration, showing the knight's path, is printed.

This was just a weekend project to spend time on [you know how programmers are ;) ]. I hope you find it interesting :)
