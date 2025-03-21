# Terminal Snake Game in Rust

This project implements a classic Snake game in the terminal using Rust. It leverages the `crossterm` crate for terminal manipulation (raw mode, event handling, cursor control) and `rand` for generating random food positions.

## Features

*   **Classic Snake Gameplay:** Implements the core mechanics of the Snake game, including snake movement, food consumption, and collision detection.
*   **Terminal Rendering:** Renders the game board using Unicode characters within the terminal.
*   **Keyboard Input:**  Handles keyboard input for changing the snake's direction (Up, Down, Left, Right) and quitting the game (Q).
*   **Scorekeeping:** Tracks and displays the player's score.
*   **Game Over Detection:** Detects game over conditions (snake colliding with itself or the walls).
*   **Random Food Generation:** Generates food at random locations on the game board.
*   **Frame Rate Control:** Uses `thread::sleep` and `Instant` to control the game's frame rate.

## Dependencies

*   crossterm
*   rand

## Installation

1.  **Install Rust:** If you don't have Rust installed, download and install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2.  **Create a new Rust project:**

    ```bash
    cargo new terminal_snake
    cd terminal_snake
    ```

3.  **Add dependencies to `Cargo.toml`:**

    ```toml
    [dependencies]
    crossterm = "0.27"
    rand = "0.8"
    ```

4.  **Replace `src/main.rs` with the provided code:**
    Copy the code you provided into the `src/main.rs` file.

## Usage

1.  **Build the project:**

    ```bash
    cargo build
    ```

2.  **Run the game:**

    ```bash
    cargo run
    ```

## Controls

*   **Up Arrow:** Move the snake up.
*   **Down Arrow:** Move the snake down.
*   **Left Arrow:** Move the snake left.
*   **Right Arrow:** Move the snake right.
*   **Q:** Quit the game.

## Code Structure

*   **`Position` struct:** Represents a position on the game board with `x` and `y` coordinates.
*   **`Direction` enum:** Defines the possible directions the snake can move (Up, Down, Left, Right).
*   **`Game` struct:** Encapsulates the game state, including the snake, food, direction, dimensions, score, and game over status.
    *   **`Game::new(width: i32, height: i32)`:** Creates a new game instance with the specified width and height, initializes the snake at the center, and generates the initial food position.
    *   **`Game::generate_food()`:** Generates a new food position at a random location that is not occupied by the snake.
    *   **`Game::update()`:** Updates the game state for each frame, moving the snake, checking for collisions, and updating the score.
    *   **`Game::change_direction(new_direction: Direction)`:** Changes the snake's direction, preventing it from reversing directly back on itself.
*   **`main()` function:**
    *   Enables raw mode for terminal input.
    *   Creates a `Game` instance.
    *   Enters the main game loop.
    *   Handles keyboard input to change the snake's direction or quit the game.
    *   Updates the game state.
    *   Clears the screen and renders the game board.
    *   Displays the score and game over message.
    *   Disables raw mode when the game ends.

## Key Implementation Details

*   **Raw Mode:** The `crossterm::terminal::enable_raw_mode()` function puts the terminal into raw mode, which allows the program to receive individual keystrokes without waiting for the user to press Enter.
*   **Event Handling:** The `crossterm::event::poll()` and `crossterm::event::read()` functions are used to check for and read keyboard events.
*   **Terminal Clearing:** The `crossterm::terminal::Clear()` function is used to clear the terminal screen before rendering each frame.
*   **Cursor Positioning:** The `crossterm::cursor::MoveTo()` function is used to move the cursor to the top-left corner of the screen before rendering the game board.
*   **Unicode Characters:** The game board is rendered using Unicode characters (e.g., "█" for the snake, "●" for food, "·" for empty spaces).
*   **Collision Detection:** The `Game::update()` function checks for collisions between the snake and the walls or itself to determine if the game is over.
*   **VecDeque:** The `VecDeque` data structure is used to represent the snake's body, allowing for efficient addition and removal of segments at the head and tail.

## Potential Improvements

*   **Color:** Add color to the game elements (snake, food, background) to improve the visual appeal.
*   **Difficulty Levels:** Implement different difficulty levels by adjusting the game speed or the size of the game board.
*   **Sound Effects:** Add sound effects for events such as eating food or game over.
*   **High Score:** Implement a high score system to track the player's best scores.
*   **Configuration:** Allow the user to configure the game settings (e.g., board size, key bindings) through command-line arguments or a configuration file.
*   **More Robust Input Handling:** Add more comprehensive input handling, possibly using a dedicated input library, to support different keyboard layouts and handle edge cases.
*   **Error Handling:** Add more robust error handling throughout the application, including more specific error messages and recovery mechanisms.
