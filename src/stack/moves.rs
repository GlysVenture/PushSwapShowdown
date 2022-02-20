use crate::stack::Stacks;

impl Stacks {
	pub fn m_pa(&mut self) {
		if self.b.len() > 0 {
			self.a.insert(0, self.b[0]);
			self.b.remove(0);
		}
	}

	pub fn m_pb(&mut self) {
		if self.a.len() > 0 {
			self.b.insert(0, self.a[0]);
			self.a.remove(0);
		}
	}

	pub fn m_sa(&mut self) {
		if self.a.len() > 1 {
			self.a.swap(0, 1);
		}
	}

	pub fn m_sb(&mut self) {
		if self.b.len() > 1 {
			self.b.swap(0, 1);
		}
	}

	pub fn m_ra(&mut self) {
		if self.a.len() > 0 {
			let temp = self.a[0];
			self.a.remove(0);
			self.a.push(temp);
		}
	}

	pub fn m_rra(&mut self) {
		if self.a.len() > 0 {
			let temp = self.a[self.a.len() - 1];
			self.a.pop();
			self.a.insert(0, temp);
		}
	}

	pub fn m_rb(&mut self) {
		if self.b.len() > 0 {
			let temp = self.b[0];
			self.b.remove(0);
			self.b.push(temp);
		}
	}

	pub fn m_rrb(&mut self) {
		if self.b.len() > 0 {
			let temp = self.b[self.b.len() - 1];
			self.b.pop();
			self.b.insert(0, temp);
		}
	}

	pub fn m_ss(&mut self){
		self.m_sa();
		self.m_sb();
	}

	pub fn m_rr(&mut self){
		self.m_ra();
		self.m_rb();
	}

	pub fn m_rrr(&mut self){
		self.m_rra();
		self.m_rrb();
	}
}
