# DualShock4 tester

Terminal user interface to test if your PlayStation4 controller is working properly. Press the buttons on your controller and watch how the TUI indicates it has been pressed.

![demo](demo.png)

*Demo shows how some of the buttons of the controller have been detected as pressed*

## Data 
The following byte array is an example of what is received from the DualShock4 controller. By staring at changes in this I figured out which buttons are represented by which byte.

`[1, 126, 124, 129, 123, 8, 0, 224, 0, 0, 215, 246, 10, 239, 255, 2, 0, 251, 255, 180, 0, 74, 32, 35, 6, 0, 0, 0, 0, 0, 26, 0, 0, 1, 246, 132, 106, 0, 0, 128, 0, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 0, 128, 0, 0, 0, 128, 0, 0, 0, 0, 128, 0]`

This raw data is a 64-byte array of 8-bit unsigned integers. It is sent from the controller via USB and interpreted by this program to see which buttons have been pressed.

## Installation & usage

1. Connect DualShock4 (PS4) controller via USB-cable
2. `cargo run` into console
3. When done, press 'q' on your keyboard