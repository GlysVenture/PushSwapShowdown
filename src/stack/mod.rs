mod moves;

use std::io::Split;
use crate::display::draw::Rectangle;
use crate::display::WIN_SIZE;

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
		space.x = space.x / 2;
		space.y = space.y / self.max_len as u32;
		self.display_stack(&self.a, frame, &space);
		space.posx += space.x;
		self.display_stack(&self.b, frame, &space);
	}

	fn display_stack(&self, stack: &Vec<i32>, frame: &mut [u8], space: &Rectangle) {
		let mut rec = Rectangle::new(0, 0, space.x, space.y);
		for (i, elem) in stack.iter().enumerate() {
			rec.posy = space.posy + i as u32 * space.y;
			rec.posx = space.posx;
			let shift = (elem - self.min) as f64 / (self.max - self.min) as f64;
			rec.x = (0.9 * space.x as f64 * shift + 0.1 * space.x as f64) as u32;
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
				_=> { },
			}
			str.remove(0);
		}
	}
}
