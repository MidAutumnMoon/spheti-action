use std::path::PathBuf;

use clap::Parser;


/// The thing that pokes settings of nix daemon.
#[ derive( Parser, Debug ) ]
enum CmdCmds {
    /// Add substituters to nix.conf
    #[ command( visible_alias = "a" ) ]
    AddSubstituter( AddSubstituterOpts ),

    /// Apply some settings around nix daemon
    #[ command( visible_alias = "t" ) ]
    TweakNixDaemon,
}

#[ derive( clap::Args, Debug ) ]
struct AddSubstituterOpts {
    /// Path to nix.conf
    #[ arg( long, short, default_value = "/etc/nix/nix.conf" ) ]
    nix_conf: PathBuf,

    /// The url of the substituter to be added.
    /// E.g. "https://what.cachix.org"
    #[ arg( long, short = 'u' ) ]
    substituter_url: String,

    /// The public key of that substituter.
    /// E.g. "what.cachix.org-1:mumblekeystring"
    #[ arg( long, short = 'p' ) ]
    substituter_pubkey: String,
}


fn main() -> anyhow::Result<()> {

    eprintln!( "Parse commands" );

    let cmd_cmds = dbg! {
        CmdCmds::parse()
    };

    match cmd_cmds {
        CmdCmds::AddSubstituter( ref opts )
            => add_substituter( opts )?,
        CmdCmds::TweakNixDaemon
            => tweak_daemon()?,
    }

    Ok(())

}


fn add_substituter( opts: &AddSubstituterOpts )
    -> anyhow::Result<()>
{

    use nix_config_parser::NixConfig;

    eprintln!( "AddSubstituter" );

    eprintln!( "Parse nix.conf" );

    let mut settings = {
        let nix_conf = match
            NixConfig::parse_file( &opts.nix_conf )
        {
            Ok( conf ) => conf,
            Err(..) => NixConfig::parse_string( "".into(), None )?,
        };
        nix_conf.into_settings()
    };

    dbg!( &settings );

    let what_to_do = [
        ( "extra-substituters", &opts.substituter_url ),
        ( "extra-trusted-public-keys", &opts.substituter_pubkey ),
    ];

    for ( what, to_do ) in what_to_do {
        settings.entry( what.into() )
            .or_insert( "".into() )
            // here's a space before value
            .push_str( &format!( " {to_do}" ) )
        ;
    }

    // stupid manual serialization

    let mut config = String::new();

    for ( key, value ) in settings.iter() {
        let line = format!( "{key} = {value}\n" );
        config.push_str( &line );
    }

    eprintln!( "Modified nix.conf" );
    eprintln!( "{config}" );

    eprintln!( "Write nix.conf" );

    std::fs::write( &opts.nix_conf, config )?;

    Ok(())

}



/// The function which changes surrounding
/// configuration of nix daemon.
///
/// 1) Set TMPDIR to a location on disk rather than
/// in memory to avoid OOM during large builds.
fn tweak_daemon() -> anyhow::Result<()> {

    eprintln!( "Do TweakNixDaemon" );

    let nixbuild_on_disk = dbg! {
        PathBuf::from( "/nixbuild" )
    };

    let service_dir = dbg! {
        PathBuf::from( "/etc/systemd/system/nix-daemon.service.d" )
    };

    let systemd_conf = dbg! {
        service_dir.join( "99-tmpdir.conf" )
    };

    eprintln!( "Create new TMPDIR" );

    std::fs::create_dir_all( &nixbuild_on_disk )?;

    eprintln!( "Write systemd conf for nix daemon" );

    std::fs::create_dir_all( &service_dir )?;

    std::fs::write( systemd_conf, format! {
        "\
            [Service]\n\
            Environment = TMPDIR={}\
        ",
        &nixbuild_on_disk.display()
    } )?;

    Ok(())

}
