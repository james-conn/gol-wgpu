{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/release-25.05";
	};

	outputs = { self, nixpkgs }:
		let pkgs = import nixpkgs {
			system = "x86_64-linux";
		};
		libpath = pkgs.lib.makeLibraryPath [ pkgs.vulkan-loader ];
		in {
			devShells.x86_64-linux.default = pkgs.mkShell {
				packages = with pkgs; [
					cargo
					clippy
				];

				env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${libpath}";
			};
		};
}
