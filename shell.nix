{ pkgs ? import ./pkgs.nix { } }:
pkgs.mkShell {
  buildInputs = with pkgs;
    ([
      bashInteractive
      cargo
      rust-analyzer
      rustc
      rustfmt
      pkgconfig
      openssl
      (nginx.override { modules = [ nginxModules.echo ]; })
    ] ++ lib.optionals stdenv.isDarwin [
      libiconv
      darwin.apple_sdk.frameworks.Security
    ]);
}
