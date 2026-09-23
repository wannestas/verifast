{
  description = "VeriFast dev environment: native toolchain from Nix, OCaml from opam, Rust from rustup";

  inputs = {
    # nixos-25.05 is the LAST release carrying llvmPackages_16, which the C++ AST
    # exporter needs (upstream ships a stock llvmorg-16.0.1 build). 25.11 onward
    # carry only LLVM 18-23. Using it as the sole input keeps this to one lock entry.
    nixpkgs.url = "tarball+https://github.com/NixOS/nixpkgs/archive/ac62194c3917d5f474c1a844b6fd6da2db95077d.tar.gz";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});

      # Neither in vfdeps nor in nixpkgs; upstream installs it with
      # `cargo install --locked --git`. Without it `dune build` fails for
      # src/rust_frontend/vf_mir_decoder, which takes merlin down across all of src/.
      mkDecoder =
        pkgs:
        pkgs.rustPlatform.buildRustPackage {
          pname = "capnpc-ocaml-decoder";
          version = "0.1.0-unstable-2d6606d";
          src = pkgs.fetchFromGitHub {
            owner = "btj";
            repo = "capnpc-ocaml-decoder";
            rev = "2d6606d9b59cd0c88a66729f3f076c10c0c8e0b2";
            hash = "sha256-sZ4mfOm1ynjN+zdbvy4sM8JnAmRmjF9Oja+O8HC8oHw=";
          };
          cargoLock.lockFile = ./nix/capnpc-ocaml-decoder-Cargo.lock;
          # its build.rs uses the capnpc crate, which shells out to `capnp`
          nativeBuildInputs = [ pkgs.capnproto ];
          meta.mainProgram = "capnpc-ocaml-decoder";
        };

      # src/cxx_frontend/ast_exporter/CMakeLists.txt:6-7 derives BOTH
      #   LLVM_DIR  = ${LLVM_INSTALL_DIR}/lib/cmake/llvm
      #   Clang_DIR = ${LLVM_INSTALL_DIR}/lib/cmake/clang
      # from one prefix, using plain set() -- so -DLLVM_DIR / -DClang_DIR cannot
      # override it. Hence a single joined tree.
      mkLlvm16 =
        pkgs:
        pkgs.symlinkJoin {
          name = "vf-llvm-clang-16";
          paths = with pkgs.llvmPackages_16; [
            llvm.dev
            llvm.lib
            llvm.out
            clang-unwrapped.dev
            clang-unwrapped.lib
            clang-unwrapped
          ];
        };
    in
    {
      packages = forAllSystems (pkgs: {
        capnpc-ocaml-decoder = mkDecoder pkgs;
        llvm16 = mkLlvm16 pkgs;
      });

      devShells = forAllSystems (
        pkgs:
        let
          llvm16 = mkLlvm16 pkgs;
          # CMakeLists.txt:8 does set(CapnProto_DIR "${VFDEPS}/lib/cmake/CapnProto"),
          # so VFDEPS must be a prefix containing lib/cmake/CapnProto.
          capnpPrefix = pkgs.capnproto;
        in
        {
          default = pkgs.mkShell {
            name = "verifast-dev";

            packages = with pkgs; [
              # build drivers
              gnumake
              cmake
              ninja
              pkg-config
              m4
              which
              binutils
              # Cap'n Proto: `capnp`, headers, libcapnp, lib/cmake/CapnProto
              capnproto
              (mkDecoder pkgs)
              # Support for opam's builds (incl. Z3 4.8.5 from source). `opam`
              # itself is deliberately NOT here:
              # nixpkgs 25.05 has 2.3.0, and running and older binary against a
              # root written by a newer one risks a format migration affecting
              # the user's other switches. Same reasoning as leaving rustup alone.
              # python311, NOT python3: Z3 4.8.5's scripts/mk_make.py imports
              # distutils, which Python 3.12 removed from the stdlib.
              gmp
              bubblewrap
              unzip
              curl
              git
              gnupatch
              rsync
              bzip2
              gnutar
              python311
              # librustc_driver from the rustup toolchain needs libz.so.1 at
              # runtime; the Nix shell's loader would not otherwise find one.
              zlib
              # Rocq metatheory: Rocq 9.0.x + Iris 4.3.0, matching the known-good pair
              coq_9_0
              coqPackages_9_0.iris
            ];

            # Deliberately absent: rustc/cargo (rustup owns those -- a Nix rustc on
            # PATH would break `rustc +nightly-2025-11-25 --print sysroot` in
            # rust_fe.ml:44, which is rustup-shim-only), and lablgtk/gtk2/
            # gtksourceview/valac (no IDE: nixpkgs removed gnome2.libgtksourceview
            # on 2026-07-23).

            # Plain mkShell attributes rather than a shellHook: these become real
            # environment variables in the derivation, so `nix print-dev-env`
            # (what direnv uses) exports them without having to run any hook.
            LLVM16_PREFIX = "${llvm16}";
            CAPNP_PREFIX = "${capnpPrefix}";
            # Put on LD_LIBRARY_PATH by .envrc so that librustc_driver-*.so,
            # dlopened via bin/vf-rust-mir-exporter, resolves libz.so.1.
            VF_RUNTIME_LIB_PATH = "${pkgs.zlib}/lib";
          };
        }
      );
    };
}
