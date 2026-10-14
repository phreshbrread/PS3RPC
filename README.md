# PS3RPC
A lightweight PS3 Discord rich presence client for Linux & Windows.<br>

## Usage
Run ``ps3rpc`` to start the service.<br>
If the PS3's IP is not already set in ``settings.toml``, you will be prompted to enter it.
Once a game is running on the PS3 with the specified IP, it will show in your Discord status. A message will also be shown on the PS3 system once a connection is established.

## Install
### Linux
Clone the repository and run ``make install`` or ``cargo install --path .`` inside the repo directory.<br>
```
git clone https://github.com/phreshbrread/ps3rpc
cd ps3rpc
cargo install --path .
```

### Windows
Download the latest ``ps3rpc.exe`` from the [releases](https://github.com/phreshbrread/PS3RPC/releases) page.

### Requirements
- ``cargo`` installed (if building from source)
- A jailbroken PlayStation 3 system with webMAN MOD installed

## Known Issues
- A "KLIC: Not in-game!" message will appear when on the PS3 XMB each refresh cycle.
- This currently only works when running PS3 games - PSX and PS2 games will not show.
- Quitting the program whilst in-game might not forcefully clear the Discord status.
    - AFAIK this is only an issue on third-party Discord clients that use ARRPC.
- Status doesn't show on the official Discord client for Linux.
    - This is an issue with that client specifically and I am therefore unable to do anything about it.

## To-Do
- Retry when failing to connect to Discord IPC.
- Cache cover images.
- Loading settings from a config file rather than just CLI args.
- Automatic local system discovery.
- Small memory optimisations from better string handling.
