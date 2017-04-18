Library for getting information on monitors and workspaces in `bspwm`.
Tested with `bspwm 0.9.1`.

# Examples

List the names of the current monitors and the desktops that are on them:

````
extern crate bspwm_info;
use bspwm_info::*;

fn main() {
    let current_info = status().next().unwrap();
    for monitor in current_info.monitors {
        println!("{}:", monitor.name);
        for desktop in monitor.desktops {
            println!("\t{}", desktop.name);
        }
    }
}
````

# To-Do

* Use a different command (probably `bspc wm -d`) to obtain more information than
    `bspc subscribe report` provides.
* Communicate with `bspwm`'s socket directly rather than wrap the `bspc` command.
    Something like `fn status(path: Option<&Path>)`, where `Some(path)` specifies
    the socket location and `None` uses the default location as specified in
    `bspwm`'s man page.
