use vulkano::{
	instance::{Instance, InstanceCreateInfo},
	VulkanLibrary,
};
use winit::{
	event::{Event, WindowEvent},
	event_loop::EventLoop,
	window::WindowBuilder,
};

fn main() {
	let library = VulkanLibrary::new().expect("No local Vulkan Library/DLL!");
	let required_extensions = vulkano_win::required_extensions(&library);
	let instance = Instance::new(
		library,
		InstanceCreateInfo {
			enabled_extensions: required_extensions,
			..Default::default()
		},
	)
	.expect("Failed to Create instance!");

	let event_loop = EventLoop::new();
	let surface = WindowBuilder::new().build(&event_loop).unwrap();

	event_loop.run(move |event, _, control_flow| match event {
		Event::WindowEvent {
			event: WindowEvent::CloseRequested,
			..
		} => {
			println!("Program closed");
			control_flow.set_exit();
		}
		_ => (),
	});
}
