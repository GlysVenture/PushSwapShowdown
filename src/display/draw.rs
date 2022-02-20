pub struct Rectangle {
	pub(crate) posx: u32,
	pub(crate) posy: u32,
	pub(crate) x: u32,
	pub(crate) y: u32,
	pub(crate) color: [u8; 4],
}

pub fn putpixel(frame: &mut [u8], color: [u8; 4], size: [u32; 2], pos: [u32; 2]) {
	if pos[0] < size[0] && pos[1] < size[1] {
		let loc: usize = ((pos[1] * size[0] + pos[0]) * 4) as usize;
		let pixel = frame.get_mut(loc..(loc + 4)).unwrap();
		pixel.copy_from_slice(&color);
	}
}

pub fn fill(frame: &mut [u8], color: [u8; 4], size: [u32; 2]) {
	for x in 0..size[0] {
		for y in 0..size[1] {
			putpixel(frame, color, size, [x, y]);
		}
	}
}

impl Rectangle {
	pub fn new(posx: u32, posy: u32, x: u32, y: u32) -> Self {
		Self {
			posx,
			posy,
			x,
			y,
			color: [0, 0, 0, 0],
		}
	}
	pub fn color(&mut self, color: [u8; 4]) -> &mut Rectangle {
		self.color = color;
		self
	}
	pub fn draw(&self, frame: &mut [u8], size: [u32; 2]) {
		for x in self.posx..(self.x + self.posx) {
			for y in self.posy..(self.y + self.posy) {
				putpixel(frame, self.color, size, [x, y]);
			}
		}
	}
}
