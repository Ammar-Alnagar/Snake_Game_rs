use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::thread;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
    cursor,
};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: VecDeque<Position>,
    food: Position,
    direction: Direction,
    width: i32,
    height: i32,
    score: u32,
    game_over: bool,
}

impl Game {
    fn new(width: i32, height: i32) -> Game {
        let mut game = Game {
            snake: VecDeque::new(),
            food: Position { x: 0, y: 0 },
            direction: Direction::Right,
            width,
            height,
            score: 0,
            game_over: false,
        };

        // Initialize snake at the center
        game.snake.push_back(Position {
            x: width / 2,
            y: height / 2,
        });

        game.generate_food();
        game
    }

    fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = Position {
                x: rng.gen_range(0..self.width),
                y: rng.gen_range(0..self.height),
            };
            if !self.snake.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let head = self.snake.front().unwrap().clone();
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y - 1 },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x - 1, y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };

        // Check for collisions
        if new_head.x < 0
            || new_head.x >= self.width
            || new_head.y < 0
            || new_head.y >= self.height
            || self.snake.contains(&new_head)
        {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        // Check if snake ate food
        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.score += 1;
            self.generate_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        let opposite = match new_direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };

        if self.direction != opposite {
            self.direction = new_direction;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;

    let mut game = Game::new(20, 20);
    let frame_duration = Duration::from_millis(100);
    let mut last_update = Instant::now();

    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => game.change_direction(Direction::Up),
                    KeyCode::Down => game.change_direction(Direction::Down),
                    KeyCode::Left => game.change_direction(Direction::Left),
                    KeyCode::Right => game.change_direction(Direction::Right),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        if last_update.elapsed() >= frame_duration {
            game.update();
            last_update = Instant::now();

            // Clear screen and render
            stdout.execute(terminal::Clear(ClearType::All))?;
            stdout.execute(cursor::MoveTo(0, 0))?;

            // Draw game board
            println!("Score: {}", game.score);
            for y in 0..game.height {
                for x in 0..game.width {
                    let pos = Position { x, y };
                    if game.snake.contains(&pos) {
                        print!("█");
                    } else if pos.x == game.food.x && pos.y == game.food.y {
                        print!("●");
                    } else {
                        print!("·");
                    }
                }
                println!();
            }

            if game.game_over {
                println!("Game Over! Final score: {}", game.score);
                break;
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    terminal::disable_raw_mode()?;
    Ok(())
}



