#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_graphics::{fonts::{Font8x16, Text}, prelude::*, pixelcolor::BinaryColor, style::TextStyleBuilder};
use ssd1306::{I2CDIBuilder, prelude::*};
use stm32f4xx_hal::{gpio::{AF4, AlternateOD, gpiob::{PB8, PB9}}, i2c::I2c, prelude::*, stm32::{self, I2C1}};

type Screen = GraphicsMode<I2CInterface<I2c<I2C1, (PB8<AlternateOD<AF4>>, PB9<AlternateOD<AF4>>)>>, DisplaySize128x64>;

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let gpio_a = p.GPIOA.split();
    let btn_a = gpio_a.pa4.into_pull_up_input();
    let btn_b = gpio_a.pa10.into_pull_up_input();
    let gpio_b = p.GPIOB.split();
    let gpio_c = p.GPIOC.split();
    let mut r = gpio_b.pb4.into_push_pull_output();
    let mut g = gpio_b.pb3.into_push_pull_output();
    let mut b = gpio_c.pc7.into_push_pull_output();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let i2c = stm32f4xx_hal::i2c::I2c::i2c1(
        p.I2C1,
        (
            gpio_b.pb8.into_alternate_af4_open_drain(),
            gpio_b.pb9.into_alternate_af4_open_drain(),
        ),
        stm32f4xx_hal::time::KiloHertz(400).into(),
        clocks,
    );
    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: Screen = ssd1306::Builder::new().connect(interface).into();
    disp.init().unwrap();
    disp.flush().unwrap();
    let mut i = 0u8;
    set_led(i, &mut r, &mut g, &mut b);
    set_screen(i, &mut disp);
    loop {
        asm::delay(2_000_000);
        let a_pressed = btn_a.is_high().unwrap();
        let b_pressed = btn_b.is_high().unwrap();
        if a_pressed == b_pressed {
            continue;
        }
        if btn_a.is_high().unwrap() {
            i += 1;
        }
        if btn_b.is_high().unwrap() {
            if i == 0 {
                i = 6;
            } else {
                i -= 1;
            }
        }
        if i > 6 {
            i = 0;
        }
        set_led(i, &mut r, &mut g, &mut b);
        set_screen(i, &mut disp);
    }
}
use embedded_hal::digital::v2::OutputPin;

fn set_led(n: u8, r: &mut impl OutputPin, g: &mut impl OutputPin, b: &mut impl OutputPin) {
    match n {
        1 => {
            r.set_high().ok();
            g.set_low().ok();
            b.set_low().ok();
        }
        2 => {
            r.set_high().ok();
            g.set_high().ok();
            b.set_low().ok();
        }
        3 => {
            r.set_high().ok();
            g.set_high().ok();
            b.set_high().ok();
        }
        4 => {
            r.set_low().ok();
            g.set_high().ok();
            b.set_high().ok();
        }
        5 => {
            r.set_high().ok();
            g.set_low().ok();
            b.set_high().ok();
        }
        6 => {
            r.set_low().ok();
            g.set_low().ok();
            b.set_high().ok();
        }
        _ => {
            r.set_low().ok();
            g.set_low().ok();
            b.set_low().ok();
        }
    }
}

fn set_screen(n: u8, disp: &mut Screen) {
    let color_name = match n {
        1 => {
            "red"
        },
        2 => {
            "yellow"
        }
        3 => {
            "white"
        }
        4 => {
            "aqua"
        }
        5 => {
            "purple"
        }
        6 => {
            "blue"
        }
        _ => {
            "black"
        }
    };
    disp.clear();
    let style = TextStyleBuilder::new(Font8x16)
        .text_color(BinaryColor::On)
        .build();
    Text::new(color_name, Point::zero())
        .into_styled(style)
        .draw(disp)
        .unwrap();
    disp.flush();
}
