{ lib, fetchFromGitHub, rustPlatform, cmake, pkg-config, gtk4, gdk-pixbuf, cairo, libadwaita, pango }:

rustPlatform.buildRustPackage rec {
  pname = "xenon-browser";
  version = "0.1.0";
    
  src = ./.;
  
  buildInputs = [ gtk4 gdk-pixbuf cairo libadwaita pango ];
  nativeBuildInputs = [cmake pkg-config];

  cargoLock.lockFile = ./Cargo.lock;
#  src = fetchFromGitHub {
#    owner = "BurntSushi";
#    repo = pname;
#    rev = version;
#    hash = "sha256-+s5RBC3XSgb8omTbUNLywZnP6jSxZBKSS1BmXOjRF8M=";
#  };

  meta = {
    description = "an experimental browser written with servo and gtk, inspired by neon.";
    # homepage = "https://github.com/BurntSushi/ripgrep";
    # license = licenses.unlicense;
    # maintainers = [ maintainers.tailhook ];
  };
}
