use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    struct SpriteFlags: u8 {
        const PRIORITY    = 1 << 7;
        const Y_FLIP      = 1 << 6;
        const X_FLIP      = 1 << 5;
        const PALLETE     = 1 << 4;
        const BANK        = 1 << 3;
        const CGB_PALETTE = 0b111;
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Sprite {
    y: u8,
    x: u8,
    tile: u8,
    flags: SpriteFlags,
}

impl Sprite {
    pub fn get_at_offset(&self, offset: u8) -> u8 {
        match offset {
            0 => self.y,
            1 => self.x,
            2 => self.tile,
            3 => self.flags.bits(),
            _ => unreachable!(),
        }
    }

    pub fn set_at_offset(&mut self, offset: u8, data: u8) {
        match offset {
            0 => self.y = data,
            1 => self.x = data,
            2 => self.tile = data,
            3 => self
                .flags
                .clone_from(&SpriteFlags::from_bits_truncate(data)),
            _ => unreachable!(),
        }
    }

    /// This is here just for completion as [`x`] is also present
    #[allow(dead_code)]
    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn screen_y(&self) -> u8 {
        self.y.wrapping_sub(16)
    }

    pub fn screen_x(&self) -> u8 {
        self.x.wrapping_sub(8)
    }

    pub fn tile(&self) -> u8 {
        self.tile
    }

    // TODO: should be used in DMG mode
    pub fn palette_selector(&self) -> u8 {
        self.flags.intersects(SpriteFlags::PALLETE) as u8
    }

    /// False if its above background (1-3)
    pub fn bg_priority(&self) -> bool {
        self.flags.intersects(SpriteFlags::PRIORITY)
    }

    pub fn y_flipped(&self) -> bool {
        self.flags.intersects(SpriteFlags::Y_FLIP)
    }

    pub fn x_flipped(&self) -> bool {
        self.flags.intersects(SpriteFlags::X_FLIP)
    }

    pub fn cgb_palette(&self) -> u8 {
        self.flags.bits() & SpriteFlags::CGB_PALETTE.bits
    }

    pub fn bank(&self) -> u8 {
        self.flags.contains(SpriteFlags::BANK) as u8
    }
}
