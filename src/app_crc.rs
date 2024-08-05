#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crc_ctrl0: CrcCtrl0,
    _reserved1: [u8; 0x04],
    crc_ctrl1: CrcCtrl1,
    _reserved2: [u8; 0x04],
    crc_ctrl2: CrcCtrl2,
    _reserved3: [u8; 0x04],
    crc_ints: CrcInts,
    _reserved4: [u8; 0x04],
    crc_intr: CrcIntr,
    _reserved5: [u8; 0x04],
    crc_status_reg: CrcStatusReg,
    _reserved6: [u8; 0x04],
    crc_int_offset_reg: CrcIntOffsetReg,
    _reserved7: [u8; 0x04],
    crc_busy: CrcBusy,
    _reserved8: [u8; 0x04],
    crc_pcount_reg1: CrcPcountReg1,
    crc_scount_reg1: CrcScountReg1,
    crc_cursec_reg1: CrcCursecReg1,
    crc_wdtopld1: CrcWdtopld1,
    crc_bctopld1: CrcBctopld1,
    _reserved13: [u8; 0x0c],
    psa_sigregl1: PsaSigregl1,
    psa_sigregh1: PsaSigregh1,
    crc_regl1: CrcRegl1,
    crc_regh1: CrcRegh1,
    psa_secsigregl1: PsaSecsigregl1,
    psa_secsigregh1: PsaSecsigregh1,
    raw_dataregl1: RawDataregl1,
    raw_dataregh1: RawDataregh1,
    crc_pcount_reg2: CrcPcountReg2,
    crc_scount_reg2: CrcScountReg2,
    crc_cursec_reg2: CrcCursecReg2,
    crc_wdtopld2: CrcWdtopld2,
    crc_bctopld2: CrcBctopld2,
    _reserved26: [u8; 0x0c],
    psa_sigregl2: PsaSigregl2,
    psa_sigregh2: PsaSigregh2,
    crc_regl2: CrcRegl2,
    crc_regh2: CrcRegh2,
    psa_secsigregl2: PsaSecsigregl2,
    psa_secsigregh2: PsaSecsigregh2,
    raw_dataregl2: RawDataregl2,
    raw_dataregh2: RawDataregh2,
    crc_pcount_reg3: CrcPcountReg3,
    crc_scount_reg3: CrcScountReg3,
    crc_cursec_reg3: CrcCursecReg3,
    crc_wdtopld3: CrcWdtopld3,
    crc_bctopld3: CrcBctopld3,
    _reserved39: [u8; 0x0c],
    psa_sigregl3: PsaSigregl3,
    psa_sigregh3: PsaSigregh3,
    crc_regl3: CrcRegl3,
    crc_regh3: CrcRegh3,
    psa_secsigregl3: PsaSecsigregl3,
    psa_secsigregh3: PsaSecsigregh3,
    raw_dataregl3: RawDataregl3,
    raw_dataregh3: RawDataregh3,
    crc_pcount_reg4: CrcPcountReg4,
    crc_scount_reg4: CrcScountReg4,
    crc_cursec_reg4: CrcCursecReg4,
    crc_wdtopld4: CrcWdtopld4,
    crc_bctopld4: CrcBctopld4,
    _reserved52: [u8; 0x0c],
    psa_sigregl4: PsaSigregl4,
    psa_sigregh4: PsaSigregh4,
    crc_regl4: CrcRegl4,
    crc_regh4: CrcRegh4,
    psa_secsigregl4: PsaSecsigregl4,
    psa_secsigregh4: PsaSecsigregh4,
    raw_dataregl4: RawDataregl4,
    raw_dataregh4: RawDataregh4,
    mcrc_bus_sel: McrcBusSel,
    mcrc_reserved: McrcReserved,
}
impl RegisterBlock {
    #[doc = "0x00 - Contains sw reset control bit to reset PSA"]
    #[inline(always)]
    pub const fn crc_ctrl0(&self) -> &CrcCtrl0 {
        &self.crc_ctrl0
    }
    #[doc = "0x08 - Contains power down control bit"]
    #[inline(always)]
    pub const fn crc_ctrl1(&self) -> &CrcCtrl1 {
        &self.crc_ctrl1
    }
    #[doc = "0x10 - Contains channel mode, data trace enable control bits"]
    #[inline(always)]
    pub const fn crc_ctrl2(&self) -> &CrcCtrl2 {
        &self.crc_ctrl2
    }
    #[doc = "0x18 - Write one to a bit to enable a interrupt"]
    #[inline(always)]
    pub const fn crc_ints(&self) -> &CrcInts {
        &self.crc_ints
    }
    #[doc = "0x20 - Write one to a bit to disable a interrupt"]
    #[inline(always)]
    pub const fn crc_intr(&self) -> &CrcIntr {
        &self.crc_intr
    }
    #[doc = "0x28 - Contains interrupt flags for different types of interrupt"]
    #[inline(always)]
    pub const fn crc_status_reg(&self) -> &CrcStatusReg {
        &self.crc_status_reg
    }
    #[doc = "0x30 - Contains the interrupt offset vector address"]
    #[inline(always)]
    pub const fn crc_int_offset_reg(&self) -> &CrcIntOffsetReg {
        &self.crc_int_offset_reg
    }
    #[doc = "0x38 - Contains the busy flag for each channel"]
    #[inline(always)]
    pub const fn crc_busy(&self) -> &CrcBusy {
        &self.crc_busy
    }
    #[doc = "0x40 - Channel 1 preload register for the pattern count"]
    #[inline(always)]
    pub const fn crc_pcount_reg1(&self) -> &CrcPcountReg1 {
        &self.crc_pcount_reg1
    }
    #[doc = "0x44 - Channel 1 preload register for the sector count"]
    #[inline(always)]
    pub const fn crc_scount_reg1(&self) -> &CrcScountReg1 {
        &self.crc_scount_reg1
    }
    #[doc = "0x48 - Channel 1 current sector register contains the sector number which causes CRC failure"]
    #[inline(always)]
    pub const fn crc_cursec_reg1(&self) -> &CrcCursecReg1 {
        &self.crc_cursec_reg1
    }
    #[doc = "0x4c - Channel 1 timeout pre-load value to check if within a given time DMA initiates a block transfer"]
    #[inline(always)]
    pub const fn crc_wdtopld1(&self) -> &CrcWdtopld1 {
        &self.crc_wdtopld1
    }
    #[doc = "0x50 - Channel 1 timeout pre-load value to check if one block of patterns are compressed with a given time"]
    #[inline(always)]
    pub const fn crc_bctopld1(&self) -> &CrcBctopld1 {
        &self.crc_bctopld1
    }
    #[doc = "0x60 - Channel 1 PSA signature low register"]
    #[inline(always)]
    pub const fn psa_sigregl1(&self) -> &PsaSigregl1 {
        &self.psa_sigregl1
    }
    #[doc = "0x64 - Channel 1 PSA signature high register"]
    #[inline(always)]
    pub const fn psa_sigregh1(&self) -> &PsaSigregh1 {
        &self.psa_sigregh1
    }
    #[doc = "0x68 - Channel 1 CRC value low register"]
    #[inline(always)]
    pub const fn crc_regl1(&self) -> &CrcRegl1 {
        &self.crc_regl1
    }
    #[doc = "0x6c - Channel 1 CRC value high register"]
    #[inline(always)]
    pub const fn crc_regh1(&self) -> &CrcRegh1 {
        &self.crc_regh1
    }
    #[doc = "0x70 - Channel 1 PSA sector signature low regis-ter"]
    #[inline(always)]
    pub const fn psa_secsigregl1(&self) -> &PsaSecsigregl1 {
        &self.psa_secsigregl1
    }
    #[doc = "0x74 - Channel 1 PSA sector signature high regis-ter"]
    #[inline(always)]
    pub const fn psa_secsigregh1(&self) -> &PsaSecsigregh1 {
        &self.psa_secsigregh1
    }
    #[doc = "0x78 - Channel 1 un-compressed raw data low register"]
    #[inline(always)]
    pub const fn raw_dataregl1(&self) -> &RawDataregl1 {
        &self.raw_dataregl1
    }
    #[doc = "0x7c - Channel 1 un-compressed raw data high register"]
    #[inline(always)]
    pub const fn raw_dataregh1(&self) -> &RawDataregh1 {
        &self.raw_dataregh1
    }
    #[doc = "0x80 - Channel 2 preload register for the pattern count"]
    #[inline(always)]
    pub const fn crc_pcount_reg2(&self) -> &CrcPcountReg2 {
        &self.crc_pcount_reg2
    }
    #[doc = "0x84 - Channel 2 preload register for the sector count"]
    #[inline(always)]
    pub const fn crc_scount_reg2(&self) -> &CrcScountReg2 {
        &self.crc_scount_reg2
    }
    #[doc = "0x88 - Channel 2 current sector register contains the sector number which causes CRC fail-ure"]
    #[inline(always)]
    pub const fn crc_cursec_reg2(&self) -> &CrcCursecReg2 {
        &self.crc_cursec_reg2
    }
    #[doc = "0x8c - Channel 2 timeout pre-load value to check if within a given time DMA initiates a block transfer"]
    #[inline(always)]
    pub const fn crc_wdtopld2(&self) -> &CrcWdtopld2 {
        &self.crc_wdtopld2
    }
    #[doc = "0x90 - Channel 2 timeout pre-load value to check if one block of patterns are compressed with a given time"]
    #[inline(always)]
    pub const fn crc_bctopld2(&self) -> &CrcBctopld2 {
        &self.crc_bctopld2
    }
    #[doc = "0xa0 - Channel 2 PSA signature low register"]
    #[inline(always)]
    pub const fn psa_sigregl2(&self) -> &PsaSigregl2 {
        &self.psa_sigregl2
    }
    #[doc = "0xa4 - Channel 2 PSA signature high register"]
    #[inline(always)]
    pub const fn psa_sigregh2(&self) -> &PsaSigregh2 {
        &self.psa_sigregh2
    }
    #[doc = "0xa8 - Channel 2 CRC value low register"]
    #[inline(always)]
    pub const fn crc_regl2(&self) -> &CrcRegl2 {
        &self.crc_regl2
    }
    #[doc = "0xac - Channel 2 CRC value high register"]
    #[inline(always)]
    pub const fn crc_regh2(&self) -> &CrcRegh2 {
        &self.crc_regh2
    }
    #[doc = "0xb0 - Channel 2 PSA sector signature low regis-ter"]
    #[inline(always)]
    pub const fn psa_secsigregl2(&self) -> &PsaSecsigregl2 {
        &self.psa_secsigregl2
    }
    #[doc = "0xb4 - Channel 2 PSA sector signature high regis-ter"]
    #[inline(always)]
    pub const fn psa_secsigregh2(&self) -> &PsaSecsigregh2 {
        &self.psa_secsigregh2
    }
    #[doc = "0xb8 - Channel 2 un-compressed raw data low register"]
    #[inline(always)]
    pub const fn raw_dataregl2(&self) -> &RawDataregl2 {
        &self.raw_dataregl2
    }
    #[doc = "0xbc - Channel 2 un-compressed raw data high Register"]
    #[inline(always)]
    pub const fn raw_dataregh2(&self) -> &RawDataregh2 {
        &self.raw_dataregh2
    }
    #[doc = "0xc0 - Channel 3 preload register for the pattern count"]
    #[inline(always)]
    pub const fn crc_pcount_reg3(&self) -> &CrcPcountReg3 {
        &self.crc_pcount_reg3
    }
    #[doc = "0xc4 - Channel 3 preload register for the sector count"]
    #[inline(always)]
    pub const fn crc_scount_reg3(&self) -> &CrcScountReg3 {
        &self.crc_scount_reg3
    }
    #[doc = "0xc8 - Channel 3 current sector register contains the sector number which causes CRC fail-ure"]
    #[inline(always)]
    pub const fn crc_cursec_reg3(&self) -> &CrcCursecReg3 {
        &self.crc_cursec_reg3
    }
    #[doc = "0xcc - Channel 3 timeout pre-load value to check if within a given time DMA initiates a block transfer"]
    #[inline(always)]
    pub const fn crc_wdtopld3(&self) -> &CrcWdtopld3 {
        &self.crc_wdtopld3
    }
    #[doc = "0xd0 - Channel 3 timeout pre-load value to check if one block of patterns are compressed with a given time"]
    #[inline(always)]
    pub const fn crc_bctopld3(&self) -> &CrcBctopld3 {
        &self.crc_bctopld3
    }
    #[doc = "0xe0 - Channel 3 PSA signature low register"]
    #[inline(always)]
    pub const fn psa_sigregl3(&self) -> &PsaSigregl3 {
        &self.psa_sigregl3
    }
    #[doc = "0xe4 - Channel 3 PSA signature high register"]
    #[inline(always)]
    pub const fn psa_sigregh3(&self) -> &PsaSigregh3 {
        &self.psa_sigregh3
    }
    #[doc = "0xe8 - Channel 3 CRC value low register"]
    #[inline(always)]
    pub const fn crc_regl3(&self) -> &CrcRegl3 {
        &self.crc_regl3
    }
    #[doc = "0xec - Channel 3 CRC value high register"]
    #[inline(always)]
    pub const fn crc_regh3(&self) -> &CrcRegh3 {
        &self.crc_regh3
    }
    #[doc = "0xf0 - Channel 3 PSA sector signature low regis-ter"]
    #[inline(always)]
    pub const fn psa_secsigregl3(&self) -> &PsaSecsigregl3 {
        &self.psa_secsigregl3
    }
    #[doc = "0xf4 - Channel 3 PSA sector signature high regis-ter"]
    #[inline(always)]
    pub const fn psa_secsigregh3(&self) -> &PsaSecsigregh3 {
        &self.psa_secsigregh3
    }
    #[doc = "0xf8 - Channel 3 un-compressed raw data low register"]
    #[inline(always)]
    pub const fn raw_dataregl3(&self) -> &RawDataregl3 {
        &self.raw_dataregl3
    }
    #[doc = "0xfc - Channel 3 un-compressed raw data high Register"]
    #[inline(always)]
    pub const fn raw_dataregh3(&self) -> &RawDataregh3 {
        &self.raw_dataregh3
    }
    #[doc = "0x100 - Channel 4 preload register for the pattern count"]
    #[inline(always)]
    pub const fn crc_pcount_reg4(&self) -> &CrcPcountReg4 {
        &self.crc_pcount_reg4
    }
    #[doc = "0x104 - Channel 4 preload register for the sector count"]
    #[inline(always)]
    pub const fn crc_scount_reg4(&self) -> &CrcScountReg4 {
        &self.crc_scount_reg4
    }
    #[doc = "0x108 - Channel 4 current sector register contains the sector number which causes CRC fail-ure"]
    #[inline(always)]
    pub const fn crc_cursec_reg4(&self) -> &CrcCursecReg4 {
        &self.crc_cursec_reg4
    }
    #[doc = "0x10c - Channel 4 timeout pre-load value to check if within a given time DMA initiates a block transfer"]
    #[inline(always)]
    pub const fn crc_wdtopld4(&self) -> &CrcWdtopld4 {
        &self.crc_wdtopld4
    }
    #[doc = "0x110 - Channel 4 timeout pre-load value to check if one block of patterns are compressed with a given time"]
    #[inline(always)]
    pub const fn crc_bctopld4(&self) -> &CrcBctopld4 {
        &self.crc_bctopld4
    }
    #[doc = "0x120 - Channel 4 PSA signature low register"]
    #[inline(always)]
    pub const fn psa_sigregl4(&self) -> &PsaSigregl4 {
        &self.psa_sigregl4
    }
    #[doc = "0x124 - Channel 4 PSA signature high register"]
    #[inline(always)]
    pub const fn psa_sigregh4(&self) -> &PsaSigregh4 {
        &self.psa_sigregh4
    }
    #[doc = "0x128 - Channel 4 CRC value low register"]
    #[inline(always)]
    pub const fn crc_regl4(&self) -> &CrcRegl4 {
        &self.crc_regl4
    }
    #[doc = "0x12c - Channel 4 CRC value high register"]
    #[inline(always)]
    pub const fn crc_regh4(&self) -> &CrcRegh4 {
        &self.crc_regh4
    }
    #[doc = "0x130 - Channel 4 PSA sector signature low regis-ter"]
    #[inline(always)]
    pub const fn psa_secsigregl4(&self) -> &PsaSecsigregl4 {
        &self.psa_secsigregl4
    }
    #[doc = "0x134 - Channel 4 PSA sector signature high regis-ter"]
    #[inline(always)]
    pub const fn psa_secsigregh4(&self) -> &PsaSecsigregh4 {
        &self.psa_secsigregh4
    }
    #[doc = "0x138 - Channel 4 un-compressed raw data low register"]
    #[inline(always)]
    pub const fn raw_dataregl4(&self) -> &RawDataregl4 {
        &self.raw_dataregl4
    }
    #[doc = "0x13c - Channel 4 un-compressed raw data high Register"]
    #[inline(always)]
    pub const fn raw_dataregh4(&self) -> &RawDataregh4 {
        &self.raw_dataregh4
    }
    #[doc = "0x140 - Disables either or all tracing of data buses"]
    #[inline(always)]
    pub const fn mcrc_bus_sel(&self) -> &McrcBusSel {
        &self.mcrc_bus_sel
    }
    #[doc = "0x144 - 0x144 to 0x1FF is reserved area."]
    #[inline(always)]
    pub const fn mcrc_reserved(&self) -> &McrcReserved {
        &self.mcrc_reserved
    }
}
#[doc = "CRC_CTRL0 (rw) register accessor: Contains sw reset control bit to reset PSA\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctrl0`]
module"]
#[doc(alias = "CRC_CTRL0")]
pub type CrcCtrl0 = crate::Reg<crc_ctrl0::CrcCtrl0Spec>;
#[doc = "Contains sw reset control bit to reset PSA"]
pub mod crc_ctrl0;
#[doc = "CRC_CTRL1 (rw) register accessor: Contains power down control bit\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctrl1`]
module"]
#[doc(alias = "CRC_CTRL1")]
pub type CrcCtrl1 = crate::Reg<crc_ctrl1::CrcCtrl1Spec>;
#[doc = "Contains power down control bit"]
pub mod crc_ctrl1;
#[doc = "CRC_CTRL2 (rw) register accessor: Contains channel mode, data trace enable control bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctrl2`]
module"]
#[doc(alias = "CRC_CTRL2")]
pub type CrcCtrl2 = crate::Reg<crc_ctrl2::CrcCtrl2Spec>;
#[doc = "Contains channel mode, data trace enable control bits"]
pub mod crc_ctrl2;
#[doc = "CRC_INTS (rw) register accessor: Write one to a bit to enable a interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ints::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ints::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ints`]
module"]
#[doc(alias = "CRC_INTS")]
pub type CrcInts = crate::Reg<crc_ints::CrcIntsSpec>;
#[doc = "Write one to a bit to enable a interrupt"]
pub mod crc_ints;
#[doc = "CRC_INTR (rw) register accessor: Write one to a bit to disable a interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_intr`]
module"]
#[doc(alias = "CRC_INTR")]
pub type CrcIntr = crate::Reg<crc_intr::CrcIntrSpec>;
#[doc = "Write one to a bit to disable a interrupt"]
pub mod crc_intr;
#[doc = "CRC_STATUS_REG (rw) register accessor: Contains interrupt flags for different types of interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_status_reg`]
module"]
#[doc(alias = "CRC_STATUS_REG")]
pub type CrcStatusReg = crate::Reg<crc_status_reg::CrcStatusRegSpec>;
#[doc = "Contains interrupt flags for different types of interrupt"]
pub mod crc_status_reg;
#[doc = "CRC_INT_OFFSET_REG (rw) register accessor: Contains the interrupt offset vector address\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_int_offset_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_int_offset_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_int_offset_reg`]
module"]
#[doc(alias = "CRC_INT_OFFSET_REG")]
pub type CrcIntOffsetReg = crate::Reg<crc_int_offset_reg::CrcIntOffsetRegSpec>;
#[doc = "Contains the interrupt offset vector address"]
pub mod crc_int_offset_reg;
#[doc = "CRC_BUSY (rw) register accessor: Contains the busy flag for each channel\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_busy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_busy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_busy`]
module"]
#[doc(alias = "CRC_BUSY")]
pub type CrcBusy = crate::Reg<crc_busy::CrcBusySpec>;
#[doc = "Contains the busy flag for each channel"]
pub mod crc_busy;
#[doc = "CRC_PCOUNT_REG1 (rw) register accessor: Channel 1 preload register for the pattern count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pcount_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pcount_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pcount_reg1`]
module"]
#[doc(alias = "CRC_PCOUNT_REG1")]
pub type CrcPcountReg1 = crate::Reg<crc_pcount_reg1::CrcPcountReg1Spec>;
#[doc = "Channel 1 preload register for the pattern count"]
pub mod crc_pcount_reg1;
#[doc = "CRC_SCOUNT_REG1 (rw) register accessor: Channel 1 preload register for the sector count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_scount_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_scount_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_scount_reg1`]
module"]
#[doc(alias = "CRC_SCOUNT_REG1")]
pub type CrcScountReg1 = crate::Reg<crc_scount_reg1::CrcScountReg1Spec>;
#[doc = "Channel 1 preload register for the sector count"]
pub mod crc_scount_reg1;
#[doc = "CRC_CURSEC_REG1 (rw) register accessor: Channel 1 current sector register contains the sector number which causes CRC failure\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_cursec_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cursec_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_cursec_reg1`]
module"]
#[doc(alias = "CRC_CURSEC_REG1")]
pub type CrcCursecReg1 = crate::Reg<crc_cursec_reg1::CrcCursecReg1Spec>;
#[doc = "Channel 1 current sector register contains the sector number which causes CRC failure"]
pub mod crc_cursec_reg1;
#[doc = "CRC_WDTOPLD1 (rw) register accessor: Channel 1 timeout pre-load value to check if within a given time DMA initiates a block transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_wdtopld1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_wdtopld1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_wdtopld1`]
module"]
#[doc(alias = "CRC_WDTOPLD1")]
pub type CrcWdtopld1 = crate::Reg<crc_wdtopld1::CrcWdtopld1Spec>;
#[doc = "Channel 1 timeout pre-load value to check if within a given time DMA initiates a block transfer"]
pub mod crc_wdtopld1;
#[doc = "CRC_BCTOPLD1 (rw) register accessor: Channel 1 timeout pre-load value to check if one block of patterns are compressed with a given time\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_bctopld1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_bctopld1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_bctopld1`]
module"]
#[doc(alias = "CRC_BCTOPLD1")]
pub type CrcBctopld1 = crate::Reg<crc_bctopld1::CrcBctopld1Spec>;
#[doc = "Channel 1 timeout pre-load value to check if one block of patterns are compressed with a given time"]
pub mod crc_bctopld1;
#[doc = "PSA_SIGREGL1 (rw) register accessor: Channel 1 PSA signature low register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_sigregl1`]
module"]
#[doc(alias = "PSA_SIGREGL1")]
pub type PsaSigregl1 = crate::Reg<psa_sigregl1::PsaSigregl1Spec>;
#[doc = "Channel 1 PSA signature low register"]
pub mod psa_sigregl1;
#[doc = "PSA_SIGREGH1 (rw) register accessor: Channel 1 PSA signature high register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_sigregh1`]
module"]
#[doc(alias = "PSA_SIGREGH1")]
pub type PsaSigregh1 = crate::Reg<psa_sigregh1::PsaSigregh1Spec>;
#[doc = "Channel 1 PSA signature high register"]
pub mod psa_sigregh1;
#[doc = "CRC_REGL1 (rw) register accessor: Channel 1 CRC value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_regl1`]
module"]
#[doc(alias = "CRC_REGL1")]
pub type CrcRegl1 = crate::Reg<crc_regl1::CrcRegl1Spec>;
#[doc = "Channel 1 CRC value low register"]
pub mod crc_regl1;
#[doc = "CRC_REGH1 (rw) register accessor: Channel 1 CRC value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_regh1`]
module"]
#[doc(alias = "CRC_REGH1")]
pub type CrcRegh1 = crate::Reg<crc_regh1::CrcRegh1Spec>;
#[doc = "Channel 1 CRC value high register"]
pub mod crc_regh1;
#[doc = "PSA_SECSIGREGL1 (rw) register accessor: Channel 1 PSA sector signature low regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_secsigregl1`]
module"]
#[doc(alias = "PSA_SECSIGREGL1")]
pub type PsaSecsigregl1 = crate::Reg<psa_secsigregl1::PsaSecsigregl1Spec>;
#[doc = "Channel 1 PSA sector signature low regis-ter"]
pub mod psa_secsigregl1;
#[doc = "PSA_SECSIGREGH1 (rw) register accessor: Channel 1 PSA sector signature high regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_secsigregh1`]
module"]
#[doc(alias = "PSA_SECSIGREGH1")]
pub type PsaSecsigregh1 = crate::Reg<psa_secsigregh1::PsaSecsigregh1Spec>;
#[doc = "Channel 1 PSA sector signature high regis-ter"]
pub mod psa_secsigregh1;
#[doc = "RAW_DATAREGL1 (rw) register accessor: Channel 1 un-compressed raw data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dataregl1`]
module"]
#[doc(alias = "RAW_DATAREGL1")]
pub type RawDataregl1 = crate::Reg<raw_dataregl1::RawDataregl1Spec>;
#[doc = "Channel 1 un-compressed raw data low register"]
pub mod raw_dataregl1;
#[doc = "RAW_DATAREGH1 (rw) register accessor: Channel 1 un-compressed raw data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dataregh1`]
module"]
#[doc(alias = "RAW_DATAREGH1")]
pub type RawDataregh1 = crate::Reg<raw_dataregh1::RawDataregh1Spec>;
#[doc = "Channel 1 un-compressed raw data high register"]
pub mod raw_dataregh1;
#[doc = "CRC_PCOUNT_REG2 (rw) register accessor: Channel 2 preload register for the pattern count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pcount_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pcount_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pcount_reg2`]
module"]
#[doc(alias = "CRC_PCOUNT_REG2")]
pub type CrcPcountReg2 = crate::Reg<crc_pcount_reg2::CrcPcountReg2Spec>;
#[doc = "Channel 2 preload register for the pattern count"]
pub mod crc_pcount_reg2;
#[doc = "CRC_SCOUNT_REG2 (rw) register accessor: Channel 2 preload register for the sector count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_scount_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_scount_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_scount_reg2`]
module"]
#[doc(alias = "CRC_SCOUNT_REG2")]
pub type CrcScountReg2 = crate::Reg<crc_scount_reg2::CrcScountReg2Spec>;
#[doc = "Channel 2 preload register for the sector count"]
pub mod crc_scount_reg2;
#[doc = "CRC_CURSEC_REG2 (rw) register accessor: Channel 2 current sector register contains the sector number which causes CRC fail-ure\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_cursec_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cursec_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_cursec_reg2`]
module"]
#[doc(alias = "CRC_CURSEC_REG2")]
pub type CrcCursecReg2 = crate::Reg<crc_cursec_reg2::CrcCursecReg2Spec>;
#[doc = "Channel 2 current sector register contains the sector number which causes CRC fail-ure"]
pub mod crc_cursec_reg2;
#[doc = "CRC_WDTOPLD2 (rw) register accessor: Channel 2 timeout pre-load value to check if within a given time DMA initiates a block transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_wdtopld2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_wdtopld2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_wdtopld2`]
module"]
#[doc(alias = "CRC_WDTOPLD2")]
pub type CrcWdtopld2 = crate::Reg<crc_wdtopld2::CrcWdtopld2Spec>;
#[doc = "Channel 2 timeout pre-load value to check if within a given time DMA initiates a block transfer"]
pub mod crc_wdtopld2;
#[doc = "CRC_BCTOPLD2 (rw) register accessor: Channel 2 timeout pre-load value to check if one block of patterns are compressed with a given time\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_bctopld2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_bctopld2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_bctopld2`]
module"]
#[doc(alias = "CRC_BCTOPLD2")]
pub type CrcBctopld2 = crate::Reg<crc_bctopld2::CrcBctopld2Spec>;
#[doc = "Channel 2 timeout pre-load value to check if one block of patterns are compressed with a given time"]
pub mod crc_bctopld2;
#[doc = "PSA_SIGREGL2 (rw) register accessor: Channel 2 PSA signature low register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_sigregl2`]
module"]
#[doc(alias = "PSA_SIGREGL2")]
pub type PsaSigregl2 = crate::Reg<psa_sigregl2::PsaSigregl2Spec>;
#[doc = "Channel 2 PSA signature low register"]
pub mod psa_sigregl2;
#[doc = "PSA_SIGREGH2 (rw) register accessor: Channel 2 PSA signature high register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_sigregh2`]
module"]
#[doc(alias = "PSA_SIGREGH2")]
pub type PsaSigregh2 = crate::Reg<psa_sigregh2::PsaSigregh2Spec>;
#[doc = "Channel 2 PSA signature high register"]
pub mod psa_sigregh2;
#[doc = "CRC_REGL2 (rw) register accessor: Channel 2 CRC value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_regl2`]
module"]
#[doc(alias = "CRC_REGL2")]
pub type CrcRegl2 = crate::Reg<crc_regl2::CrcRegl2Spec>;
#[doc = "Channel 2 CRC value low register"]
pub mod crc_regl2;
#[doc = "CRC_REGH2 (rw) register accessor: Channel 2 CRC value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_regh2`]
module"]
#[doc(alias = "CRC_REGH2")]
pub type CrcRegh2 = crate::Reg<crc_regh2::CrcRegh2Spec>;
#[doc = "Channel 2 CRC value high register"]
pub mod crc_regh2;
#[doc = "PSA_SECSIGREGL2 (rw) register accessor: Channel 2 PSA sector signature low regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_secsigregl2`]
module"]
#[doc(alias = "PSA_SECSIGREGL2")]
pub type PsaSecsigregl2 = crate::Reg<psa_secsigregl2::PsaSecsigregl2Spec>;
#[doc = "Channel 2 PSA sector signature low regis-ter"]
pub mod psa_secsigregl2;
#[doc = "PSA_SECSIGREGH2 (rw) register accessor: Channel 2 PSA sector signature high regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_secsigregh2`]
module"]
#[doc(alias = "PSA_SECSIGREGH2")]
pub type PsaSecsigregh2 = crate::Reg<psa_secsigregh2::PsaSecsigregh2Spec>;
#[doc = "Channel 2 PSA sector signature high regis-ter"]
pub mod psa_secsigregh2;
#[doc = "RAW_DATAREGL2 (rw) register accessor: Channel 2 un-compressed raw data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dataregl2`]
module"]
#[doc(alias = "RAW_DATAREGL2")]
pub type RawDataregl2 = crate::Reg<raw_dataregl2::RawDataregl2Spec>;
#[doc = "Channel 2 un-compressed raw data low register"]
pub mod raw_dataregl2;
#[doc = "RAW_DATAREGH2 (rw) register accessor: Channel 2 un-compressed raw data high Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dataregh2`]
module"]
#[doc(alias = "RAW_DATAREGH2")]
pub type RawDataregh2 = crate::Reg<raw_dataregh2::RawDataregh2Spec>;
#[doc = "Channel 2 un-compressed raw data high Register"]
pub mod raw_dataregh2;
#[doc = "CRC_PCOUNT_REG3 (rw) register accessor: Channel 3 preload register for the pattern count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pcount_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pcount_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pcount_reg3`]
module"]
#[doc(alias = "CRC_PCOUNT_REG3")]
pub type CrcPcountReg3 = crate::Reg<crc_pcount_reg3::CrcPcountReg3Spec>;
#[doc = "Channel 3 preload register for the pattern count"]
pub mod crc_pcount_reg3;
#[doc = "CRC_SCOUNT_REG3 (rw) register accessor: Channel 3 preload register for the sector count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_scount_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_scount_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_scount_reg3`]
module"]
#[doc(alias = "CRC_SCOUNT_REG3")]
pub type CrcScountReg3 = crate::Reg<crc_scount_reg3::CrcScountReg3Spec>;
#[doc = "Channel 3 preload register for the sector count"]
pub mod crc_scount_reg3;
#[doc = "CRC_CURSEC_REG3 (rw) register accessor: Channel 3 current sector register contains the sector number which causes CRC fail-ure\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_cursec_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cursec_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_cursec_reg3`]
module"]
#[doc(alias = "CRC_CURSEC_REG3")]
pub type CrcCursecReg3 = crate::Reg<crc_cursec_reg3::CrcCursecReg3Spec>;
#[doc = "Channel 3 current sector register contains the sector number which causes CRC fail-ure"]
pub mod crc_cursec_reg3;
#[doc = "CRC_WDTOPLD3 (rw) register accessor: Channel 3 timeout pre-load value to check if within a given time DMA initiates a block transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_wdtopld3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_wdtopld3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_wdtopld3`]
module"]
#[doc(alias = "CRC_WDTOPLD3")]
pub type CrcWdtopld3 = crate::Reg<crc_wdtopld3::CrcWdtopld3Spec>;
#[doc = "Channel 3 timeout pre-load value to check if within a given time DMA initiates a block transfer"]
pub mod crc_wdtopld3;
#[doc = "CRC_BCTOPLD3 (rw) register accessor: Channel 3 timeout pre-load value to check if one block of patterns are compressed with a given time\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_bctopld3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_bctopld3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_bctopld3`]
module"]
#[doc(alias = "CRC_BCTOPLD3")]
pub type CrcBctopld3 = crate::Reg<crc_bctopld3::CrcBctopld3Spec>;
#[doc = "Channel 3 timeout pre-load value to check if one block of patterns are compressed with a given time"]
pub mod crc_bctopld3;
#[doc = "PSA_SIGREGL3 (rw) register accessor: Channel 3 PSA signature low register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_sigregl3`]
module"]
#[doc(alias = "PSA_SIGREGL3")]
pub type PsaSigregl3 = crate::Reg<psa_sigregl3::PsaSigregl3Spec>;
#[doc = "Channel 3 PSA signature low register"]
pub mod psa_sigregl3;
#[doc = "PSA_SIGREGH3 (rw) register accessor: Channel 3 PSA signature high register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_sigregh3`]
module"]
#[doc(alias = "PSA_SIGREGH3")]
pub type PsaSigregh3 = crate::Reg<psa_sigregh3::PsaSigregh3Spec>;
#[doc = "Channel 3 PSA signature high register"]
pub mod psa_sigregh3;
#[doc = "CRC_REGL3 (rw) register accessor: Channel 3 CRC value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_regl3`]
module"]
#[doc(alias = "CRC_REGL3")]
pub type CrcRegl3 = crate::Reg<crc_regl3::CrcRegl3Spec>;
#[doc = "Channel 3 CRC value low register"]
pub mod crc_regl3;
#[doc = "CRC_REGH3 (rw) register accessor: Channel 3 CRC value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_regh3`]
module"]
#[doc(alias = "CRC_REGH3")]
pub type CrcRegh3 = crate::Reg<crc_regh3::CrcRegh3Spec>;
#[doc = "Channel 3 CRC value high register"]
pub mod crc_regh3;
#[doc = "PSA_SECSIGREGL3 (rw) register accessor: Channel 3 PSA sector signature low regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_secsigregl3`]
module"]
#[doc(alias = "PSA_SECSIGREGL3")]
pub type PsaSecsigregl3 = crate::Reg<psa_secsigregl3::PsaSecsigregl3Spec>;
#[doc = "Channel 3 PSA sector signature low regis-ter"]
pub mod psa_secsigregl3;
#[doc = "PSA_SECSIGREGH3 (rw) register accessor: Channel 3 PSA sector signature high regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_secsigregh3`]
module"]
#[doc(alias = "PSA_SECSIGREGH3")]
pub type PsaSecsigregh3 = crate::Reg<psa_secsigregh3::PsaSecsigregh3Spec>;
#[doc = "Channel 3 PSA sector signature high regis-ter"]
pub mod psa_secsigregh3;
#[doc = "RAW_DATAREGL3 (rw) register accessor: Channel 3 un-compressed raw data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dataregl3`]
module"]
#[doc(alias = "RAW_DATAREGL3")]
pub type RawDataregl3 = crate::Reg<raw_dataregl3::RawDataregl3Spec>;
#[doc = "Channel 3 un-compressed raw data low register"]
pub mod raw_dataregl3;
#[doc = "RAW_DATAREGH3 (rw) register accessor: Channel 3 un-compressed raw data high Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dataregh3`]
module"]
#[doc(alias = "RAW_DATAREGH3")]
pub type RawDataregh3 = crate::Reg<raw_dataregh3::RawDataregh3Spec>;
#[doc = "Channel 3 un-compressed raw data high Register"]
pub mod raw_dataregh3;
#[doc = "CRC_PCOUNT_REG4 (rw) register accessor: Channel 4 preload register for the pattern count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pcount_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pcount_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pcount_reg4`]
module"]
#[doc(alias = "CRC_PCOUNT_REG4")]
pub type CrcPcountReg4 = crate::Reg<crc_pcount_reg4::CrcPcountReg4Spec>;
#[doc = "Channel 4 preload register for the pattern count"]
pub mod crc_pcount_reg4;
#[doc = "CRC_SCOUNT_REG4 (rw) register accessor: Channel 4 preload register for the sector count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_scount_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_scount_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_scount_reg4`]
module"]
#[doc(alias = "CRC_SCOUNT_REG4")]
pub type CrcScountReg4 = crate::Reg<crc_scount_reg4::CrcScountReg4Spec>;
#[doc = "Channel 4 preload register for the sector count"]
pub mod crc_scount_reg4;
#[doc = "CRC_CURSEC_REG4 (rw) register accessor: Channel 4 current sector register contains the sector number which causes CRC fail-ure\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_cursec_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cursec_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_cursec_reg4`]
module"]
#[doc(alias = "CRC_CURSEC_REG4")]
pub type CrcCursecReg4 = crate::Reg<crc_cursec_reg4::CrcCursecReg4Spec>;
#[doc = "Channel 4 current sector register contains the sector number which causes CRC fail-ure"]
pub mod crc_cursec_reg4;
#[doc = "CRC_WDTOPLD4 (rw) register accessor: Channel 4 timeout pre-load value to check if within a given time DMA initiates a block transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_wdtopld4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_wdtopld4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_wdtopld4`]
module"]
#[doc(alias = "CRC_WDTOPLD4")]
pub type CrcWdtopld4 = crate::Reg<crc_wdtopld4::CrcWdtopld4Spec>;
#[doc = "Channel 4 timeout pre-load value to check if within a given time DMA initiates a block transfer"]
pub mod crc_wdtopld4;
#[doc = "CRC_BCTOPLD4 (rw) register accessor: Channel 4 timeout pre-load value to check if one block of patterns are compressed with a given time\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_bctopld4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_bctopld4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_bctopld4`]
module"]
#[doc(alias = "CRC_BCTOPLD4")]
pub type CrcBctopld4 = crate::Reg<crc_bctopld4::CrcBctopld4Spec>;
#[doc = "Channel 4 timeout pre-load value to check if one block of patterns are compressed with a given time"]
pub mod crc_bctopld4;
#[doc = "PSA_SIGREGL4 (rw) register accessor: Channel 4 PSA signature low register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_sigregl4`]
module"]
#[doc(alias = "PSA_SIGREGL4")]
pub type PsaSigregl4 = crate::Reg<psa_sigregl4::PsaSigregl4Spec>;
#[doc = "Channel 4 PSA signature low register"]
pub mod psa_sigregl4;
#[doc = "PSA_SIGREGH4 (rw) register accessor: Channel 4 PSA signature high register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_sigregh4`]
module"]
#[doc(alias = "PSA_SIGREGH4")]
pub type PsaSigregh4 = crate::Reg<psa_sigregh4::PsaSigregh4Spec>;
#[doc = "Channel 4 PSA signature high register"]
pub mod psa_sigregh4;
#[doc = "CRC_REGL4 (rw) register accessor: Channel 4 CRC value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_regl4`]
module"]
#[doc(alias = "CRC_REGL4")]
pub type CrcRegl4 = crate::Reg<crc_regl4::CrcRegl4Spec>;
#[doc = "Channel 4 CRC value low register"]
pub mod crc_regl4;
#[doc = "CRC_REGH4 (rw) register accessor: Channel 4 CRC value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_regh4`]
module"]
#[doc(alias = "CRC_REGH4")]
pub type CrcRegh4 = crate::Reg<crc_regh4::CrcRegh4Spec>;
#[doc = "Channel 4 CRC value high register"]
pub mod crc_regh4;
#[doc = "PSA_SECSIGREGL4 (rw) register accessor: Channel 4 PSA sector signature low regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_secsigregl4`]
module"]
#[doc(alias = "PSA_SECSIGREGL4")]
pub type PsaSecsigregl4 = crate::Reg<psa_secsigregl4::PsaSecsigregl4Spec>;
#[doc = "Channel 4 PSA sector signature low regis-ter"]
pub mod psa_secsigregl4;
#[doc = "PSA_SECSIGREGH4 (rw) register accessor: Channel 4 PSA sector signature high regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psa_secsigregh4`]
module"]
#[doc(alias = "PSA_SECSIGREGH4")]
pub type PsaSecsigregh4 = crate::Reg<psa_secsigregh4::PsaSecsigregh4Spec>;
#[doc = "Channel 4 PSA sector signature high regis-ter"]
pub mod psa_secsigregh4;
#[doc = "RAW_DATAREGL4 (rw) register accessor: Channel 4 un-compressed raw data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dataregl4`]
module"]
#[doc(alias = "RAW_DATAREGL4")]
pub type RawDataregl4 = crate::Reg<raw_dataregl4::RawDataregl4Spec>;
#[doc = "Channel 4 un-compressed raw data low register"]
pub mod raw_dataregl4;
#[doc = "RAW_DATAREGH4 (rw) register accessor: Channel 4 un-compressed raw data high Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dataregh4`]
module"]
#[doc(alias = "RAW_DATAREGH4")]
pub type RawDataregh4 = crate::Reg<raw_dataregh4::RawDataregh4Spec>;
#[doc = "Channel 4 un-compressed raw data high Register"]
pub mod raw_dataregh4;
#[doc = "MCRC_BUS_SEL (rw) register accessor: Disables either or all tracing of data buses\n\nYou can [`read`](crate::Reg::read) this register and get [`mcrc_bus_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcrc_bus_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc_bus_sel`]
module"]
#[doc(alias = "MCRC_BUS_SEL")]
pub type McrcBusSel = crate::Reg<mcrc_bus_sel::McrcBusSelSpec>;
#[doc = "Disables either or all tracing of data buses"]
pub mod mcrc_bus_sel;
#[doc = "MCRC_RESERVED (rw) register accessor: 0x144 to 0x1FF is reserved area.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcrc_reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcrc_reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc_reserved`]
module"]
#[doc(alias = "MCRC_RESERVED")]
pub type McrcReserved = crate::Reg<mcrc_reserved::McrcReservedSpec>;
#[doc = "0x144 to 0x1FF is reserved area."]
pub mod mcrc_reserved;
