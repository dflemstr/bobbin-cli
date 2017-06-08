pub const CHIP_IDS: [(&'static str, u16); 41] = [
   ("STM32_F1_MEDIUM", 0x410),
   ("STM32_F2", 0x411),
   ("STM32_F1_LOW", 0x412),
   ("STM32_F4", 0x413),
   ("STM32_F1_HIGH", 0x414),
   ("STM32_L4", 0x415),
   ("STM32_L1_MEDIUM", 0x416),
   ("STM32_L0", 0x417),
   ("STM32_F1_CONN", 0x418),
   ("STM32_F4_HD", 0x419),
   ("STM32_F1_VL_MEDIUM_LOW", 0x420),
   ("STM32_F446", 0x421),
   ("STM32_F3", 0x422),
   ("STM32_F4_LP", 0x423),
   ("STM32_L0_CAT2", 0x425),
   ("STM32_L1_MEDIUM_PLUS", 0x427),
   ("STM32_F1_VL_HIGH", 0x428),
   ("STM32_L1_CAT2", 0x429),
   ("STM32_F1_XL", 0x430),
   ("STM32_F411RE", 0x431),
   ("STM32_F37x", 0x432),
   ("STM32_F4_DE", 0x433),
   ("STM32_F4_DSI", 0x434),
   ("STM32_L43X", 0x435),
   ("STM32_L1_HIGH", 0x436),
   ("STM32_L152_RE", 0x437),
   ("STM32_F334", 0x438),
   ("STM32_F3_SMALL", 0x439),
   ("STM32_F0", 0x440),
   ("STM32_F412", 0x441),
   ("STM32_F09X", 0x442),
   ("STM32_F0_SMALL", 0x444),
   ("STM32_F04", 0x445),
   ("STM32_F303_HIGH", 0x446),
   ("STM32_L0_CAT5", 0x447),
   ("STM32_F0_CAN", 0x448),
   ("STM32_F7", 0x449),
   ("STM32_F7XXXX", 0x451),
   ("STM32_L011", 0x457),
   ("STM32_F410", 0x458),
   ("STM32_F413", 0x463),
];

pub const CORE_IDS: [(&'static str, u32); 6] = [
    ("Cortex-M0 r0p0", 0x0bb11477),
    ("Cortex-M0+ r0p0", 0x0bc11477),
    ("Cortex-M3 r1p0", 0x1ba00477),
    ("Cortex-M3 r2p0", 0x4ba00477),
    ("Cortex-M4 r0p0", 0x2ba00477),
    ("Cortex-M4 r0p1", 0x2ba01477),
];