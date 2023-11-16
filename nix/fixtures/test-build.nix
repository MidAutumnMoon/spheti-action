let

    pkgs = import ( 
        builtins.getFlake "nixpkgs" 
    ) {};

in

pkgs.hello.outPath
