#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rev: Rev,
    _reserved1: [u8; 0x04],
    vector: Vector,
    stat: Stat,
    _reserved3: [u8; 0x04],
    ctrl: Ctrl,
    err_ctrl1: ErrCtrl1,
    err_ctrl2: ErrCtrl2,
    err_stat1: ErrStat1,
    err_stat2: ErrStat2,
    err_stat3: ErrStat3,
    _reserved9: [u8; 0x10],
    sec_eoi_reg: SecEoiReg,
    sec_status_reg0: SecStatusReg0,
    _reserved11: [u8; 0x3c],
    sec_enable_set_reg0: SecEnableSetReg0,
    _reserved12: [u8; 0x3c],
    sec_enable_clr_reg0: SecEnableClrReg0,
    _reserved13: [u8; 0x78],
    ded_eoi_reg: DedEoiReg,
    ded_status_reg0: DedStatusReg0,
    _reserved15: [u8; 0x3c],
    ded_enable_set_reg0: DedEnableSetReg0,
    _reserved16: [u8; 0x3c],
    ded_enable_clr_reg0: DedEnableClrReg0,
    _reserved17: [u8; 0x3c],
    aggr_enable_set: AggrEnableSet,
    aggr_enable_clr: AggrEnableClr,
    aggr_status_set: AggrStatusSet,
    aggr_status_clr: AggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Aggregator Revision Register"]
    #[inline(always)]
    pub const fn rev(&self) -> &Rev {
        &self.rev
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn vector(&self) -> &Vector {
        &self.vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x14 - CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x18 - ERR_CTRL1"]
    #[inline(always)]
    pub const fn err_ctrl1(&self) -> &ErrCtrl1 {
        &self.err_ctrl1
    }
    #[doc = "0x1c - ERR_CTRL2"]
    #[inline(always)]
    pub const fn err_ctrl2(&self) -> &ErrCtrl2 {
        &self.err_ctrl2
    }
    #[doc = "0x20 - ERR_STAT1"]
    #[inline(always)]
    pub const fn err_stat1(&self) -> &ErrStat1 {
        &self.err_stat1
    }
    #[doc = "0x24 - ERR_STAT2"]
    #[inline(always)]
    pub const fn err_stat2(&self) -> &ErrStat2 {
        &self.err_stat2
    }
    #[doc = "0x28 - ERR_STAT3"]
    #[inline(always)]
    pub const fn err_stat3(&self) -> &ErrStat3 {
        &self.err_stat3
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn sec_eoi_reg(&self) -> &SecEoiReg {
        &self.sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn sec_status_reg0(&self) -> &SecStatusReg0 {
        &self.sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn sec_enable_set_reg0(&self) -> &SecEnableSetReg0 {
        &self.sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn sec_enable_clr_reg0(&self) -> &SecEnableClrReg0 {
        &self.sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ded_eoi_reg(&self) -> &DedEoiReg {
        &self.ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ded_status_reg0(&self) -> &DedStatusReg0 {
        &self.ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ded_enable_set_reg0(&self) -> &DedEnableSetReg0 {
        &self.ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ded_enable_clr_reg0(&self) -> &DedEnableClrReg0 {
        &self.ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn aggr_enable_set(&self) -> &AggrEnableSet {
        &self.aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn aggr_enable_clr(&self) -> &AggrEnableClr {
        &self.aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn aggr_status_set(&self) -> &AggrStatusSet {
        &self.aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn aggr_status_clr(&self) -> &AggrStatusClr {
        &self.aggr_status_clr
    }
}
#[doc = "REV (rw) register accessor: Aggregator Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rev`]
module"]
#[doc(alias = "REV")]
pub type Rev = crate::Reg<rev::RevSpec>;
#[doc = "Aggregator Revision Register"]
pub mod rev;
#[doc = "VECTOR (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vector::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vector::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vector`]
module"]
#[doc(alias = "VECTOR")]
pub type Vector = crate::Reg<vector::VectorSpec>;
#[doc = "ECC Vector Register"]
pub mod vector;
#[doc = "STAT (rw) register accessor: Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Misc Status"]
pub mod stat;
#[doc = "CTRL (rw) register accessor: CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "CTRL"]
pub mod ctrl;
#[doc = "ERR_CTRL1 (rw) register accessor: ERR_CTRL1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_ctrl1`]
module"]
#[doc(alias = "ERR_CTRL1")]
pub type ErrCtrl1 = crate::Reg<err_ctrl1::ErrCtrl1Spec>;
#[doc = "ERR_CTRL1"]
pub mod err_ctrl1;
#[doc = "ERR_CTRL2 (rw) register accessor: ERR_CTRL2\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_ctrl2`]
module"]
#[doc(alias = "ERR_CTRL2")]
pub type ErrCtrl2 = crate::Reg<err_ctrl2::ErrCtrl2Spec>;
#[doc = "ERR_CTRL2"]
pub mod err_ctrl2;
#[doc = "ERR_STAT1 (rw) register accessor: ERR_STAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat1`]
module"]
#[doc(alias = "ERR_STAT1")]
pub type ErrStat1 = crate::Reg<err_stat1::ErrStat1Spec>;
#[doc = "ERR_STAT1"]
pub mod err_stat1;
#[doc = "ERR_STAT2 (rw) register accessor: ERR_STAT2\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat2`]
module"]
#[doc(alias = "ERR_STAT2")]
pub type ErrStat2 = crate::Reg<err_stat2::ErrStat2Spec>;
#[doc = "ERR_STAT2"]
pub mod err_stat2;
#[doc = "ERR_STAT3 (rw) register accessor: ERR_STAT3\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stat3`]
module"]
#[doc(alias = "ERR_STAT3")]
pub type ErrStat3 = crate::Reg<err_stat3::ErrStat3Spec>;
#[doc = "ERR_STAT3"]
pub mod err_stat3;
#[doc = "SEC_EOI_REG (rw) register accessor: EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_eoi_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_eoi_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_eoi_reg`]
module"]
#[doc(alias = "SEC_EOI_REG")]
pub type SecEoiReg = crate::Reg<sec_eoi_reg::SecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod sec_eoi_reg;
#[doc = "SEC_STATUS_REG0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_status_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_status_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_status_reg0`]
module"]
#[doc(alias = "SEC_STATUS_REG0")]
pub type SecStatusReg0 = crate::Reg<sec_status_reg0::SecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod sec_status_reg0;
#[doc = "SEC_ENABLE_SET_REG0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_enable_set_reg0`]
module"]
#[doc(alias = "SEC_ENABLE_SET_REG0")]
pub type SecEnableSetReg0 = crate::Reg<sec_enable_set_reg0::SecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod sec_enable_set_reg0;
#[doc = "SEC_ENABLE_CLR_REG0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_clr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_clr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_enable_clr_reg0`]
module"]
#[doc(alias = "SEC_ENABLE_CLR_REG0")]
pub type SecEnableClrReg0 = crate::Reg<sec_enable_clr_reg0::SecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod sec_enable_clr_reg0;
#[doc = "DED_EOI_REG (rw) register accessor: EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_eoi_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_eoi_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_eoi_reg`]
module"]
#[doc(alias = "DED_EOI_REG")]
pub type DedEoiReg = crate::Reg<ded_eoi_reg::DedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ded_eoi_reg;
#[doc = "DED_STATUS_REG0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_status_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_status_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_status_reg0`]
module"]
#[doc(alias = "DED_STATUS_REG0")]
pub type DedStatusReg0 = crate::Reg<ded_status_reg0::DedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ded_status_reg0;
#[doc = "DED_ENABLE_SET_REG0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_set_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_set_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_enable_set_reg0`]
module"]
#[doc(alias = "DED_ENABLE_SET_REG0")]
pub type DedEnableSetReg0 = crate::Reg<ded_enable_set_reg0::DedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ded_enable_set_reg0;
#[doc = "DED_ENABLE_CLR_REG0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_clr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_clr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ded_enable_clr_reg0`]
module"]
#[doc(alias = "DED_ENABLE_CLR_REG0")]
pub type DedEnableClrReg0 = crate::Reg<ded_enable_clr_reg0::DedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ded_enable_clr_reg0;
#[doc = "AGGR_ENABLE_SET (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_enable_set`]
module"]
#[doc(alias = "AGGR_ENABLE_SET")]
pub type AggrEnableSet = crate::Reg<aggr_enable_set::AggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod aggr_enable_set;
#[doc = "AGGR_ENABLE_CLR (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_enable_clr`]
module"]
#[doc(alias = "AGGR_ENABLE_CLR")]
pub type AggrEnableClr = crate::Reg<aggr_enable_clr::AggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod aggr_enable_clr;
#[doc = "AGGR_STATUS_SET (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_status_set`]
module"]
#[doc(alias = "AGGR_STATUS_SET")]
pub type AggrStatusSet = crate::Reg<aggr_status_set::AggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod aggr_status_set;
#[doc = "AGGR_STATUS_CLR (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_status_clr`]
module"]
#[doc(alias = "AGGR_STATUS_CLR")]
pub type AggrStatusClr = crate::Reg<aggr_status_clr::AggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod aggr_status_clr;
