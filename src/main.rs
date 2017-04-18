use std::io;
extern crate rand;
extern crate ncurses;
use ncurses::*;


#[derive(PartialEq)]
enum Status { SUCCESS, FAILURE }

#[derive(Clone, Copy, Debug)]
enum Direction { UP, DOWN, LEFT, RIGHT }

#[derive(Clone, Copy, PartialEq)]
struct Point {
  x: i32,
  y: i32
}

struct Board {
  xmax: u32,
  ymax: u32,
  snake: Vec<Point>,
  foods: Vec<Point>
}

impl Board {
  fn eat_food(&mut self, point: Point) {
    self.snake.insert(0, point);
  }

  fn move_to(&mut self, point: Point) {
    self.snake.insert(0, point);
    self.snake.pop();
  }

  fn add_new_food(&mut self) {
    let point = self.create_random_cell();
    self.foods.push(point);
  }

  fn create_random_cell(&self) -> Point {
    let xrand = rand::random::<u32>();
    let yrand = rand::random::<u32>();
    return Point{x: (xrand % self.xmax) as i32, y: (yrand % self.ymax) as i32};
  }

  fn initialize_snake(&mut self) {
    self.snake.push(Point{x: 2, y: 3});
    self.snake.push(Point{x: 2, y: 2});
  }
}



fn next_move(board: &Board, dir: Direction) -> Result<Point, ()> {
  let snake = &board.snake;
  let head = &snake[0];
  let mut new_x = head.x;
  let mut new_y = head.y;
  match dir {
    Direction::UP => {
      new_y -= 1;
    }
    Direction::DOWN => {
      new_y += 1;
    }
    Direction::RIGHT => {
      new_x += 1;
    }
    Direction::LEFT => {
      new_x -= 1;
    }
  }
  if new_x < 0 || new_y < 0 || new_x >= board.xmax as i32 || new_y >= board.ymax as i32 {
    return Err(());
  } else {
    return Ok(Point{x: new_x, y: new_y});
  }
}

fn move_snake(board: &mut Board, dir: Direction) -> Status {
   let beginning = next_move(board, dir);
   match beginning {
     Err(_) => {return Status::FAILURE}
     Ok(_) => {}
   }
   let point: Point = beginning.unwrap();

   // if we're going backwards, ignore and move on
   if board.snake[1] == point {
    return Status::SUCCESS;
   }
   // Check for collisions!
   if board.snake.contains(&point) {
    return Status::FAILURE;
   }
   if board.foods.contains(&point) {
    board.eat_food(point);
//     remove_from_list(beginning, &(board->foods));
    board.foods.retain(|&x| x != point);
    board.add_new_food();
    return Status::SUCCESS;
   }
   board.move_to(point);
   return Status::SUCCESS;
}


fn display_points(snake: &Vec<Point>, symbol: chtype) {
  for point in snake {
    mvaddch(point.y, point.x, symbol);
  }
}

fn get_next_move(previous: Direction) -> Direction {
  let ch = getch();
  match ch {
    KEY_LEFT => {
      return Direction::LEFT;
    }
    KEY_RIGHT => {
      return Direction::RIGHT;
    } 
    KEY_DOWN => {
      return Direction::DOWN;
    }
    KEY_UP => {
      return Direction::UP;
    }
    _ => {}
  }
  return previous;
}


fn main() {
  initscr();
  cbreak();
  noecho();
  keypad(stdscr(), true); // make keys work
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // hide cursor
  timeout(100);

  let mut xmax: i32 = 0;
  let mut ymax: i32 = 0;
  getmaxyx(stdscr(), &mut ymax, &mut xmax);
  let mut dir = Direction::RIGHT;

  let mut board = Board{xmax: xmax as u32, ymax: ymax as u32, foods: vec!(), snake: vec!()};
  board.initialize_snake();

  for i in 1..6 {
    board.add_new_food();
  }

  while true {
    clear();
    display_points(&board.snake, ACS_BLOCK());
    display_points(&board.foods, ACS_DIAMOND());
    refresh();
    dir = get_next_move(dir.clone());
    let status = move_snake(&mut board, dir);
    if  status == Status::FAILURE{
      break
    }
  }
  endwin();
}
