extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};

// This has been tested to work on Odroid-C4 using the stock Ubuntu Odroid image
// May need to add `overlays="spi0 i2c0 i2c1 uart0 pwm_ab pwm_cd pwm_ef` to
// the /media/boot/config.ini file and restart before the PWM pins are available
// Pin #35 -> pwmchip0, channel 1
// Pin #33 -> pwmchip0, channel 0
// Pin #11 -> pwmchip4, channel 1
// Pin #7  -> pwmchip4, channel 0
// Pin #15 -> pwmchip8, channel 1
// Pin #12 -> pwmchip8, channel 0
const C4_PWM_CHIP: u32 = 8;
const C4_PWM_CHANNEL: u32 = 1;

/// Make an LED "breathe" by increasing and
/// decreasing the brightness
fn main() {
    let pwm = Pwm::new(C4_PWM_CHIP, C4_PWM_CHANNEL)
        .expect("An error occurred while trying to set up the PWM struct"); // number depends on chip, etc.
    pwm.export().expect("An error occurred while trying to export the PWM struct");
    pwm.set_period_ns(50_000).expect("Error while setting PWM period");
    pwm.set_duty_cycle_ns(2000).expect("Error while setting PWM duty cycle");

    pwm.enable(true).expect("An error occurred while trying to enable the PWM pin");
    println!("pwm enabled");
    loop {
        pwm_increase_to_max(&pwm).unwrap();
        pwm_decrease_to_min(&pwm).unwrap();
    }
    pwm.enable(false).expect("An error occurred while trying to disable the PWM pin");
    println!("pwm disabled");
}

fn pwm_increase_to_max(pwm: &Pwm) -> Result<()> {
    let period_ns: u32 = pwm.get_period_ns()?;
    for i in 1..33 {
        let frac = 1f32 / (i as f32);
        pwm.set_duty_cycle_ns((period_ns as f32 * dbg!(frac)) as u32)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    Ok(())
}

fn pwm_decrease_to_min(pwm: &Pwm) -> Result<()> {
    let period_ns: u32 = pwm.get_period_ns()?;
    for i in {1..33}.rev() {
        let frac = 1f32 / (i as f32);
        pwm.set_duty_cycle_ns((period_ns as f32 * dbg!(frac)) as u32)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    Ok(())
}
