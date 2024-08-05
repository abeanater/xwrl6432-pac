#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    padaa_cfg_reg: PadaaCfgReg,
    padab_cfg_reg: PadabCfgReg,
    padac_cfg_reg: PadacCfgReg,
    padad_cfg_reg: PadadCfgReg,
    padae_cfg_reg: PadaeCfgReg,
    padaf_cfg_reg: PadafCfgReg,
    padag_cfg_reg: PadagCfgReg,
    padah_cfg_reg: PadahCfgReg,
    padai_cfg_reg: PadaiCfgReg,
    padaj_cfg_reg: PadajCfgReg,
    padak_cfg_reg: PadakCfgReg,
    padal_cfg_reg: PadalCfgReg,
    padam_cfg_reg: PadamCfgReg,
    padan_cfg_reg: PadanCfgReg,
    padao_cfg_reg: PadaoCfgReg,
    padap_cfg_reg: PadapCfgReg,
    padaq_cfg_reg: PadaqCfgReg,
    padar_cfg_reg: PadarCfgReg,
    padas_cfg_reg: PadasCfgReg,
    padat_cfg_reg: PadatCfgReg,
    padau_cfg_reg: PadauCfgReg,
    padav_cfg_reg: PadavCfgReg,
    padaw_cfg_reg: PadawCfgReg,
    padax_cfg_reg: PadaxCfgReg,
    usermodeen: Usermodeen,
    padglblcfgreg: Padglblcfgreg,
    iocfgkick0: Iocfgkick0,
    iocfgkick1: Iocfgkick1,
}
impl RegisterBlock {
    #[doc = "0x00 - PADAA_cfg_reg"]
    #[inline(always)]
    pub const fn padaa_cfg_reg(&self) -> &PadaaCfgReg {
        &self.padaa_cfg_reg
    }
    #[doc = "0x04 - PADAB_cfg_reg"]
    #[inline(always)]
    pub const fn padab_cfg_reg(&self) -> &PadabCfgReg {
        &self.padab_cfg_reg
    }
    #[doc = "0x08 - PADAC_cfg_reg"]
    #[inline(always)]
    pub const fn padac_cfg_reg(&self) -> &PadacCfgReg {
        &self.padac_cfg_reg
    }
    #[doc = "0x0c - PADAD_cfg_reg"]
    #[inline(always)]
    pub const fn padad_cfg_reg(&self) -> &PadadCfgReg {
        &self.padad_cfg_reg
    }
    #[doc = "0x10 - PADAE_cfg_reg"]
    #[inline(always)]
    pub const fn padae_cfg_reg(&self) -> &PadaeCfgReg {
        &self.padae_cfg_reg
    }
    #[doc = "0x14 - PADAF_cfg_reg"]
    #[inline(always)]
    pub const fn padaf_cfg_reg(&self) -> &PadafCfgReg {
        &self.padaf_cfg_reg
    }
    #[doc = "0x18 - PADAG_cfg_reg"]
    #[inline(always)]
    pub const fn padag_cfg_reg(&self) -> &PadagCfgReg {
        &self.padag_cfg_reg
    }
    #[doc = "0x1c - PADAH_cfg_reg"]
    #[inline(always)]
    pub const fn padah_cfg_reg(&self) -> &PadahCfgReg {
        &self.padah_cfg_reg
    }
    #[doc = "0x20 - PADAI_cfg_reg"]
    #[inline(always)]
    pub const fn padai_cfg_reg(&self) -> &PadaiCfgReg {
        &self.padai_cfg_reg
    }
    #[doc = "0x24 - PADAJ_cfg_reg"]
    #[inline(always)]
    pub const fn padaj_cfg_reg(&self) -> &PadajCfgReg {
        &self.padaj_cfg_reg
    }
    #[doc = "0x28 - PADAK_cfg_reg"]
    #[inline(always)]
    pub const fn padak_cfg_reg(&self) -> &PadakCfgReg {
        &self.padak_cfg_reg
    }
    #[doc = "0x2c - PADAL_cfg_reg"]
    #[inline(always)]
    pub const fn padal_cfg_reg(&self) -> &PadalCfgReg {
        &self.padal_cfg_reg
    }
    #[doc = "0x30 - PADAM_cfg_reg"]
    #[inline(always)]
    pub const fn padam_cfg_reg(&self) -> &PadamCfgReg {
        &self.padam_cfg_reg
    }
    #[doc = "0x34 - PADAN_cfg_reg"]
    #[inline(always)]
    pub const fn padan_cfg_reg(&self) -> &PadanCfgReg {
        &self.padan_cfg_reg
    }
    #[doc = "0x38 - PADAO_cfg_reg"]
    #[inline(always)]
    pub const fn padao_cfg_reg(&self) -> &PadaoCfgReg {
        &self.padao_cfg_reg
    }
    #[doc = "0x3c - PADAP_cfg_reg"]
    #[inline(always)]
    pub const fn padap_cfg_reg(&self) -> &PadapCfgReg {
        &self.padap_cfg_reg
    }
    #[doc = "0x40 - PADAQ_cfg_reg"]
    #[inline(always)]
    pub const fn padaq_cfg_reg(&self) -> &PadaqCfgReg {
        &self.padaq_cfg_reg
    }
    #[doc = "0x44 - PADAR_cfg_reg"]
    #[inline(always)]
    pub const fn padar_cfg_reg(&self) -> &PadarCfgReg {
        &self.padar_cfg_reg
    }
    #[doc = "0x48 - PADAS_cfg_reg"]
    #[inline(always)]
    pub const fn padas_cfg_reg(&self) -> &PadasCfgReg {
        &self.padas_cfg_reg
    }
    #[doc = "0x4c - PADAT_cfg_reg"]
    #[inline(always)]
    pub const fn padat_cfg_reg(&self) -> &PadatCfgReg {
        &self.padat_cfg_reg
    }
    #[doc = "0x50 - PADAU_cfg_reg"]
    #[inline(always)]
    pub const fn padau_cfg_reg(&self) -> &PadauCfgReg {
        &self.padau_cfg_reg
    }
    #[doc = "0x54 - PADAV_cfg_reg"]
    #[inline(always)]
    pub const fn padav_cfg_reg(&self) -> &PadavCfgReg {
        &self.padav_cfg_reg
    }
    #[doc = "0x58 - PADAW_cfg_reg"]
    #[inline(always)]
    pub const fn padaw_cfg_reg(&self) -> &PadawCfgReg {
        &self.padaw_cfg_reg
    }
    #[doc = "0x5c - PADAX_cfg_reg"]
    #[inline(always)]
    pub const fn padax_cfg_reg(&self) -> &PadaxCfgReg {
        &self.padax_cfg_reg
    }
    #[doc = "0x60 - USERMODEEN"]
    #[inline(always)]
    pub const fn usermodeen(&self) -> &Usermodeen {
        &self.usermodeen
    }
    #[doc = "0x64 - PADGLBLCFGREG"]
    #[inline(always)]
    pub const fn padglblcfgreg(&self) -> &Padglblcfgreg {
        &self.padglblcfgreg
    }
    #[doc = "0x68 - IOCFGKICK0"]
    #[inline(always)]
    pub const fn iocfgkick0(&self) -> &Iocfgkick0 {
        &self.iocfgkick0
    }
    #[doc = "0x6c - IOCFGKICK1"]
    #[inline(always)]
    pub const fn iocfgkick1(&self) -> &Iocfgkick1 {
        &self.iocfgkick1
    }
}
#[doc = "PADAA_cfg_reg (rw) register accessor: PADAA_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padaa_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padaa_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padaa_cfg_reg`]
module"]
#[doc(alias = "PADAA_cfg_reg")]
pub type PadaaCfgReg = crate::Reg<padaa_cfg_reg::PadaaCfgRegSpec>;
#[doc = "PADAA_cfg_reg"]
pub mod padaa_cfg_reg;
#[doc = "PADAB_cfg_reg (rw) register accessor: PADAB_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padab_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padab_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padab_cfg_reg`]
module"]
#[doc(alias = "PADAB_cfg_reg")]
pub type PadabCfgReg = crate::Reg<padab_cfg_reg::PadabCfgRegSpec>;
#[doc = "PADAB_cfg_reg"]
pub mod padab_cfg_reg;
#[doc = "PADAC_cfg_reg (rw) register accessor: PADAC_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padac_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padac_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padac_cfg_reg`]
module"]
#[doc(alias = "PADAC_cfg_reg")]
pub type PadacCfgReg = crate::Reg<padac_cfg_reg::PadacCfgRegSpec>;
#[doc = "PADAC_cfg_reg"]
pub mod padac_cfg_reg;
#[doc = "PADAD_cfg_reg (rw) register accessor: PADAD_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padad_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padad_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padad_cfg_reg`]
module"]
#[doc(alias = "PADAD_cfg_reg")]
pub type PadadCfgReg = crate::Reg<padad_cfg_reg::PadadCfgRegSpec>;
#[doc = "PADAD_cfg_reg"]
pub mod padad_cfg_reg;
#[doc = "PADAE_cfg_reg (rw) register accessor: PADAE_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padae_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padae_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padae_cfg_reg`]
module"]
#[doc(alias = "PADAE_cfg_reg")]
pub type PadaeCfgReg = crate::Reg<padae_cfg_reg::PadaeCfgRegSpec>;
#[doc = "PADAE_cfg_reg"]
pub mod padae_cfg_reg;
#[doc = "PADAF_cfg_reg (rw) register accessor: PADAF_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padaf_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padaf_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padaf_cfg_reg`]
module"]
#[doc(alias = "PADAF_cfg_reg")]
pub type PadafCfgReg = crate::Reg<padaf_cfg_reg::PadafCfgRegSpec>;
#[doc = "PADAF_cfg_reg"]
pub mod padaf_cfg_reg;
#[doc = "PADAG_cfg_reg (rw) register accessor: PADAG_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padag_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padag_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padag_cfg_reg`]
module"]
#[doc(alias = "PADAG_cfg_reg")]
pub type PadagCfgReg = crate::Reg<padag_cfg_reg::PadagCfgRegSpec>;
#[doc = "PADAG_cfg_reg"]
pub mod padag_cfg_reg;
#[doc = "PADAH_cfg_reg (rw) register accessor: PADAH_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padah_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padah_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padah_cfg_reg`]
module"]
#[doc(alias = "PADAH_cfg_reg")]
pub type PadahCfgReg = crate::Reg<padah_cfg_reg::PadahCfgRegSpec>;
#[doc = "PADAH_cfg_reg"]
pub mod padah_cfg_reg;
#[doc = "PADAI_cfg_reg (rw) register accessor: PADAI_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padai_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padai_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padai_cfg_reg`]
module"]
#[doc(alias = "PADAI_cfg_reg")]
pub type PadaiCfgReg = crate::Reg<padai_cfg_reg::PadaiCfgRegSpec>;
#[doc = "PADAI_cfg_reg"]
pub mod padai_cfg_reg;
#[doc = "PADAJ_cfg_reg (rw) register accessor: PADAJ_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padaj_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padaj_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padaj_cfg_reg`]
module"]
#[doc(alias = "PADAJ_cfg_reg")]
pub type PadajCfgReg = crate::Reg<padaj_cfg_reg::PadajCfgRegSpec>;
#[doc = "PADAJ_cfg_reg"]
pub mod padaj_cfg_reg;
#[doc = "PADAK_cfg_reg (rw) register accessor: PADAK_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padak_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padak_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padak_cfg_reg`]
module"]
#[doc(alias = "PADAK_cfg_reg")]
pub type PadakCfgReg = crate::Reg<padak_cfg_reg::PadakCfgRegSpec>;
#[doc = "PADAK_cfg_reg"]
pub mod padak_cfg_reg;
#[doc = "PADAL_cfg_reg (rw) register accessor: PADAL_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padal_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padal_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padal_cfg_reg`]
module"]
#[doc(alias = "PADAL_cfg_reg")]
pub type PadalCfgReg = crate::Reg<padal_cfg_reg::PadalCfgRegSpec>;
#[doc = "PADAL_cfg_reg"]
pub mod padal_cfg_reg;
#[doc = "PADAM_cfg_reg (rw) register accessor: PADAM_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padam_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padam_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padam_cfg_reg`]
module"]
#[doc(alias = "PADAM_cfg_reg")]
pub type PadamCfgReg = crate::Reg<padam_cfg_reg::PadamCfgRegSpec>;
#[doc = "PADAM_cfg_reg"]
pub mod padam_cfg_reg;
#[doc = "PADAN_cfg_reg (rw) register accessor: PADAN_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padan_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padan_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padan_cfg_reg`]
module"]
#[doc(alias = "PADAN_cfg_reg")]
pub type PadanCfgReg = crate::Reg<padan_cfg_reg::PadanCfgRegSpec>;
#[doc = "PADAN_cfg_reg"]
pub mod padan_cfg_reg;
#[doc = "PADAO_cfg_reg (rw) register accessor: PADAO_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padao_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padao_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padao_cfg_reg`]
module"]
#[doc(alias = "PADAO_cfg_reg")]
pub type PadaoCfgReg = crate::Reg<padao_cfg_reg::PadaoCfgRegSpec>;
#[doc = "PADAO_cfg_reg"]
pub mod padao_cfg_reg;
#[doc = "PADAP_cfg_reg (rw) register accessor: PADAP_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padap_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padap_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padap_cfg_reg`]
module"]
#[doc(alias = "PADAP_cfg_reg")]
pub type PadapCfgReg = crate::Reg<padap_cfg_reg::PadapCfgRegSpec>;
#[doc = "PADAP_cfg_reg"]
pub mod padap_cfg_reg;
#[doc = "PADAQ_cfg_reg (rw) register accessor: PADAQ_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padaq_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padaq_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padaq_cfg_reg`]
module"]
#[doc(alias = "PADAQ_cfg_reg")]
pub type PadaqCfgReg = crate::Reg<padaq_cfg_reg::PadaqCfgRegSpec>;
#[doc = "PADAQ_cfg_reg"]
pub mod padaq_cfg_reg;
#[doc = "PADAR_cfg_reg (rw) register accessor: PADAR_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padar_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padar_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padar_cfg_reg`]
module"]
#[doc(alias = "PADAR_cfg_reg")]
pub type PadarCfgReg = crate::Reg<padar_cfg_reg::PadarCfgRegSpec>;
#[doc = "PADAR_cfg_reg"]
pub mod padar_cfg_reg;
#[doc = "PADAS_cfg_reg (rw) register accessor: PADAS_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padas_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padas_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padas_cfg_reg`]
module"]
#[doc(alias = "PADAS_cfg_reg")]
pub type PadasCfgReg = crate::Reg<padas_cfg_reg::PadasCfgRegSpec>;
#[doc = "PADAS_cfg_reg"]
pub mod padas_cfg_reg;
#[doc = "PADAT_cfg_reg (rw) register accessor: PADAT_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padat_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padat_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padat_cfg_reg`]
module"]
#[doc(alias = "PADAT_cfg_reg")]
pub type PadatCfgReg = crate::Reg<padat_cfg_reg::PadatCfgRegSpec>;
#[doc = "PADAT_cfg_reg"]
pub mod padat_cfg_reg;
#[doc = "PADAU_cfg_reg (rw) register accessor: PADAU_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padau_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padau_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padau_cfg_reg`]
module"]
#[doc(alias = "PADAU_cfg_reg")]
pub type PadauCfgReg = crate::Reg<padau_cfg_reg::PadauCfgRegSpec>;
#[doc = "PADAU_cfg_reg"]
pub mod padau_cfg_reg;
#[doc = "PADAV_cfg_reg (rw) register accessor: PADAV_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padav_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padav_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padav_cfg_reg`]
module"]
#[doc(alias = "PADAV_cfg_reg")]
pub type PadavCfgReg = crate::Reg<padav_cfg_reg::PadavCfgRegSpec>;
#[doc = "PADAV_cfg_reg"]
pub mod padav_cfg_reg;
#[doc = "PADAW_cfg_reg (rw) register accessor: PADAW_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padaw_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padaw_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padaw_cfg_reg`]
module"]
#[doc(alias = "PADAW_cfg_reg")]
pub type PadawCfgReg = crate::Reg<padaw_cfg_reg::PadawCfgRegSpec>;
#[doc = "PADAW_cfg_reg"]
pub mod padaw_cfg_reg;
#[doc = "PADAX_cfg_reg (rw) register accessor: PADAX_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padax_cfg_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padax_cfg_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padax_cfg_reg`]
module"]
#[doc(alias = "PADAX_cfg_reg")]
pub type PadaxCfgReg = crate::Reg<padax_cfg_reg::PadaxCfgRegSpec>;
#[doc = "PADAX_cfg_reg"]
pub mod padax_cfg_reg;
#[doc = "USERMODEEN (rw) register accessor: USERMODEEN\n\nYou can [`read`](crate::Reg::read) this register and get [`usermodeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usermodeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usermodeen`]
module"]
#[doc(alias = "USERMODEEN")]
pub type Usermodeen = crate::Reg<usermodeen::UsermodeenSpec>;
#[doc = "USERMODEEN"]
pub mod usermodeen;
#[doc = "PADGLBLCFGREG (rw) register accessor: PADGLBLCFGREG\n\nYou can [`read`](crate::Reg::read) this register and get [`padglblcfgreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padglblcfgreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padglblcfgreg`]
module"]
#[doc(alias = "PADGLBLCFGREG")]
pub type Padglblcfgreg = crate::Reg<padglblcfgreg::PadglblcfgregSpec>;
#[doc = "PADGLBLCFGREG"]
pub mod padglblcfgreg;
#[doc = "IOCFGKICK0 (rw) register accessor: IOCFGKICK0\n\nYou can [`read`](crate::Reg::read) this register and get [`iocfgkick0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocfgkick0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfgkick0`]
module"]
#[doc(alias = "IOCFGKICK0")]
pub type Iocfgkick0 = crate::Reg<iocfgkick0::Iocfgkick0Spec>;
#[doc = "IOCFGKICK0"]
pub mod iocfgkick0;
#[doc = "IOCFGKICK1 (rw) register accessor: IOCFGKICK1\n\nYou can [`read`](crate::Reg::read) this register and get [`iocfgkick1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocfgkick1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfgkick1`]
module"]
#[doc(alias = "IOCFGKICK1")]
pub type Iocfgkick1 = crate::Reg<iocfgkick1::Iocfgkick1Spec>;
#[doc = "IOCFGKICK1"]
pub mod iocfgkick1;
