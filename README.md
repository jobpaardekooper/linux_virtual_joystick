# Linux Virtual Joystick
This is a simple program that creates a virtual joystick on your computer. I use it to develop software that needs a joystick, without actually having a joystick on-hand. it was inspired by the basic functionality of [vJoy](http://vjoystick.sourceforge.net/site/). 

This repo was forked and update to display 4 sliders:
- Roll (Left X)
- Pitch (Left Y)
- Yaw (Right Z)
- Throttle

One joystick button with type `BTN_TRIGGER` (`0x120`) called "Panic" is also added.

A button is availible in the program to move all sliders to easily generate an initial event for each of the sliders.

## Installation

Clone the repo and run:

```bash
caro install --path .
```

The program should now be available to run using `sudo linux_virtual_joystick`.

Since this program creates a virtual input device, it needs to be run as root. You can ensure it always runs as root by running:

```bash
sudo chown root:root $(which linux_virtual_joystick)
sudo chmod u+s $(which linux_virtual_joystick)
```

Now you can use the program without prefixing it with `sudo` as follows:

```bash
linux_virtual_joystick
```

## Compilation
To compile from source you will need [cargo and rust](https://www.rust-lang.org/tools/install).To install from source clone this repository and `cargo build`.
## Usage
To start the program just `cargo run`.