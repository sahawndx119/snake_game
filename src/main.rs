use crossterm::event::{self, poll, Event};
// use colorize::Style;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, queue, terminal, ExecutableCommand};
use std::io::{stdout, Write};
use std::time::Duration;
// use std::thread::sleep;
// use std::time::Duration;

struct Snake {
    len: u32,
    direction: Direction,
    body: Vec<(u16, u16)>,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let mut snake = Snake {
        len: 1,
        direction: Direction::Up,
        body: vec![(15, 15)],
    };
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap()
        .execute(cursor::Hide)
        .unwrap();
    play_ground();

    stdout.flush().unwrap();
    enable_raw_mode().unwrap();

    loop {
        if poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                let _ = match key_event.code {
                    event::KeyCode::Char('w') | event::KeyCode::Up => {
                        snake.direction = Direction::Up
                    }
                    event::KeyCode::Char('s') | event::KeyCode::Down => {
                        snake.direction = Direction::Down
                    }
                    event::KeyCode::Char('a') | event::KeyCode::Left => {
                        snake.direction = Direction::Left
                    }
                    event::KeyCode::Char('d') | event::KeyCode::Right => {
                        snake.direction = Direction::Right
                    }

                    event::KeyCode::Esc => {
                        disable_raw_mode().unwrap();
                        return;
                    }

                    _ => {}
                };
            }
            snake_printor(&mut snake);
        } else {
            snake_printor(&mut snake);
        }
    }
}

fn play_ground() {
    for i in 0..31 {
        for j in 0..31 {
            if i == 0 || i == 30 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("ø")).unwrap();
            } else if j == 0 || j == 30 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("_")).unwrap();
            }
        }
    }

    stdout().flush().unwrap();
}

fn snake_printor(snake: &mut Snake) {
    match snake.direction {
        Direction::Up => {
            let head = snake.body[0];
            let end = snake.body[(snake.len - 1) as usize];
            queue!(stdout(), cursor::MoveTo(head.0, head.1 - 1), Print("O")).unwrap();
            queue!(stdout(), cursor::MoveTo(end.0, end.1), Print(" ")).unwrap();
            snake.body.pop().unwrap();
            snake.body.push((head.0, head.1 - 1));
        }
        Direction::Down => {
            let head = snake.body[0];
            let end = snake.body[(snake.len - 1) as usize];
            queue!(stdout(), cursor::MoveTo(head.0, head.1 + 1), Print("O")).unwrap();
            queue!(stdout(), cursor::MoveTo(end.0, end.1), Print(" ")).unwrap();
            snake.body.pop().unwrap();
            snake.body.push((head.0, head.1 + 1));
          }
        Direction::Right => {
            let head = snake.body[0];
            let end = snake.body[(snake.len - 1) as usize];
            queue!(stdout(), cursor::MoveTo((head.0) + 1, head.1), Print("O")).unwrap();
            queue!(stdout(), cursor::MoveTo(end.0, end.1), Print(" ")).unwrap();
            snake.body.pop().unwrap();
            snake.body.push((head.0 + 1, head.1));
        }
        Direction::Left => {
            let head = snake.body[0];
            let end = snake.body[(snake.len - 1) as usize];
            queue!(stdout(), cursor::MoveTo((head.0) - 1, head.1), Print("O")).unwrap();
            queue!(stdout(), cursor::MoveTo(end.0, end.1), Print(" ")).unwrap();
            snake.body.pop().unwrap();
            snake.body.push((head.0 - 1, head.1));
        }
    }
    stdout().flush().unwrap();
}
