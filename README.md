# **Recontrolled - By Strawberry Foundations**

## What is recontrolled?
recontrolled is a rewrite of the old version of recontrolled, called [ledcontrol](https://github.com/Strawberry-Software-Industries/ledcontrol). 
With recontrolled you can control the on-board LEDs of your Raspberry Pi. It is currently compatible with the Raspberry Pi 4 (Kernel 6.1+) You can turn on or off the Activity or Power LED. Or even let them blink in different rythm!

## How does it work?
recontrolled was originally written in C, now it has been rewritten in Rust. It directly writes to the device files using `std::fs::File` and `std::io::Write`.

## How can I install recontrolled?
We offer pre-built binaries. 
Go to our release page ([Click here](https://github.com/Strawberry-Foundations/recontrolled/releases)) and download the latest build.

However, you can also compile recontrolled yourself on your Raspberry Pi. You need `rust` and `cargo` for this.
You can compile recontrolled with the following command:
```
cargo build --release
```

## Supported Platforms
To start recontrolled, all you need to do is download the executable. Just execute it and you will get a help menu. <br>
The important thing is that you need a Linux distribution that has `/sys/class/leds/ACT` and `/sys/class/leds/PWR` 
Otherwise recontrolled will not work

| Model             | State                                                 | Process                                                       | 
| --                | --                                                    | --                                                            | 
| Pi 4              | ![](https://img.shields.io/badge/Supported-success)   | ![](https://img.shields.io/badge/Tested-success)              |   
| Pi 5              | ![](https://img.shields.io/badge/Supported-success)   | ![](https://img.shields.io/badge/Tested-success)              |  
| Pi Zero 2W        | ![](https://img.shields.io/badge/Supported-success)   | ![](https://img.shields.io/badge/Tested-success)              |
| Pi Zero           | ![](https://img.shields.io/badge/Untested-orange)     | ![](https://img.shields.io/badge/Should_work-orange)          |
| Pi Zero W         | ![](https://img.shields.io/badge/Untested-orange)     | ![](https://img.shields.io/badge/Should_work-orange)          |
| Pi 3B+            | ![](https://img.shields.io/badge/Untested-orange)     | ![](https://img.shields.io/badge/Not_planned/No_tester-red)   |  
| Pi 3B             | ![](https://img.shields.io/badge/Untested-orange)     | ![](https://img.shields.io/badge/Not_planned/No_tester-red)   |  
| Pi 3A+            | ![](https://img.shields.io/badge/Untested-orange)     | ![](https://img.shields.io/badge/Not_planned/No_tester-red)   |  
| Pi 1B+            | ![](https://img.shields.io/badge/Untested-orange)     | ![](https://img.shields.io/badge/Not_planned/No_tester-red)   |  
| Pi 1A+            | ![](https://img.shields.io/badge/Untested-orange)     | ![](https://img.shields.io/badge/Not_planned/No_tester-red)   |  
| Pi 1B             | ![](https://img.shields.io/badge/Unsupported-red)     | ![](https://img.shields.io/badge/Not_supported_(Official)-red)|  
| Pi 2B             | ![](https://img.shields.io/badge/Unsupported-red)     | ![](https://img.shields.io/badge/Not_supported_(Official)-red)|  




## Fixes
You can try following fixes (/boot/config.txt) if recontrolled doesn't work on your Raspberry Pi (Check tested/supported platforms first!)
```
dtparam=pwr_led_trigger=none
dtparam=pwr_led_activelow=off
```

```
dtparam=pwr_led_trigger=default-on
dtparam=pwr_led_activelow=off
```