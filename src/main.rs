use bevy::{prelude::*, sprite::Anchor, window::PresentMode};
use binary::{BinaryPlugin, FONT_HANDLE};
use rand::{random, Rng};

pub const WINDOW_SIZE: Vec2 = Vec2::new(800., 600.);
pub const PADDLE_SIZE: Vec2 = Vec2::new(10., 100.);

#[derive(PartialEq)]
enum Config {
	Player,
	AiOne,
	AiBoth,
}

const CONFIG: Config = Config::Player;

mod binary;

#[derive(Resource, Default)]
struct Score {
	left: u32,
	right: u32,
}
#[derive(Event)]
struct ScoreEvent;

fn main() {
	#[cfg(not(debug_assertions))]
	std::env::set_var("RUST_LOG", "");

	App::new()
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "pong".into(),
				resolution: WINDOW_SIZE.into(),
				present_mode: PresentMode::AutoVsync,
				fit_canvas_to_parent: true,
				prevent_default_event_handling: false,
				..default()
			}),
			..default()
		}))
		.add_plugins(BinaryPlugin)
		.insert_resource(Msaa::Sample8)
		.init_resource::<Score>()
		.add_event::<ScoreEvent>()
		.add_systems(Startup, setup_camera)
		.add_systems(
			Update,
			(
				move_paddle_system,
				move_ball_system,
				update_score_system,
				random_ball_dir,
			),
		)
		.run();
}

#[derive(Component, Debug)]
struct LeftPaddle;

#[derive(Component, Debug)]
struct RightPaddle;

#[derive(Component, Debug)]
struct Paddle;

#[derive(Component, Debug)]
struct LeftScoreText;

#[derive(Component, Debug)]
struct RightScoreText;

#[derive(Component, Debug)]
struct Ball {
	vel: Vec2,
	speed: f32,
}

fn spawn_paddle<T: Component, U: Component>(commands: &mut Commands, comp: T, comp2: U, pos: Vec3) {
	commands
		.spawn(SpriteBundle {
			transform: Transform {
				translation: pos,
				scale: PADDLE_SIZE.extend(0.),
				..default()
			},
			sprite: Sprite {
				color: Color::WHITE,
				..default()
			},
			..default()
		})
		.insert(comp)
		.insert(comp2);
}

fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());

	let text_style = TextStyle {
		font: FONT_HANDLE.typed(),
		font_size: 75.0,
		color: Color::WHITE,
	};

	commands
		.spawn(Text2dBundle {
			text: Text {
				sections: vec![TextSection::new(String::from("0"), text_style.clone())],
				..default()
			},
			transform: Transform::from_translation(Vec3::new(-20., (WINDOW_SIZE.y / 2.) - 55., 0.)),
			text_anchor: Anchor::CenterRight,
			..default()
		})
		.insert(LeftScoreText);

	commands
		.spawn(Text2dBundle {
			text: Text {
				sections: vec![TextSection::new(String::from("0"), text_style)],
				..default()
			},
			transform: Transform::from_translation(Vec3::new(20., (WINDOW_SIZE.y / 2.) - 55., 0.)),
			text_anchor: Anchor::CenterLeft,
			..default()
		})
		.insert(RightScoreText);

	commands.spawn(SpriteBundle {
		transform: Transform {
			translation: Vec3::new(0., (WINDOW_SIZE.y / 2.) - 10., 0.),
			scale: Vec3::new(WINDOW_SIZE.x, 20., 0.0),
			..default()
		},
		sprite: Sprite {
			color: Color::WHITE,
			..default()
		},
		..default()
	});
	commands.spawn(SpriteBundle {
		transform: Transform {
			translation: Vec3::new(0., -((WINDOW_SIZE.y / 2.) - 10.), 0.),
			scale: Vec3::new(WINDOW_SIZE.x, 20., 0.0),
			..default()
		},
		sprite: Sprite {
			color: Color::WHITE,
			..default()
		},
		..default()
	});
	let gap = WINDOW_SIZE.y - 40.;
	let inc = gap / 15.;
	for i in 0..15 {
		commands.spawn(SpriteBundle {
			transform: Transform {
				translation: Vec3::new(0., inc.mul_add(-i as f32, (WINDOW_SIZE.y / 2.) - 30.), 0.),
				scale: Vec3::new(20., 20., 0.0),
				..default()
			},
			sprite: Sprite {
				color: Color::WHITE,
				..default()
			},
			..default()
		});
	}

	let mut rng = rand::thread_rng();

	let angle = if random::<bool>() {
		f32::from(rng.gen_range(-45.0..=45.0)).to_radians()
	} else {
		f32::from(rng.gen_range(135.0..=225.0)).to_radians()
	};

	let vel = Vec2::from_angle(angle).normalize();

	commands
		.spawn(SpriteBundle {
			transform: Transform {
				translation: Vec3::new(0., 0., 0.),
				scale: Vec3::new(20., 20., 0.0),
				..default()
			},
			sprite: Sprite {
				color: Color::WHITE,
				..default()
			},
			..default()
		})
		.insert(Ball { vel, speed: 200. });

	spawn_paddle(
		&mut commands,
		Paddle,
		LeftPaddle,
		Vec3::new(-((WINDOW_SIZE.x / 2.) - 20.), 0., 0.),
	);

	spawn_paddle(
		&mut commands,
		Paddle,
		RightPaddle,
		Vec3::new((WINDOW_SIZE.x / 2.) - 20., 0., 0.),
	);
}

fn move_paddle_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut ball: Query<(
		&mut Ball,
		&Transform,
		Without<LeftPaddle>,
		Without<RightPaddle>,
	)>,
	mut left_paddle: Query<(&Paddle, &mut Transform, With<LeftPaddle>)>,
	mut right_paddle: Query<(
		&Paddle,
		&mut Transform,
		With<RightPaddle>,
		Without<LeftPaddle>,
	)>,
) {
	let mut left_paddle = left_paddle.get_single_mut().unwrap();
	let mut right_paddle = right_paddle.get_single_mut().unwrap();
	let mut ball = ball.get_single_mut().unwrap();
	let top = (WINDOW_SIZE.y / 2.) - (PADDLE_SIZE.y / 2.) - 20.;
	let bottom = -top;
	let speed = 700.;

	match CONFIG {
		Config::Player => {
			if (keyboard_input.pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::W))
				&& left_paddle.1.translation.y <= top
			{
				left_paddle.1.translation.y += speed * time.delta_seconds();
			}

			if (keyboard_input.pressed(KeyCode::S) || keyboard_input.just_pressed(KeyCode::S))
				&& left_paddle.1.translation.y >= bottom
			{
				left_paddle.1.translation.y -= speed * time.delta_seconds();
			}

			if (keyboard_input.pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::Up))
				&& right_paddle.1.translation.y <= top
			{
				right_paddle.1.translation.y += speed * time.delta_seconds();
			}

			if (keyboard_input.pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::Down))
				&& right_paddle.1.translation.y >= bottom
			{
				right_paddle.1.translation.y -= speed * time.delta_seconds();
			}
		}
		Config::AiOne => {
			ball.0.speed = 200.;
			left_paddle.1.translation.y = ball.1.translation.y;
			if (keyboard_input.pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::Up))
				&& right_paddle.1.translation.y <= top
			{
				right_paddle.1.translation.y += speed * time.delta_seconds();
			}

			if (keyboard_input.pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::Down))
				&& right_paddle.1.translation.y >= bottom
			{
				right_paddle.1.translation.y -= speed * time.delta_seconds();
			}
		}
		Config::AiBoth => {
			ball.0.speed = 2000.;
			if ball.1.translation.x <= 0. {
				left_paddle.1.translation.y = ball.1.translation.y;
			} else {
				right_paddle.1.translation.y = ball.1.translation.y;
			}
		}
	}
}

fn move_ball_system(
	time: Res<Time>,
	mut ball: Query<(&mut Ball, &mut Transform)>,
	mut left_paddle: Query<(&Paddle, &mut Transform, With<LeftPaddle>, Without<Ball>)>,
	mut right_paddle: Query<(
		&Paddle,
		&mut Transform,
		With<RightPaddle>,
		Without<LeftPaddle>,
		Without<Ball>,
	)>,
	mut score: ResMut<Score>,
	mut ev_score: EventWriter<ScoreEvent>,
) {
	let mut ball = ball.get_single_mut().unwrap();
	let mut left_paddle = left_paddle.get_single_mut().unwrap();
	let mut right_paddle = right_paddle.get_single_mut().unwrap();

	let top = (WINDOW_SIZE.y / 2.) - (ball.1.scale.y / 2.) - 20.;
	let bottom = -top;
	let left = (WINDOW_SIZE.x / 2.) - (ball.1.scale.x / 2.);
	let right = -left;

	ball.1.translation.x += ball.0.vel.x * time.delta_seconds() * ball.0.speed;
	ball.1.translation.y += ball.0.vel.y * time.delta_seconds() * ball.0.speed;

	if ball.1.translation.y >= top {
		ball.0.vel.y *= -1.;
	}

	if ball.1.translation.y <= bottom {
		ball.0.vel.y *= -1.;
	}

	if ball.1.translation.x >= right_paddle.1.translation.x - PADDLE_SIZE.x
		&& ball.1.translation.y <= right_paddle.1.translation.y + (PADDLE_SIZE.y / 2.)
		&& ball.1.translation.y >= right_paddle.1.translation.y - (PADDLE_SIZE.y / 2.)
	{
		ball.0.vel.x *= -1.;
	}

	if ball.1.translation.x <= left_paddle.1.translation.x + PADDLE_SIZE.x
		&& ball.1.translation.y <= left_paddle.1.translation.y + (PADDLE_SIZE.y / 2.)
		&& ball.1.translation.y >= left_paddle.1.translation.y - (PADDLE_SIZE.y / 2.)
	{
		ball.0.vel.x *= -1.;
	}

	if ball.1.translation.x >= left {
		ball.1.translation.x = 0.;
		ball.1.translation.y = 0.;
		left_paddle.1.translation.y = 0.;
		right_paddle.1.translation.y = 0.;
		score.left += 1;
		ev_score.send(ScoreEvent);
	}

	if ball.1.translation.x <= right {
		ball.1.translation.x = 0.;
		ball.1.translation.y = 0.;
		left_paddle.1.translation.y = 0.;
		right_paddle.1.translation.y = 0.;
		score.right += 1;
		ev_score.send(ScoreEvent);
	}
}

fn update_score_system(
	score: Res<Score>,
	mut left_score_text: Query<(&mut Text, With<LeftScoreText>, Without<RightScoreText>)>,
	mut right_score_text: Query<(&mut Text, With<RightScoreText>, Without<LeftScoreText>)>,
) {
	left_score_text.get_single_mut().unwrap().0.sections[0].value = score.left.to_string();
	right_score_text.get_single_mut().unwrap().0.sections[0].value = score.right.to_string();
}

fn random_ball_dir(
	mut ball: Query<(&mut Ball, &mut Transform)>,
	mut ev_score: EventReader<ScoreEvent>,
) {
	let mut ball = ball.get_single_mut().unwrap();

	let mut rng = rand::thread_rng();
	for _ in ev_score.iter() {
		let angle = if random::<bool>() {
			f32::from(rng.gen_range(-45.0..=45.0)).to_radians()
		} else {
			f32::from(rng.gen_range(135.0..=225.0)).to_radians()
		};

		let vel = Vec2::from_angle(angle).normalize();

		ball.0.vel = vel;
	}
}
