pub mod mem {
    use maikor_platform::mem::sizes;

    pub const CODE_BANK: usize = sizes::CODE_BANK as usize;
    pub const RAM_BANK: usize = sizes::RAM_BANK as usize;
    pub const ATLAS_BANK: usize = sizes::ATLAS as usize;
    pub const CONTROLLER_GRAPHICS_BANK: usize = sizes::CONTROLLER_GRAPHICS as usize;
}
