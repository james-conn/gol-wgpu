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
	input_buf: wgpu::Buffer,
	output_buf: wgpu::Buffer,
	encoder: wgpu::CommandEncoder,
	pipeline: wgpu::ComputePipeline,
	bind_group: wgpu::BindGroup
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
			label: Some("device"),
			trace: wgpu::Trace::Off
		}).await.unwrap();

		let input_buf = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("input_buf"),
			size: (H * W) as u64,
			usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
			mapped_at_creation: false
		});

		let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("output_buf"),
			size: (H * W) as u64,
			usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
			mapped_at_creation: false
		});

		let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

		let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
			label: None,
			layout: None,
			module: &shader,
			entry_point: Some("gol"),
			compilation_options: wgpu::PipelineCompilationOptions::default(),
			cache: None
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("bind_group"),
			layout: &pipeline.get_bind_group_layout(0),
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: input_buf.as_entire_binding()
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: output_buf.as_entire_binding()
				}
			]
		});

		let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("encoder")
		});

		Self {
			instance,
			device,
			queue,
			input_buf,
			output_buf,
			encoder,
			pipeline,
			bind_group
		}
	}
}

impl<const W: usize, const H: usize> GameState<W, H> {
	async fn next_state(&self, gpu: &WgpuStuff<W, H>) -> Self {
		todo!()
	}
}
