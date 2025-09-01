{
  rustPlatform,
  installShellFiles,
  stdenv,
  libiconv,
  lib,
}:
let
  manifest = lib.importTOML ./Cargo.toml;
in
rustPlatform.buildRustPackage rec {
  pname = manifest.package.name;
  version = manifest.package.version;

  src = lib.cleanSource ./.;

  nativeBuildInputs = [ installShellFiles ];

  buildInputs = lib.optionals stdenv.hostPlatform.isDarwin [ libiconv ];

  cargoHash = "sha256-EhZXo8NzoWeMCGHaMSUoP05F8wHG0e2k2e/qImyTFEo=";

  meta = with lib; {
    description = "A tool to help managing secrets";
    mainProgram = manifest.package.name;
    longDescription = ''
      Secreta is a tool to help managing secrets in a declarative way.
    '';
    homepage = "https://github.com/marcocondrache/secreta";
    license = licenses.mit;
  };
}
