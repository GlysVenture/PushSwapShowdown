use crate::display::draw::Rectangle;

pub struct GuiButton {
	space: Rectangle,
	state: u8,
	height: u32,
}

impl GuiButton {
	pub fn new(space: Rectangle) -> Self{
		Self {
			space,
			state: 0,
			height: 4,
		}
	}

	pub fn draw(&self, frame: &mut [u8], size: [u32; 2]){
		self.space.draw(frame, size);
		if self.state == 0 {
			 Rectangle::new(self.space.posx,
							self.space.posy,
							self.height,
							self.space.y)
				.color([100, 100, 100, 150]).draw(frame, size);
			Rectangle::new(self.space.posx,
							self.space.posy + self.space.y - self.height,
							self.space.x,
							self.height)
				.color([100, 100, 100, 150]).draw(frame, size);
		} else {
			Rectangle::new(self.space.posx,
							self.space.posy,
							self.space.x,
							self.height)
				.color([100, 100, 100, 150]).draw(frame, size);
			Rectangle::new(self.space.posx + self.space.x - self.height,
							self.space.posy,
							self.height,
							self.space.y)
				.color([100, 100, 100, 150]).draw(frame, size);
		}
	}

	pub fn click(&mut self, coord: [u32; 2]) {
		if self.space.posx <= coord[0] && coord[0] < self.space.x + self.space.posx
			&& self.space.posy <= coord[1] && coord[1] < self.space.y + self.space.posy {
			self.state = self.state ^ 1;
		}
	}

	pub fn get_state(&self) -> u8{
		self.state
	}

	pub fn set_state(&mut self, state: u8) {
		self.state = state;
	}
}
