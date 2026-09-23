{
  description = "VeriFast dev environment: native toolchain and OCaml stack from Nix, Rust from rustup";

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

      # Upstream (vfdeps adf88dc) builds on the OCaml 4.14 line; nixpkgs has 4.14.2.
      ocamlPackagesFor = pkgs: pkgs.ocaml-ng.ocamlPackages_4_14;

      # Z3 4.8.5 with OCaml bindings, as upstream pins it (vfdeps Makefile). nixpkgs
      # dropped z3_4_8, but fstar still needs 4.8.5 and pkgs/by-name/fs/fstar/z3
      # carries the gcc/typo/distutils patches that make it build today; we reuse
      # that and only switch on the OCaml bindings.
      #
      # Not nixpkgs' ocamlPackages.z3: that wrapper renames the findlib package to
      # lowercase `z3`, but src/dune:7 asks for `Z3` (what Z3's own install uses).
      # Here the `ocaml` output keeps site-lib/Z3, and `lib` has the shared libz3.so
      # that src/GNUmakefile:111,168 copies into bin/ (found there via $ORIGIN).
      mkZ3 =
        pkgs:
        let
          ocamlPackages = ocamlPackagesFor pkgs;
        in
        (pkgs.fstar.passthru.z3.z3_4_8_5.override {
          ocamlBindings = true;
          pythonBindings = false;
          inherit (ocamlPackages) ocaml findlib zarith;
        }).overrideAttrs
          (prev: {
            # 4.8.5's bindings use `num` (zarith only replaced it later), and
            # vfdeps builds them against num too
            propagatedBuildInputs = (prev.propagatedBuildInputs or [ ]) ++ [ ocamlPackages.num ];
            # the full self-test adds a lot to an already long, uncached build
            doCheck = false;
          });

      # Not in nixpkgs. Same version and tarball hash as upstream's vfdeps.
      mkPpxParser =
        pkgs:
        let
          ocamlPackages = ocamlPackagesFor pkgs;
        in
        ocamlPackages.buildDunePackage rec {
          pname = "ppx_parser";
          version = "0.1.0";
          src = pkgs.fetchurl {
            url = "https://github.com/NielsMommen/ppx_parser/archive/refs/tags/${version}.tar.gz";
            sha256 = "42007eb6dfd7c6cdc02a4acae8a4d48626ba06fca4d5590aeeb1420943d0dc79";
          };
          propagatedBuildInputs = with ocamlPackages; [
            ppxlib
            camlp-streams
          ];
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
        z3 = mkZ3 pkgs;
        ppx_parser = mkPpxParser pkgs;
      });

      devShells = forAllSystems (
        pkgs:
        let
          llvm16 = mkLlvm16 pkgs;
          z3 = mkZ3 pkgs;
          ocamlPackages = ocamlPackagesFor pkgs;
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
              # OCaml stack. findlib's setup hook puts every package here on
              # OCAMLPATH, so `ocamlfind query Z3` etc. resolve without opam.
              ocamlPackages.ocaml
              ocamlPackages.dune_3
              ocamlPackages.findlib
              ocamlPackages.num
              ocamlPackages.camlp-streams
              (mkPpxParser pkgs)
              ocamlPackages.capnp # library + the capnpc-ocaml plugin
              ocamlPackages.stdint
              ocamlPackages.dune-configurator
              ocamlPackages.zarith
              ocamlPackages.merlin
              ocamlPackages.ocaml-lsp
              z3.ocaml
              z3.lib
              z3.out # the z3 binary itself, handy for debugging queries
              # librustc_driver from the rustup toolchain needs libz.so.1 at
              # runtime; the Nix shell's loader would not otherwise find one.
              zlib
              # Rocq metatheory: Rocq 9.0.x + Iris 4.3.0, matching the known-good pair
              coq_9_0
              coqPackages_9_0.iris
            ];


            # Constant configuration: plain mkShell attributes, so they are real
            # environment variables of the derivation.
            LLVM16_PREFIX = "${llvm16}";
            CAPNP_PREFIX = "${capnpPrefix}";

            WITHOUT_LABLGTK = "yes";
            # read by src/dune:5; bin/verifast then finds the libz3.so copied next to it
            OCAMLOPT_CCLIB_FLAGS = "-Wl,-rpath=$ORIGIN";
            # src/GNUmakefile:102-104 computes this from `ocamlfind query`, which
            # would point into the `ocaml` output; libz3.so lives in `lib`.
            Z3_DLL_DIR = "${z3.lib}/lib";

            # Cap'n Proto, under every name the tree reads:
            #   CAPNP_INC_DIR -- src/rust_frontend/vf_mir/dune:11, vf_mir_decoder/dune:12
            #                    (never exported by the makefile, so upstream silently
            #                    compiles schemas with -I "")
            #   CAPNP_INCLUDE -- src/cxx_frontend/stubs/dune:9, which otherwise defaults
            #                    to the literal string "CAPNP_INCLUDE"
            #   CAPNP_BIN     -- a DIRECTORY: rust_frontend/Makefile:5-7 and
            #                    cxx_frontend/Makefile:8-9 derive ../include, ../lib from it
            CAPNP_INC_DIR = "${capnpPrefix}/include";
            CAPNP_INCLUDE = "${capnpPrefix}/include";
            CAPNP_LIBS = "${capnpPrefix}/lib";
            CAPNP_BIN = "${capnpPrefix}/bin";

            # Only what depends on the environment at entry time. `nix print-dev-env`
            # (what direnv uses) does run this: its script ends in
            # `eval "${shellHook:-}"`.
            shellHook = ''
              # Rocq 9.0 renamed COQPATH -> ROCQPATH; nixpkgs' coq setup hook still
              # sets the old one, and every coqc call warns about it.
              if [ -n "''${COQPATH:-}" ]; then export ROCQPATH="$COQPATH"; fi

              # bin/vf-rust-mir-exporter links librustc_driver-*.so from the rustup
              # sysroot, which needs libz.so.1 -- not found by the Nix loader on its
              # own. rust_fe.ml:120 prepends the sysroot to LD_LIBRARY_PATH rather
              # than replacing it, so this entry survives into the exporter process.
              case ":''${LD_LIBRARY_PATH:-}:" in
                *":${pkgs.zlib}/lib:"*) ;;
                *) export LD_LIBRARY_PATH="${pkgs.zlib}/lib''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" ;;
              esac
            '';
          };
        }
      );
    };
}
