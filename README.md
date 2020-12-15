## Use PWM to Breathe an LED on an Odroid-C4

This is a code sample for using the sysfs interface to "breathe" an
LED using a pulsed-width modulation (PWM) signal, written in Rust. It can
be run using the standard Rust `$ cargo run` command or cross-compiled for
the aarch64 architecture. 

This code was tested on an Odroid-C4, using the following OS: 
ubuntu-20.04-4.9-mate-odroid-c4-hc4-20201020.img.xz 
which is available [here](https://wiki.odroid.com/odroid-c4/os_images/ubuntu/mate/20201020).

After flashing this image, and logging in as root (default pwd: odroid), run
`$ sudo apt-get update && sudo apt-get upgrade && sudo reboot`

After this, you will likely need to edit the `/media/boot/config.ini` file to
have the line `overlays="spi0 i2c0 i2c1 uart0 pwm_ab pwm_cd pwm_ef` and restart
again in order to actually have access to all 6 PWM pins. From there, the pin mapping
should be: 
```
  Pin #35 -> pwmchip0, channel 1
  Pin #33 -> pwmchip0, channel 0
  Pin #11 -> pwmchip4, channel 1
  Pin #7  -> pwmchip4, channel 0
  Pin #15 -> pwmchip8, channel 1
  Pin #12 -> pwmchip8, channel 0
```
For those interested, a discussion thread (thanks to those who helped!) is available
at https://forum.odroid.com/viewtopic.php?f=207&t=41185&sid=44d7c94d217493a781dac31b04867346
