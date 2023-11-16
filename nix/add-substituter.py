#!/usr/bin/env -S python3

#
# Apparently Python's configparser works well enough
# on nix.conf, but be aware of mythical edge cases.
#
# There's a Rust crate by DeterminateSystems
# for r/w nix.conf though, but it's a hassle to use a
# compiled language, maybe later depending on how
# screwed up the stupid Python is.
#

import os
import sys
import subprocess
import pathlib


if not os.getuid() == 0:
    print( "::debug::sudo python" )
    subprocess.call( [ "sudo", "python3", *sys.argv ] )
    sys.exit()


USAGE = """\
./this.py nix.conf-path substituter-url substituter-pubkey\
"""


def cmd_opts():
    from itertools import islice
    opts = list( islice(sys.argv, 1, None) )
    assert len(opts) == 3, \
        f"Not enough arguments.\nUsage: {USAGE}"
    return [ o.strip() for o in opts ]


[ NIX_CONF, SUBSTER_URL, SUBSTER_PUBKEY ] = cmd_opts()

del cmd_opts


NIX_CONF = pathlib.Path( NIX_CONF )

assert NIX_CONF.exists(), \
    f'nix.conf "{NIX_CONF}" doesn\'t exist'

assert SUBSTER_URL and SUBSTER_PUBKEY, \
    "Empty substituter url or pubkey. Check your Actions config."



# configparser forcefully requires section headers,
# besides provides no easy way to convert its content
# back to string.
#
# Oh Python, suck to be you :/

from configparser import ConfigParser


# is python's "lambda" designed by marketing team instead of
# whom actually using it?
def configparser__str__( self: ConfigParser ) -> str:
    from io import StringIO
    content: str
    with StringIO() as sio:
        self.write( sio )
        content = sio.getvalue()
    # remove stupid "[section]" lines
    no_header = filter(
        lambda x: not x.startswith( '[' ),
        content.splitlines()
    )
    return "\n".join( no_header )

ConfigParser.__str__ = configparser__str__

del configparser__str__


# better autocompletion
DEFAULT = "DEFAULT"

parsed_conf = ConfigParser( allow_no_value=True )

parsed_conf.read_string(
    # assemble with the stupid section header
    f"[{DEFAULT}]\n{NIX_CONF.read_text()}"
)
# ...and add insult to injury,
# the stupid API is not chainable.


# focusing on "extra-*" only to reduce headache
# duplicated substituters won't hurt too much
# compared to battling with Python

STUPID_MAPPING = {
    "extra-substituters": SUBSTER_URL,
    "extra-trusted-public-keys": SUBSTER_PUBKEY,
}

for field in STUPID_MAPPING:
    value = parsed_conf.get( DEFAULT, field, fallback="" )
    value += ( " " + STUPID_MAPPING[field] )
    parsed_conf.set( DEFAULT, field, value )


with NIX_CONF.open( "w" ) as w:
    w.write( str(parsed_conf) )


# But, so glad Python is not shell :)