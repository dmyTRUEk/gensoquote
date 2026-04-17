{
	description = "Like fortune, but in Gensokyo and memory safe.";

	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

	outputs = { self, nixpkgs }:
	let
		system = "x86_64-linux";
		pkgs = import nixpkgs {
			inherit system;
		};
	in {
		packages.${system}.default =
		let
			cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
			pname = cargoToml.package.name;
			version = cargoToml.package.version;
		in
			pkgs.rustPlatform.buildRustPackage {
				inherit pname version;
				src = self;
				cargoLock.lockFile = ./Cargo.lock;
				# nativeBuildInputs = [ rust ];
			};
		apps.${system}.default = {
			type = "app";
			program = "${self.packages.${system}.default}/bin/${self.packages.${system}.default.pname}";
		};
	};
}
