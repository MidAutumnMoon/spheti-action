builtins.derivation {
    name = "test";
    builder = "/bin/sh";
    system = builtins.currentSystem;

    args = [ "-c" ''
        echo test > $out
    '' ];

    preferLocalBuild = true;
    allowSubstitutes = false;
}
