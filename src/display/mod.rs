pub mod draw;

use std::str::Split;
use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture};
use crate::display::draw::{fill, Rectangle};
use crate::stack::Stacks;

pub(crate) const WIN_SIZE: [u32; 2] = [1000, 800];

pub fn visualize(_progs: &[String], mut out1: Vec<String>, mut out2: Vec<String>, _input: Vec<i32>) {
	let event_loop = EventLoop::new();
	let window = {
		let s = LogicalSize::new(WIN_SIZE[0] as f64, WIN_SIZE[1] as f64);
		WindowBuilder::new()
			.with_title("Push Swap Showdown")
			.with_inner_size(s)
			.with_resizable(false)
			.build(&event_loop).unwrap()
	};
	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIN_SIZE[0], WIN_SIZE[1], surface_texture).unwrap()
	};
	let mut stacks = Stacks::new(_input);

	event_loop.run(move |event, _, control_flow| {
		//*control_flow = ControlFlow::Poll;
		*control_flow = ControlFlow::Wait;

		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				println!("The close button was pressed; stopping");
				*control_flow = ControlFlow::Exit
			},
			Event::MainEventsCleared => {
				//std::thread::sleep(std::time::Duration::from_millis(100));
				stacks.next_move(&mut out1);
				window.request_redraw();
			},
			Event::RedrawRequested(_) => {
				fill(pixels.get_frame(), [0xa9, 0xe8, 0x90, 0xff], WIN_SIZE);
				stacks.draw(pixels.get_frame(), Rectangle::new(10, 10, WIN_SIZE[0] / 2 - 20, WIN_SIZE[1] - 20));
				if pixels
					.render()
					.map_err(|_| eprintln!("pixels.render() failed"))
					.is_err()
				{
					*control_flow = ControlFlow::Exit;
					return;
				}
			},
			_ => ()
		}
	});
}
