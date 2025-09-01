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

  cargoHash = "sha256-gbA8tb35IfWaYIUrYOg46asC07lkHA5w/gRu7z1KAA4=";

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
