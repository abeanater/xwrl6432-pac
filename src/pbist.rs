#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reserved: Reserved,
    _reserved1: [u8; 0x0160],
    pbist_dlr: PbistDlr,
    _reserved2: [u8; 0x06],
    pbist_pc: PbistPc,
    _reserved3: [u8; 0x13],
    pbist_pact: PbistPact,
    _reserved4: [u8; 0x07],
    pbist_ovr: PbistOvr,
    _reserved5: [u8; 0x07],
    pbist_fsfr0: PbistFsfr0,
    pbist_fsfr1: PbistFsfr1,
    _reserved7: [u8; 0x28],
    pbist_rom: PbistRom,
    _reserved8: [u8; 0x03],
    pbist_algo: PbistAlgo,
    pbist_rinfol: PbistRinfol,
    pbist_rinfou: PbistRinfou,
}
impl RegisterBlock {
    #[doc = "0x00 - Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
    #[doc = "0x164 - Datalogger 0"]
    #[inline(always)]
    pub const fn pbist_dlr(&self) -> &PbistDlr {
        &self.pbist_dlr
    }
    #[doc = "0x16c - Program Control"]
    #[inline(always)]
    pub const fn pbist_pc(&self) -> &PbistPc {
        &self.pbist_pc
    }
    #[doc = "0x180 - Pbist Active"]
    #[inline(always)]
    pub const fn pbist_pact(&self) -> &PbistPact {
        &self.pbist_pact
    }
    #[doc = "0x188 - PBIST Overrides"]
    #[inline(always)]
    pub const fn pbist_ovr(&self) -> &PbistOvr {
        &self.pbist_ovr
    }
    #[doc = "0x190 - Fail status fail - port 0"]
    #[inline(always)]
    pub const fn pbist_fsfr0(&self) -> &PbistFsfr0 {
        &self.pbist_fsfr0
    }
    #[doc = "0x194 - Fail status fail - port 1"]
    #[inline(always)]
    pub const fn pbist_fsfr1(&self) -> &PbistFsfr1 {
        &self.pbist_fsfr1
    }
    #[doc = "0x1c0 - Rom Mask"]
    #[inline(always)]
    pub const fn pbist_rom(&self) -> &PbistRom {
        &self.pbist_rom
    }
    #[doc = "0x1c4 - ROM Algorithm Mask 0"]
    #[inline(always)]
    pub const fn pbist_algo(&self) -> &PbistAlgo {
        &self.pbist_algo
    }
    #[doc = "0x1c8 - RAM Info Mask Lower 0"]
    #[inline(always)]
    pub const fn pbist_rinfol(&self) -> &PbistRinfol {
        &self.pbist_rinfol
    }
    #[doc = "0x1cc - RAM Info Mask Upper 0"]
    #[inline(always)]
    pub const fn pbist_rinfou(&self) -> &PbistRinfou {
        &self.pbist_rinfou
    }
}
#[doc = "RESERVED (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`]
module"]
#[doc(alias = "RESERVED")]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = "Reserved"]
pub mod reserved;
#[doc = "PBIST_DLR (rw) register accessor: Datalogger 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_dlr`]
module"]
#[doc(alias = "PBIST_DLR")]
pub type PbistDlr = crate::Reg<pbist_dlr::PbistDlrSpec>;
#[doc = "Datalogger 0"]
pub mod pbist_dlr;
#[doc = "PBIST_PC (rw) register accessor: Program Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_pc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_pc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_pc`]
module"]
#[doc(alias = "PBIST_PC")]
pub type PbistPc = crate::Reg<pbist_pc::PbistPcSpec>;
#[doc = "Program Control"]
pub mod pbist_pc;
#[doc = "PBIST_PACT (rw) register accessor: Pbist Active\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_pact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_pact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_pact`]
module"]
#[doc(alias = "PBIST_PACT")]
pub type PbistPact = crate::Reg<pbist_pact::PbistPactSpec>;
#[doc = "Pbist Active"]
pub mod pbist_pact;
#[doc = "PBIST_OVR (rw) register accessor: PBIST Overrides\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_ovr`]
module"]
#[doc(alias = "PBIST_OVR")]
pub type PbistOvr = crate::Reg<pbist_ovr::PbistOvrSpec>;
#[doc = "PBIST Overrides"]
pub mod pbist_ovr;
#[doc = "PBIST_FSFR0 (rw) register accessor: Fail status fail - port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsfr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsfr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsfr0`]
module"]
#[doc(alias = "PBIST_FSFR0")]
pub type PbistFsfr0 = crate::Reg<pbist_fsfr0::PbistFsfr0Spec>;
#[doc = "Fail status fail - port 0"]
pub mod pbist_fsfr0;
#[doc = "PBIST_FSFR1 (rw) register accessor: Fail status fail - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsfr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsfr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_fsfr1`]
module"]
#[doc(alias = "PBIST_FSFR1")]
pub type PbistFsfr1 = crate::Reg<pbist_fsfr1::PbistFsfr1Spec>;
#[doc = "Fail status fail - port 1"]
pub mod pbist_fsfr1;
#[doc = "PBIST_ROM (rw) register accessor: Rom Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_rom`]
module"]
#[doc(alias = "PBIST_ROM")]
pub type PbistRom = crate::Reg<pbist_rom::PbistRomSpec>;
#[doc = "Rom Mask"]
pub mod pbist_rom;
#[doc = "PBIST_ALGO (rw) register accessor: ROM Algorithm Mask 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_algo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_algo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_algo`]
module"]
#[doc(alias = "PBIST_ALGO")]
pub type PbistAlgo = crate::Reg<pbist_algo::PbistAlgoSpec>;
#[doc = "ROM Algorithm Mask 0"]
pub mod pbist_algo;
#[doc = "PBIST_RINFOL (rw) register accessor: RAM Info Mask Lower 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rinfol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rinfol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_rinfol`]
module"]
#[doc(alias = "PBIST_RINFOL")]
pub type PbistRinfol = crate::Reg<pbist_rinfol::PbistRinfolSpec>;
#[doc = "RAM Info Mask Lower 0"]
pub mod pbist_rinfol;
#[doc = "PBIST_RINFOU (rw) register accessor: RAM Info Mask Upper 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rinfou::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rinfou::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbist_rinfou`]
module"]
#[doc(alias = "PBIST_RINFOU")]
pub type PbistRinfou = crate::Reg<pbist_rinfou::PbistRinfouSpec>;
#[doc = "RAM Info Mask Upper 0"]
pub mod pbist_rinfou;
