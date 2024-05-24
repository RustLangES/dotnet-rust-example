{
  system,
  pkgs,
  lib ? pkgs.lib,
  stdenv ? pkgs.stdenv,
  crane,
  fenix,
  ...
}: let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.${system}.fromToolchainFile {
    file = ./rustlib/rust-toolchain.toml;
    sha256 = "sha256-opUgs6ckUQCyDxcB9Wy51pqhd0MPGHUVbwRKKPGiwZU=";
  };
  # crane: cargo and artifacts manager
  craneLib = crane.${system}.overrideToolchain toolchain;

  nativeBuildInputs = with pkgs; [
    dotnet-sdk_8
  ];

  buildInputs = with pkgs; [
    pkg-config
    autoPatchelfHook
  ]
  ++ lib.optionals stdenv.buildPlatform.isDarwin [
    pkgs.libiconv
  ];

  example = craneLib.buildPackage {
    doCheck = false;
    src = craneLib.cleanCargoSource (craneLib.path ./.);
    buildPhaseCargoCommand = "HOME=$(mktemp -d fake-homeXXXX) worker-build --release --mode no-install";

    installPhaseCommand = ''
      cp -r ./build $out
    '';

    nativeBuildInputs = with pkgs; nativeBuildInputs ++ p ++ [
    ];

    inherit buildInputs;
  };
in
{
  # `nix build`
  packages.default = example;

  # `nix develop`
  devShells.default = craneLib.devShell {
    buildInputs = nativeBuildInputs ++ buildInputs;
    DOTNET_ROOT = "${pkgs.dotnet-runtime_8}";
  };
}
