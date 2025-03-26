builtins.derivation {
    name = "test";
    builder = "/bin/sh";
    system = builtins.currentSystem;

    # test pipe operator
    NUMBER = 1 |> ( it: it + 10 );

    args = [ "-c" ''
        [[ "$NUMBER" = "11" ]]
        echo test > $out
    '' ];

    preferLocalBuild = true;
    allowSubstitutes = false;
}
