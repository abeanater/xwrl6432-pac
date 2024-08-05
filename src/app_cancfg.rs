#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ss_pid: SsPid,
    ss_ctrl: SsCtrl,
    ss_stat: SsStat,
    ss_ics: SsIcs,
    ss_irs: SsIrs,
    ss_iecs: SsIecs,
    ss_ie: SsIe,
    ss_ies: SsIes,
    ss_eoi: SsEoi,
    ss_ext_ts_ps: SsExtTsPs,
    ss_ext_ts_usic: SsExtTsUsic,
    _reserved11: [u8; 0x01d4],
    crel: Crel,
    endn: Endn,
    cust: Cust,
    dbtp: Dbtp,
    test: Test,
    rwd: Rwd,
    cccr: Cccr,
    nbtp: Nbtp,
    tscc: Tscc,
    tscv: Tscv,
    tocc: Tocc,
    tocv: Tocv,
    res00: Res00,
    res01: Res01,
    res02: Res02,
    res03: Res03,
    ecr: Ecr,
    psr: Psr,
    tdcr: Tdcr,
    res04: Res04,
    ir: Ir,
    ie: Ie,
    ils: Ils,
    ile: Ile,
    res05: Res05,
    res06: Res06,
    res07: Res07,
    res08: Res08,
    res09: Res09,
    res10: Res10,
    res11: Res11,
    res12: Res12,
    gfc: Gfc,
    sidfc: Sidfc,
    xidfc: Xidfc,
    res13: Res13,
    xidam: Xidam,
    hpms: Hpms,
    ndat1: Ndat1,
    ndat2: Ndat2,
    rxf0c: Rxf0c,
    rxf0s: Rxf0s,
    rxf0a: Rxf0a,
    rxbc: Rxbc,
    rxf1c: Rxf1c,
    rxf1s: Rxf1s,
    rxf1a: Rxf1a,
    rxesc: Rxesc,
    txbc: Txbc,
    txfqs: Txfqs,
    txesc: Txesc,
    txbrp: Txbrp,
    txbar: Txbar,
    txbcr: Txbcr,
    txbto: Txbto,
    txbcf: Txbcf,
    txbtie: Txbtie,
    txbcie: Txbcie,
    res14: Res14,
    res15: Res15,
    txefc: Txefc,
    txefs: Txefs,
    txefa: Txefa,
    res16: Res16,
}
impl RegisterBlock {
    #[doc = "0x00 - SS_PID"]
    #[inline(always)]
    pub const fn ss_pid(&self) -> &SsPid {
        &self.ss_pid
    }
    #[doc = "0x04 - SS_CTRL"]
    #[inline(always)]
    pub const fn ss_ctrl(&self) -> &SsCtrl {
        &self.ss_ctrl
    }
    #[doc = "0x08 - SS_STAT"]
    #[inline(always)]
    pub const fn ss_stat(&self) -> &SsStat {
        &self.ss_stat
    }
    #[doc = "0x0c - SS_ICS"]
    #[inline(always)]
    pub const fn ss_ics(&self) -> &SsIcs {
        &self.ss_ics
    }
    #[doc = "0x10 - SS_IRS"]
    #[inline(always)]
    pub const fn ss_irs(&self) -> &SsIrs {
        &self.ss_irs
    }
    #[doc = "0x14 - SS_IECS"]
    #[inline(always)]
    pub const fn ss_iecs(&self) -> &SsIecs {
        &self.ss_iecs
    }
    #[doc = "0x18 - SS_IE"]
    #[inline(always)]
    pub const fn ss_ie(&self) -> &SsIe {
        &self.ss_ie
    }
    #[doc = "0x1c - SS_IES"]
    #[inline(always)]
    pub const fn ss_ies(&self) -> &SsIes {
        &self.ss_ies
    }
    #[doc = "0x20 - SS_EOI"]
    #[inline(always)]
    pub const fn ss_eoi(&self) -> &SsEoi {
        &self.ss_eoi
    }
    #[doc = "0x24 - SS_EXT_TS_PS"]
    #[inline(always)]
    pub const fn ss_ext_ts_ps(&self) -> &SsExtTsPs {
        &self.ss_ext_ts_ps
    }
    #[doc = "0x28 - SS_EXT_TS_USIC"]
    #[inline(always)]
    pub const fn ss_ext_ts_usic(&self) -> &SsExtTsUsic {
        &self.ss_ext_ts_usic
    }
    #[doc = "0x200 - CREL"]
    #[inline(always)]
    pub const fn crel(&self) -> &Crel {
        &self.crel
    }
    #[doc = "0x204 - ENDN"]
    #[inline(always)]
    pub const fn endn(&self) -> &Endn {
        &self.endn
    }
    #[doc = "0x208 - CUST"]
    #[inline(always)]
    pub const fn cust(&self) -> &Cust {
        &self.cust
    }
    #[doc = "0x20c - DBTP"]
    #[inline(always)]
    pub const fn dbtp(&self) -> &Dbtp {
        &self.dbtp
    }
    #[doc = "0x210 - TEST"]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x214 - RWD"]
    #[inline(always)]
    pub const fn rwd(&self) -> &Rwd {
        &self.rwd
    }
    #[doc = "0x218 - CCCR"]
    #[inline(always)]
    pub const fn cccr(&self) -> &Cccr {
        &self.cccr
    }
    #[doc = "0x21c - NBTP"]
    #[inline(always)]
    pub const fn nbtp(&self) -> &Nbtp {
        &self.nbtp
    }
    #[doc = "0x220 - TSCC"]
    #[inline(always)]
    pub const fn tscc(&self) -> &Tscc {
        &self.tscc
    }
    #[doc = "0x224 - TSCV"]
    #[inline(always)]
    pub const fn tscv(&self) -> &Tscv {
        &self.tscv
    }
    #[doc = "0x228 - TOCC"]
    #[inline(always)]
    pub const fn tocc(&self) -> &Tocc {
        &self.tocc
    }
    #[doc = "0x22c - TOCV"]
    #[inline(always)]
    pub const fn tocv(&self) -> &Tocv {
        &self.tocv
    }
    #[doc = "0x230 - RES00"]
    #[inline(always)]
    pub const fn res00(&self) -> &Res00 {
        &self.res00
    }
    #[doc = "0x234 - RES01"]
    #[inline(always)]
    pub const fn res01(&self) -> &Res01 {
        &self.res01
    }
    #[doc = "0x238 - RES02"]
    #[inline(always)]
    pub const fn res02(&self) -> &Res02 {
        &self.res02
    }
    #[doc = "0x23c - RES03"]
    #[inline(always)]
    pub const fn res03(&self) -> &Res03 {
        &self.res03
    }
    #[doc = "0x240 - ECR"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x244 - PSR"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x248 - TDCR"]
    #[inline(always)]
    pub const fn tdcr(&self) -> &Tdcr {
        &self.tdcr
    }
    #[doc = "0x24c - RES04"]
    #[inline(always)]
    pub const fn res04(&self) -> &Res04 {
        &self.res04
    }
    #[doc = "0x250 - IR"]
    #[inline(always)]
    pub const fn ir(&self) -> &Ir {
        &self.ir
    }
    #[doc = "0x254 - IE"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x258 - ILS"]
    #[inline(always)]
    pub const fn ils(&self) -> &Ils {
        &self.ils
    }
    #[doc = "0x25c - ILE"]
    #[inline(always)]
    pub const fn ile(&self) -> &Ile {
        &self.ile
    }
    #[doc = "0x260 - RES05"]
    #[inline(always)]
    pub const fn res05(&self) -> &Res05 {
        &self.res05
    }
    #[doc = "0x264 - RES06"]
    #[inline(always)]
    pub const fn res06(&self) -> &Res06 {
        &self.res06
    }
    #[doc = "0x268 - RES07"]
    #[inline(always)]
    pub const fn res07(&self) -> &Res07 {
        &self.res07
    }
    #[doc = "0x26c - RES08"]
    #[inline(always)]
    pub const fn res08(&self) -> &Res08 {
        &self.res08
    }
    #[doc = "0x270 - RES09"]
    #[inline(always)]
    pub const fn res09(&self) -> &Res09 {
        &self.res09
    }
    #[doc = "0x274 - RES10"]
    #[inline(always)]
    pub const fn res10(&self) -> &Res10 {
        &self.res10
    }
    #[doc = "0x278 - RES11"]
    #[inline(always)]
    pub const fn res11(&self) -> &Res11 {
        &self.res11
    }
    #[doc = "0x27c - RES12"]
    #[inline(always)]
    pub const fn res12(&self) -> &Res12 {
        &self.res12
    }
    #[doc = "0x280 - GFC"]
    #[inline(always)]
    pub const fn gfc(&self) -> &Gfc {
        &self.gfc
    }
    #[doc = "0x284 - SIDFC"]
    #[inline(always)]
    pub const fn sidfc(&self) -> &Sidfc {
        &self.sidfc
    }
    #[doc = "0x288 - XIDFC"]
    #[inline(always)]
    pub const fn xidfc(&self) -> &Xidfc {
        &self.xidfc
    }
    #[doc = "0x28c - RES13"]
    #[inline(always)]
    pub const fn res13(&self) -> &Res13 {
        &self.res13
    }
    #[doc = "0x290 - XIDAM"]
    #[inline(always)]
    pub const fn xidam(&self) -> &Xidam {
        &self.xidam
    }
    #[doc = "0x294 - HPMS"]
    #[inline(always)]
    pub const fn hpms(&self) -> &Hpms {
        &self.hpms
    }
    #[doc = "0x298 - NDAT1"]
    #[inline(always)]
    pub const fn ndat1(&self) -> &Ndat1 {
        &self.ndat1
    }
    #[doc = "0x29c - NDAT2"]
    #[inline(always)]
    pub const fn ndat2(&self) -> &Ndat2 {
        &self.ndat2
    }
    #[doc = "0x2a0 - RXF0C"]
    #[inline(always)]
    pub const fn rxf0c(&self) -> &Rxf0c {
        &self.rxf0c
    }
    #[doc = "0x2a4 - RXF0S"]
    #[inline(always)]
    pub const fn rxf0s(&self) -> &Rxf0s {
        &self.rxf0s
    }
    #[doc = "0x2a8 - RXF0A"]
    #[inline(always)]
    pub const fn rxf0a(&self) -> &Rxf0a {
        &self.rxf0a
    }
    #[doc = "0x2ac - RXBC"]
    #[inline(always)]
    pub const fn rxbc(&self) -> &Rxbc {
        &self.rxbc
    }
    #[doc = "0x2b0 - RXF1C"]
    #[inline(always)]
    pub const fn rxf1c(&self) -> &Rxf1c {
        &self.rxf1c
    }
    #[doc = "0x2b4 - RXF1S"]
    #[inline(always)]
    pub const fn rxf1s(&self) -> &Rxf1s {
        &self.rxf1s
    }
    #[doc = "0x2b8 - RXF1A"]
    #[inline(always)]
    pub const fn rxf1a(&self) -> &Rxf1a {
        &self.rxf1a
    }
    #[doc = "0x2bc - RXESC"]
    #[inline(always)]
    pub const fn rxesc(&self) -> &Rxesc {
        &self.rxesc
    }
    #[doc = "0x2c0 - TXBC"]
    #[inline(always)]
    pub const fn txbc(&self) -> &Txbc {
        &self.txbc
    }
    #[doc = "0x2c4 - TXFQS"]
    #[inline(always)]
    pub const fn txfqs(&self) -> &Txfqs {
        &self.txfqs
    }
    #[doc = "0x2c8 - TXESC"]
    #[inline(always)]
    pub const fn txesc(&self) -> &Txesc {
        &self.txesc
    }
    #[doc = "0x2cc - TXBRP"]
    #[inline(always)]
    pub const fn txbrp(&self) -> &Txbrp {
        &self.txbrp
    }
    #[doc = "0x2d0 - TXBAR"]
    #[inline(always)]
    pub const fn txbar(&self) -> &Txbar {
        &self.txbar
    }
    #[doc = "0x2d4 - TXBCR"]
    #[inline(always)]
    pub const fn txbcr(&self) -> &Txbcr {
        &self.txbcr
    }
    #[doc = "0x2d8 - TXBTO"]
    #[inline(always)]
    pub const fn txbto(&self) -> &Txbto {
        &self.txbto
    }
    #[doc = "0x2dc - TXBCF"]
    #[inline(always)]
    pub const fn txbcf(&self) -> &Txbcf {
        &self.txbcf
    }
    #[doc = "0x2e0 - TXBTIE"]
    #[inline(always)]
    pub const fn txbtie(&self) -> &Txbtie {
        &self.txbtie
    }
    #[doc = "0x2e4 - TXBCIE"]
    #[inline(always)]
    pub const fn txbcie(&self) -> &Txbcie {
        &self.txbcie
    }
    #[doc = "0x2e8 - RES14"]
    #[inline(always)]
    pub const fn res14(&self) -> &Res14 {
        &self.res14
    }
    #[doc = "0x2ec - RES15"]
    #[inline(always)]
    pub const fn res15(&self) -> &Res15 {
        &self.res15
    }
    #[doc = "0x2f0 - TXEFC"]
    #[inline(always)]
    pub const fn txefc(&self) -> &Txefc {
        &self.txefc
    }
    #[doc = "0x2f4 - TXEFS"]
    #[inline(always)]
    pub const fn txefs(&self) -> &Txefs {
        &self.txefs
    }
    #[doc = "0x2f8 - TXEFA"]
    #[inline(always)]
    pub const fn txefa(&self) -> &Txefa {
        &self.txefa
    }
    #[doc = "0x2fc - RES16"]
    #[inline(always)]
    pub const fn res16(&self) -> &Res16 {
        &self.res16
    }
}
#[doc = "SS_PID (rw) register accessor: SS_PID\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_pid`]
module"]
#[doc(alias = "SS_PID")]
pub type SsPid = crate::Reg<ss_pid::SsPidSpec>;
#[doc = "SS_PID"]
pub mod ss_pid;
#[doc = "SS_CTRL (rw) register accessor: SS_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_ctrl`]
module"]
#[doc(alias = "SS_CTRL")]
pub type SsCtrl = crate::Reg<ss_ctrl::SsCtrlSpec>;
#[doc = "SS_CTRL"]
pub mod ss_ctrl;
#[doc = "SS_STAT (rw) register accessor: SS_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_stat`]
module"]
#[doc(alias = "SS_STAT")]
pub type SsStat = crate::Reg<ss_stat::SsStatSpec>;
#[doc = "SS_STAT"]
pub mod ss_stat;
#[doc = "SS_ICS (rw) register accessor: SS_ICS\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ics::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ics::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_ics`]
module"]
#[doc(alias = "SS_ICS")]
pub type SsIcs = crate::Reg<ss_ics::SsIcsSpec>;
#[doc = "SS_ICS"]
pub mod ss_ics;
#[doc = "SS_IRS (rw) register accessor: SS_IRS\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_irs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_irs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_irs`]
module"]
#[doc(alias = "SS_IRS")]
pub type SsIrs = crate::Reg<ss_irs::SsIrsSpec>;
#[doc = "SS_IRS"]
pub mod ss_irs;
#[doc = "SS_IECS (rw) register accessor: SS_IECS\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_iecs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_iecs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_iecs`]
module"]
#[doc(alias = "SS_IECS")]
pub type SsIecs = crate::Reg<ss_iecs::SsIecsSpec>;
#[doc = "SS_IECS"]
pub mod ss_iecs;
#[doc = "SS_IE (rw) register accessor: SS_IE\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_ie`]
module"]
#[doc(alias = "SS_IE")]
pub type SsIe = crate::Reg<ss_ie::SsIeSpec>;
#[doc = "SS_IE"]
pub mod ss_ie;
#[doc = "SS_IES (rw) register accessor: SS_IES\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_ies`]
module"]
#[doc(alias = "SS_IES")]
pub type SsIes = crate::Reg<ss_ies::SsIesSpec>;
#[doc = "SS_IES"]
pub mod ss_ies;
#[doc = "SS_EOI (rw) register accessor: SS_EOI\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_eoi`]
module"]
#[doc(alias = "SS_EOI")]
pub type SsEoi = crate::Reg<ss_eoi::SsEoiSpec>;
#[doc = "SS_EOI"]
pub mod ss_eoi;
#[doc = "SS_EXT_TS_PS (rw) register accessor: SS_EXT_TS_PS\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ext_ts_ps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ext_ts_ps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_ext_ts_ps`]
module"]
#[doc(alias = "SS_EXT_TS_PS")]
pub type SsExtTsPs = crate::Reg<ss_ext_ts_ps::SsExtTsPsSpec>;
#[doc = "SS_EXT_TS_PS"]
pub mod ss_ext_ts_ps;
#[doc = "SS_EXT_TS_USIC (rw) register accessor: SS_EXT_TS_USIC\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ext_ts_usic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ext_ts_usic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_ext_ts_usic`]
module"]
#[doc(alias = "SS_EXT_TS_USIC")]
pub type SsExtTsUsic = crate::Reg<ss_ext_ts_usic::SsExtTsUsicSpec>;
#[doc = "SS_EXT_TS_USIC"]
pub mod ss_ext_ts_usic;
#[doc = "CREL (rw) register accessor: CREL\n\nYou can [`read`](crate::Reg::read) this register and get [`crel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`]
module"]
#[doc(alias = "CREL")]
pub type Crel = crate::Reg<crel::CrelSpec>;
#[doc = "CREL"]
pub mod crel;
#[doc = "ENDN (rw) register accessor: ENDN\n\nYou can [`read`](crate::Reg::read) this register and get [`endn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`]
module"]
#[doc(alias = "ENDN")]
pub type Endn = crate::Reg<endn::EndnSpec>;
#[doc = "ENDN"]
pub mod endn;
#[doc = "CUST (rw) register accessor: CUST\n\nYou can [`read`](crate::Reg::read) this register and get [`cust::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cust::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cust`]
module"]
#[doc(alias = "CUST")]
pub type Cust = crate::Reg<cust::CustSpec>;
#[doc = "CUST"]
pub mod cust;
#[doc = "DBTP (rw) register accessor: DBTP\n\nYou can [`read`](crate::Reg::read) this register and get [`dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`]
module"]
#[doc(alias = "DBTP")]
pub type Dbtp = crate::Reg<dbtp::DbtpSpec>;
#[doc = "DBTP"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: TEST\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "TEST"]
pub mod test;
#[doc = "RWD (rw) register accessor: RWD\n\nYou can [`read`](crate::Reg::read) this register and get [`rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`]
module"]
#[doc(alias = "RWD")]
pub type Rwd = crate::Reg<rwd::RwdSpec>;
#[doc = "RWD"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: CCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
#[doc(alias = "CCCR")]
pub type Cccr = crate::Reg<cccr::CccrSpec>;
#[doc = "CCCR"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: NBTP\n\nYou can [`read`](crate::Reg::read) this register and get [`nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`]
module"]
#[doc(alias = "NBTP")]
pub type Nbtp = crate::Reg<nbtp::NbtpSpec>;
#[doc = "NBTP"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: TSCC\n\nYou can [`read`](crate::Reg::read) this register and get [`tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`]
module"]
#[doc(alias = "TSCC")]
pub type Tscc = crate::Reg<tscc::TsccSpec>;
#[doc = "TSCC"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: TSCV\n\nYou can [`read`](crate::Reg::read) this register and get [`tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`]
module"]
#[doc(alias = "TSCV")]
pub type Tscv = crate::Reg<tscv::TscvSpec>;
#[doc = "TSCV"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: TOCC\n\nYou can [`read`](crate::Reg::read) this register and get [`tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`]
module"]
#[doc(alias = "TOCC")]
pub type Tocc = crate::Reg<tocc::ToccSpec>;
#[doc = "TOCC"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: TOCV\n\nYou can [`read`](crate::Reg::read) this register and get [`tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`]
module"]
#[doc(alias = "TOCV")]
pub type Tocv = crate::Reg<tocv::TocvSpec>;
#[doc = "TOCV"]
pub mod tocv;
#[doc = "RES00 (rw) register accessor: RES00\n\nYou can [`read`](crate::Reg::read) this register and get [`res00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res00`]
module"]
#[doc(alias = "RES00")]
pub type Res00 = crate::Reg<res00::Res00Spec>;
#[doc = "RES00"]
pub mod res00;
#[doc = "RES01 (rw) register accessor: RES01\n\nYou can [`read`](crate::Reg::read) this register and get [`res01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res01`]
module"]
#[doc(alias = "RES01")]
pub type Res01 = crate::Reg<res01::Res01Spec>;
#[doc = "RES01"]
pub mod res01;
#[doc = "RES02 (rw) register accessor: RES02\n\nYou can [`read`](crate::Reg::read) this register and get [`res02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res02`]
module"]
#[doc(alias = "RES02")]
pub type Res02 = crate::Reg<res02::Res02Spec>;
#[doc = "RES02"]
pub mod res02;
#[doc = "RES03 (rw) register accessor: RES03\n\nYou can [`read`](crate::Reg::read) this register and get [`res03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res03`]
module"]
#[doc(alias = "RES03")]
pub type Res03 = crate::Reg<res03::Res03Spec>;
#[doc = "RES03"]
pub mod res03;
#[doc = "ECR (rw) register accessor: ECR\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "ECR"]
pub mod ecr;
#[doc = "PSR (rw) register accessor: PSR\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "PSR"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: TDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`]
module"]
#[doc(alias = "TDCR")]
pub type Tdcr = crate::Reg<tdcr::TdcrSpec>;
#[doc = "TDCR"]
pub mod tdcr;
#[doc = "RES04 (rw) register accessor: RES04\n\nYou can [`read`](crate::Reg::read) this register and get [`res04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res04`]
module"]
#[doc(alias = "RES04")]
pub type Res04 = crate::Reg<res04::Res04Spec>;
#[doc = "RES04"]
pub mod res04;
#[doc = "IR (rw) register accessor: IR\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "IR"]
pub mod ir;
#[doc = "IE (rw) register accessor: IE\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "IE"]
pub mod ie;
#[doc = "ILS (rw) register accessor: ILS\n\nYou can [`read`](crate::Reg::read) this register and get [`ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`]
module"]
#[doc(alias = "ILS")]
pub type Ils = crate::Reg<ils::IlsSpec>;
#[doc = "ILS"]
pub mod ils;
#[doc = "ILE (rw) register accessor: ILE\n\nYou can [`read`](crate::Reg::read) this register and get [`ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`]
module"]
#[doc(alias = "ILE")]
pub type Ile = crate::Reg<ile::IleSpec>;
#[doc = "ILE"]
pub mod ile;
#[doc = "RES05 (rw) register accessor: RES05\n\nYou can [`read`](crate::Reg::read) this register and get [`res05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res05`]
module"]
#[doc(alias = "RES05")]
pub type Res05 = crate::Reg<res05::Res05Spec>;
#[doc = "RES05"]
pub mod res05;
#[doc = "RES06 (rw) register accessor: RES06\n\nYou can [`read`](crate::Reg::read) this register and get [`res06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res06`]
module"]
#[doc(alias = "RES06")]
pub type Res06 = crate::Reg<res06::Res06Spec>;
#[doc = "RES06"]
pub mod res06;
#[doc = "RES07 (rw) register accessor: RES07\n\nYou can [`read`](crate::Reg::read) this register and get [`res07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res07`]
module"]
#[doc(alias = "RES07")]
pub type Res07 = crate::Reg<res07::Res07Spec>;
#[doc = "RES07"]
pub mod res07;
#[doc = "RES08 (rw) register accessor: RES08\n\nYou can [`read`](crate::Reg::read) this register and get [`res08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res08`]
module"]
#[doc(alias = "RES08")]
pub type Res08 = crate::Reg<res08::Res08Spec>;
#[doc = "RES08"]
pub mod res08;
#[doc = "RES09 (rw) register accessor: RES09\n\nYou can [`read`](crate::Reg::read) this register and get [`res09::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res09::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res09`]
module"]
#[doc(alias = "RES09")]
pub type Res09 = crate::Reg<res09::Res09Spec>;
#[doc = "RES09"]
pub mod res09;
#[doc = "RES10 (rw) register accessor: RES10\n\nYou can [`read`](crate::Reg::read) this register and get [`res10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res10`]
module"]
#[doc(alias = "RES10")]
pub type Res10 = crate::Reg<res10::Res10Spec>;
#[doc = "RES10"]
pub mod res10;
#[doc = "RES11 (rw) register accessor: RES11\n\nYou can [`read`](crate::Reg::read) this register and get [`res11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res11`]
module"]
#[doc(alias = "RES11")]
pub type Res11 = crate::Reg<res11::Res11Spec>;
#[doc = "RES11"]
pub mod res11;
#[doc = "RES12 (rw) register accessor: RES12\n\nYou can [`read`](crate::Reg::read) this register and get [`res12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res12`]
module"]
#[doc(alias = "RES12")]
pub type Res12 = crate::Reg<res12::Res12Spec>;
#[doc = "RES12"]
pub mod res12;
#[doc = "GFC (rw) register accessor: GFC\n\nYou can [`read`](crate::Reg::read) this register and get [`gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfc`]
module"]
#[doc(alias = "GFC")]
pub type Gfc = crate::Reg<gfc::GfcSpec>;
#[doc = "GFC"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: SIDFC\n\nYou can [`read`](crate::Reg::read) this register and get [`sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidfc`]
module"]
#[doc(alias = "SIDFC")]
pub type Sidfc = crate::Reg<sidfc::SidfcSpec>;
#[doc = "SIDFC"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: XIDFC\n\nYou can [`read`](crate::Reg::read) this register and get [`xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidfc`]
module"]
#[doc(alias = "XIDFC")]
pub type Xidfc = crate::Reg<xidfc::XidfcSpec>;
#[doc = "XIDFC"]
pub mod xidfc;
#[doc = "RES13 (rw) register accessor: RES13\n\nYou can [`read`](crate::Reg::read) this register and get [`res13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res13`]
module"]
#[doc(alias = "RES13")]
pub type Res13 = crate::Reg<res13::Res13Spec>;
#[doc = "RES13"]
pub mod res13;
#[doc = "XIDAM (rw) register accessor: XIDAM\n\nYou can [`read`](crate::Reg::read) this register and get [`xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`]
module"]
#[doc(alias = "XIDAM")]
pub type Xidam = crate::Reg<xidam::XidamSpec>;
#[doc = "XIDAM"]
pub mod xidam;
#[doc = "HPMS (rw) register accessor: HPMS\n\nYou can [`read`](crate::Reg::read) this register and get [`hpms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`]
module"]
#[doc(alias = "HPMS")]
pub type Hpms = crate::Reg<hpms::HpmsSpec>;
#[doc = "HPMS"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: NDAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat1`]
module"]
#[doc(alias = "NDAT1")]
pub type Ndat1 = crate::Reg<ndat1::Ndat1Spec>;
#[doc = "NDAT1"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: NDAT2\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat2`]
module"]
#[doc(alias = "NDAT2")]
pub type Ndat2 = crate::Reg<ndat2::Ndat2Spec>;
#[doc = "NDAT2"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: RXF0C\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0c`]
module"]
#[doc(alias = "RXF0C")]
pub type Rxf0c = crate::Reg<rxf0c::Rxf0cSpec>;
#[doc = "RXF0C"]
pub mod rxf0c;
#[doc = "RXF0S (rw) register accessor: RXF0S\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`]
module"]
#[doc(alias = "RXF0S")]
pub type Rxf0s = crate::Reg<rxf0s::Rxf0sSpec>;
#[doc = "RXF0S"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: RXF0A\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`]
module"]
#[doc(alias = "RXF0A")]
pub type Rxf0a = crate::Reg<rxf0a::Rxf0aSpec>;
#[doc = "RXF0A"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: RXBC\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbc`]
module"]
#[doc(alias = "RXBC")]
pub type Rxbc = crate::Reg<rxbc::RxbcSpec>;
#[doc = "RXBC"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: RXF1C\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1c`]
module"]
#[doc(alias = "RXF1C")]
pub type Rxf1c = crate::Reg<rxf1c::Rxf1cSpec>;
#[doc = "RXF1C"]
pub mod rxf1c;
#[doc = "RXF1S (rw) register accessor: RXF1S\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`]
module"]
#[doc(alias = "RXF1S")]
pub type Rxf1s = crate::Reg<rxf1s::Rxf1sSpec>;
#[doc = "RXF1S"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: RXF1A\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`]
module"]
#[doc(alias = "RXF1A")]
pub type Rxf1a = crate::Reg<rxf1a::Rxf1aSpec>;
#[doc = "RXF1A"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: RXESC\n\nYou can [`read`](crate::Reg::read) this register and get [`rxesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxesc`]
module"]
#[doc(alias = "RXESC")]
pub type Rxesc = crate::Reg<rxesc::RxescSpec>;
#[doc = "RXESC"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: TXBC\n\nYou can [`read`](crate::Reg::read) this register and get [`txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`]
module"]
#[doc(alias = "TXBC")]
pub type Txbc = crate::Reg<txbc::TxbcSpec>;
#[doc = "TXBC"]
pub mod txbc;
#[doc = "TXFQS (rw) register accessor: TXFQS\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfqs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`]
module"]
#[doc(alias = "TXFQS")]
pub type Txfqs = crate::Reg<txfqs::TxfqsSpec>;
#[doc = "TXFQS"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: TXESC\n\nYou can [`read`](crate::Reg::read) this register and get [`txesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txesc`]
module"]
#[doc(alias = "TXESC")]
pub type Txesc = crate::Reg<txesc::TxescSpec>;
#[doc = "TXESC"]
pub mod txesc;
#[doc = "TXBRP (rw) register accessor: TXBRP\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`]
module"]
#[doc(alias = "TXBRP")]
pub type Txbrp = crate::Reg<txbrp::TxbrpSpec>;
#[doc = "TXBRP"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: TXBAR\n\nYou can [`read`](crate::Reg::read) this register and get [`txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`]
module"]
#[doc(alias = "TXBAR")]
pub type Txbar = crate::Reg<txbar::TxbarSpec>;
#[doc = "TXBAR"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: TXBCR\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`]
module"]
#[doc(alias = "TXBCR")]
pub type Txbcr = crate::Reg<txbcr::TxbcrSpec>;
#[doc = "TXBCR"]
pub mod txbcr;
#[doc = "TXBTO (rw) register accessor: TXBTO\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`]
module"]
#[doc(alias = "TXBTO")]
pub type Txbto = crate::Reg<txbto::TxbtoSpec>;
#[doc = "TXBTO"]
pub mod txbto;
#[doc = "TXBCF (rw) register accessor: TXBCF\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`]
module"]
#[doc(alias = "TXBCF")]
pub type Txbcf = crate::Reg<txbcf::TxbcfSpec>;
#[doc = "TXBCF"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: TXBTIE\n\nYou can [`read`](crate::Reg::read) this register and get [`txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`]
module"]
#[doc(alias = "TXBTIE")]
pub type Txbtie = crate::Reg<txbtie::TxbtieSpec>;
#[doc = "TXBTIE"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: TXBCIE\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`]
module"]
#[doc(alias = "TXBCIE")]
pub type Txbcie = crate::Reg<txbcie::TxbcieSpec>;
#[doc = "TXBCIE"]
pub mod txbcie;
#[doc = "RES14 (rw) register accessor: RES14\n\nYou can [`read`](crate::Reg::read) this register and get [`res14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res14`]
module"]
#[doc(alias = "RES14")]
pub type Res14 = crate::Reg<res14::Res14Spec>;
#[doc = "RES14"]
pub mod res14;
#[doc = "RES15 (rw) register accessor: RES15\n\nYou can [`read`](crate::Reg::read) this register and get [`res15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res15`]
module"]
#[doc(alias = "RES15")]
pub type Res15 = crate::Reg<res15::Res15Spec>;
#[doc = "RES15"]
pub mod res15;
#[doc = "TXEFC (rw) register accessor: TXEFC\n\nYou can [`read`](crate::Reg::read) this register and get [`txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefc`]
module"]
#[doc(alias = "TXEFC")]
pub type Txefc = crate::Reg<txefc::TxefcSpec>;
#[doc = "TXEFC"]
pub mod txefc;
#[doc = "TXEFS (rw) register accessor: TXEFS\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`]
module"]
#[doc(alias = "TXEFS")]
pub type Txefs = crate::Reg<txefs::TxefsSpec>;
#[doc = "TXEFS"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: TXEFA\n\nYou can [`read`](crate::Reg::read) this register and get [`txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`]
module"]
#[doc(alias = "TXEFA")]
pub type Txefa = crate::Reg<txefa::TxefaSpec>;
#[doc = "TXEFA"]
pub mod txefa;
#[doc = "RES16 (rw) register accessor: RES16\n\nYou can [`read`](crate::Reg::read) this register and get [`res16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res16`]
module"]
#[doc(alias = "RES16")]
pub type Res16 = crate::Reg<res16::Res16Spec>;
#[doc = "RES16"]
pub mod res16;
