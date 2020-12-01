mod mbc1;
mod mbc2;
mod no_mapper;

pub(super) use mbc1::Mbc1;
pub(super) use mbc2::Mbc2;
pub(super) use no_mapper::NoMapper;

#[derive(Debug, Clone, Copy)]
pub enum MapperType {
    NoMapper,
    Mbc1,
    Mbc2,
    Mbc3 { timer: bool },
    Mbc5 { rumble: bool },
    Mmm01,
    Mbc6,
    Mbc7,
}

pub enum MappingResult {
    Addr(usize),
    Value(u8),
    NotMapped,
}

pub trait Mapper {
    fn init(&mut self, rom_banks: u16, ram_size: usize);

    fn map_read_rom0(&self, addr: u16) -> usize;

    fn map_read_romx(&self, addr: u16) -> usize;

    fn map_ram_read(&self, addr: u16) -> MappingResult;

    fn map_ram_write(&mut self, addr: u16, data: u8) -> MappingResult;

    fn write_bank_controller_register(&mut self, addr: u16, data: u8) {
        // ignored
    }

    fn save_battery_size(&self) -> usize {
        0
    }

    fn save_battery(&self) -> &[u8] {
        &[]
    }

    fn load_battery(&mut self, data: &[u8]) {
        // ignored
    }
}
