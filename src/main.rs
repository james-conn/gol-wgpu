use pollster::FutureExt;

#[derive(Debug)]
struct GameState<const W: usize, const H: usize> {
	cells: [[bool; W]; H]
}

impl<const W: usize, const H: usize> Default for GameState<W, H> {
	fn default() -> Self {
		let cells = [[false; W]; H];
		Self { cells }
	}
}

const GAME_WIDTH: usize = 30;
const GAME_HEIGHT: usize = 30;

fn main() {
	let game = GameState::<GAME_WIDTH, GAME_HEIGHT>::default();
	let gpu = WgpuStuff::<GAME_WIDTH, GAME_HEIGHT>::new().block_on();

	// TODO: print before
	let next_game = game.next_state(&gpu).block_on();
	// TODO: print after
}

struct WgpuStuff<const W: usize, const H: usize> {
	instance: wgpu::Instance,
	device: wgpu::Device,
	queue: wgpu::Queue,
	input_buf: wgpu::Buffer
}

impl<const W: usize, const H: usize> WgpuStuff<W, H> {
	async fn new() -> Self {
		let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
			backends: wgpu::Backends::all(),
			flags: wgpu::InstanceFlags::VALIDATION,
			backend_options: wgpu::BackendOptions::default()
		});

		let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();

		let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
			required_features: wgpu::Features::empty(),
			required_limits: wgpu::Limits::default(),
			memory_hints: wgpu::MemoryHints::Performance,
			label: None,
			trace: wgpu::Trace::Off
		}).await.unwrap();

		let input_buf = device.create_buffer(&wgpu::BufferDescriptor {
			label: None,
			size: (H * W) as u64,
			usage: wgpu::BufferUsages::COPY_SRC,
			mapped_at_creation: false
		});

		Self {
			instance,
			device,
			queue,
			input_buf
		}
	}
}

impl<const W: usize, const H: usize> GameState<W, H> {
	async fn next_state(&self, gpu: &WgpuStuff<W, H>) -> Self {
		todo!()
	}
}
