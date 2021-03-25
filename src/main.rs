// use std::time::Instant;
use macroquad::prelude::*;
pub use quad_rand as rand;

const PLAYER_ACCEL: f64 = 3000.0;
const PLAYER_BOOST_MAX_ACCEL: f64 = 6000.0;
const PLAYER_RADIUS: f64 = 30.0;
const PLAYER_RADIUS_PULSE: f64 = 0.0;
const PLAYER_FRICTION: f64 = 0.9;
const PLAYER_FRICTION_MULTIPLIER: f64 = 60.0;

const BALL_SPEED: f64 = 100.0;
const BALL_RADIUS: f64 = 35.0;
const BALL_COUNT: i32 = 2;
const BALL_FRICTION: f64 = 0.99;
const BALL_FRICTION_MULTIPLIER: f64 = 20.0;

const ARENA_WIDTH: f64 = 1500.0;
const ARENA_HEIGHT: f64 = 1000.0;
const ZOOM: f64 = 1000.0;

struct GameState {
	player: Player,
	arena: Arena,
	render_debug: bool,
	ball: Vec<Ball>,
}

#[derive(Debug)]
struct Vector {
	x: f64,
	y: f64,
}

impl Vector {
	fn new(x: f64, y: f64) -> Vector {
		Vector { x, y }
	}
}


struct Arena {
	x: f64,
	y: f64,
	width: f64,
	height: f64,
}

#[derive(Clone, Copy, Debug)]
struct Bound {
	x: f64,
	y: f64,
	width: f64,
	height: f64,
}

struct Player {
	x: f64,
	y: f64,
	xv: f64,
	yv: f64,
	radius: f64,
	accel: f64,
	timer: f64,
	boost: bool,
}

struct Circle {
	x: f64,
	y: f64,
	radius: f64,
}

impl Circle {
	fn from_player(player: &Player) -> Circle {
		Circle {
			x: player.x,
			y: player.y,
			radius: player.radius
		}
	}
	fn from_ball(ball: &Ball) -> Circle {
		Circle {
			x: ball.x,
			y: ball.y,
			radius: ball.radius
		}
	}
}

#[derive(Clone, Copy, Debug)]
struct Ball {
	x: f64,
	y: f64,
	xv: f64,
	yv: f64,
	radius: f64,
	id: f64,
	bound: Bound,
}



impl Ball {
	fn new(speed: f64, bound: Bound) -> Ball {
		let angle = random() * 360.0;
		let radius = BALL_RADIUS;
		let x = random() * (bound.width - radius) + bound.x + radius;
      let y = random() * (bound.height - radius * 2.0) + bound.y + radius;
		Ball {
			x,
			y,
			bound,
			id: random(),
			radius,
			xv: angle.cos() * speed,
			yv: angle.sin() * speed,
		}
	}
	fn update(&mut self, player: &mut Player, delta: &f64) {
	  self.xv *= BALL_FRICTION.powf(delta * BALL_FRICTION_MULTIPLIER);
	  self.yv *= BALL_FRICTION.powf(delta * BALL_FRICTION_MULTIPLIER);
	  self.x += self.xv * delta;
     if self.x + self.radius >= self.bound.x + self.bound.width {
         self.xv = -self.xv;
         self.x = (self.bound.x + self.bound.width) * 2.0 - self.x - self.radius * 2.0;
     }
     else if self.x - self.radius <= self.bound.x {
         self.xv = -self.xv;
         self.x = self.bound.x * 2.0 - self.x + self.radius * 2.0;
     }
     self.y += self.yv * delta;
     if self.y + self.radius >= self.bound.y + self.bound.height {
         self.yv = -self.yv;
         self.y = (self.bound.y + self.bound.height) * 2.0 - self.y - self.radius * 2.0;
     }
     else if self.y - self.radius <= self.bound.y {
         self.yv = -self.yv;
         self.y = self.bound.y * 2.0 - self.y + self.radius * 2.0;
     }
     if self.intersect_circle(Circle::from_player(player)) {
     		let v_collision = Vector::new((player.x - self.x) as f64, (player.y - self.y) as f64);
			let distance = ((player.x - self.x) * (player.x - self.x) + (player.y - self.y) * (player.y - self.y)).sqrt();
			let v_collision_norm = Vector::new(v_collision.x / distance, v_collision.y / distance);
			let v_relative_velocity = Vector::new(self.xv - player.xv, self.yv - player.yv);
			let speed = v_relative_velocity.x * v_collision_norm.x + v_relative_velocity.y * v_collision_norm.y;
			(if speed < 0.0 { return; });
			let impulse = 2.0 * speed / (self.radius + player.radius - 8.0); // mass -> radius
			self.xv -= impulse * player.radius * v_collision_norm.x * 1.2;
			self.yv -= impulse * player.radius * v_collision_norm.y * 1.2;
			player.xv += impulse * self.radius * v_collision_norm.x;
			player.yv += impulse * self.radius * v_collision_norm.y;
     }
	}
	fn intersect_circle(&self, circle: Circle) -> bool {
		return (self.x - circle.x) * (self.x - circle.x) + (self.y - circle.y) * (self.y - circle.y)
			<= (circle.radius + self.radius) * (circle.radius + self.radius)
	}
	fn collide(ball: &mut Vec<Ball>) {
		let length = ball.len();
		for i in 0..length {
			for j in 1..length {
				if ball[i].id != ball[j].id && ball[i].intersect_circle(Circle::from_ball(&ball[j])) {
					let v_collision = Vector::new((ball[j].x - ball[i].x) as f64, (ball[j].y - ball[i].y) as f64);
					let distance = ((ball[j].x - ball[i].x) * (ball[j].x - ball[i].x) + (ball[j].y - ball[i].y) * (ball[j].y - ball[i].y)).sqrt();
					let v_collision_norm = Vector::new(v_collision.x / distance, v_collision.y / distance);
					let v_relative_velocity = Vector::new(ball[i].xv - ball[j].xv, ball[i].yv - ball[j].yv);
					let speed = v_relative_velocity.x * v_collision_norm.x + v_relative_velocity.y * v_collision_norm.y;
					(if speed < 0.0 { continue; });
					let impulse = 2.0 * speed / (ball[i].radius + ball[j].radius - 8.0); // mass -> radius
					ball[i].xv -= impulse * ball[j].radius * v_collision_norm.x;
					ball[i].yv -= impulse * ball[j].radius * v_collision_norm.y;
					ball[j].xv += impulse * ball[i].radius * v_collision_norm.x;
					ball[j].yv += impulse * ball[i].radius * v_collision_norm.y;
				}
			}
		}
	}
}

impl Bound {
	fn from(arena: &Arena) -> Bound {
		Bound {
			x: arena.x,
			y: arena.y,
			width: arena.width,
			height: arena.height,
		}
	}
}


impl Player {
	fn new(x: f64, y: f64) -> Player {
		Player { x, y, xv: 0.0, yv: 0.0, radius: PLAYER_RADIUS, timer: 0.0, boost: false, accel: PLAYER_ACCEL }
	}
	fn update(&mut self, arena: &Arena, delta: &f64)  {
		// update timer
	  self.timer += delta;
		// move
	  // if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
	  //     self.xv += PLAYER_ACCEL * delta;
	  // }
	  // if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
	  //     self.xv -= PLAYER_ACCEL * delta;
	  // }
	  // if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
	  //     self.yv += PLAYER_ACCEL * delta;
	  // }
	  // if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
	  //     self.yv -= PLAYER_ACCEL * delta;
	  // }
	  if is_mouse_button_down(MouseButton::Left) {
     		self.accel = PLAYER_BOOST_MAX_ACCEL;
     		self.boost = true;
     } else {
     		self.boost = false;
     		self.accel = PLAYER_ACCEL;
     }
	  let (mouse_x, mouse_y) = mouse_position();
	  let dist = Vector::new((mouse_x - screen_width() / 2.0).into(), (mouse_y - screen_height() / 2.0).into());
	  let distance = min((dist.x * dist.x + dist.y * dist.y).sqrt(), 80.0);
	  let angle = dist.y.atan2(dist.x);
	  self.xv += angle.cos() * self.accel * delta;
	  self.yv += angle.sin() * self.accel * delta;
	  self.xv *= PLAYER_FRICTION.powf(delta * PLAYER_FRICTION_MULTIPLIER);
	  self.yv *= PLAYER_FRICTION.powf(delta * PLAYER_FRICTION_MULTIPLIER);
	  // bound velocity
	  // if self.xv > PLAYER_MAX_SPEED {
	  // 		self.xv = PLAYER_MAX_SPEED;
	  // }
	  // if self.xv < -PLAYER_MAX_SPEED {
	  // 		self.xv = -PLAYER_MAX_SPEED;
	  // }
	  // if self.yv > PLAYER_MAX_SPEED {
	  // 	   self.yv = PLAYER_MAX_SPEED;
	  // }
	  // if self.yv < -PLAYER_MAX_SPEED {
	  // 	   self.yv = -PLAYER_MAX_SPEED;
	  // }
	  self.x += self.xv * (distance / 80.0) * delta;
	  self.y += self.yv * (distance / 80.0) * delta;
	  // self.x += self.xv * delta;
	  // self.y += self.yv * delta;
	  // change radius
	  self.radius = PLAYER_RADIUS + self.timer.sin() * PLAYER_RADIUS_PULSE;
	  // bound
	  if self.x + self.radius > arena.x +arena.width {
	  		self.x = (arena.x + arena.width) - self.radius;
	  }
	  if self.x - self.radius < arena.x {
	  		self.x = arena.x + self.radius;
	  }
	  if self.y + self.radius > arena.y + arena.height {
	  		self.y = (arena.y + arena.height) - self.radius;
	  }
	  if self.y - self.radius < arena.y {
	  		self.y = arena.y + self.radius;
	  }
	}
}

impl Arena {
	fn new(x: f64, y: f64, width: f64, height: f64) -> Arena {
		Arena { x, y, width, height }
	}
}

impl GameState {
	fn new() -> GameState {
		 GameState {
		 	player: Player::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0),
		 	arena: Arena::new(0.0, 0.0, ARENA_WIDTH, ARENA_HEIGHT),
		 	render_debug: false,
		 	ball: Vec::new(),
		 }
	}
}



fn update(state: &mut GameState, delta: &f64) {
	state.player.update(&state.arena, delta);
	for i in 0..state.ball.len() {
		state.ball[i].update(&mut state.player, delta);
	}
	Ball::collide(&mut state.ball);
}

fn render(state: &GameState) {
	let scale: f64 = get_scale();
	let width: f64 = screen_width().into();
	let height: f64 = screen_height().into();
	clear_background(GREEN);
	let arena_x = ((state.arena.x - state.player.x) * scale + width / 2.0) as f32;
   let arena_y = ((state.arena.y - state.player.y) * scale + height / 2.0) as f32;
   // draw arena
	draw_rectangle(arena_x, arena_y, (state.arena.width * scale).round() as f32, (state.arena.height * scale).round() as f32, DARKGREEN);
	// draw ball
	for ball in &state.ball {
		let ball_x = ((ball.x - state.player.x) * scale + width / 2.0) as f32;
		let ball_y = ((ball.y - state.player.y) * scale + height / 2.0) as f32;
		circle(ball_x, ball_y, (ball.radius * scale) as f32, WHITE);
	}
	// draw player
	circle((width / 2f64) as f32, (height / 2f64) as f32, (state.player.radius * scale) as f32, BLACK);
	if state.player.boost {
		stroke_circle((width / 2f64) as f32, (height / 2f64) as f32, (state.player.radius * scale) as f32, 4.0, WHITE);
	}
}

fn circle(x: f32, y: f32, r: f32, color: Color) {
    draw_poly(x, y, 10, r, 0., color);
}

fn stroke_circle(x: f32, y: f32, r: f32, thickness: f32, color: Color) {
    draw_poly_lines(x, y, 200, r, 0., thickness, color);
}

fn max(first: f64, second: f64) -> f64 {
	if first >= second {
		first
	} else {
		second
	}
}

fn min(first: f64, second: f64) -> f64 {
	if first <= second {
		first
	} else {
		second
	}
}

fn random () -> f64 {
	rand::gen_range(0.0, 1.0) as f64
}

fn get_scale() -> f64 {
   return max(screen_height() as f64, screen_width() as f64 * (9.0 / 16.0)) as f64 / ZOOM as f64;
}


#[macroquad::main("InputKeys")]
async fn main() {
   let mut state = GameState::new();

   for _i in 0..BALL_COUNT {
   	state.ball.push(Ball::new(BALL_SPEED, Bound::from(&state.arena)));
   }

    // let mut processing_time: f64 = 0.0;
    // let mut milli: f64 = 0.0;

    loop {
    	  // let before = Instant::now();
    	  let delta = get_frame_time();

    	  // milli += delta as f64;
    	  // if milli >= 1 as f64 {
    	  // 		// println!("processing milliseconds: {}", processing_time / 1000.0);
    	  // 		processing_time = 0.0;
    	  // 		milli = 0.0;
    	  // }
    	  render(&state);
    	  // processing_time += before.elapsed().as_micros() as f64;
    	  update(&mut state, &(delta as f64));
    	  state.render_debug = false;
        next_frame().await
    }
}