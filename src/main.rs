#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate tiny;

use avr_device::atmega328p as mcu;
use core::cell;
use panic_halt as _;
use tiny::time_source::TimeSource;
use tiny::timer::TimerGroup;

static TICKS: avr_device::interrupt::Mutex<cell::Cell<u16>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = TICKS.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter.wrapping_add(1));
    })
}

struct SysTickTimeSource {}

impl SysTickTimeSource {
    fn new(tc0: mcu::TC0) -> Self {
        // Clear timer on compare match
        tc0.tccr0a.write(|w| w.wgm0().ctc());

        // Prescalar 64
        // Count 125
        // 8 MHz / 64 / 125 == 1000 Hz
        tc0.tccr0b.write(|w| w.cs0().prescale_64());
        tc0.ocr0a.write(|w| unsafe { w.bits(125) });

        // Enable compare match A interrupt
        tc0.timsk0.write(|w| w.ocie0a().set_bit());

        Self {}
    }
}

impl TimeSource for SysTickTimeSource {
    fn ticks(&self) -> u32 {
        avr_device::interrupt::free(|cs| TICKS.borrow(cs).get()) as u32
    }
}

#[avr_device::entry]
fn main() -> ! {
    let dp = mcu::Peripherals::take().unwrap();

    let portb = dp.PORTB;
    portb.ddrb.write(|w| w.pb5().set_bit());

    let time_source = SysTickTimeSource::new(dp.TC0);

    // let timer_group = TimerGroup::new(&time_source);
    // let timer = TimerGroup::new_timer();

    // timer_group.start_periodic(&timer, 500, &portb, |portb, _| {
    //     portb.pinb.write(|w| w.pb5().set_bit());
    // });

    unsafe { avr_device::interrupt::enable() };

    loop {
        if time_source.ticks() % 500 == 0 {
            portb.pinb.write(|w| w.pb5().set_bit());
        }

        // timer_group.run();
    }
}
