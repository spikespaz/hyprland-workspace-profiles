{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixfmt.url = "github:serokell/nixfmt";
  };

  outputs = { self, nixpkgs, systems, rust-overlay, nixfmt, ... }:
    let
      inherit (nixpkgs) lib;
      eachSystem = lib.genAttrs (import systems);
      pkgsFor = eachSystem (system:
        import nixpkgs {
          localSystem = system;
          overlays = [ rust-overlay.overlays.default ];
        });
    in {
      devShells = eachSystem (system:
        let pkgs = pkgsFor.${system};
        in {
          default = pkgs.mkShell {
            strictDeps = true;
            packages = with pkgs; [
              rust-bin.stable.latest.default
              pkg-config
              openssl
            ];
          };
        });

      formatter = eachSystem (system: nixfmt.packages.${system}.default);
    };
}
