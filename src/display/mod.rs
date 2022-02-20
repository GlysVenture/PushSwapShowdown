pub mod draw;
mod button;

use std::str::Split;
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}, event, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use pixels::{Pixels, SurfaceTexture};
use winit::event::DeviceEvent::Button;
use winit::event::VirtualKeyCode::W;
use crate::display::button::GuiButton;
use crate::display::draw::{fill, Rectangle};
use crate::stack::Stacks;

pub(crate) const WIN_SIZE: [u32; 2] = [1600, 800];
pub(crate) const BORDER: u32 = 10;
pub(crate) const MID: [u32; 2] = [200, 800 - BORDER * 2];
pub(crate) const BOT: u32 = 100;

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
	let mut mouse: [u32; 2] = [0, 0];

	let mut stacks = Stacks::new(_input.clone());
	let mut stacks2 = Stacks::new(_input);

	let mut start_stop = GuiButton::new(Rectangle {
		posx: (WIN_SIZE[0] - MID[0]) / 2 + BORDER,
		posy: (WIN_SIZE[1]) / 2 + BORDER,
		x: MID[0] - BORDER * 2,
		y: MID[0] - BORDER * 2,
		color: [0, 100, 200, 0xff]
	});
	let mut step = GuiButton::new(Rectangle {
		posx: (WIN_SIZE[0] - MID[0]) / 2 + BORDER,
		posy: (WIN_SIZE[1]) / 2 - MID[0] + BORDER,
		x: MID[0] - BORDER * 2,
		y: MID[0] - BORDER * 2,
		color: [200, 0, 100, 0xff]
	});

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
			Event::WindowEvent {
				event: WindowEvent::CursorMoved {
					position,
					..
				},
				..
			} => {
				mouse = position.into();
			},
			Event::WindowEvent {
				event: WindowEvent::MouseInput {
					button: winit::event::MouseButton::Left,
					state: winit::event::ElementState::Pressed,
					..
				},
				..
			} => {
				start_stop.click(mouse);
				step.click(mouse);
			},
			Event::MainEventsCleared => {
				//std::thread::sleep(std::time::Duration::from_millis(100));
				if start_stop.get_state() == 1  {
					stacks.next_move(&mut out1);
					stacks2.next_move(&mut out2);
				}
				if step.get_state() == 1 {
					stacks.next_move(&mut out1);
					stacks2.next_move(&mut out2);
					step.set_state(0);
				}
				window.request_redraw();
			},
			Event::RedrawRequested(_) => {
				fill(pixels.get_frame(), [200, 200, 200, 0xff], WIN_SIZE);
				stacks.draw(pixels.get_frame(), Rectangle::new(BORDER,
															   BORDER,
															   (WIN_SIZE[0] - MID[0]) / 2 - BORDER * 2,
															   WIN_SIZE[1] - BORDER * 2 - BOT));
				stacks2.draw(pixels.get_frame(), Rectangle::new((WIN_SIZE[0] + MID[0]) / 2 + BORDER,
																BORDER,
																(WIN_SIZE[0] - MID[0]) / 2 - BORDER * 2,
																WIN_SIZE[1] - BORDER * 2 - BOT));
				start_stop.draw(pixels.get_frame(), WIN_SIZE);
				step.draw(pixels.get_frame(), WIN_SIZE);
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
