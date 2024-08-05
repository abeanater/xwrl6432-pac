#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    plldig_en: PlldigEn,
    plldig_mdiv_ndiv: PlldigMdivNdiv,
    plldig_ctrl: PlldigCtrl,
    plldig_mode_en: PlldigModeEn,
    plldig_apll_sw_dis_delay1: PlldigApllSwDisDelay1,
    plldig_apll_sw_dis_delay2: PlldigApllSwDisDelay2,
    plldig_override: PlldigOverride,
    plldig_status: PlldigStatus,
    fast_clk_mux_postdiv: FastClkMuxPostdiv,
    fast_clk_status: FastClkStatus,
    _reserved11: [u8; 0x0fdc],
    lock0_kick0: Lock0Kick0,
    lock0_kick1: Lock0Kick1,
    intr_raw_status: IntrRawStatus,
    intr_enabled_status_clear: IntrEnabledStatusClear,
    intr_enable: IntrEnable,
    intr_enable_clear: IntrEnableClear,
    eoi: Eoi,
    fault_address: FaultAddress,
    fault_type_status: FaultTypeStatus,
    fault_attr_status: FaultAttrStatus,
    fault_clear: FaultClear,
}
impl RegisterBlock {
    #[doc = "0x00 - PID register"]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
    #[doc = "0x04 - PLLDIG_EN"]
    #[inline(always)]
    pub const fn plldig_en(&self) -> &PlldigEn {
        &self.plldig_en
    }
    #[doc = "0x08 - PLLDIG_MDIV_NDIV"]
    #[inline(always)]
    pub const fn plldig_mdiv_ndiv(&self) -> &PlldigMdivNdiv {
        &self.plldig_mdiv_ndiv
    }
    #[doc = "0x0c - PLLDIG_CTRL"]
    #[inline(always)]
    pub const fn plldig_ctrl(&self) -> &PlldigCtrl {
        &self.plldig_ctrl
    }
    #[doc = "0x10 - PLLDIG_MODE_EN"]
    #[inline(always)]
    pub const fn plldig_mode_en(&self) -> &PlldigModeEn {
        &self.plldig_mode_en
    }
    #[doc = "0x14 - PLLDIG_APLL_SW_DIS_DELAY1"]
    #[inline(always)]
    pub const fn plldig_apll_sw_dis_delay1(&self) -> &PlldigApllSwDisDelay1 {
        &self.plldig_apll_sw_dis_delay1
    }
    #[doc = "0x18 - PLLDIG_APLL_SW_DIS_DELAY2"]
    #[inline(always)]
    pub const fn plldig_apll_sw_dis_delay2(&self) -> &PlldigApllSwDisDelay2 {
        &self.plldig_apll_sw_dis_delay2
    }
    #[doc = "0x1c - PLLDIG_OVERRIDE"]
    #[inline(always)]
    pub const fn plldig_override(&self) -> &PlldigOverride {
        &self.plldig_override
    }
    #[doc = "0x20 - PLLDIG_STATUS"]
    #[inline(always)]
    pub const fn plldig_status(&self) -> &PlldigStatus {
        &self.plldig_status
    }
    #[doc = "0x24 - FAST_CLK_MUX_POSTDIV"]
    #[inline(always)]
    pub const fn fast_clk_mux_postdiv(&self) -> &FastClkMuxPostdiv {
        &self.fast_clk_mux_postdiv
    }
    #[doc = "0x28 - FAST_CLK_STATUS"]
    #[inline(always)]
    pub const fn fast_clk_status(&self) -> &FastClkStatus {
        &self.fast_clk_status
    }
    #[doc = "0x1008 - - KICK0 component"]
    #[inline(always)]
    pub const fn lock0_kick0(&self) -> &Lock0Kick0 {
        &self.lock0_kick0
    }
    #[doc = "0x100c - - KICK1 component"]
    #[inline(always)]
    pub const fn lock0_kick1(&self) -> &Lock0Kick1 {
        &self.lock0_kick1
    }
    #[doc = "0x1010 - Interrupt Raw Status/Set Register"]
    #[inline(always)]
    pub const fn intr_raw_status(&self) -> &IntrRawStatus {
        &self.intr_raw_status
    }
    #[doc = "0x1014 - Interrupt Enabled Status/Clear register"]
    #[inline(always)]
    pub const fn intr_enabled_status_clear(&self) -> &IntrEnabledStatusClear {
        &self.intr_enabled_status_clear
    }
    #[doc = "0x1018 - Interrupt Enable register"]
    #[inline(always)]
    pub const fn intr_enable(&self) -> &IntrEnable {
        &self.intr_enable
    }
    #[doc = "0x101c - Interrupt Enable Clear register"]
    #[inline(always)]
    pub const fn intr_enable_clear(&self) -> &IntrEnableClear {
        &self.intr_enable_clear
    }
    #[doc = "0x1020 - EOI register"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
    #[doc = "0x1024 - Fault Address register"]
    #[inline(always)]
    pub const fn fault_address(&self) -> &FaultAddress {
        &self.fault_address
    }
    #[doc = "0x1028 - Fault Type Status register"]
    #[inline(always)]
    pub const fn fault_type_status(&self) -> &FaultTypeStatus {
        &self.fault_type_status
    }
    #[doc = "0x102c - Fault Attribute Status register"]
    #[inline(always)]
    pub const fn fault_attr_status(&self) -> &FaultAttrStatus {
        &self.fault_attr_status
    }
    #[doc = "0x1030 - Fault Clear register"]
    #[inline(always)]
    pub const fn fault_clear(&self) -> &FaultClear {
        &self.fault_clear
    }
}
#[doc = "PID (rw) register accessor: PID register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "PID register"]
pub mod pid;
#[doc = "PLLDIG_EN (rw) register accessor: PLLDIG_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldig_en`]
module"]
#[doc(alias = "PLLDIG_EN")]
pub type PlldigEn = crate::Reg<plldig_en::PlldigEnSpec>;
#[doc = "PLLDIG_EN"]
pub mod plldig_en;
#[doc = "PLLDIG_MDIV_NDIV (rw) register accessor: PLLDIG_MDIV_NDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_mdiv_ndiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_mdiv_ndiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldig_mdiv_ndiv`]
module"]
#[doc(alias = "PLLDIG_MDIV_NDIV")]
pub type PlldigMdivNdiv = crate::Reg<plldig_mdiv_ndiv::PlldigMdivNdivSpec>;
#[doc = "PLLDIG_MDIV_NDIV"]
pub mod plldig_mdiv_ndiv;
#[doc = "PLLDIG_CTRL (rw) register accessor: PLLDIG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldig_ctrl`]
module"]
#[doc(alias = "PLLDIG_CTRL")]
pub type PlldigCtrl = crate::Reg<plldig_ctrl::PlldigCtrlSpec>;
#[doc = "PLLDIG_CTRL"]
pub mod plldig_ctrl;
#[doc = "PLLDIG_MODE_EN (rw) register accessor: PLLDIG_MODE_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_mode_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_mode_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldig_mode_en`]
module"]
#[doc(alias = "PLLDIG_MODE_EN")]
pub type PlldigModeEn = crate::Reg<plldig_mode_en::PlldigModeEnSpec>;
#[doc = "PLLDIG_MODE_EN"]
pub mod plldig_mode_en;
#[doc = "PLLDIG_APLL_SW_DIS_DELAY1 (rw) register accessor: PLLDIG_APLL_SW_DIS_DELAY1\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_apll_sw_dis_delay1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_apll_sw_dis_delay1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldig_apll_sw_dis_delay1`]
module"]
#[doc(alias = "PLLDIG_APLL_SW_DIS_DELAY1")]
pub type PlldigApllSwDisDelay1 = crate::Reg<plldig_apll_sw_dis_delay1::PlldigApllSwDisDelay1Spec>;
#[doc = "PLLDIG_APLL_SW_DIS_DELAY1"]
pub mod plldig_apll_sw_dis_delay1;
#[doc = "PLLDIG_APLL_SW_DIS_DELAY2 (rw) register accessor: PLLDIG_APLL_SW_DIS_DELAY2\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_apll_sw_dis_delay2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_apll_sw_dis_delay2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldig_apll_sw_dis_delay2`]
module"]
#[doc(alias = "PLLDIG_APLL_SW_DIS_DELAY2")]
pub type PlldigApllSwDisDelay2 = crate::Reg<plldig_apll_sw_dis_delay2::PlldigApllSwDisDelay2Spec>;
#[doc = "PLLDIG_APLL_SW_DIS_DELAY2"]
pub mod plldig_apll_sw_dis_delay2;
#[doc = "PLLDIG_OVERRIDE (rw) register accessor: PLLDIG_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldig_override`]
module"]
#[doc(alias = "PLLDIG_OVERRIDE")]
pub type PlldigOverride = crate::Reg<plldig_override::PlldigOverrideSpec>;
#[doc = "PLLDIG_OVERRIDE"]
pub mod plldig_override;
#[doc = "PLLDIG_STATUS (rw) register accessor: PLLDIG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldig_status`]
module"]
#[doc(alias = "PLLDIG_STATUS")]
pub type PlldigStatus = crate::Reg<plldig_status::PlldigStatusSpec>;
#[doc = "PLLDIG_STATUS"]
pub mod plldig_status;
#[doc = "FAST_CLK_MUX_POSTDIV (rw) register accessor: FAST_CLK_MUX_POSTDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_clk_mux_postdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_clk_mux_postdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fast_clk_mux_postdiv`]
module"]
#[doc(alias = "FAST_CLK_MUX_POSTDIV")]
pub type FastClkMuxPostdiv = crate::Reg<fast_clk_mux_postdiv::FastClkMuxPostdivSpec>;
#[doc = "FAST_CLK_MUX_POSTDIV"]
pub mod fast_clk_mux_postdiv;
#[doc = "FAST_CLK_STATUS (rw) register accessor: FAST_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fast_clk_status`]
module"]
#[doc(alias = "FAST_CLK_STATUS")]
pub type FastClkStatus = crate::Reg<fast_clk_status::FastClkStatusSpec>;
#[doc = "FAST_CLK_STATUS"]
pub mod fast_clk_status;
#[doc = "LOCK0_KICK0 (rw) register accessor: - KICK0 component\n\nYou can [`read`](crate::Reg::read) this register and get [`lock0_kick0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock0_kick0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock0_kick0`]
module"]
#[doc(alias = "LOCK0_KICK0")]
pub type Lock0Kick0 = crate::Reg<lock0_kick0::Lock0Kick0Spec>;
#[doc = "- KICK0 component"]
pub mod lock0_kick0;
#[doc = "LOCK0_KICK1 (rw) register accessor: - KICK1 component\n\nYou can [`read`](crate::Reg::read) this register and get [`lock0_kick1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock0_kick1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock0_kick1`]
module"]
#[doc(alias = "LOCK0_KICK1")]
pub type Lock0Kick1 = crate::Reg<lock0_kick1::Lock0Kick1Spec>;
#[doc = "- KICK1 component"]
pub mod lock0_kick1;
#[doc = "intr_raw_status (rw) register accessor: Interrupt Raw Status/Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_raw_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_raw_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_raw_status`]
module"]
#[doc(alias = "intr_raw_status")]
pub type IntrRawStatus = crate::Reg<intr_raw_status::IntrRawStatusSpec>;
#[doc = "Interrupt Raw Status/Set Register"]
pub mod intr_raw_status;
#[doc = "intr_enabled_status_clear (rw) register accessor: Interrupt Enabled Status/Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enabled_status_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enabled_status_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enabled_status_clear`]
module"]
#[doc(alias = "intr_enabled_status_clear")]
pub type IntrEnabledStatusClear = crate::Reg<intr_enabled_status_clear::IntrEnabledStatusClearSpec>;
#[doc = "Interrupt Enabled Status/Clear register"]
pub mod intr_enabled_status_clear;
#[doc = "intr_enable (rw) register accessor: Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enable`]
module"]
#[doc(alias = "intr_enable")]
pub type IntrEnable = crate::Reg<intr_enable::IntrEnableSpec>;
#[doc = "Interrupt Enable register"]
pub mod intr_enable;
#[doc = "intr_enable_clear (rw) register accessor: Interrupt Enable Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enable_clear`]
module"]
#[doc(alias = "intr_enable_clear")]
pub type IntrEnableClear = crate::Reg<intr_enable_clear::IntrEnableClearSpec>;
#[doc = "Interrupt Enable Clear register"]
pub mod intr_enable_clear;
#[doc = "eoi (rw) register accessor: EOI register\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`]
module"]
#[doc(alias = "eoi")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "EOI register"]
pub mod eoi;
#[doc = "fault_address (rw) register accessor: Fault Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_address`]
module"]
#[doc(alias = "fault_address")]
pub type FaultAddress = crate::Reg<fault_address::FaultAddressSpec>;
#[doc = "Fault Address register"]
pub mod fault_address;
#[doc = "fault_type_status (rw) register accessor: Fault Type Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_type_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_type_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_type_status`]
module"]
#[doc(alias = "fault_type_status")]
pub type FaultTypeStatus = crate::Reg<fault_type_status::FaultTypeStatusSpec>;
#[doc = "Fault Type Status register"]
pub mod fault_type_status;
#[doc = "fault_attr_status (rw) register accessor: Fault Attribute Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_attr_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_attr_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_attr_status`]
module"]
#[doc(alias = "fault_attr_status")]
pub type FaultAttrStatus = crate::Reg<fault_attr_status::FaultAttrStatusSpec>;
#[doc = "Fault Attribute Status register"]
pub mod fault_attr_status;
#[doc = "fault_clear (rw) register accessor: Fault Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_clear`]
module"]
#[doc(alias = "fault_clear")]
pub type FaultClear = crate::Reg<fault_clear::FaultClearSpec>;
#[doc = "Fault Clear register"]
pub mod fault_clear;
