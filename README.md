# PS3RPC
PS3 Discord rich presence client for Linux & Windows.<br>
Only tested on Linux, but should work on other platforms.

## Install
Simply clone the repository, ``cd`` into it and install using ``make install`` or ``cargo install --path .``<br>
Unfortunately there are no pre-built binaries at the current time.

### Requirements
- ``cargo`` (if building from source)
- A jailbroken Playstation 3 system with WebmanMOD installed

## Usage
Run ``ps3rpc <PS3 IP>`` to start the service.<br>
Once a game is running on the PS3 with the specified IP, it will show in your Discord status. A message will also be shown on the PS3 system once a connection is established.

## Known Issues
- This currently only works when running PS3 games - PSX and PS2 games will not show.
- A "KLIC: Not in-game!" message will appear when on the PS3 XMB each refresh cycle.
- Can crash when failing to connect to a PS3 after exiting a game.
- Quitting the program whilst in-game doesn't forcefully clear the Discord status.
    - AFAIK this is only an issue on third-party Discord clients that use ARRPC.

## To-Do
- User configurable refresh/update delay.
- Loading settings from a config file rather than just CLI args.
- Cover art for running games.
- Automatic local system discovery.
- Optimisations from better string handling.
- Pre-built binary releases.
