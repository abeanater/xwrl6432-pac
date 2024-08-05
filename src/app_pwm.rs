#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tbctl_tbsts: TbctlTbsts,
    tbphs: Tbphs,
    tbctr_tbprd: TbctrTbprd,
    cmpctl: Cmpctl,
    cmpa: Cmpa,
    cmpb_aqctla: CmpbAqctla,
    aqctlb_aqsfrc: AqctlbAqsfrc,
    aqcsfrc_dbctl: AqcsfrcDbctl,
    dbred_dbfed: DbredDbfed,
    tzsel_tzdcsel: TzselTzdcsel,
    tzctl_tzeint: TzctlTzeint,
    tzflg_tzclr: TzflgTzclr,
    tzfrc_etsel: TzfrcEtsel,
    etps_etflg: EtpsEtflg,
    etclr_etfrc: EtclrEtfrc,
    pcctl: Pcctl,
    reserved1: Reserved1,
    reserved2: Reserved2,
    reserved3: Reserved3,
    reserved4: Reserved4,
    reserved5: Reserved5,
    reserved6: Reserved6,
    reserved7: Reserved7,
    reserved8: Reserved8,
    dctripsel_dcactl: DctripselDcactl,
    dcbctl_dcfctl: DcbctlDcfctl,
    dccapctl_dcfoffset: DccapctlDcfoffset,
    dcfoffsetcnt_dcfwindow: DcfoffsetcntDcfwindow,
    dcfwindowcnt_dccap: DcfwindowcntDccap,
}
impl RegisterBlock {
    #[doc = "0x00 - Time-Base Control Register/ Status Register"]
    #[inline(always)]
    pub const fn tbctl_tbsts(&self) -> &TbctlTbsts {
        &self.tbctl_tbsts
    }
    #[doc = "0x04 - Time-Base Phase Register"]
    #[inline(always)]
    pub const fn tbphs(&self) -> &Tbphs {
        &self.tbphs
    }
    #[doc = "0x08 - Time-Base Counter Register/ Period Register"]
    #[inline(always)]
    pub const fn tbctr_tbprd(&self) -> &TbctrTbprd {
        &self.tbctr_tbprd
    }
    #[doc = "0x0c - Counter-Compare Control Register"]
    #[inline(always)]
    pub const fn cmpctl(&self) -> &Cmpctl {
        &self.cmpctl
    }
    #[doc = "0x10 - Counter-Compare A Register"]
    #[inline(always)]
    pub const fn cmpa(&self) -> &Cmpa {
        &self.cmpa
    }
    #[doc = "0x14 - Counter-Compare B Register/ Action-Qualifier Control Register for Output A (EPWMxA)"]
    #[inline(always)]
    pub const fn cmpb_aqctla(&self) -> &CmpbAqctla {
        &self.cmpb_aqctla
    }
    #[doc = "0x18 - Action-Qualifier Control Register for Output B (EPWMxB)/ Action-Qualifier Software Force Register"]
    #[inline(always)]
    pub const fn aqctlb_aqsfrc(&self) -> &AqctlbAqsfrc {
        &self.aqctlb_aqsfrc
    }
    #[doc = "0x1c - Dead-Band Generator Control Register/ Action-Qualifier Continuous S/W Force Register Set"]
    #[inline(always)]
    pub const fn aqcsfrc_dbctl(&self) -> &AqcsfrcDbctl {
        &self.aqcsfrc_dbctl
    }
    #[doc = "0x20 - Dead-Band Generator Rising Edge Delay Count Register/ Dead-Band Generator Falling Edge Delay Count Register"]
    #[inline(always)]
    pub const fn dbred_dbfed(&self) -> &DbredDbfed {
        &self.dbred_dbfed
    }
    #[doc = "0x24 - Trip Zone Digital Compare Select Register/ Trip-Zone Select Register"]
    #[inline(always)]
    pub const fn tzsel_tzdcsel(&self) -> &TzselTzdcsel {
        &self.tzsel_tzdcsel
    }
    #[doc = "0x28 - Trip-Zone Control Register/ Trip-Zone Enable Interrupt Register"]
    #[inline(always)]
    pub const fn tzctl_tzeint(&self) -> &TzctlTzeint {
        &self.tzctl_tzeint
    }
    #[doc = "0x2c - Trip-Zone Flag Register/ Trip-Zone Clear Register"]
    #[inline(always)]
    pub const fn tzflg_tzclr(&self) -> &TzflgTzclr {
        &self.tzflg_tzclr
    }
    #[doc = "0x30 - Trip-Zone Force Register / Event-Trigger Selection Register"]
    #[inline(always)]
    pub const fn tzfrc_etsel(&self) -> &TzfrcEtsel {
        &self.tzfrc_etsel
    }
    #[doc = "0x34 - Event-Trigger Pre-Scale Register/ Event-Trigger Flag Register"]
    #[inline(always)]
    pub const fn etps_etflg(&self) -> &EtpsEtflg {
        &self.etps_etflg
    }
    #[doc = "0x38 - Event-Trigger Clear Register/ Event-Trigger Force Register"]
    #[inline(always)]
    pub const fn etclr_etfrc(&self) -> &EtclrEtfrc {
        &self.etclr_etfrc
    }
    #[doc = "0x3c - PWM-Chopper Control Register"]
    #[inline(always)]
    pub const fn pcctl(&self) -> &Pcctl {
        &self.pcctl
    }
    #[doc = "0x40 - Reserved"]
    #[inline(always)]
    pub const fn reserved1(&self) -> &Reserved1 {
        &self.reserved1
    }
    #[doc = "0x44 - Reserved"]
    #[inline(always)]
    pub const fn reserved2(&self) -> &Reserved2 {
        &self.reserved2
    }
    #[doc = "0x48 - Reserved"]
    #[inline(always)]
    pub const fn reserved3(&self) -> &Reserved3 {
        &self.reserved3
    }
    #[doc = "0x4c - Reserved"]
    #[inline(always)]
    pub const fn reserved4(&self) -> &Reserved4 {
        &self.reserved4
    }
    #[doc = "0x50 - Reserved"]
    #[inline(always)]
    pub const fn reserved5(&self) -> &Reserved5 {
        &self.reserved5
    }
    #[doc = "0x54 - Reserved"]
    #[inline(always)]
    pub const fn reserved6(&self) -> &Reserved6 {
        &self.reserved6
    }
    #[doc = "0x58 - Reserved"]
    #[inline(always)]
    pub const fn reserved7(&self) -> &Reserved7 {
        &self.reserved7
    }
    #[doc = "0x5c - Reserved"]
    #[inline(always)]
    pub const fn reserved8(&self) -> &Reserved8 {
        &self.reserved8
    }
    #[doc = "0x60 - Digital Compare Trip Select Register/ Digital Compare A Control Register"]
    #[inline(always)]
    pub const fn dctripsel_dcactl(&self) -> &DctripselDcactl {
        &self.dctripsel_dcactl
    }
    #[doc = "0x64 - Digital Compare B Control Register/ Digital Compare Filter Control Register"]
    #[inline(always)]
    pub const fn dcbctl_dcfctl(&self) -> &DcbctlDcfctl {
        &self.dcbctl_dcfctl
    }
    #[doc = "0x68 - Digital Compare Capture Control Register/ Digital Compare Filter Offset Register"]
    #[inline(always)]
    pub const fn dccapctl_dcfoffset(&self) -> &DccapctlDcfoffset {
        &self.dccapctl_dcfoffset
    }
    #[doc = "0x6c - Digital Compare Filter Offset Counter Register/ Digital Compare Filter Window Register"]
    #[inline(always)]
    pub const fn dcfoffsetcnt_dcfwindow(&self) -> &DcfoffsetcntDcfwindow {
        &self.dcfoffsetcnt_dcfwindow
    }
    #[doc = "0x70 - Digital Compare Filter Window Counter Register/ Digital Compare Counter Capture Register"]
    #[inline(always)]
    pub const fn dcfwindowcnt_dccap(&self) -> &DcfwindowcntDccap {
        &self.dcfwindowcnt_dccap
    }
}
#[doc = "TBCTL_TBSTS (rw) register accessor: Time-Base Control Register/ Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctl_tbsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctl_tbsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctl_tbsts`]
module"]
#[doc(alias = "TBCTL_TBSTS")]
pub type TbctlTbsts = crate::Reg<tbctl_tbsts::TbctlTbstsSpec>;
#[doc = "Time-Base Control Register/ Status Register"]
pub mod tbctl_tbsts;
#[doc = "TBPHS (rw) register accessor: Time-Base Phase Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbphs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbphs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbphs`]
module"]
#[doc(alias = "TBPHS")]
pub type Tbphs = crate::Reg<tbphs::TbphsSpec>;
#[doc = "Time-Base Phase Register"]
pub mod tbphs;
#[doc = "TBCTR_TBPRD (rw) register accessor: Time-Base Counter Register/ Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctr_tbprd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctr_tbprd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctr_tbprd`]
module"]
#[doc(alias = "TBCTR_TBPRD")]
pub type TbctrTbprd = crate::Reg<tbctr_tbprd::TbctrTbprdSpec>;
#[doc = "Time-Base Counter Register/ Period Register"]
pub mod tbctr_tbprd;
#[doc = "CMPCTL (rw) register accessor: Counter-Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpctl`]
module"]
#[doc(alias = "CMPCTL")]
pub type Cmpctl = crate::Reg<cmpctl::CmpctlSpec>;
#[doc = "Counter-Compare Control Register"]
pub mod cmpctl;
#[doc = "CMPA (rw) register accessor: Counter-Compare A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpa`]
module"]
#[doc(alias = "CMPA")]
pub type Cmpa = crate::Reg<cmpa::CmpaSpec>;
#[doc = "Counter-Compare A Register"]
pub mod cmpa;
#[doc = "CMPB_AQCTLA (rw) register accessor: Counter-Compare B Register/ Action-Qualifier Control Register for Output A (EPWMxA)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpb_aqctla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpb_aqctla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpb_aqctla`]
module"]
#[doc(alias = "CMPB_AQCTLA")]
pub type CmpbAqctla = crate::Reg<cmpb_aqctla::CmpbAqctlaSpec>;
#[doc = "Counter-Compare B Register/ Action-Qualifier Control Register for Output A (EPWMxA)"]
pub mod cmpb_aqctla;
#[doc = "AQCTLB_AQSFRC (rw) register accessor: Action-Qualifier Control Register for Output B (EPWMxB)/ Action-Qualifier Software Force Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aqctlb_aqsfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aqctlb_aqsfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aqctlb_aqsfrc`]
module"]
#[doc(alias = "AQCTLB_AQSFRC")]
pub type AqctlbAqsfrc = crate::Reg<aqctlb_aqsfrc::AqctlbAqsfrcSpec>;
#[doc = "Action-Qualifier Control Register for Output B (EPWMxB)/ Action-Qualifier Software Force Register"]
pub mod aqctlb_aqsfrc;
#[doc = "AQCSFRC_DBCTL (rw) register accessor: Dead-Band Generator Control Register/ Action-Qualifier Continuous S/W Force Register Set\n\nYou can [`read`](crate::Reg::read) this register and get [`aqcsfrc_dbctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aqcsfrc_dbctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aqcsfrc_dbctl`]
module"]
#[doc(alias = "AQCSFRC_DBCTL")]
pub type AqcsfrcDbctl = crate::Reg<aqcsfrc_dbctl::AqcsfrcDbctlSpec>;
#[doc = "Dead-Band Generator Control Register/ Action-Qualifier Continuous S/W Force Register Set"]
pub mod aqcsfrc_dbctl;
#[doc = "DBRED_DBFED (rw) register accessor: Dead-Band Generator Rising Edge Delay Count Register/ Dead-Band Generator Falling Edge Delay Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbred_dbfed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbred_dbfed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbred_dbfed`]
module"]
#[doc(alias = "DBRED_DBFED")]
pub type DbredDbfed = crate::Reg<dbred_dbfed::DbredDbfedSpec>;
#[doc = "Dead-Band Generator Rising Edge Delay Count Register/ Dead-Band Generator Falling Edge Delay Count Register"]
pub mod dbred_dbfed;
#[doc = "TZSEL_TZDCSEL (rw) register accessor: Trip Zone Digital Compare Select Register/ Trip-Zone Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsel_tzdcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsel_tzdcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsel_tzdcsel`]
module"]
#[doc(alias = "TZSEL_TZDCSEL")]
pub type TzselTzdcsel = crate::Reg<tzsel_tzdcsel::TzselTzdcselSpec>;
#[doc = "Trip Zone Digital Compare Select Register/ Trip-Zone Select Register"]
pub mod tzsel_tzdcsel;
#[doc = "TZCTL_TZEINT (rw) register accessor: Trip-Zone Control Register/ Trip-Zone Enable Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzctl_tzeint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzctl_tzeint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzctl_tzeint`]
module"]
#[doc(alias = "TZCTL_TZEINT")]
pub type TzctlTzeint = crate::Reg<tzctl_tzeint::TzctlTzeintSpec>;
#[doc = "Trip-Zone Control Register/ Trip-Zone Enable Interrupt Register"]
pub mod tzctl_tzeint;
#[doc = "TZFLG_TZCLR (rw) register accessor: Trip-Zone Flag Register/ Trip-Zone Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzflg_tzclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzflg_tzclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzflg_tzclr`]
module"]
#[doc(alias = "TZFLG_TZCLR")]
pub type TzflgTzclr = crate::Reg<tzflg_tzclr::TzflgTzclrSpec>;
#[doc = "Trip-Zone Flag Register/ Trip-Zone Clear Register"]
pub mod tzflg_tzclr;
#[doc = "TZFRC_ETSEL (rw) register accessor: Trip-Zone Force Register / Event-Trigger Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzfrc_etsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfrc_etsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzfrc_etsel`]
module"]
#[doc(alias = "TZFRC_ETSEL")]
pub type TzfrcEtsel = crate::Reg<tzfrc_etsel::TzfrcEtselSpec>;
#[doc = "Trip-Zone Force Register / Event-Trigger Selection Register"]
pub mod tzfrc_etsel;
#[doc = "ETPS_ETFLG (rw) register accessor: Event-Trigger Pre-Scale Register/ Event-Trigger Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etps_etflg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etps_etflg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etps_etflg`]
module"]
#[doc(alias = "ETPS_ETFLG")]
pub type EtpsEtflg = crate::Reg<etps_etflg::EtpsEtflgSpec>;
#[doc = "Event-Trigger Pre-Scale Register/ Event-Trigger Flag Register"]
pub mod etps_etflg;
#[doc = "ETCLR_ETFRC (rw) register accessor: Event-Trigger Clear Register/ Event-Trigger Force Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etclr_etfrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etclr_etfrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etclr_etfrc`]
module"]
#[doc(alias = "ETCLR_ETFRC")]
pub type EtclrEtfrc = crate::Reg<etclr_etfrc::EtclrEtfrcSpec>;
#[doc = "Event-Trigger Clear Register/ Event-Trigger Force Register"]
pub mod etclr_etfrc;
#[doc = "PCCTL (rw) register accessor: PWM-Chopper Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcctl`]
module"]
#[doc(alias = "PCCTL")]
pub type Pcctl = crate::Reg<pcctl::PcctlSpec>;
#[doc = "PWM-Chopper Control Register"]
pub mod pcctl;
#[doc = "Reserved1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved1`]
module"]
pub type Reserved1 = crate::Reg<reserved1::Reserved1Spec>;
#[doc = "Reserved"]
pub mod reserved1;
#[doc = "Reserved2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved2`]
module"]
pub type Reserved2 = crate::Reg<reserved2::Reserved2Spec>;
#[doc = "Reserved"]
pub mod reserved2;
#[doc = "Reserved3 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved3`]
module"]
pub type Reserved3 = crate::Reg<reserved3::Reserved3Spec>;
#[doc = "Reserved"]
pub mod reserved3;
#[doc = "Reserved4 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved4`]
module"]
pub type Reserved4 = crate::Reg<reserved4::Reserved4Spec>;
#[doc = "Reserved"]
pub mod reserved4;
#[doc = "Reserved5 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved5`]
module"]
pub type Reserved5 = crate::Reg<reserved5::Reserved5Spec>;
#[doc = "Reserved"]
pub mod reserved5;
#[doc = "Reserved6 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved6`]
module"]
pub type Reserved6 = crate::Reg<reserved6::Reserved6Spec>;
#[doc = "Reserved"]
pub mod reserved6;
#[doc = "Reserved7 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved7`]
module"]
pub type Reserved7 = crate::Reg<reserved7::Reserved7Spec>;
#[doc = "Reserved"]
pub mod reserved7;
#[doc = "Reserved8 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved8`]
module"]
pub type Reserved8 = crate::Reg<reserved8::Reserved8Spec>;
#[doc = "Reserved"]
pub mod reserved8;
#[doc = "DCTRIPSEL_DCACTL (rw) register accessor: Digital Compare Trip Select Register/ Digital Compare A Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dctripsel_dcactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctripsel_dcactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctripsel_dcactl`]
module"]
#[doc(alias = "DCTRIPSEL_DCACTL")]
pub type DctripselDcactl = crate::Reg<dctripsel_dcactl::DctripselDcactlSpec>;
#[doc = "Digital Compare Trip Select Register/ Digital Compare A Control Register"]
pub mod dctripsel_dcactl;
#[doc = "DCBCTL_DCFCTL (rw) register accessor: Digital Compare B Control Register/ Digital Compare Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcbctl_dcfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcbctl_dcfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcbctl_dcfctl`]
module"]
#[doc(alias = "DCBCTL_DCFCTL")]
pub type DcbctlDcfctl = crate::Reg<dcbctl_dcfctl::DcbctlDcfctlSpec>;
#[doc = "Digital Compare B Control Register/ Digital Compare Filter Control Register"]
pub mod dcbctl_dcfctl;
#[doc = "DCCAPCTL_DCFOFFSET (rw) register accessor: Digital Compare Capture Control Register/ Digital Compare Filter Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dccapctl_dcfoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccapctl_dcfoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccapctl_dcfoffset`]
module"]
#[doc(alias = "DCCAPCTL_DCFOFFSET")]
pub type DccapctlDcfoffset = crate::Reg<dccapctl_dcfoffset::DccapctlDcfoffsetSpec>;
#[doc = "Digital Compare Capture Control Register/ Digital Compare Filter Offset Register"]
pub mod dccapctl_dcfoffset;
#[doc = "DCFOFFSETCNT_DCFWINDOW (rw) register accessor: Digital Compare Filter Offset Counter Register/ Digital Compare Filter Window Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfoffsetcnt_dcfwindow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfoffsetcnt_dcfwindow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfoffsetcnt_dcfwindow`]
module"]
#[doc(alias = "DCFOFFSETCNT_DCFWINDOW")]
pub type DcfoffsetcntDcfwindow = crate::Reg<dcfoffsetcnt_dcfwindow::DcfoffsetcntDcfwindowSpec>;
#[doc = "Digital Compare Filter Offset Counter Register/ Digital Compare Filter Window Register"]
pub mod dcfoffsetcnt_dcfwindow;
#[doc = "DCFWINDOWCNT_DCCAP (rw) register accessor: Digital Compare Filter Window Counter Register/ Digital Compare Counter Capture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfwindowcnt_dccap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfwindowcnt_dccap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfwindowcnt_dccap`]
module"]
#[doc(alias = "DCFWINDOWCNT_DCCAP")]
pub type DcfwindowcntDccap = crate::Reg<dcfwindowcnt_dccap::DcfwindowcntDccapSpec>;
#[doc = "Digital Compare Filter Window Counter Register/ Digital Compare Counter Capture Register"]
pub mod dcfwindowcnt_dccap;
