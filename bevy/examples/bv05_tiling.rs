use bevy::{
	window::PresentMode,
	prelude::*,	
};
use rand::Rng;

const TITLE: &str = "Tiling";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const TILE_SIZE: f32 = 100.;
const NUM_BIRDS: usize = 8;
const NUM_TILES: usize = 13;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Brick;

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
		.run();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let bird_handle = asset_server.load("birds.png");
	let bird_atlas = TextureAtlas::from_grid(bird_handle, Vec2::splat(TILE_SIZE), 2, 2);
	let bird_atlas_len = bird_atlas.textures.len();
	let bird_atlas_handle = texture_atlases.add(bird_atlas);

	//TODO: load the sprite sheet stored in `bricks.png`

	let brick_handle = asset_server.load("bricks.png");
	let brick_atlas = TextureAtlas::from_grid(brick_handle, Vec2::splat(TILE_SIZE), 4, 1);
	let brick_atlas_len = brick_atlas.textures.len();
	let brick_atlas_handle = texture_atlases.add(brick_atlas);

	commands.spawn_bundle(Camera2dBundle::default());

	let mut rng = rand::thread_rng();
	let x_bound = WIN_W/2. - TILE_SIZE/2.;
	let y_bound = WIN_H/2. - TILE_SIZE/2.;

	for i in 0..NUM_BIRDS {
		let t = Vec3::new(
			rng.gen_range(-x_bound..x_bound),
			rng.gen_range(-y_bound..y_bound),
			900.,
		);
		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: bird_atlas_handle.clone(),
				transform: Transform {
					translation: t,
					..default()
				},
				sprite: TextureAtlasSprite {
					index: i % bird_atlas_len,
					..default()
				},
				..default()
			})
			.insert(Bird);

	}

	//TODO: Place brick tiles along the bottom of the entire window
	let mut count: f32 = 0.;

	for i in 0..NUM_TILES {
		
		let t = Vec3::new(
			-x_bound + (TILE_SIZE * count),
			-y_bound,
			800.,
		);
		count += 1.;
		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: brick_atlas_handle.clone(),
				transform: Transform {// moving the actual sprite
					translation: t,
					..default()
				},
				sprite: TextureAtlasSprite {
					index: i % brick_atlas_len,
					..default()
				},
				..default()
			})
			.insert(Brick);
		}
			
	
}
