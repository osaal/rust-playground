{
  pkgs,
  lib,
  config,
  ...
}:
{
  # --- RUST ---
  # Enable Rust and C toolchains
  languages = {
    rust = {
      enable = true;
      lsp = {
        enable = true;
        package = pkgs.rust-analyzer;
      };
      # Use a toolchain TOML file (see rust-toolchain_template.toml)
      toolchainFile = ./rust-toolchain.toml;
    };
    c.enable = true;
  };

  # Common build tools for C-based Rust projects
  packages = [
    pkgs.gcc
    pkgs.pkg-config
  ];
}

