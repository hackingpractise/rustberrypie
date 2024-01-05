pub mod mmio {

    pub const GPIO_FSEL0: u32 = 0x3F20_0000;
    pub const GPIO_FSEL1: u32 = 0x3F20_0004;
    pub const GPIO_FSEL2: u32 = 0x3F20_0008;

    pub const GPIO_SET0: u32 = 0x3F20_001C;
    pub const GPIO_CLEAR0: u32 = 0x3F20_0028;
}
