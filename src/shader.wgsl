@group(0) @binding(0) var<storage, read> input_buf: array<u32>;
@group(0) @binding(1) var<storage, read_write> output_buf: array<u32>;
@group(0) @binding(2) var<uniform> width: u32;
@group(0) @binding(3) var<uniform> height: u32;

fn get_index(x: u32, y: u32) -> u32 {
	return (y * width + x);
}

fn get_neighbor_count(x: u32, y: u32) -> u32 {
	var count: u32 = 0;

	for (var dy = -1; dy <= 1; dy++){
		for (var dx = -1; dx <= 1; dx++) {
			var new_x = i32(x) + dx;
			var new_y = i32(y) + dy;

			let index = get_index(u32(new_x), u32(new_y));
			count += select(0u, 1u,
				(dx != 0 || dy != 0) &&
				input_buf[index] == 1u);
		}
	}

	return count;
}

@compute @workgroup_size(16, 16, 1)
fn gol(@builtin(global_invocation_id) global_id: vec3<u32>) {
	let x = global_id.x;
	let y = global_id.y;

	if (
		x == 0 ||
		y == 0 ||
		x >= width - 1 ||
		y >= height - 1
	) {
		return;
	}

	let idx = get_index(x, y);
	let cell: u32 = input_buf[idx];
	let neighbors = get_neighbor_count(x, y);

	output_buf[idx] = select(0u, 1u,
		(
			cell == 1u &&
			neighbors >= 2 &&
			neighbors <= 3
		) || (
			cell == 0u &&
			neighbors == 3
		)
	);
}
