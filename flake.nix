{
  description = "Trak — Minimalist Rust & Tailwind web application";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
    ];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
  in {
    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          rust-analyzer
          clippy
          rustfmt
          pkg-config
          cargo-leptos
          tailwindcss
          sqlite
          openssl
          lld
          binaryen
          cargo-binstall
        ];

        shellHook = ''
          export CARGO_HOME="$PWD/.direnv/cargo"
          export PATH="$CARGO_HOME/bin:$PATH"
          export RUST_SRC_PATH="${pkgs.rustPlatform.rustLibSrc}"
          mkdir -p "$CARGO_HOME/bin"
          if ! command -v wasm-bindgen-bin >/dev/null 2>&1; then
            cargo binstall wasm-bindgen-cli@0.2.130 --root "$CARGO_HOME" --no-confirm >/dev/null 2>&1 || true
            if [ -f "$CARGO_HOME/bin/wasm-bindgen" ] && [ ! -f "$CARGO_HOME/bin/wasm-bindgen-bin" ]; then
              mv "$CARGO_HOME/bin/wasm-bindgen" "$CARGO_HOME/bin/wasm-bindgen-bin"
              printf '#!/usr/bin/env bash\nset -e\n"$(dirname "$0")/wasm-bindgen-bin" "$@"\nif [ -f "target/site/pkg/trak.wasm" ] && [ ! -f "target/site/pkg/trak_bg.wasm" ]; then\n  cp "target/site/pkg/trak.wasm" "target/site/pkg/trak_bg.wasm"\nfi\n' > "$CARGO_HOME/bin/wasm-bindgen"
              chmod +x "$CARGO_HOME/bin/wasm-bindgen"
            fi
          fi
          echo "⚡ [Trak Rust Dev Environment Loaded]"
          echo "  • Rust:  $(rustc --version 2>/dev/null || echo 'ready')"
          echo "  • Cargo: $(cargo --version 2>/dev/null || echo 'ready')"
        '';
      };
    });
  };
}
