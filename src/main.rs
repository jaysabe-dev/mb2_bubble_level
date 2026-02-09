#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use panic_rtt_target as _;                                                    
use rtt_target::rtt_init_print;                                   

use microbit::{
    display::blocking::Display,
    hal::{Timer, twim},
    pac::{twim0::frequency::FREQUENCY_A}
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    // setup I2C bus for accelerometer
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100)};

    //init values
    let mut timer0 = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    //init accelometer
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
             AccelMode::HighResolution, 
             AccelOutputDataRate::Hz10
            )
            .unwrap();

    let mut leds = [[0u8; 5]; 5];

    //start in coarse mode
    let mut coarse_mode = true;

    loop {
        //Wait for new accel information
        while !sensor.accel_status().unwrap().xyz_new_data() {
            timer0.delay_ms(1u32);
        }

        //Read accel in milli-g
        let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();

        //check if board is upside down (== z positive)
        if z > 0 {
            //blank the display
            leds = [[0u8; 5]; 5];
            display.show(&mut timer0, leds, 200);
        } 
        else {
            let range = if coarse_mode { 500.0 } else { 50.0 };

            //scale x and y to LED coors
            let led_x = scale_to_led(-x as f32, range);
            let led_y = scale_to_led(y as f32, range);

            //clear all LEDS
            leds = [[0u8; 5]; 5];

            leds[led_y][led_x] = 255u8;

            //display for 200 ms
            display.show(&mut timer0, leds, 200);
        }

        if button_a.is_low().unwrap() && button_b.is_high().unwrap() {
            coarse_mode = true;
        } else if button_b.is_low().unwrap() && button_a.is_high().unwrap(){
            coarse_mode = false;
        }
    }
}


//Scale acceleration value to LED coors (0-4)
// Maps -range...+range to 0..4, clamping vals outside the range
fn scale_to_led(value: f32, range: f32) -> usize {
    let scaled = ((value / range) * 2.0) + 2.0;

    if scaled < 0.0 {
        0
    } else if scaled > 4.0 {
        4
    } else{
        scaled as usize
    }
}