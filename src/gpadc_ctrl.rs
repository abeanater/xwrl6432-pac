#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg0: Reg0,
    reg1: Reg1,
    reg2: Reg2,
    reg3: Reg3,
    reg4: Reg4,
    reg5: Reg5,
    reg6: Reg6,
    reg7: Reg7,
    reg8: Reg8,
    reg9: Reg9,
    reg10: Reg10,
    reg11: Reg11,
    reg12: Reg12,
    reg13: Reg13,
    reg14: Reg14,
    reg15: Reg15,
    reg16: Reg16,
    reg17: Reg17,
    reg18: Reg18,
    reg19: Reg19,
    reg20: Reg20,
    reg21: Reg21,
    reg22: Reg22,
}
impl RegisterBlock {
    #[doc = "0x00 - gpadc modes and enable"]
    #[inline(always)]
    pub const fn reg0(&self) -> &Reg0 {
        &self.reg0
    }
    #[doc = "0x04 - gpadc start trigger for Inter frame mode"]
    #[inline(always)]
    pub const fn reg1(&self) -> &Reg1 {
        &self.reg1
    }
    #[doc = "0x08 - gpadc config for IFM"]
    #[inline(always)]
    pub const fn reg2(&self) -> &Reg2 {
        &self.reg2
    }
    #[doc = "0x0c - gpadc param, skip samples and collect samples for IFM"]
    #[inline(always)]
    pub const fn reg3(&self) -> &Reg3 {
        &self.reg3
    }
    #[doc = "0x10 - Base address for Chirp profile 0 in instruction packet RAM"]
    #[inline(always)]
    pub const fn reg4(&self) -> &Reg4 {
        &self.reg4
    }
    #[doc = "0x14 - Base address for Chirp profile 1 in instruction packet RAM"]
    #[inline(always)]
    pub const fn reg5(&self) -> &Reg5 {
        &self.reg5
    }
    #[doc = "0x18 - Base address for Chirp profile 2 in instruction packet RAM"]
    #[inline(always)]
    pub const fn reg6(&self) -> &Reg6 {
        &self.reg6
    }
    #[doc = "0x1c - Base address for Chirp profile 3 in instruction packet RAM"]
    #[inline(always)]
    pub const fn reg7(&self) -> &Reg7 {
        &self.reg7
    }
    #[doc = "0x20 - REG8"]
    #[inline(always)]
    pub const fn reg8(&self) -> &Reg8 {
        &self.reg8
    }
    #[doc = "0x24 - REG9"]
    #[inline(always)]
    pub const fn reg9(&self) -> &Reg9 {
        &self.reg9
    }
    #[doc = "0x28 - REG10"]
    #[inline(always)]
    pub const fn reg10(&self) -> &Reg10 {
        &self.reg10
    }
    #[doc = "0x2c - REG11"]
    #[inline(always)]
    pub const fn reg11(&self) -> &Reg11 {
        &self.reg11
    }
    #[doc = "0x30 - REG12"]
    #[inline(always)]
    pub const fn reg12(&self) -> &Reg12 {
        &self.reg12
    }
    #[doc = "0x34 - REG13"]
    #[inline(always)]
    pub const fn reg13(&self) -> &Reg13 {
        &self.reg13
    }
    #[doc = "0x38 - Sum of GP ADC readings"]
    #[inline(always)]
    pub const fn reg14(&self) -> &Reg14 {
        &self.reg14
    }
    #[doc = "0x3c - Min and Max of GP ADC readings"]
    #[inline(always)]
    pub const fn reg15(&self) -> &Reg15 {
        &self.reg15
    }
    #[doc = "0x40 - REG16"]
    #[inline(always)]
    pub const fn reg16(&self) -> &Reg16 {
        &self.reg16
    }
    #[doc = "0x44 - REG17"]
    #[inline(always)]
    pub const fn reg17(&self) -> &Reg17 {
        &self.reg17
    }
    #[doc = "0x48 - REG18"]
    #[inline(always)]
    pub const fn reg18(&self) -> &Reg18 {
        &self.reg18
    }
    #[doc = "0x4c - REG19"]
    #[inline(always)]
    pub const fn reg19(&self) -> &Reg19 {
        &self.reg19
    }
    #[doc = "0x50 - REG20"]
    #[inline(always)]
    pub const fn reg20(&self) -> &Reg20 {
        &self.reg20
    }
    #[doc = "0x54 - REG21"]
    #[inline(always)]
    pub const fn reg21(&self) -> &Reg21 {
        &self.reg21
    }
    #[doc = "0x58 - REG22"]
    #[inline(always)]
    pub const fn reg22(&self) -> &Reg22 {
        &self.reg22
    }
}
#[doc = "REG0 (rw) register accessor: gpadc modes and enable\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg0`]
module"]
#[doc(alias = "REG0")]
pub type Reg0 = crate::Reg<reg0::Reg0Spec>;
#[doc = "gpadc modes and enable"]
pub mod reg0;
#[doc = "REG1 (rw) register accessor: gpadc start trigger for Inter frame mode\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg1`]
module"]
#[doc(alias = "REG1")]
pub type Reg1 = crate::Reg<reg1::Reg1Spec>;
#[doc = "gpadc start trigger for Inter frame mode"]
pub mod reg1;
#[doc = "REG2 (rw) register accessor: gpadc config for IFM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg2`]
module"]
#[doc(alias = "REG2")]
pub type Reg2 = crate::Reg<reg2::Reg2Spec>;
#[doc = "gpadc config for IFM"]
pub mod reg2;
#[doc = "REG3 (rw) register accessor: gpadc param, skip samples and collect samples for IFM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg3`]
module"]
#[doc(alias = "REG3")]
pub type Reg3 = crate::Reg<reg3::Reg3Spec>;
#[doc = "gpadc param, skip samples and collect samples for IFM"]
pub mod reg3;
#[doc = "REG4 (rw) register accessor: Base address for Chirp profile 0 in instruction packet RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg4`]
module"]
#[doc(alias = "REG4")]
pub type Reg4 = crate::Reg<reg4::Reg4Spec>;
#[doc = "Base address for Chirp profile 0 in instruction packet RAM"]
pub mod reg4;
#[doc = "REG5 (rw) register accessor: Base address for Chirp profile 1 in instruction packet RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg5`]
module"]
#[doc(alias = "REG5")]
pub type Reg5 = crate::Reg<reg5::Reg5Spec>;
#[doc = "Base address for Chirp profile 1 in instruction packet RAM"]
pub mod reg5;
#[doc = "REG6 (rw) register accessor: Base address for Chirp profile 2 in instruction packet RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg6`]
module"]
#[doc(alias = "REG6")]
pub type Reg6 = crate::Reg<reg6::Reg6Spec>;
#[doc = "Base address for Chirp profile 2 in instruction packet RAM"]
pub mod reg6;
#[doc = "REG7 (rw) register accessor: Base address for Chirp profile 3 in instruction packet RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg7`]
module"]
#[doc(alias = "REG7")]
pub type Reg7 = crate::Reg<reg7::Reg7Spec>;
#[doc = "Base address for Chirp profile 3 in instruction packet RAM"]
pub mod reg7;
#[doc = "REG8 (rw) register accessor: REG8\n\nYou can [`read`](crate::Reg::read) this register and get [`reg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg8`]
module"]
#[doc(alias = "REG8")]
pub type Reg8 = crate::Reg<reg8::Reg8Spec>;
#[doc = "REG8"]
pub mod reg8;
#[doc = "REG9 (rw) register accessor: REG9\n\nYou can [`read`](crate::Reg::read) this register and get [`reg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg9`]
module"]
#[doc(alias = "REG9")]
pub type Reg9 = crate::Reg<reg9::Reg9Spec>;
#[doc = "REG9"]
pub mod reg9;
#[doc = "REG10 (rw) register accessor: REG10\n\nYou can [`read`](crate::Reg::read) this register and get [`reg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg10`]
module"]
#[doc(alias = "REG10")]
pub type Reg10 = crate::Reg<reg10::Reg10Spec>;
#[doc = "REG10"]
pub mod reg10;
#[doc = "REG11 (rw) register accessor: REG11\n\nYou can [`read`](crate::Reg::read) this register and get [`reg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg11`]
module"]
#[doc(alias = "REG11")]
pub type Reg11 = crate::Reg<reg11::Reg11Spec>;
#[doc = "REG11"]
pub mod reg11;
#[doc = "REG12 (rw) register accessor: REG12\n\nYou can [`read`](crate::Reg::read) this register and get [`reg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg12`]
module"]
#[doc(alias = "REG12")]
pub type Reg12 = crate::Reg<reg12::Reg12Spec>;
#[doc = "REG12"]
pub mod reg12;
#[doc = "REG13 (rw) register accessor: REG13\n\nYou can [`read`](crate::Reg::read) this register and get [`reg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg13`]
module"]
#[doc(alias = "REG13")]
pub type Reg13 = crate::Reg<reg13::Reg13Spec>;
#[doc = "REG13"]
pub mod reg13;
#[doc = "REG14 (rw) register accessor: Sum of GP ADC readings\n\nYou can [`read`](crate::Reg::read) this register and get [`reg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg14`]
module"]
#[doc(alias = "REG14")]
pub type Reg14 = crate::Reg<reg14::Reg14Spec>;
#[doc = "Sum of GP ADC readings"]
pub mod reg14;
#[doc = "REG15 (rw) register accessor: Min and Max of GP ADC readings\n\nYou can [`read`](crate::Reg::read) this register and get [`reg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg15`]
module"]
#[doc(alias = "REG15")]
pub type Reg15 = crate::Reg<reg15::Reg15Spec>;
#[doc = "Min and Max of GP ADC readings"]
pub mod reg15;
#[doc = "REG16 (rw) register accessor: REG16\n\nYou can [`read`](crate::Reg::read) this register and get [`reg16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg16`]
module"]
#[doc(alias = "REG16")]
pub type Reg16 = crate::Reg<reg16::Reg16Spec>;
#[doc = "REG16"]
pub mod reg16;
#[doc = "REG17 (rw) register accessor: REG17\n\nYou can [`read`](crate::Reg::read) this register and get [`reg17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg17`]
module"]
#[doc(alias = "REG17")]
pub type Reg17 = crate::Reg<reg17::Reg17Spec>;
#[doc = "REG17"]
pub mod reg17;
#[doc = "REG18 (rw) register accessor: REG18\n\nYou can [`read`](crate::Reg::read) this register and get [`reg18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg18`]
module"]
#[doc(alias = "REG18")]
pub type Reg18 = crate::Reg<reg18::Reg18Spec>;
#[doc = "REG18"]
pub mod reg18;
#[doc = "REG19 (rw) register accessor: REG19\n\nYou can [`read`](crate::Reg::read) this register and get [`reg19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg19`]
module"]
#[doc(alias = "REG19")]
pub type Reg19 = crate::Reg<reg19::Reg19Spec>;
#[doc = "REG19"]
pub mod reg19;
#[doc = "REG20 (rw) register accessor: REG20\n\nYou can [`read`](crate::Reg::read) this register and get [`reg20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg20`]
module"]
#[doc(alias = "REG20")]
pub type Reg20 = crate::Reg<reg20::Reg20Spec>;
#[doc = "REG20"]
pub mod reg20;
#[doc = "REG21 (rw) register accessor: REG21\n\nYou can [`read`](crate::Reg::read) this register and get [`reg21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg21`]
module"]
#[doc(alias = "REG21")]
pub type Reg21 = crate::Reg<reg21::Reg21Spec>;
#[doc = "REG21"]
pub mod reg21;
#[doc = "REG22 (rw) register accessor: REG22\n\nYou can [`read`](crate::Reg::read) this register and get [`reg22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg22`]
module"]
#[doc(alias = "REG22")]
pub type Reg22 = crate::Reg<reg22::Reg22Spec>;
#[doc = "REG22"]
pub mod reg22;
