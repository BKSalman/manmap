{
  description = "A GUI for yt-dlp written in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    rust-overlay,
    ...
  }:
      let
        system = "x86_64-linux";
        craneLib = crane.lib.${system};
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };

        cargoArtifacts = craneLib.buildDepsOnly ({
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          pname = "manmap";
        });
      in with pkgs; rec {
        packages = rec {
          manmap = craneLib.buildPackage {
            src = craneLib.path ./.;

            inherit cargoArtifacts;

            GIT_HASH = self.rev or self.dirtyRev;
          };

          default = manmap;
        };

        overlay = final: prev: {
          manmap = packages.manmap;
        };

        devShells.${system}.default = mkShell {
          packages = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            cargo-watch
            gnome.zenity
            libsForQt5.kdialog
            act
          ];
        };
      };
}

