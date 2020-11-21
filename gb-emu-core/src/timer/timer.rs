use crate::memory::{InterruptManager, InterruptType};
use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    struct TimerControl: u8 {
        const TIMER_ENABLE = 1 <<  2;
        const FREQ_DIVIDER = 0b11;
    }
}

impl TimerControl {
    fn timer_enabled(&self) -> bool {
        self.intersects(Self::TIMER_ENABLE)
    }

    fn freq_divider_selection_bit(&self) -> u16 {
        // which bit to check for falling edge when incrementing
        match self.bits() & Self::FREQ_DIVIDER.bits {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct Timer {
    divider: u16,
    timer_counter: u8,
    timer_modulo: u8,
    timer_control: TimerControl,
}

impl Timer {
    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => (self.divider >> 8) as u8,
            0xFF05 => self.timer_counter,
            0xFF06 => self.timer_modulo,
            0xFF07 => self.timer_control.bits() | 0xF8,
            _ => unreachable!(),
        }
    }

    pub fn write_register(&mut self, addr: u16, data: u8) {
        match addr {
            0xFF04 => self.divider = 0, // reset
            0xFF05 => self.timer_counter = data,
            0xFF06 => self.timer_modulo = data,
            0xFF07 => self
                .timer_control
                .clone_from(&TimerControl::from_bits_truncate(data)),
            _ => unreachable!(),
        }
    }

    pub fn reset(&mut self) {
        self.divider = 0xABCC;
    }

    pub fn clock_divider<I: InterruptManager>(&mut self, interrupt: &mut I) {
        let bit = self.timer_control.freq_divider_selection_bit();
        let saved_divider_bit = (self.divider >> bit) & 1;

        // because each CPU M-cycle is 4 T-cycles
        self.divider = self.divider.wrapping_add(4);

        let new_divider_bit = (self.divider >> bit) & 1;

        if self.timer_control.timer_enabled() && saved_divider_bit == 1 && new_divider_bit == 0 {
            self.increment_timer(interrupt)
        }
    }
}

impl Timer {
    fn increment_timer<I: InterruptManager>(&mut self, interrupt: &mut I) {
        let (new_counter, overflow) = self.timer_counter.overflowing_add(1);

        if overflow {
            self.timer_counter = self.timer_modulo;
            // generate interrupt
            interrupt.request_interrupt(InterruptType::Timer);
        } else {
            self.timer_counter = new_counter;
        }
    }
}