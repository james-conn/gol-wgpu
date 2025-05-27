use pollster::FutureExt;

fn main() {
	test().block_on();
}

async fn test() {
	let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
		backends: wgpu::Backends::all(),
		flags: wgpu::InstanceFlags::VALIDATION,
		backend_options: wgpu::BackendOptions::default()
	});

	let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();

	let (device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor {
		required_features: wgpu::Features::empty(),
		required_limits: wgpu::Limits::default(),
		memory_hints: wgpu::MemoryHints::Performance,
		label: None,
		trace: wgpu::Trace::Off
	}).await.unwrap();

	println!("{device:?}");
}
