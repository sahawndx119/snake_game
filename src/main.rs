//in the name of God

use colorize::AnsiColor;
use crossterm::event::{self, poll, Event};
// use colorize::Style;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute, queue, terminal, ExecutableCommand};
use rand::Rng;
use std::io::{stdout, Write};
use std::sync::mpsc::{self, channel};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
// use std::thread::sleep;
// use std::time::Duration;
struct Snake {
    len: usize,
    direction: Direction,
    body: Vec<(i16, i16)>,
}
#[derive(PartialEq, PartialOrd)]
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
    let (tx, rx) = channel();
    let mut apples = Vec::new();
    std::thread::spawn(move || loop {
        sleep(Duration::from_secs(2));
        let rand_x: i16 = rand::thread_rng().gen_range(2..29);
        let rand_y: i16 = rand::thread_rng().gen_range(1..19);
        tx.send((rand_x, rand_y)).unwrap();
    });

    loop {
        if poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                let _ = match key_event.code {
                    event::KeyCode::Char('w') | event::KeyCode::Up => {
                        if snake.direction == Direction::Down {
                            snake.direction = Direction::Down;
                        } else {
                            snake.direction = Direction::Up;
                        }
                    }
                    event::KeyCode::Char('s') | event::KeyCode::Down => {
                        if snake.direction == Direction::Up {
                            snake.direction = Direction::Up;
                        } else {
                            snake.direction = Direction::Down;
                        }
                    }
                    event::KeyCode::Char('a') | event::KeyCode::Left => {
                        if snake.direction == Direction::Right {
                            snake.direction = Direction::Right;
                        } else {
                            snake.direction = Direction::Left;
                        }
                    }
                    event::KeyCode::Char('d') | event::KeyCode::Right => {
                        if snake.direction == Direction::Left {
                            snake.direction = Direction::Left;
                        } else {
                            snake.direction = Direction::Right;
                        }
                    }

                    event::KeyCode::Esc => {
                        disable_raw_mode().unwrap();
                        return;
                    }

                    _ => {}
                };
            }

            
            apples = snake_printor(&mut snake, apples);
        } else {
            apples = snake_printor(&mut snake, apples);
        }
        
        if snake_eats_itself(&snake) {
            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap()
                .execute(cursor::MoveTo(50, 10))
                .unwrap()
                .execute(Print("GAME OVER".bold().red()))
                .unwrap();
            return;
        }
        if apples.len() >= 1 {
            if apples[0].0 == 119 {
                stdout
                    .execute(terminal::Clear(terminal::ClearType::All))
                    .unwrap()
                    .execute(cursor::MoveTo(50, 10))
                    .unwrap()
                    .execute(Print("GAME OVER".bold().red()))
                    .unwrap();
                return;
            }
        }

        match rx.try_recv() {
            Ok((mut x, mut y)) => {
                if x == 0 {
                    x += 1;
                }
                if x == 30 {
                    x -= 1;
                }
                if y == 0 {
                    y += 1;
                }
                if y == 30 {
                    y -= 1;
                }
                execute!(stdout, cursor::MoveTo(x as u16, y as u16), Print("•".red())).unwrap();
                apples.push((x, y));
            }

            Err(_) => {}
        }
    }
}

fn play_ground() {
    for i in 0..31 {
        for j in 0..21 {
            if i == 0 || i == 30 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("ø")).unwrap();
            } else if j == 0 || j == 20 {
                queue!(stdout(), cursor::MoveTo(i, j), Print("∞")).unwrap();
            }
        }
    }

    stdout().flush().unwrap();
}

fn snake_printor(snake: &mut Snake, mut apples: Vec<(i16, i16)>) -> Vec<(i16, i16)> {
    let arc_apples = Arc::new(apples.clone());
    let (tx, rx) = mpsc::channel();
    let handle;
    match snake.direction {
        Direction::Up => {
            let apple_for_use = arc_apples.clone();
            let head = (snake.body[0].0, snake.body[0].1 - 1);
            if (head.0 == 0 || head.1 == 0) || (head.0 == 30 || head.1 == 20) {
                return vec![(119, 119)];
            }
            handle = std::thread::spawn(move || {
                let new_head = (head.0, head.1);

                match eat_apple(&apple_for_use, new_head) {
                    Some(index) => {
                        tx.send(index).unwrap();
                    }
                    None => {}
                }
            });
            queue!(
                stdout(),
                cursor::MoveTo(
                    (snake.body[snake.len - 1].0) as u16,
                    (snake.body[snake.len - 1].1) as u16
                ),
                Print(" ")
            )
            .unwrap();

            for i in (1..snake.len).rev() {
                snake.body[i as usize] = snake.body[(i - 1) as usize];
                queue!(
                    stdout(),
                    cursor::MoveTo(
                        (snake.body[i as usize].0) as u16,
                        (snake.body[i as usize].1) as u16
                    ),
                    Print("O")
                )
                .unwrap();
            }
            snake.body[0].1 -= 1;
            queue!(
                stdout(),
                cursor::MoveTo((snake.body[0].0) as u16, (snake.body[0].1) as u16),
                Print("O".green())
            )
            .unwrap();
        }
        Direction::Down => {
            let head = (snake.body[0].0, snake.body[0].1 + 1);
            let apple_for_use = arc_apples.clone();
            if (head.0 == 0 || head.1 == 0) || (head.0 == 30 || head.1 == 20) {
                return vec![(119, 119)];
            }

            handle = std::thread::spawn(move || {
                let new_head = (head.0, head.1);

                match eat_apple(&apple_for_use, new_head) {
                    Some(index) => {
                        tx.send(index).unwrap();
                    }
                    None => {}
                }
            });

            queue!(
                stdout(),
                cursor::MoveTo(
                    (snake.body[snake.len - 1].0) as u16,
                    (snake.body[snake.len - 1].1) as u16
                ),
                Print(" ")
            )
            .unwrap();

            for i in (1..snake.len).rev() {
                snake.body[i as usize] = snake.body[(i - 1) as usize];
                queue!(
                    stdout(),
                    cursor::MoveTo(
                        (snake.body[i as usize].0) as u16,
                        (snake.body[i as usize].1) as u16
                    ),
                    Print("O")
                )
                .unwrap();
            }
            snake.body[0].1 += 1;
            queue!(
                stdout(),
                cursor::MoveTo((snake.body[0].0) as u16, (snake.body[0].1) as u16),
                Print("O".green())
            )
            .unwrap();
        }
        Direction::Right => {
            let head = (snake.body[0].0 + 1, snake.body[0].1);
            if (head.0 == 0 || head.1 == 0) || (head.0 == 30 || head.1 == 20) {
                return vec![(119, 119)];
            }

            let apple_for_use = arc_apples.clone();
            handle = std::thread::spawn(move || {
                let new_head = (head.0, head.1);

                match eat_apple(&apple_for_use, new_head) {
                    Some(index) => {
                        tx.send(index).unwrap();
                    }
                    None => {}
                }
            });
            queue!(
                stdout(),
                cursor::MoveTo(
                    (snake.body[snake.len - 1].0) as u16,
                    (snake.body[snake.len - 1].1) as u16
                ),
                Print(" ")
            )
            .unwrap();

            for i in (1..snake.len).rev() {
                snake.body[i as usize] = snake.body[(i - 1) as usize];
                queue!(
                    stdout(),
                    cursor::MoveTo(
                        (snake.body[i as usize].0) as u16,
                        (snake.body[i as usize].1) as u16
                    ),
                    Print("O")
                )
                .unwrap();
            }
            snake.body[0].0 += 1;
            queue!(
                stdout(),
                cursor::MoveTo((snake.body[0].0) as u16, (snake.body[0].1) as u16),
                Print("O".green())
            )
            .unwrap();
        }
        Direction::Left => {
            let head = (snake.body[0].0 - 1, snake.body[0].1);
            if (head.0 == 0 || head.1 == 0) || (head.0 == 30 || head.1 == 20) {
                return vec![(119, 119)];
            }
            let apple_for_use = arc_apples.clone();
            handle = std::thread::spawn(move || {
                let new_head = (head.0, head.1);

                match eat_apple(&apple_for_use, new_head) {
                    Some(index) => {
                        tx.send(index).unwrap();
                    }
                    None => {}
                }
            });
            queue!(
                stdout(),
                cursor::MoveTo(
                    (snake.body[snake.len - 1].0) as u16,
                    (snake.body[snake.len - 1].1) as u16
                ),
                Print(" ")
            )
            .unwrap();

            for i in (1..snake.len).rev() {
                snake.body[i as usize] = snake.body[(i - 1) as usize];
                queue!(
                    stdout(),
                    cursor::MoveTo(
                        (snake.body[i as usize].0) as u16,
                        (snake.body[i as usize].1) as u16
                    ),
                    Print("O")
                )
                .unwrap();
            }
            snake.body[0].0 -= 1;
            queue!(
                stdout(),
                cursor::MoveTo((snake.body[0].0) as u16, (snake.body[0].1) as u16),
                Print("O".green())
            )
            .unwrap();
        }
    }
    stdout().flush().unwrap();
    handle.join().unwrap();
    let _ = match rx.try_recv() {
        Ok(index) => {
            apples.remove(index);
            add_to_snake(snake);
        }
        Err(_) => {}
    };
    apples
}

fn eat_apple(apples: &Vec<(i16, i16)>, new_head: (i16, i16)) -> Option<usize> {
    for (index, apple) in apples.iter().enumerate() {
        if *apple == new_head {
            return Some(index);
        }
    }
    None
}

fn add_to_snake(snake: &mut Snake) {
    match snake.direction {
        Direction::Up => {
            if snake.len == 1 {
                snake.body.push((snake.body[0].0, snake.body[0].1 + 1));
            } else {
                let body_closer = snake.body[snake.len - 2];
                let body = snake.body[snake.len - 1];
                let res = (body_closer.0 - body.0, body_closer.1 - body.1);
                snake.body.push((body.0 - res.0, body.1 - res.1));
            }
        }
        Direction::Down => {
            if snake.len == 1 {
                snake.body.push((snake.body[0].0, snake.body[0].1 - 1));
            } else {
                let body_closer = snake.body[snake.len - 2];
                let body = snake.body[snake.len - 1];
                let res = (body_closer.0 - body.0, body_closer.1 - body.1);
                snake.body.push((body.0 - res.0, body.1 - res.1));
            }
        }
        Direction::Right => {
            if snake.len == 1 {
                snake.body.push((snake.body[0].0 - 1, snake.body[0].1));
            } else {
                let body_closer = snake.body[snake.len - 2];
                let body = snake.body[snake.len - 1];
                let res = (body_closer.0 - body.0, body_closer.1 - body.1);
                snake.body.push((body.0 - res.0, body.1 - res.1));
            }
        }
        Direction::Left => {
            if snake.len == 1 {
                snake.body.push((snake.body[0].0 + 1, snake.body[0].1));
            } else {
                let body_closer = snake.body[snake.len - 2];
                let body = snake.body[snake.len - 1];
                let res = (body_closer.0 - body.0, body_closer.1 - body.1);
                snake.body.push((body.0 - res.0, body.1 - res.1));
            }
        }
    }
    snake.len += 1;
}

fn snake_eats_itself(snake: &Snake) -> bool {
    let new_head = snake.body[0];
    for i in 1..snake.body.len() {
        if new_head == snake.body[i] {
            return true;
        }
    }
    false
}
