{

    inputs.nuran.url = "github:MidAutumnMoon/Nuran";

    outputs = { self, nuran, ... }: let
        lib = nuran.lib;
        pkgsBrew = nuran.pkgsBrew.appendOverlays [ self.overlays.default ];
    in {
        overlays.default = final: prev: let
            inherit ( final.pkgsStatic.rustTeapot )
                buildRustPackage
            ;
        in {
            florida-man = buildRustPackage rec {
                pname = "florida";
                version = "unstable";
                src = lib.cleanSource ./.;
                cargoLock.lockFile = ./Cargo.lock;
                doCheck = false;
                meta.mainProgram = pname;
            };
            install-florida-man = final.writers.writeBashBin "ins" ''
                install -Dm755 \
                    "${lib.getExe final.florida-man}" "$(pwd)/../man.florida"
            '';
        };

        packages = pkgsBrew lib.id;

    };

}
