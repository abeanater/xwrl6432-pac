#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aggr_revision: AggrRevision,
    _reserved1: [u8; 0x04],
    ecc_vector: EccVector,
    misc_status: MiscStatus,
    ecc_wrap_revision: EccWrapRevision,
    control: Control,
    error_ctrl1: ErrorCtrl1,
    error_ctrl2: ErrorCtrl2,
    error_status1: ErrorStatus1,
    error_status2: ErrorStatus2,
    error_status3: ErrorStatus3,
    _reserved10: [u8; 0x10],
    sec_eoi_reg: SecEoiReg,
    sec_status_reg0: SecStatusReg0,
    _reserved12: [u8; 0x3c],
    sec_enable_set_reg0: SecEnableSetReg0,
    _reserved13: [u8; 0x3c],
    sec_enable_clr_reg0: SecEnableClrReg0,
    _reserved14: [u8; 0x78],
    ded_eoi_reg: DedEoiReg,
    ded_status_reg0: DedStatusReg0,
    _reserved16: [u8; 0x3c],
    ded_enable_set_reg0: DedEnableSetReg0,
    _reserved17: [u8; 0x3c],
    ded_enable_clr_reg0: DedEnableClrReg0,
    _reserved18: [u8; 0x3c],
    aggr_enable_set: AggrEnableSet,
    aggr_enable_clr: AggrEnableClr,
    aggr_status_set: AggrStatusSet,
    aggr_status_clr: AggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn aggr_revision(&self) -> &AggrRevision {
        &self.aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn ecc_vector(&self) -> &EccVector {
        &self.ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn misc_status(&self) -> &MiscStatus {
        &self.misc_status
    }
    #[doc = "0x10 - Revision parameters"]
    #[inline(always)]
    pub const fn ecc_wrap_revision(&self) -> &EccWrapRevision {
        &self.ecc_wrap_revision
    }
    #[doc = "0x14 - ECC Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x18 - ECC Error Control1 Register"]
    #[inline(always)]
    pub const fn error_ctrl1(&self) -> &ErrorCtrl1 {
        &self.error_ctrl1
    }
    #[doc = "0x1c - ECC Error Control2 Register"]
    #[inline(always)]
    pub const fn error_ctrl2(&self) -> &ErrorCtrl2 {
        &self.error_ctrl2
    }
    #[doc = "0x20 - ECC Error Status1 Register"]
    #[inline(always)]
    pub const fn error_status1(&self) -> &ErrorStatus1 {
        &self.error_status1
    }
    #[doc = "0x24 - ECC Error Status2 Register"]
    #[inline(always)]
    pub const fn error_status2(&self) -> &ErrorStatus2 {
        &self.error_status2
    }
    #[doc = "0x28 - ECC Error Status3 Register"]
    #[inline(always)]
    pub const fn error_status3(&self) -> &ErrorStatus3 {
        &self.error_status3
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
#[doc = "AGGR_REVISION (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_revision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_revision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aggr_revision`]
module"]
#[doc(alias = "AGGR_REVISION")]
pub type AggrRevision = crate::Reg<aggr_revision::AggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod aggr_revision;
#[doc = "ECC_VECTOR (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_vector::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_vector::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_vector`]
module"]
#[doc(alias = "ECC_VECTOR")]
pub type EccVector = crate::Reg<ecc_vector::EccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod ecc_vector;
#[doc = "MISC_STATUS (rw) register accessor: Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_status`]
module"]
#[doc(alias = "MISC_STATUS")]
pub type MiscStatus = crate::Reg<misc_status::MiscStatusSpec>;
#[doc = "Misc Status"]
pub mod misc_status;
#[doc = "ECC_WRAP_REVISION (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_wrap_revision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_wrap_revision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_wrap_revision`]
module"]
#[doc(alias = "ECC_WRAP_REVISION")]
pub type EccWrapRevision = crate::Reg<ecc_wrap_revision::EccWrapRevisionSpec>;
#[doc = "Revision parameters"]
pub mod ecc_wrap_revision;
#[doc = "CONTROL (rw) register accessor: ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "ECC Control Register"]
pub mod control;
#[doc = "ERROR_CTRL1 (rw) register accessor: ECC Error Control1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_ctrl1`]
module"]
#[doc(alias = "ERROR_CTRL1")]
pub type ErrorCtrl1 = crate::Reg<error_ctrl1::ErrorCtrl1Spec>;
#[doc = "ECC Error Control1 Register"]
pub mod error_ctrl1;
#[doc = "ERROR_CTRL2 (rw) register accessor: ECC Error Control2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_ctrl2`]
module"]
#[doc(alias = "ERROR_CTRL2")]
pub type ErrorCtrl2 = crate::Reg<error_ctrl2::ErrorCtrl2Spec>;
#[doc = "ECC Error Control2 Register"]
pub mod error_ctrl2;
#[doc = "ERROR_STATUS1 (rw) register accessor: ECC Error Status1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_status1`]
module"]
#[doc(alias = "ERROR_STATUS1")]
pub type ErrorStatus1 = crate::Reg<error_status1::ErrorStatus1Spec>;
#[doc = "ECC Error Status1 Register"]
pub mod error_status1;
#[doc = "ERROR_STATUS2 (rw) register accessor: ECC Error Status2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_status2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_status2`]
module"]
#[doc(alias = "ERROR_STATUS2")]
pub type ErrorStatus2 = crate::Reg<error_status2::ErrorStatus2Spec>;
#[doc = "ECC Error Status2 Register"]
pub mod error_status2;
#[doc = "ERROR_STATUS3 (rw) register accessor: ECC Error Status3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_status3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_status3`]
module"]
#[doc(alias = "ERROR_STATUS3")]
pub type ErrorStatus3 = crate::Reg<error_status3::ErrorStatus3Spec>;
#[doc = "ECC Error Status3 Register"]
pub mod error_status3;
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
