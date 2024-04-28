#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::Timer;
use microbit::hal::prelude::*;
#[derive(PartialEq)]
enum POSITIONS{
    None,
    LR,
    RL,
}
fn default_leds() -> [[u8;5];5]{
    [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ]
}
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut index = 0;
    let mut reverse_loop = false;
    let mut led_reverse:bool = false;
    let mut last_pos: POSITIONS = POSITIONS::None;
    loop {
        
        let statuses: [u8;5] = if index == 0 {
            last_pos = POSITIONS::LR;
            [1, 1, 1, 1, 1]
        }
        else if index == 4 {
            last_pos = POSITIONS::RL;
            led_reverse = true;
            [1, 1, 1, 1, 1]
        } else {
            match last_pos {
                POSITIONS::LR => {
                    led_reverse = true;
                    [0, 0, 0, 0, 1]
                }
                POSITIONS::RL => {
                    led_reverse = false;
                    [1, 0, 0, 0, 0]
                }
                _ => {
                    [0, 0, 0, 0, 0]
                }
            }
        };
        for i in 0..5 {
            let led_index: usize = if led_reverse {
                4 - i
            }
            else{
                i
            };
            let mut leds = default_leds();   
            let led_status: u8 = statuses[led_index];
            display.show(&mut timer, leds,0);
            if led_status == 1 {
                leds[index][led_index] = led_status;
                display.show(&mut timer, leds,30);
            }
        }
        led_reverse = false;
        if reverse_loop == false {
            index += 1;
            if index == 5 {
                index = 3;
                reverse_loop = true;
            }
        }
        else{
            index -= 1;
            if index == 0 {
                reverse_loop = false;
            }
        }
        
    }
}
