#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    xtal_freq: XtalFreq,
    sop_mode: SopMode,
    rs232_bitinterval_0_1: Rs232Bitinterval0_1,
    rs232_bitinterval_2_3: Rs232Bitinterval2_3,
    dig_sync_select: DigSyncSelect,
    limp_mode_gen_en: LimpModeGenEn,
    cti_intr_mux_sel: CtiIntrMuxSel,
    secap_tx_data: SecapTxData,
    secap_tx_control: SecapTxControl,
    secap_rx_data: SecapRxData,
    secap_rx_control: SecapRxControl,
    dft_proc_dmled_exec: DftProcDmledExec,
    dft_proc_dmled_status: DftProcDmledStatus,
    dft_config_reg: DftConfigReg,
    dft_pbist_st_key: DftPbistStKey,
    dft_pbist_st_rst: DftPbistStRst,
    top_intmask: TopIntmask,
    debug_status_aon_1: DebugStatusAon1,
    debug_status_aon_2: DebugStatusAon2,
    debug_status_aon_3: DebugStatusAon3,
    debug_status_aon_4: DebugStatusAon4,
    debug_status_aon_5: DebugStatusAon5,
    debug_status_aon_6: DebugStatusAon6,
    debug_status_aon_7: DebugStatusAon7,
    debug_status_aon_8: DebugStatusAon8,
    debug_status_aon_9: DebugStatusAon9,
    debug_status_aon_10: DebugStatusAon10,
    debug_status_aon_11: DebugStatusAon11,
    debug_status_aon_12: DebugStatusAon12,
    debug_status_aon_13: DebugStatusAon13,
    debug_status_aon_14: DebugStatusAon14,
    debug_status_aon_15: DebugStatusAon15,
    debug_status_aon_16: DebugStatusAon16,
    appss_dynamic_clk_gate_status: AppssDynamicClkGateStatus,
    _reserved35: [u8; 0x0f7c],
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
    #[doc = "0x04 - XTAL_FREQ"]
    #[inline(always)]
    pub const fn xtal_freq(&self) -> &XtalFreq {
        &self.xtal_freq
    }
    #[doc = "0x08 - SOP_MODE"]
    #[inline(always)]
    pub const fn sop_mode(&self) -> &SopMode {
        &self.sop_mode
    }
    #[doc = "0x0c - RS232_BITINTERVAL_0_1"]
    #[inline(always)]
    pub const fn rs232_bitinterval_0_1(&self) -> &Rs232Bitinterval0_1 {
        &self.rs232_bitinterval_0_1
    }
    #[doc = "0x10 - RS232_BITINTERVAL_2_3"]
    #[inline(always)]
    pub const fn rs232_bitinterval_2_3(&self) -> &Rs232Bitinterval2_3 {
        &self.rs232_bitinterval_2_3
    }
    #[doc = "0x14 - DIG_SYNC_SELECT"]
    #[inline(always)]
    pub const fn dig_sync_select(&self) -> &DigSyncSelect {
        &self.dig_sync_select
    }
    #[doc = "0x18 - LIMP_MODE_GEN_EN"]
    #[inline(always)]
    pub const fn limp_mode_gen_en(&self) -> &LimpModeGenEn {
        &self.limp_mode_gen_en
    }
    #[doc = "0x1c - CTI_INTR_MUX_SEL"]
    #[inline(always)]
    pub const fn cti_intr_mux_sel(&self) -> &CtiIntrMuxSel {
        &self.cti_intr_mux_sel
    }
    #[doc = "0x20 - SECAP_TX_DATA"]
    #[inline(always)]
    pub const fn secap_tx_data(&self) -> &SecapTxData {
        &self.secap_tx_data
    }
    #[doc = "0x24 - SECAP_TX_CONTROL"]
    #[inline(always)]
    pub const fn secap_tx_control(&self) -> &SecapTxControl {
        &self.secap_tx_control
    }
    #[doc = "0x28 - SECAP_RX_DATA"]
    #[inline(always)]
    pub const fn secap_rx_data(&self) -> &SecapRxData {
        &self.secap_rx_data
    }
    #[doc = "0x2c - SECAP_RX_CONTROL"]
    #[inline(always)]
    pub const fn secap_rx_control(&self) -> &SecapRxControl {
        &self.secap_rx_control
    }
    #[doc = "0x30 - dft_proc_dmled_exec"]
    #[inline(always)]
    pub const fn dft_proc_dmled_exec(&self) -> &DftProcDmledExec {
        &self.dft_proc_dmled_exec
    }
    #[doc = "0x34 - dft_proc_dmled_status"]
    #[inline(always)]
    pub const fn dft_proc_dmled_status(&self) -> &DftProcDmledStatus {
        &self.dft_proc_dmled_status
    }
    #[doc = "0x38 - dft_config_reg"]
    #[inline(always)]
    pub const fn dft_config_reg(&self) -> &DftConfigReg {
        &self.dft_config_reg
    }
    #[doc = "0x3c - dft_pbist_st_key"]
    #[inline(always)]
    pub const fn dft_pbist_st_key(&self) -> &DftPbistStKey {
        &self.dft_pbist_st_key
    }
    #[doc = "0x40 - dft_pbist_st_rst"]
    #[inline(always)]
    pub const fn dft_pbist_st_rst(&self) -> &DftPbistStRst {
        &self.dft_pbist_st_rst
    }
    #[doc = "0x44 - TOP_INTMASK"]
    #[inline(always)]
    pub const fn top_intmask(&self) -> &TopIntmask {
        &self.top_intmask
    }
    #[doc = "0x48 - DEBUG_STATUS_AON_1"]
    #[inline(always)]
    pub const fn debug_status_aon_1(&self) -> &DebugStatusAon1 {
        &self.debug_status_aon_1
    }
    #[doc = "0x4c - DEBUG_STATUS_AON_2"]
    #[inline(always)]
    pub const fn debug_status_aon_2(&self) -> &DebugStatusAon2 {
        &self.debug_status_aon_2
    }
    #[doc = "0x50 - DEBUG_STATUS_AON_3"]
    #[inline(always)]
    pub const fn debug_status_aon_3(&self) -> &DebugStatusAon3 {
        &self.debug_status_aon_3
    }
    #[doc = "0x54 - DEBUG_STATUS_AON_4"]
    #[inline(always)]
    pub const fn debug_status_aon_4(&self) -> &DebugStatusAon4 {
        &self.debug_status_aon_4
    }
    #[doc = "0x58 - DEBUG_STATUS_AON_5"]
    #[inline(always)]
    pub const fn debug_status_aon_5(&self) -> &DebugStatusAon5 {
        &self.debug_status_aon_5
    }
    #[doc = "0x5c - DEBUG_STATUS_AON_6"]
    #[inline(always)]
    pub const fn debug_status_aon_6(&self) -> &DebugStatusAon6 {
        &self.debug_status_aon_6
    }
    #[doc = "0x60 - DEBUG_STATUS_AON_7"]
    #[inline(always)]
    pub const fn debug_status_aon_7(&self) -> &DebugStatusAon7 {
        &self.debug_status_aon_7
    }
    #[doc = "0x64 - DEBUG_STATUS_AON_8"]
    #[inline(always)]
    pub const fn debug_status_aon_8(&self) -> &DebugStatusAon8 {
        &self.debug_status_aon_8
    }
    #[doc = "0x68 - DEBUG_STATUS_AON_9"]
    #[inline(always)]
    pub const fn debug_status_aon_9(&self) -> &DebugStatusAon9 {
        &self.debug_status_aon_9
    }
    #[doc = "0x6c - DEBUG_STATUS_AON_10"]
    #[inline(always)]
    pub const fn debug_status_aon_10(&self) -> &DebugStatusAon10 {
        &self.debug_status_aon_10
    }
    #[doc = "0x70 - DEBUG_STATUS_AON_11"]
    #[inline(always)]
    pub const fn debug_status_aon_11(&self) -> &DebugStatusAon11 {
        &self.debug_status_aon_11
    }
    #[doc = "0x74 - DEBUG_STATUS_AON_12"]
    #[inline(always)]
    pub const fn debug_status_aon_12(&self) -> &DebugStatusAon12 {
        &self.debug_status_aon_12
    }
    #[doc = "0x78 - DEBUG_STATUS_AON_13"]
    #[inline(always)]
    pub const fn debug_status_aon_13(&self) -> &DebugStatusAon13 {
        &self.debug_status_aon_13
    }
    #[doc = "0x7c - DEBUG_STATUS_AON_14"]
    #[inline(always)]
    pub const fn debug_status_aon_14(&self) -> &DebugStatusAon14 {
        &self.debug_status_aon_14
    }
    #[doc = "0x80 - DEBUG_STATUS_AON_15"]
    #[inline(always)]
    pub const fn debug_status_aon_15(&self) -> &DebugStatusAon15 {
        &self.debug_status_aon_15
    }
    #[doc = "0x84 - DEBUG_STATUS_AON_16"]
    #[inline(always)]
    pub const fn debug_status_aon_16(&self) -> &DebugStatusAon16 {
        &self.debug_status_aon_16
    }
    #[doc = "0x88 - APPSS_DYNAMIC_CLK_GATE_STATUS"]
    #[inline(always)]
    pub const fn appss_dynamic_clk_gate_status(&self) -> &AppssDynamicClkGateStatus {
        &self.appss_dynamic_clk_gate_status
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
#[doc = "XTAL_FREQ (rw) register accessor: XTAL_FREQ\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_freq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_freq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_freq`]
module"]
#[doc(alias = "XTAL_FREQ")]
pub type XtalFreq = crate::Reg<xtal_freq::XtalFreqSpec>;
#[doc = "XTAL_FREQ"]
pub mod xtal_freq;
#[doc = "SOP_MODE (rw) register accessor: SOP_MODE\n\nYou can [`read`](crate::Reg::read) this register and get [`sop_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sop_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sop_mode`]
module"]
#[doc(alias = "SOP_MODE")]
pub type SopMode = crate::Reg<sop_mode::SopModeSpec>;
#[doc = "SOP_MODE"]
pub mod sop_mode;
#[doc = "RS232_BITINTERVAL_0_1 (rw) register accessor: RS232_BITINTERVAL_0_1\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_bitinterval_0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_bitinterval_0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs232_bitinterval_0_1`]
module"]
#[doc(alias = "RS232_BITINTERVAL_0_1")]
pub type Rs232Bitinterval0_1 = crate::Reg<rs232_bitinterval_0_1::Rs232Bitinterval0_1Spec>;
#[doc = "RS232_BITINTERVAL_0_1"]
pub mod rs232_bitinterval_0_1;
#[doc = "RS232_BITINTERVAL_2_3 (rw) register accessor: RS232_BITINTERVAL_2_3\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_bitinterval_2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_bitinterval_2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs232_bitinterval_2_3`]
module"]
#[doc(alias = "RS232_BITINTERVAL_2_3")]
pub type Rs232Bitinterval2_3 = crate::Reg<rs232_bitinterval_2_3::Rs232Bitinterval2_3Spec>;
#[doc = "RS232_BITINTERVAL_2_3"]
pub mod rs232_bitinterval_2_3;
#[doc = "DIG_SYNC_SELECT (rw) register accessor: DIG_SYNC_SELECT\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_sync_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_sync_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_sync_select`]
module"]
#[doc(alias = "DIG_SYNC_SELECT")]
pub type DigSyncSelect = crate::Reg<dig_sync_select::DigSyncSelectSpec>;
#[doc = "DIG_SYNC_SELECT"]
pub mod dig_sync_select;
#[doc = "LIMP_MODE_GEN_EN (rw) register accessor: LIMP_MODE_GEN_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`limp_mode_gen_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limp_mode_gen_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limp_mode_gen_en`]
module"]
#[doc(alias = "LIMP_MODE_GEN_EN")]
pub type LimpModeGenEn = crate::Reg<limp_mode_gen_en::LimpModeGenEnSpec>;
#[doc = "LIMP_MODE_GEN_EN"]
pub mod limp_mode_gen_en;
#[doc = "CTI_INTR_MUX_SEL (rw) register accessor: CTI_INTR_MUX_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`cti_intr_mux_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cti_intr_mux_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cti_intr_mux_sel`]
module"]
#[doc(alias = "CTI_INTR_MUX_SEL")]
pub type CtiIntrMuxSel = crate::Reg<cti_intr_mux_sel::CtiIntrMuxSelSpec>;
#[doc = "CTI_INTR_MUX_SEL"]
pub mod cti_intr_mux_sel;
#[doc = "SECAP_TX_DATA (rw) register accessor: SECAP_TX_DATA\n\nYou can [`read`](crate::Reg::read) this register and get [`secap_tx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secap_tx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secap_tx_data`]
module"]
#[doc(alias = "SECAP_TX_DATA")]
pub type SecapTxData = crate::Reg<secap_tx_data::SecapTxDataSpec>;
#[doc = "SECAP_TX_DATA"]
pub mod secap_tx_data;
#[doc = "SECAP_TX_CONTROL (rw) register accessor: SECAP_TX_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`secap_tx_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secap_tx_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secap_tx_control`]
module"]
#[doc(alias = "SECAP_TX_CONTROL")]
pub type SecapTxControl = crate::Reg<secap_tx_control::SecapTxControlSpec>;
#[doc = "SECAP_TX_CONTROL"]
pub mod secap_tx_control;
#[doc = "SECAP_RX_DATA (rw) register accessor: SECAP_RX_DATA\n\nYou can [`read`](crate::Reg::read) this register and get [`secap_rx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secap_rx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secap_rx_data`]
module"]
#[doc(alias = "SECAP_RX_DATA")]
pub type SecapRxData = crate::Reg<secap_rx_data::SecapRxDataSpec>;
#[doc = "SECAP_RX_DATA"]
pub mod secap_rx_data;
#[doc = "SECAP_RX_CONTROL (rw) register accessor: SECAP_RX_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`secap_rx_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secap_rx_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secap_rx_control`]
module"]
#[doc(alias = "SECAP_RX_CONTROL")]
pub type SecapRxControl = crate::Reg<secap_rx_control::SecapRxControlSpec>;
#[doc = "SECAP_RX_CONTROL"]
pub mod secap_rx_control;
#[doc = "dft_proc_dmled_exec (rw) register accessor: dft_proc_dmled_exec\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_proc_dmled_exec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_proc_dmled_exec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_proc_dmled_exec`]
module"]
#[doc(alias = "dft_proc_dmled_exec")]
pub type DftProcDmledExec = crate::Reg<dft_proc_dmled_exec::DftProcDmledExecSpec>;
#[doc = "dft_proc_dmled_exec"]
pub mod dft_proc_dmled_exec;
#[doc = "dft_proc_dmled_status (rw) register accessor: dft_proc_dmled_status\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_proc_dmled_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_proc_dmled_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_proc_dmled_status`]
module"]
#[doc(alias = "dft_proc_dmled_status")]
pub type DftProcDmledStatus = crate::Reg<dft_proc_dmled_status::DftProcDmledStatusSpec>;
#[doc = "dft_proc_dmled_status"]
pub mod dft_proc_dmled_status;
#[doc = "dft_config_reg (rw) register accessor: dft_config_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_config_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_config_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_config_reg`]
module"]
#[doc(alias = "dft_config_reg")]
pub type DftConfigReg = crate::Reg<dft_config_reg::DftConfigRegSpec>;
#[doc = "dft_config_reg"]
pub mod dft_config_reg;
#[doc = "dft_pbist_st_key (rw) register accessor: dft_pbist_st_key\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_pbist_st_key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_pbist_st_key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_pbist_st_key`]
module"]
#[doc(alias = "dft_pbist_st_key")]
pub type DftPbistStKey = crate::Reg<dft_pbist_st_key::DftPbistStKeySpec>;
#[doc = "dft_pbist_st_key"]
pub mod dft_pbist_st_key;
#[doc = "dft_pbist_st_rst (rw) register accessor: dft_pbist_st_rst\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_pbist_st_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_pbist_st_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_pbist_st_rst`]
module"]
#[doc(alias = "dft_pbist_st_rst")]
pub type DftPbistStRst = crate::Reg<dft_pbist_st_rst::DftPbistStRstSpec>;
#[doc = "dft_pbist_st_rst"]
pub mod dft_pbist_st_rst;
#[doc = "TOP_INTMASK (rw) register accessor: TOP_INTMASK\n\nYou can [`read`](crate::Reg::read) this register and get [`top_intmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top_intmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top_intmask`]
module"]
#[doc(alias = "TOP_INTMASK")]
pub type TopIntmask = crate::Reg<top_intmask::TopIntmaskSpec>;
#[doc = "TOP_INTMASK"]
pub mod top_intmask;
#[doc = "DEBUG_STATUS_AON_1 (rw) register accessor: DEBUG_STATUS_AON_1\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_1`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_1")]
pub type DebugStatusAon1 = crate::Reg<debug_status_aon_1::DebugStatusAon1Spec>;
#[doc = "DEBUG_STATUS_AON_1"]
pub mod debug_status_aon_1;
#[doc = "DEBUG_STATUS_AON_2 (rw) register accessor: DEBUG_STATUS_AON_2\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_2`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_2")]
pub type DebugStatusAon2 = crate::Reg<debug_status_aon_2::DebugStatusAon2Spec>;
#[doc = "DEBUG_STATUS_AON_2"]
pub mod debug_status_aon_2;
#[doc = "DEBUG_STATUS_AON_3 (rw) register accessor: DEBUG_STATUS_AON_3\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_3`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_3")]
pub type DebugStatusAon3 = crate::Reg<debug_status_aon_3::DebugStatusAon3Spec>;
#[doc = "DEBUG_STATUS_AON_3"]
pub mod debug_status_aon_3;
#[doc = "DEBUG_STATUS_AON_4 (rw) register accessor: DEBUG_STATUS_AON_4\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_4`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_4")]
pub type DebugStatusAon4 = crate::Reg<debug_status_aon_4::DebugStatusAon4Spec>;
#[doc = "DEBUG_STATUS_AON_4"]
pub mod debug_status_aon_4;
#[doc = "DEBUG_STATUS_AON_5 (rw) register accessor: DEBUG_STATUS_AON_5\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_5`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_5")]
pub type DebugStatusAon5 = crate::Reg<debug_status_aon_5::DebugStatusAon5Spec>;
#[doc = "DEBUG_STATUS_AON_5"]
pub mod debug_status_aon_5;
#[doc = "DEBUG_STATUS_AON_6 (rw) register accessor: DEBUG_STATUS_AON_6\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_6`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_6")]
pub type DebugStatusAon6 = crate::Reg<debug_status_aon_6::DebugStatusAon6Spec>;
#[doc = "DEBUG_STATUS_AON_6"]
pub mod debug_status_aon_6;
#[doc = "DEBUG_STATUS_AON_7 (rw) register accessor: DEBUG_STATUS_AON_7\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_7`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_7")]
pub type DebugStatusAon7 = crate::Reg<debug_status_aon_7::DebugStatusAon7Spec>;
#[doc = "DEBUG_STATUS_AON_7"]
pub mod debug_status_aon_7;
#[doc = "DEBUG_STATUS_AON_8 (rw) register accessor: DEBUG_STATUS_AON_8\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_8`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_8")]
pub type DebugStatusAon8 = crate::Reg<debug_status_aon_8::DebugStatusAon8Spec>;
#[doc = "DEBUG_STATUS_AON_8"]
pub mod debug_status_aon_8;
#[doc = "DEBUG_STATUS_AON_9 (rw) register accessor: DEBUG_STATUS_AON_9\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_9`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_9")]
pub type DebugStatusAon9 = crate::Reg<debug_status_aon_9::DebugStatusAon9Spec>;
#[doc = "DEBUG_STATUS_AON_9"]
pub mod debug_status_aon_9;
#[doc = "DEBUG_STATUS_AON_10 (rw) register accessor: DEBUG_STATUS_AON_10\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_10`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_10")]
pub type DebugStatusAon10 = crate::Reg<debug_status_aon_10::DebugStatusAon10Spec>;
#[doc = "DEBUG_STATUS_AON_10"]
pub mod debug_status_aon_10;
#[doc = "DEBUG_STATUS_AON_11 (rw) register accessor: DEBUG_STATUS_AON_11\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_11`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_11")]
pub type DebugStatusAon11 = crate::Reg<debug_status_aon_11::DebugStatusAon11Spec>;
#[doc = "DEBUG_STATUS_AON_11"]
pub mod debug_status_aon_11;
#[doc = "DEBUG_STATUS_AON_12 (rw) register accessor: DEBUG_STATUS_AON_12\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_12`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_12")]
pub type DebugStatusAon12 = crate::Reg<debug_status_aon_12::DebugStatusAon12Spec>;
#[doc = "DEBUG_STATUS_AON_12"]
pub mod debug_status_aon_12;
#[doc = "DEBUG_STATUS_AON_13 (rw) register accessor: DEBUG_STATUS_AON_13\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_13`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_13")]
pub type DebugStatusAon13 = crate::Reg<debug_status_aon_13::DebugStatusAon13Spec>;
#[doc = "DEBUG_STATUS_AON_13"]
pub mod debug_status_aon_13;
#[doc = "DEBUG_STATUS_AON_14 (rw) register accessor: DEBUG_STATUS_AON_14\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_14`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_14")]
pub type DebugStatusAon14 = crate::Reg<debug_status_aon_14::DebugStatusAon14Spec>;
#[doc = "DEBUG_STATUS_AON_14"]
pub mod debug_status_aon_14;
#[doc = "DEBUG_STATUS_AON_15 (rw) register accessor: DEBUG_STATUS_AON_15\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_15`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_15")]
pub type DebugStatusAon15 = crate::Reg<debug_status_aon_15::DebugStatusAon15Spec>;
#[doc = "DEBUG_STATUS_AON_15"]
pub mod debug_status_aon_15;
#[doc = "DEBUG_STATUS_AON_16 (rw) register accessor: DEBUG_STATUS_AON_16\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_status_aon_16`]
module"]
#[doc(alias = "DEBUG_STATUS_AON_16")]
pub type DebugStatusAon16 = crate::Reg<debug_status_aon_16::DebugStatusAon16Spec>;
#[doc = "DEBUG_STATUS_AON_16"]
pub mod debug_status_aon_16;
#[doc = "APPSS_DYNAMIC_CLK_GATE_STATUS (rw) register accessor: APPSS_DYNAMIC_CLK_GATE_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_dynamic_clk_gate_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_dynamic_clk_gate_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_dynamic_clk_gate_status`]
module"]
#[doc(alias = "APPSS_DYNAMIC_CLK_GATE_STATUS")]
pub type AppssDynamicClkGateStatus =
    crate::Reg<appss_dynamic_clk_gate_status::AppssDynamicClkGateStatusSpec>;
#[doc = "APPSS_DYNAMIC_CLK_GATE_STATUS"]
pub mod appss_dynamic_clk_gate_status;
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
