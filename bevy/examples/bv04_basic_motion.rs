use bevy::{
	window::PresentMode,
	prelude::*,	
};

const TITLE: &str = "bv03 Basic Motion";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SZ: f32 = 32.;



#[derive(Component)]
struct Player;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from(TITLE),
			width: WIN_W,
			height: WIN_H,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::DARK_GRAY))
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(move_player)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default());

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::SEA_GREEN,
				custom_size: Some(Vec2::splat(PLAYER_SZ)),
				..default()
			},
			..default()
		})
		.insert(Player);
}


fn move_player(
	input: Res<Input<KeyCode>>,
	mut player: Query<&mut Transform, With<Player>>,
){
	let mut player_transform = player.single_mut();

	let mut x_vel = 0.;
	let mut y_vel = 0.;
	

	if input.pressed(KeyCode::A) {
		x_vel -= 5.;
		
	}

	if input.pressed(KeyCode::D) {
		x_vel += 5.;
	}

	if input.pressed(KeyCode::W) {
		y_vel += 5.;
			
	}

	if input.pressed(KeyCode::S) {
		y_vel -= 5.;
		
	}

	if input.pressed(KeyCode::S) && input.pressed(KeyCode::A){
		y_vel = -2.5;
		x_vel = -2.5;
		
	}
	if input.pressed(KeyCode::S) && input.pressed(KeyCode::D){
		y_vel = -2.5;
		x_vel = 2.5;
		
	}
	if input.pressed(KeyCode::W) && input.pressed(KeyCode::A){
		y_vel = 2.5;
		x_vel = -2.5;
		
	}
	if input.pressed(KeyCode::W) && input.pressed(KeyCode::D){
		y_vel = 2.5;
		x_vel = 2.5;
		
	}
	
	
	player_transform.translation.x += x_vel;
	player_transform.translation.y += y_vel;

	player_transform.translation.x = player_transform.translation.x.clamp((-WIN_W/2.) + (PLAYER_SZ/2.), (WIN_W/2.) - (PLAYER_SZ/2.));
	player_transform.translation.y = player_transform.translation.y.clamp((-WIN_H/2.) + (PLAYER_SZ/2.) , (WIN_H/2.) - (PLAYER_SZ/2.));
}

/* TODO: 
 * Can we slowly ramp up to speed limit instead of instantly hitting it?
 * Can we normalize behavior on different refresh rates?
 * How do we stay inside the window?
 * How do we avoid breaking the speed limit on the diagonal?
 */


