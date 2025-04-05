{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      mkshell =
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          ${system}.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              iconv
              pkg-config

              # rust specific tooling
              cargo
              clippy
              rust-analyzer
              rustc
              rustfmt
            ];
            RUST_BACKTRACE = "ALL";
            RUST_SRC_PATH = pkgs.rust.packages.stable.rustPlatform.rustLibSrc;
          };

        };
      mkpackage =
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          ${system} = pkgs.rustPlatform.buildRustPackage rec {
            name = "coptic-font-conversion";
            version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        };
    in
    {
      devShells = nixpkgs.lib.foldr nixpkgs.lib.mergeAttrs { } (map mkshell supportedSystems);
      defaultPackage = nixpkgs.lib.foldr nixpkgs.lib.mergeAttrs { } (map mkpackage supportedSystems);
    };
}
