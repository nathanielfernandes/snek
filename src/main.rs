use macroquad::{prelude::*, rand::gen_range};
use std::collections::VecDeque;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const SIZE: f32 = 20.0;

struct Snek {
    body: VecDeque<Vec2>,
    dir: Vec2,

    tick: i32,
    initial_safety: i32,
}

impl Snek {
    const TPS: i32 = 15;

    fn new(segments: i32) -> Self {
        let mut body = VecDeque::new();
        body.push_back(vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0));

        let mut snek = Self {
            body,
            dir: Vec2::NEG_Y,

            tick: 60 / Self::TPS,
            initial_safety: segments + 1,
        };

        for _ in 0..segments {
            snek.add_segment();
        }

        snek
    }

    fn update(&mut self) {
        if self.tick > 0 {
            self.tick -= 1;
            return;
        }

        self.tick = 60 / Self::TPS;

        let head = self.body[0];
        let new_head = head + self.dir * SIZE;

        // let snake wrap around the screen
        let new_head = vec2(
            (new_head.x + WIDTH as f32) % WIDTH as f32,
            (new_head.y + HEIGHT as f32) % HEIGHT as f32,
        );

        self.body.push_front(new_head);
        self.body.pop_back();
    }

    fn check_food(&mut self, food: &Vec2) -> bool {
        let head = self.body[0];
        if head.x == food.x && head.y == food.y {
            self.add_segment();
            true
        } else {
            false
        }
    }

    fn check_collision(&mut self) -> bool {
        if self.initial_safety > 0 {
            self.initial_safety -= 1;
            return false;
        }

        let head = self.body[0];

        for i in 1..self.body.len() {
            if head == self.body[i] {
                return true;
            }
        }

        false
    }

    fn change_direction(&mut self, dir: Vec2) {
        if dir == self.dir * -1.0 {
            return;
        }

        self.dir = dir;
    }

    fn add_segment(&mut self) {
        let tail = self.body[self.body.len() - 1];
        self.body.push_back(tail);
    }

    fn draw(&self) {
        for part in &self.body {
            draw_rectangle(part.x, part.y, SIZE - 2.0, SIZE - 2.0, GREEN);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "snek".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}

fn random_pos() -> Vec2 {
    vec2(
        gen_range(0, WIDTH / SIZE as i32) as f32 * SIZE,
        gen_range(0, HEIGHT / SIZE as i32) as f32 * SIZE,
    )
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);

    let mut snek = Snek::new(3);

    let mut food = random_pos();
    let mut score = 0;

    loop {
        // inputs
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            snek.change_direction(Vec2::NEG_Y);
        } else if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            snek.change_direction(Vec2::Y);
        } else if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            snek.change_direction(Vec2::NEG_X);
        } else if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            snek.change_direction(Vec2::X);
        }

        snek.update();
        if snek.check_collision() {
            snek = Snek::new(3);
            score = 0;
        }

        if snek.check_food(&food) {
            food = random_pos();
            score += 1;
        }

        clear_background(WHITE);

        snek.draw();

        draw_rectangle(food.x, food.y, SIZE - 2.0, SIZE - 2.0, RED);

        draw_text(&format!("Score: {}", score), 10.0, 30.0, 40.0, BLACK);
        draw_text(
            "nathanielfernandes",
            WIDTH as f32 - 170.0,
            20.0,
            20.0,
            BLACK,
        );
        draw_text("snek", WIDTH as f32 - 45.5, 35.0, 20.0, BLACK);

        next_frame().await
    }
}
