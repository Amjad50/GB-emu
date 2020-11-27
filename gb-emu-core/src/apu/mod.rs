mod apu;
mod envelope;
mod pulse_channel;

pub use apu::Apu;

trait ApuChannel {
    fn output(&mut self) -> u8;
    fn muted(&self) -> bool;
}

struct LengthCountedChannel<C: ApuChannel> {
    max_length: u8,
    length: u8,
    current_counter: u8,
    counter_decrease_enable: bool,
    muted: bool,
    channel: C,
}

impl<C: ApuChannel> LengthCountedChannel<C> {
    pub fn new(channel: C, max_length: u8) -> Self {
        Self {
            max_length,
            length: 0,
            current_counter: 0,
            counter_decrease_enable: false,
            muted: false,
            channel,
        }
    }
    pub fn channel(&self) -> &C {
        &self.channel
    }

    pub fn channel_mut(&mut self) -> &mut C {
        &mut self.channel
    }

    pub fn write_sound_length(&mut self, data: u8) {
        self.length = self.max_length - data;
        self.current_counter = self.length;
    }

    pub fn write_length_enable(&mut self, data: bool) {
        self.counter_decrease_enable = data;
        self.current_counter = self.length;
    }

    pub fn read_length_enable(&self) -> bool {
        self.counter_decrease_enable
    }

    pub fn restart_channel(&mut self) {
        self.muted = false;
        self.current_counter = self.length;
    }

    pub fn clock_length_counter(&mut self) {
        if self.counter_decrease_enable {
            if self.current_counter == 0 {
                self.muted = true;
            } else {
                self.current_counter -= 1;
                if self.current_counter == 0 {
                    self.muted = true;
                    self.counter_decrease_enable = false;
                }
            }
        }
    }
}

impl<C: ApuChannel> ApuChannel for LengthCountedChannel<C> {
    fn output(&mut self) -> u8 {
        if self.muted {
            0
        } else {
            self.channel.output()
        }
    }

    fn muted(&self) -> bool {
        self.muted || self.channel.muted()
    }
}