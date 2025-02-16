# Brackeys-Game-Jam-2025.1
https://itch.io/jam/brackeys-13

## Setup
All requirements to build and develop the game are handled through nix already. [If not installed require here](https://nixos.org/download/). This is not required but everything is setup if on a platform that supports it.

If not using nix you'll need to install these programs.
* godot 4.1+ 
* rust 1.84+ and it's clang backend that should be included.

Once that is down you'll have to build the rust library by doing.
```sh
env -C rust cargo build
```

Finally everyone has to open godot and open the `godot/` folder within the engine.

