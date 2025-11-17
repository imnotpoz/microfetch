{
  lib,
  rustPlatform,
  stdenvAdapters,
  llvm,
}: let
  toml = (lib.importTOML ../Cargo.toml).package;
  pname = toml.name;
  inherit (toml) version;
in
  rustPlatform.buildRustPackage.override {stdenv = stdenvAdapters.useMoldLinker llvm.stdenv;} {
    inherit pname version;
    src = let
      fs = lib.fileset;
      s = ../.;
    in
      fs.toSource {
        root = s;
        fileset = fs.unions [
          (fs.fileFilter (file: builtins.any file.hasExt ["rs"]) (s + /src))
          (s + /Cargo.lock)
          (s + /Cargo.toml)
          (s + /benches)
        ];
      };

    cargoLock.lockFile = ../Cargo.lock;
    enableParallelBuilding = true;
    env.RUSTFLAGS = "-C link-arg=-fuse-ld=mold";

    meta = {
      description = "Microscopic fetch script in Rust, for NixOS systems";
      homepage = "https://github.com/NotAShelf/microfetch";
      license = lib.licenses.gpl3Only;
      maintainers = [lib.maintainers.NotAShelf];
      mainProgram = "microfetch";
    };
  }
