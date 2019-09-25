use ggez::graphics::{self, Color};
use ggez::conf;
use ggez::timer;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra as na;
use rand::rngs::ThreadRng;
use rand::Rng;

use std::path;

use tetromino::*;

mod tetromino;

type Point2f = na::Point2<f32>;
type Point2i = na::Point2<i32>;
type Vector2i = na::Vector2<i32>;

const UNIT_SIZE: i32 = 32;
const BOARD_HEIGHT: i32 = 20;
const BOARD_WIDTH: i32 = 10;
const SPEED_SLOW: u128 = 500;
const SPEED_FAST: u128 = 100;

struct TetrisInputState {
  dir: i32,
  rotate: i32,
  speed: Option<bool>,
  is_paused: bool,
}

impl Default for TetrisInputState {
  fn default() -> Self {
    TetrisInputState {
      dir: 0,
      rotate: 0,
      speed: None,
      is_paused: false,
    }
  }
}

struct Assets {
  block_image: graphics::Image,
  background_image: graphics::Image,
}

impl Assets {
  fn new(ctx: &mut Context) -> GameResult<Assets> {
    let block_image = graphics::Image::new(ctx, "/block.png")?;
    let background_image = graphics::Image::new(ctx, "/background.png")?;
    Ok(Assets {
      block_image,
      background_image,
    })
  }
}

#[derive(Clone)]
struct Block {
  color_index: usize,
  position: Vector2i,
}

impl Block {
  fn color(&self) -> Color {
    ALL_TETROMINOS[self.color_index].color
  }
}

struct TetrisState {
  input: TetrisInputState,
  position: na::Vector2<i32>,
  curr_tetromino_index: usize,
  curr_blocks: Vec<Vector2i>,
  board_blocks: Vec<Block>,
  advance_interval: u128,
  advance_timer: u128,
  assets: Assets,
  rng: ThreadRng,
}

impl TetrisState {
  fn new(ctx: &mut Context) -> GameResult<TetrisState> {
    let assets = Assets::new(ctx)?;
    let mut rng = rand::thread_rng();
    let curr_tetromino_index = rng.gen_range(0, ALL_TETROMINOS.len());

    Ok(TetrisState {
      input: TetrisInputState::default(),
      position: na::Vector2::new(0, 0),
      curr_tetromino_index,
      curr_blocks: ALL_TETROMINOS[curr_tetromino_index].block_positions.iter().map(|p| na::Vector2::new(p[0], p[1])).collect(),
      board_blocks: vec![],
      advance_interval: SPEED_SLOW,
      advance_timer: 0,
      assets,
      rng,
    })
  }

  fn curr_tetromino<'a>(self: &'a TetrisState) -> &'a Tetromino {
    &ALL_TETROMINOS[self.curr_tetromino_index]
  }

  fn check_collisions<T: Iterator<Item = Vector2i>>(&self, new_positions: &mut T) -> bool {
    new_positions.any(|p| p.y >= BOARD_HEIGHT || p.x >= BOARD_WIDTH || p.x < 0 || self.board_blocks.iter().any(|b| {
      b.position.x == p.x && b.position.y == p.y
    }))
  }

  fn check_collision(&self, transform: &na::Matrix3<i32>) -> bool {
    let new_pos = transform * na::Vector3::new(self.position.x, self.position.y, 1);
    self.curr_blocks.iter()
      .map(|p| (Point2i::new(new_pos.x + p[0], new_pos.y + p[1])))
      .any(|p| p.y >= BOARD_HEIGHT || p.x >= BOARD_WIDTH || p.x < 0 || self.board_blocks.iter().any(|b| {
        b.position.x == p.x && b.position.y == p.y
      }))
  }

  fn commit_tetromino(&mut self) {
    let pending: Vec<_> = self.curr_blocks.iter()
      .map(|p| (na::Vector2::new(self.position.x + p[0], self.position.y + p[1])))
      .collect();
    for v in pending {
      self.board_blocks.push(Block {
        position: v,
        color_index: self.curr_tetromino_index,
      });
    }
    self.curr_tetromino_index = self.rng.gen_range(0, ALL_TETROMINOS.len());
    self.curr_blocks = ALL_TETROMINOS[self.curr_tetromino_index].block_positions.iter().map(|p| na::Vector2::new(p[0], p[1])).collect();
    self.position = na::Vector2::new(0, 0);
  }

  fn clear_rows(&mut self) {
    let to_remove: Vec<_> = (0..BOARD_HEIGHT).filter(|y| {
      (0..BOARD_WIDTH).all(|x| {
        self.board_blocks.iter().any(|b| {
          b.position.x == x && b.position.y == *y
        })
      })
    }).collect();
    self.board_blocks = self.board_blocks.iter().filter(|b| {
      !to_remove.iter().any(|y| {
        b.position.y == *y
      })
    }).map(|b| b.clone()).collect();
    for b in self.board_blocks.iter_mut() {
      b.position.y += to_remove.iter().filter(|y| {
        **y > b.position.y
      }).count() as i32;
    }
  }
}

impl EventHandler for TetrisState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    if self.input.is_paused {
      return Ok(())
    }

    const FPS: u32 = 60;

    while timer::check_update_time(ctx, FPS) {
      if !self.check_collision(&na::Matrix3::new_translation(&na::Vector2::new(self.input.dir, 0))) {
        self.position.x += self.input.dir;
      }

      if self.input.rotate != 0 {
        let max = self.curr_blocks.iter().flat_map(|b| vec![b.x, b.y]).max().unwrap();
        let rotate = |b: &Vector2i| {
          if self.input.rotate > 0 {
            na::Vector2::new(max - b.y, b.x)
          }
          else {
            na::Vector2::new(b.y, max - b.x)
          }
        };
        if !self.check_collisions(&mut self.curr_blocks.iter().map(rotate).map(|b| b + self.position)) {
          self.curr_blocks = self.curr_blocks.iter().map(rotate).collect();
        }
      }

      if let Some(speed) = self.input.speed {
        self.advance_interval = if speed {
          SPEED_FAST
        } else {
          SPEED_SLOW
        }
      }

      self.input = TetrisInputState::default();
    }

    self.advance_timer += timer::delta(ctx).as_millis();
    if self.advance_timer > self.advance_interval {
      if !self.check_collision(&na::Matrix3::new_translation(&na::Vector2::new(0, 1))) {
        self.position.y += 1;
      }
      else {
        self.commit_tetromino();
        self.clear_rows();
      }
      while self.advance_timer > self.advance_interval {
        self.advance_timer -= self.advance_interval;
      }
    }

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);

    // draw background
    for x in (0..BOARD_WIDTH).map(|x| x * UNIT_SIZE) {
      for y in (0..BOARD_HEIGHT).map(|y| y * UNIT_SIZE) {
        graphics::draw(ctx, &self.assets.background_image, graphics::DrawParam::new()
          .dest(Point2f::new(x as f32, y as f32)))?;
      }
    }

    if !self.input.is_paused {
      for block in self.board_blocks.iter() {
        graphics::draw(ctx, &self.assets.block_image, graphics::DrawParam::new()
          .dest(Point2f::new(
            (block.position.x * UNIT_SIZE) as f32,
            (block.position.y * UNIT_SIZE) as f32)
          ).color(block.color()))?;
      }

      let curr_tetromino = self.curr_tetromino();
      for v in self.curr_blocks.iter() {
        graphics::draw(ctx, &self.assets.block_image, graphics::DrawParam::new()
          .dest(Point2f::new(
            ((v.x + self.position.x) * UNIT_SIZE) as f32,
            ((v.y + self.position.y) * UNIT_SIZE) as f32)
          ).color(curr_tetromino.color))?;
      }
    }

    graphics::present(ctx)?;

    timer::yield_now();
    Ok(())
  }

  fn key_down_event(
    &mut self,
    _ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
    _repeat: bool,
  ) {
    if _repeat {
      return ()
    }
    match keycode {
      KeyCode::Left => {
        self.input.dir = -1;
      }
      KeyCode::Right => {
        self.input.dir = 1;
      }
      KeyCode::Up => {
        self.input.rotate = 1;
      }
      KeyCode::Down => {
        self.input.rotate = -1;
      }
      KeyCode::Space => {
        self.input.speed = Some(true);
      }
      KeyCode::Escape => {
        self.input.is_paused = !self.input.is_paused;
      }
      _ => ()
    }
  }

  fn key_up_event(
    &mut self,
    _ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
  ) {
    match keycode {
      KeyCode::Space => {
        self.input.speed = Some(false);
      }
      _ => ()
    }
  }
}

pub fn run_game() -> GameResult {
  let cb = ContextBuilder::new("Tetris", "Tanner Evins")
    .window_setup(conf::WindowSetup::default().title("Tetris"))
    .window_mode(conf::WindowMode::default()
      .dimensions((BOARD_WIDTH * UNIT_SIZE) as f32, (BOARD_HEIGHT * UNIT_SIZE) as f32))
    .add_resource_path(path::PathBuf::from("./resources"));
  let (ctx, events_loop) = &mut cb.build()?;
  let game = &mut TetrisState::new(ctx)?;
  event::run(ctx, events_loop, game)
}
