# mc-manager
Automate Minecraft server management

This is a work in progress. Use at your own risk.

## Features
### Implemented
- Create a Minecraft server directory
- Download specified version of minecraft_server.jar
- Check for updates and upgrade minecraft_server.jar 
### Planned
- Backup management
- Server start and restart
- Execute server console commands

## Installation
### Build from source
You will need to have `cargo`.

`git clone https://github.com/nicolaschan/mc-manager.git`
`cargo build --release`

The `mc-manager` binary will be in `target/release`.

## Usage
See usage on `./mc-manager --help`

