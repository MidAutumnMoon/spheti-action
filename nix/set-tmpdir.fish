#!/usr/bin/env -S fish -PN

set -l BuildRoot "/nixbuild"

set -l Config "
[Service]
Environment = TMPDIR=$BuildRoot
"

set -l ServiceOverrideDir \
    "/etc/systemd/system/nix-daemon.service.d"


sudo mkdir --parent --verbose \
    "$BuildRoot" \
    "$ServiceOverrideDir"

printf "$Config" \
    | sudo tee "$ServiceOverrideDir/99-tmpdir.conf" 1> /dev/null