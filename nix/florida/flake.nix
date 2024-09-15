{

inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";


outputs = { self, nixpkgs, ... }: let

    system = "x86_64-linux";

    pkgs = import nixpkgs { inherit system; };


    inherit ( pkgs )
        lib
        pkgsStatic
        writers
    ;

    inherit ( pkgsStatic.rustPlatform )
        buildRustPackage
    ;

    inherit ( writers )
        writeFishBin
    ;

in {

    inherit pkgs;

    packages.${system} = rec {

        default = buildRustPackage rec {
            name = "florida";
            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
            doCheck = false;
            meta = { mainProgram = name; };
        };

        install = writeFishBin "install" ''
            install -Dm755 \
                "${lib.getExe default}" \
                "$(pwd)/../man.florida"
        '';

    };

};

}
