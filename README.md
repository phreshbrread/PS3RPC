# PS3RPC
A lightweight PS3 Discord rich presence client for Linux & Windows.<br>

## Usage
Run ``ps3rpc <PS3 IP>`` to start the service.<br>
A custom sleep duration and reconnect attempt delay can be specified after ``<PS3 IP>``, if no values are specified, they will default to 10 and 20 seconds respectively.<br>
Once a game is running on the PS3 with the specified IP, it will show in your Discord status. A message will also be shown on the PS3 system once a connection is established.

## Install
Simply clone the repository, ``cd`` into it and install using ``make install`` or ``cargo install --path .``<br>
Unfortunately there are no pre-built binaries at the current time.

### Requirements
- ``cargo`` installed (if building from source)
- A jailbroken Playstation 3 system with WebmanMOD installed

## Known Issues
- A "KLIC: Not in-game!" message will appear when on the PS3 XMB each refresh cycle.
- This currently only works when running PS3 games - PSX and PS2 games will not show.
- Quitting the program whilst in-game might not forcefully clear the Discord status.
    - AFAIK this is only an issue on third-party Discord clients that use ARRPC.
- Status doesn't show on the official Discord client for Linux.
    - This is an issue with that client specifically and I am therefore unable to do anything about it.

## To-Do
- Loading settings from a config file rather than just CLI args.
- Cover art for running games.
- Automatic local system discovery.
- Optimisations from better string handling.
- Pre-built binary releases.
