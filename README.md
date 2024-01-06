# bar-rs (name needs work)
it's a status bar... in rust...

License: MIT

This is just a person project for my own hyprland bar,
    and is not intended for use by anyone else.

You can go ahead and tell me, along with reporting bugs,
    but you are mostly on your own. (unless I wanna help fix it)

GLHF

## Features:
- Backlight Widget (with proper permissions)
- Battery Widget
- Volume Widget (dummy widget for now)
- RAM and CPU warning gauges
- A Clock (and calender)
- Dynamic CSS loading (if build feature is enabled)
- Hyprland Workspaces and Submap Widget (via Hyprland sockets).

## TODO:
- Get proper Backlight setting permissions (maybe through polkit)
- Actually, like, make the volume widget
- Get tooltips working. (gtk4-layer-shell bug currently)
- Create extra windows (like the calender) for more information and interactive things.
- Figure out how to get those windows to close when you click off of them (without creating a window to fill the entire screen...)
- Likely some other stuff I cannot think of at the moment.


## Logging:
Logging format: "*user message*. *varible*=*...*, *variable2*=*...*, ..., *error*=*{rust error}*"

The user message should be a simple statement about what went wrong.
The variables are general information to get a idea of how it went wrong.
The error is just a insert for the underlying rust error to give more context on how it went wrong.


