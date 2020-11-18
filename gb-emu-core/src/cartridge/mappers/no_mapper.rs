use super::{Mapper, MappingResult};

pub struct NoMapper {
    ram_size: usize,
}

impl Default for NoMapper {
    fn default() -> Self {
        Self { ram_size: 0 }
    }
}

impl Mapper for NoMapper {
    fn init(&mut self, rom_banks: u8, ram_size: usize) {
        // only support 32KB
        assert!(rom_banks == 2);
        assert!(ram_size <= 0x2000);

        self.ram_size = ram_size;
    }

    fn map_read_romx(&self, addr: u16) -> usize {
        // return the same address
        addr as usize
    }

    fn map_ram_read(&self, addr: u16) -> MappingResult {
        if self.ram_size == 0 {
            MappingResult::NotMapped
        } else {
            MappingResult::Addr((addr as usize) & (self.ram_size - 1))
        }
    }

    fn map_ram_write(&mut self, addr: u16, _data: u8) -> MappingResult {
        self.map_ram_read(addr)
    }
}