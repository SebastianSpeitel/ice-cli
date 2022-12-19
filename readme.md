# ICE CLI

Queries the ICE API for the current status of your train.

## Installation

```sh
cargo install --git https://github.com/SebastianSpeitel/ice-cli
```

## Use as starship module

Append this to you `~/.config/starship.toml`:

```toml
[custom.ice]
when = 'ice-cli --check'
command = 'ice-cli prompt'
format = 'on 🚅 [$output]($style) '
```
