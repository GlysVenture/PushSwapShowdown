mod moves;

use crate::display::draw::Rectangle;
use crate::display::WIN_SIZE;

pub(crate) const STACK_BORDER: u32 = 5;

pub(crate) struct Stacks {
	a: Vec<i32>,
	b: Vec<i32>,
	max_len: usize,
	max: i32,
	min: i32,
}

impl Stacks {
	pub fn new(list: Vec<i32>) -> Self {
		Self {
			max_len: list.len(),
			max: *list.iter().max().unwrap(),
			min: *list.iter().min().unwrap(),
			a: list,
			b: vec![],
		}
	}

	pub fn draw(&self, frame: &mut [u8], mut space: Rectangle) {
		space.color([150, 150, 150, 0xff]);
		space.draw(frame, WIN_SIZE);
		space.x = space.x / 2;
		space.y = (space.y - STACK_BORDER * 2) / self.max_len as u32;
		self.display_stack(&self.a, frame, &space);
		space.posx += space.x;
		self.display_stack(&self.b, frame, &space);
	}

	fn display_stack(&self, stack: &Vec<i32>, frame: &mut [u8], space: &Rectangle) {
		let coords: [u32; 2] = [space.posx + STACK_BORDER, space.posy + STACK_BORDER];
		let mut rec = Rectangle::new(0, 0, space.x - STACK_BORDER * 2, space.y);
		for (i, elem) in stack.iter().enumerate() {
			rec.posy = coords[1] + i as u32 * space.y;
			rec.posx = coords[0];
			let shift = (elem - self.min) as f64 / (self.max - self.min) as f64;
			rec.x = (0.9 * (space.x - STACK_BORDER * 2) as f64 * shift + 0.1 * (space.x - STACK_BORDER * 2) as f64) as u32;
			rec.color([0xff, (0xff as f64 * shift) as u8, 0, 0xff]);
			rec.draw(frame, WIN_SIZE);
		}
	}

	pub(crate) fn next_move(&mut self, str: &mut Vec<String>) {
		if str.len() > 0 {
			match str[0].as_str() {
				"pa" => self.m_pa(),
				"pb" => self.m_pb(),
				"sa" => self.m_sa(),
				"sb" => self.m_sb(),
				"ra" => self.m_ra(),
				"rra" => self.m_rra(),
				"rb" => self.m_rb(),
				"rrb" => self.m_rrb(),
				"ss" => self.m_ss(),
				"rr" => self.m_rr(),
				"rrr" => self.m_rrr(),
				_=> { },
			}
			str.remove(0);
		}
	}
}
