{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
	buildInputs = [
		# rust deps
		cargo
		rustc

		# build deps
		cmake
		pkg-config

		# gtk deps
		gtk4
		gdk-pixbuf
		cairo
		libadwaita

		# servo deps
		pango

		# editor deps
		lapce
		vscodium
		rust-analyzer
		nil
	];
}

