#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
	Floor,
	Wall,
}

pub struct Map {
	pub tiles: Vec<Tile>,
	pub width: usize,
	pub height: usize
}

impl Map {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width, height,
			tiles: vec![Tile::Floor; width * height]
		}
	}

	pub fn xy_index(&self, x: i32, y: i32) -> usize {
		if x < 0 || y < 0 {
			0
		} else {
			self.width * y as usize + x as usize
		}
	}

	pub fn is_solid(&self, x: i32, y: i32) -> bool {
		let index = self.xy_index(x, y);
		self.tiles[index] == Tile::Wall
	}
}

fn draw_rect(
	map: &mut Map, 
	x1: i32, y1: i32, 
	x2: i32, y2: i32, 
	tile: Tile
) {
	for x in (x1.min(x2))..(x1.max(x2)) {
		for y in (y1.min(y2))..(y1.max(y2)) {
			let index = map.xy_index(x, y);
			map.tiles[index] = tile.clone();
		}
	}
}

pub fn generate_map() -> Map {
	let mut map = Map::new(100, 100);

	draw_rect(&mut map, 20, 20, 40, 40, Tile::Wall);
	draw_rect(&mut map, 60, 60, 80, 80, Tile::Wall);
	draw_rect(&mut map, 20, 60, 40, 80, Tile::Wall);
	draw_rect(&mut map, 60, 20, 80, 40, Tile::Wall);

	map
}