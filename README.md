# i3-toggle-ws

Simple program to toggle i3 workspaces on the same output.

## Install

```bash
git clone git@github.com:samaanhane98/i3-toggle-ws.git
cd i3-toggle-ws

cargo build --release

 mv ./target/release/i3-toggle-ws /usr/local/bin/i3-toggle-ws
```

## Usage

Add the following line to your i3 config:

```md
bindsym $mod+Tab exec i3-toggle-ws
```
