with import <nixpkgs> {};

let
  rustChannel = rustChannelOf {
    channel = "stable";
    date = "2023-08-16";
  };
in
rustChannel.workspaceMembers.libadwaita.build.override {
  src = ./.;
}
