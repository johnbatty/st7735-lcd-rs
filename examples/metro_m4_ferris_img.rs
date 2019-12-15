//! Install Imagemagick and convert png from https://rustacean.net/ to centered 86x64 size .raw bytes (where 11008 is 86x64x2)
//! convert -resize 86x64^ -gravity center -extent 86x64 -background black rustacean-orig-noshadow.png -flip -type truecolor -define bmp:subtype=RGB565 -depth 16 -strip ferris.bmp && tail -c 11008 ferris.bmp > ferris.raw

#![no_std]
#![no_main]

extern crate metro_m4 as hal;
extern crate panic_halt;

use embedded_graphics::egrectangle;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::{raw::LittleEndian, Rgb565, RgbColor};
use embedded_graphics::prelude::*;

use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::spi_master;
use hal::{entry, CorePeripherals, Peripherals};
use st7735_lcd;
use st7735_lcd::Orientation;

#[entry]
fn main() -> ! {
    let core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);

    let spi = spi_master(
        &mut clocks,
        16.mhz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
    );

    let dc = pins.d0.into_push_pull_output(&mut pins.port);
    let rst = pins.d1.into_push_pull_output(&mut pins.port);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, false, true);
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    // My particular lcd seems to be off a few pixels
    disp.set_offset(1, 25);

    //black backdrop
    disp.draw(egrectangle!(
        (0, 0),
        (160, 128),
        stroke = None,
        fill = Some(RgbColor::BLACK)
    ));

    let ferris: Image<Rgb565, LittleEndian> =
        Image::new(include_bytes!("./ferris.raw"), 86, 64).translate(Point::new(40, 33));

    disp.draw(ferris.into_iter());

    loop {}
}
