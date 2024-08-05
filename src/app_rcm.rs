#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    app_cpu_clkctl: AppCpuClkctl,
    app_cpu_clkstat: AppCpuClkstat,
    app_can_clkctl: AppCanClkctl,
    app_can_clkstat: AppCanClkstat,
    app_spi_clkctl: AppSpiClkctl,
    app_spi_clkstat: AppSpiClkstat,
    app_spi_busif_clkctl: AppSpiBusifClkctl,
    app_spi_busif_clkstat: AppSpiBusifClkstat,
    app_qspi_clkctl: AppQspiClkctl,
    app_qspi_clkstat: AppQspiClkstat,
    topss_clkctl: TopssClkctl,
    topss_clkstat: TopssClkstat,
    app_rti_clkctl: AppRtiClkctl,
    app_rti_clkstat: AppRtiClkstat,
    app_wd_clkctl: AppWdClkctl,
    app_wd_clkstat: AppWdClkstat,
    app_uart_0_clkctl: AppUart0Clkctl,
    app_uart_0_clkstat: AppUart0Clkstat,
    app_uart_1_clkctl: AppUart1Clkctl,
    app_uart_1_clkstat: AppUart1Clkstat,
    app_i2c_clkctl: AppI2cClkctl,
    app_i2c_clkstat: AppI2cClkstat,
    app_lin_clkctl: AppLinClkctl,
    app_lin_clkstat: AppLinClkstat,
    reserved0: Reserved0,
    reserved1: Reserved1,
    reserved2: Reserved2,
    reserved3: Reserved3,
    ipcfgclkgate0: Ipcfgclkgate0,
    ipcfgclkgate1: Ipcfgclkgate1,
    ipcfgclkgate2: Ipcfgclkgate2,
    blockreset0: Blockreset0,
    blockreset1: Blockreset1,
    blockreset2: Blockreset2,
    platform_signature: PlatformSignature,
    powermode: Powermode,
    rst_wficheck: RstWficheck,
    rst_assertdly: RstAssertdly,
    rst2assertdly: Rst2assertdly,
    rst_fsm_trig: RstFsmTrig,
    rst_cause: RstCause,
    rst_cause_clr: RstCauseClr,
    xtalclk_clk_gate: XtalclkClkGate,
    xtalclkx2_clk_gate: Xtalclkx2ClkGate,
    aplldiv2_clk_gate: Aplldiv2ClkGate,
    dft_appss_lstc_clk_gate: DftAppssLstcClkGate,
    dft_appss_lstc_vbusp_clk_gate: DftAppssLstcVbuspClkGate,
    app_rom_clock_gate: AppRomClockGate,
    app_ram1_clock_gate: AppRam1ClockGate,
    app_ram2_clock_gate: AppRam2ClockGate,
    app_ram3_clock_gate: AppRam3ClockGate,
    cfg_xbara_dynamic_cg: CfgXbaraDynamicCg,
    cfg_tptc1_dynamic_cg: CfgTptc1DynamicCg,
    cfg_tptc2_dynamic_cg: CfgTptc2DynamicCg,
    cfg_xbara_set_dynamic_cg: CfgXbaraSetDynamicCg,
    cfg_tptc1_set_dynamic_cg: CfgTptc1SetDynamicCg,
    cfg_tptc2_set_dynamic_cg: CfgTptc2SetDynamicCg,
    cm4_force_hclk_gate: Cm4ForceHclkGate,
    lin_sci_div: LinSciDiv,
    app_lstc_en: AppLstcEn,
    _reserved61: [u8; 0x0f14],
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
    #[doc = "0x04 - APP_CPU_CLKCTL"]
    #[inline(always)]
    pub const fn app_cpu_clkctl(&self) -> &AppCpuClkctl {
        &self.app_cpu_clkctl
    }
    #[doc = "0x08 - APP_CPU_CLKSTAT"]
    #[inline(always)]
    pub const fn app_cpu_clkstat(&self) -> &AppCpuClkstat {
        &self.app_cpu_clkstat
    }
    #[doc = "0x0c - APP_CAN_CLKCTL"]
    #[inline(always)]
    pub const fn app_can_clkctl(&self) -> &AppCanClkctl {
        &self.app_can_clkctl
    }
    #[doc = "0x10 - APP_CAN_CLKSTAT"]
    #[inline(always)]
    pub const fn app_can_clkstat(&self) -> &AppCanClkstat {
        &self.app_can_clkstat
    }
    #[doc = "0x14 - APP_SPI_CLKCTL"]
    #[inline(always)]
    pub const fn app_spi_clkctl(&self) -> &AppSpiClkctl {
        &self.app_spi_clkctl
    }
    #[doc = "0x18 - APP_SPI_CLKSTAT"]
    #[inline(always)]
    pub const fn app_spi_clkstat(&self) -> &AppSpiClkstat {
        &self.app_spi_clkstat
    }
    #[doc = "0x1c - APP_SPI_BUSIF_CLKCTL"]
    #[inline(always)]
    pub const fn app_spi_busif_clkctl(&self) -> &AppSpiBusifClkctl {
        &self.app_spi_busif_clkctl
    }
    #[doc = "0x20 - APP_SPI_BUSIF_CLKSTAT"]
    #[inline(always)]
    pub const fn app_spi_busif_clkstat(&self) -> &AppSpiBusifClkstat {
        &self.app_spi_busif_clkstat
    }
    #[doc = "0x24 - APP_QSPI_CLKCTL"]
    #[inline(always)]
    pub const fn app_qspi_clkctl(&self) -> &AppQspiClkctl {
        &self.app_qspi_clkctl
    }
    #[doc = "0x28 - APP_QSPI_CLKSTAT"]
    #[inline(always)]
    pub const fn app_qspi_clkstat(&self) -> &AppQspiClkstat {
        &self.app_qspi_clkstat
    }
    #[doc = "0x2c - TOPSS_CLKCTL"]
    #[inline(always)]
    pub const fn topss_clkctl(&self) -> &TopssClkctl {
        &self.topss_clkctl
    }
    #[doc = "0x30 - TOPSS_CLKSTAT"]
    #[inline(always)]
    pub const fn topss_clkstat(&self) -> &TopssClkstat {
        &self.topss_clkstat
    }
    #[doc = "0x34 - APP_RTI_CLKCTL"]
    #[inline(always)]
    pub const fn app_rti_clkctl(&self) -> &AppRtiClkctl {
        &self.app_rti_clkctl
    }
    #[doc = "0x38 - APP_RTI_CLKSTAT"]
    #[inline(always)]
    pub const fn app_rti_clkstat(&self) -> &AppRtiClkstat {
        &self.app_rti_clkstat
    }
    #[doc = "0x3c - APP_WD_CLKCTL"]
    #[inline(always)]
    pub const fn app_wd_clkctl(&self) -> &AppWdClkctl {
        &self.app_wd_clkctl
    }
    #[doc = "0x40 - APP_WD_CLKSTAT"]
    #[inline(always)]
    pub const fn app_wd_clkstat(&self) -> &AppWdClkstat {
        &self.app_wd_clkstat
    }
    #[doc = "0x44 - APP_UART_0_CLKCTL"]
    #[inline(always)]
    pub const fn app_uart_0_clkctl(&self) -> &AppUart0Clkctl {
        &self.app_uart_0_clkctl
    }
    #[doc = "0x48 - APP_UART_0_CLKSTAT"]
    #[inline(always)]
    pub const fn app_uart_0_clkstat(&self) -> &AppUart0Clkstat {
        &self.app_uart_0_clkstat
    }
    #[doc = "0x4c - APP_UART_1_CLKCTL"]
    #[inline(always)]
    pub const fn app_uart_1_clkctl(&self) -> &AppUart1Clkctl {
        &self.app_uart_1_clkctl
    }
    #[doc = "0x50 - APP_UART_1_CLKSTAT"]
    #[inline(always)]
    pub const fn app_uart_1_clkstat(&self) -> &AppUart1Clkstat {
        &self.app_uart_1_clkstat
    }
    #[doc = "0x54 - APP_I2C_CLKCTL"]
    #[inline(always)]
    pub const fn app_i2c_clkctl(&self) -> &AppI2cClkctl {
        &self.app_i2c_clkctl
    }
    #[doc = "0x58 - APP_I2C_CLKSTAT"]
    #[inline(always)]
    pub const fn app_i2c_clkstat(&self) -> &AppI2cClkstat {
        &self.app_i2c_clkstat
    }
    #[doc = "0x5c - APP_LIN_CLKCTL"]
    #[inline(always)]
    pub const fn app_lin_clkctl(&self) -> &AppLinClkctl {
        &self.app_lin_clkctl
    }
    #[doc = "0x60 - APP_LIN_CLKSTAT"]
    #[inline(always)]
    pub const fn app_lin_clkstat(&self) -> &AppLinClkstat {
        &self.app_lin_clkstat
    }
    #[doc = "0x64 - RESERVED0"]
    #[inline(always)]
    pub const fn reserved0(&self) -> &Reserved0 {
        &self.reserved0
    }
    #[doc = "0x68 - RESERVED1"]
    #[inline(always)]
    pub const fn reserved1(&self) -> &Reserved1 {
        &self.reserved1
    }
    #[doc = "0x6c - RESERVED2"]
    #[inline(always)]
    pub const fn reserved2(&self) -> &Reserved2 {
        &self.reserved2
    }
    #[doc = "0x70 - RESERVED3"]
    #[inline(always)]
    pub const fn reserved3(&self) -> &Reserved3 {
        &self.reserved3
    }
    #[doc = "0x74 - IPCFGCLKGATE0"]
    #[inline(always)]
    pub const fn ipcfgclkgate0(&self) -> &Ipcfgclkgate0 {
        &self.ipcfgclkgate0
    }
    #[doc = "0x78 - IPCFGCLKGATE1"]
    #[inline(always)]
    pub const fn ipcfgclkgate1(&self) -> &Ipcfgclkgate1 {
        &self.ipcfgclkgate1
    }
    #[doc = "0x7c - IPCFGCLKGATE2"]
    #[inline(always)]
    pub const fn ipcfgclkgate2(&self) -> &Ipcfgclkgate2 {
        &self.ipcfgclkgate2
    }
    #[doc = "0x80 - BLOCKRESET0"]
    #[inline(always)]
    pub const fn blockreset0(&self) -> &Blockreset0 {
        &self.blockreset0
    }
    #[doc = "0x84 - BLOCKRESET1"]
    #[inline(always)]
    pub const fn blockreset1(&self) -> &Blockreset1 {
        &self.blockreset1
    }
    #[doc = "0x88 - BLOCKRESET2"]
    #[inline(always)]
    pub const fn blockreset2(&self) -> &Blockreset2 {
        &self.blockreset2
    }
    #[doc = "0x8c - PLATFORM_SIGNATURE"]
    #[inline(always)]
    pub const fn platform_signature(&self) -> &PlatformSignature {
        &self.platform_signature
    }
    #[doc = "0x90 - POWERMODE"]
    #[inline(always)]
    pub const fn powermode(&self) -> &Powermode {
        &self.powermode
    }
    #[doc = "0x94 - RST_WFICHECK"]
    #[inline(always)]
    pub const fn rst_wficheck(&self) -> &RstWficheck {
        &self.rst_wficheck
    }
    #[doc = "0x98 - RST_ASSERTDLY"]
    #[inline(always)]
    pub const fn rst_assertdly(&self) -> &RstAssertdly {
        &self.rst_assertdly
    }
    #[doc = "0x9c - RST2ASSERTDLY"]
    #[inline(always)]
    pub const fn rst2assertdly(&self) -> &Rst2assertdly {
        &self.rst2assertdly
    }
    #[doc = "0xa0 - RST_FSM_TRIG"]
    #[inline(always)]
    pub const fn rst_fsm_trig(&self) -> &RstFsmTrig {
        &self.rst_fsm_trig
    }
    #[doc = "0xa4 - RST_CAUSE"]
    #[inline(always)]
    pub const fn rst_cause(&self) -> &RstCause {
        &self.rst_cause
    }
    #[doc = "0xa8 - RST_CAUSE_CLR"]
    #[inline(always)]
    pub const fn rst_cause_clr(&self) -> &RstCauseClr {
        &self.rst_cause_clr
    }
    #[doc = "0xac - XTALCLK_CLK_GATE"]
    #[inline(always)]
    pub const fn xtalclk_clk_gate(&self) -> &XtalclkClkGate {
        &self.xtalclk_clk_gate
    }
    #[doc = "0xb0 - XTALCLKX2_CLK_GATE"]
    #[inline(always)]
    pub const fn xtalclkx2_clk_gate(&self) -> &Xtalclkx2ClkGate {
        &self.xtalclkx2_clk_gate
    }
    #[doc = "0xb4 - APLLDIV2_CLK_GATE"]
    #[inline(always)]
    pub const fn aplldiv2_clk_gate(&self) -> &Aplldiv2ClkGate {
        &self.aplldiv2_clk_gate
    }
    #[doc = "0xb8 - DFT_APPSS_LSTC_CLK_GATE"]
    #[inline(always)]
    pub const fn dft_appss_lstc_clk_gate(&self) -> &DftAppssLstcClkGate {
        &self.dft_appss_lstc_clk_gate
    }
    #[doc = "0xbc - DFT_APPSS_LSTC_VBUSP_CLK_GATE"]
    #[inline(always)]
    pub const fn dft_appss_lstc_vbusp_clk_gate(&self) -> &DftAppssLstcVbuspClkGate {
        &self.dft_appss_lstc_vbusp_clk_gate
    }
    #[doc = "0xc0 - APP_ROM_CLOCK_GATE"]
    #[inline(always)]
    pub const fn app_rom_clock_gate(&self) -> &AppRomClockGate {
        &self.app_rom_clock_gate
    }
    #[doc = "0xc4 - APP_RAM1_CLOCK_GATE"]
    #[inline(always)]
    pub const fn app_ram1_clock_gate(&self) -> &AppRam1ClockGate {
        &self.app_ram1_clock_gate
    }
    #[doc = "0xc8 - APP_RAM2_CLOCK_GATE"]
    #[inline(always)]
    pub const fn app_ram2_clock_gate(&self) -> &AppRam2ClockGate {
        &self.app_ram2_clock_gate
    }
    #[doc = "0xcc - APP_RAM3_CLOCK_GATE"]
    #[inline(always)]
    pub const fn app_ram3_clock_gate(&self) -> &AppRam3ClockGate {
        &self.app_ram3_clock_gate
    }
    #[doc = "0xd0 - CFG_XBARA_DYNAMIC_CG"]
    #[inline(always)]
    pub const fn cfg_xbara_dynamic_cg(&self) -> &CfgXbaraDynamicCg {
        &self.cfg_xbara_dynamic_cg
    }
    #[doc = "0xd4 - CFG_TPTC1_DYNAMIC_CG"]
    #[inline(always)]
    pub const fn cfg_tptc1_dynamic_cg(&self) -> &CfgTptc1DynamicCg {
        &self.cfg_tptc1_dynamic_cg
    }
    #[doc = "0xd8 - CFG_TPTC2_DYNAMIC_CG"]
    #[inline(always)]
    pub const fn cfg_tptc2_dynamic_cg(&self) -> &CfgTptc2DynamicCg {
        &self.cfg_tptc2_dynamic_cg
    }
    #[doc = "0xdc - CFG_XBARA_SET_DYNAMIC_CG"]
    #[inline(always)]
    pub const fn cfg_xbara_set_dynamic_cg(&self) -> &CfgXbaraSetDynamicCg {
        &self.cfg_xbara_set_dynamic_cg
    }
    #[doc = "0xe0 - CFG_TPTC1_SET_DYNAMIC_CG"]
    #[inline(always)]
    pub const fn cfg_tptc1_set_dynamic_cg(&self) -> &CfgTptc1SetDynamicCg {
        &self.cfg_tptc1_set_dynamic_cg
    }
    #[doc = "0xe4 - CFG_TPTC2_SET_DYNAMIC_CG"]
    #[inline(always)]
    pub const fn cfg_tptc2_set_dynamic_cg(&self) -> &CfgTptc2SetDynamicCg {
        &self.cfg_tptc2_set_dynamic_cg
    }
    #[doc = "0xe8 - CM4_FORCE_HCLK_GATE"]
    #[inline(always)]
    pub const fn cm4_force_hclk_gate(&self) -> &Cm4ForceHclkGate {
        &self.cm4_force_hclk_gate
    }
    #[doc = "0xec - LIN_SCI_DIV"]
    #[inline(always)]
    pub const fn lin_sci_div(&self) -> &LinSciDiv {
        &self.lin_sci_div
    }
    #[doc = "0xf0 - APP_LSTC_EN"]
    #[inline(always)]
    pub const fn app_lstc_en(&self) -> &AppLstcEn {
        &self.app_lstc_en
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
#[doc = "APP_CPU_CLKCTL (rw) register accessor: APP_CPU_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cpu_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cpu_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_clkctl`]
module"]
#[doc(alias = "APP_CPU_CLKCTL")]
pub type AppCpuClkctl = crate::Reg<app_cpu_clkctl::AppCpuClkctlSpec>;
#[doc = "APP_CPU_CLKCTL"]
pub mod app_cpu_clkctl;
#[doc = "APP_CPU_CLKSTAT (rw) register accessor: APP_CPU_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cpu_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cpu_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cpu_clkstat`]
module"]
#[doc(alias = "APP_CPU_CLKSTAT")]
pub type AppCpuClkstat = crate::Reg<app_cpu_clkstat::AppCpuClkstatSpec>;
#[doc = "APP_CPU_CLKSTAT"]
pub mod app_cpu_clkstat;
#[doc = "APP_CAN_CLKCTL (rw) register accessor: APP_CAN_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_can_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_can_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_can_clkctl`]
module"]
#[doc(alias = "APP_CAN_CLKCTL")]
pub type AppCanClkctl = crate::Reg<app_can_clkctl::AppCanClkctlSpec>;
#[doc = "APP_CAN_CLKCTL"]
pub mod app_can_clkctl;
#[doc = "APP_CAN_CLKSTAT (rw) register accessor: APP_CAN_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_can_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_can_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_can_clkstat`]
module"]
#[doc(alias = "APP_CAN_CLKSTAT")]
pub type AppCanClkstat = crate::Reg<app_can_clkstat::AppCanClkstatSpec>;
#[doc = "APP_CAN_CLKSTAT"]
pub mod app_can_clkstat;
#[doc = "APP_SPI_CLKCTL (rw) register accessor: APP_SPI_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_spi_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_spi_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi_clkctl`]
module"]
#[doc(alias = "APP_SPI_CLKCTL")]
pub type AppSpiClkctl = crate::Reg<app_spi_clkctl::AppSpiClkctlSpec>;
#[doc = "APP_SPI_CLKCTL"]
pub mod app_spi_clkctl;
#[doc = "APP_SPI_CLKSTAT (rw) register accessor: APP_SPI_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_spi_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_spi_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi_clkstat`]
module"]
#[doc(alias = "APP_SPI_CLKSTAT")]
pub type AppSpiClkstat = crate::Reg<app_spi_clkstat::AppSpiClkstatSpec>;
#[doc = "APP_SPI_CLKSTAT"]
pub mod app_spi_clkstat;
#[doc = "APP_SPI_BUSIF_CLKCTL (rw) register accessor: APP_SPI_BUSIF_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_spi_busif_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_spi_busif_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi_busif_clkctl`]
module"]
#[doc(alias = "APP_SPI_BUSIF_CLKCTL")]
pub type AppSpiBusifClkctl = crate::Reg<app_spi_busif_clkctl::AppSpiBusifClkctlSpec>;
#[doc = "APP_SPI_BUSIF_CLKCTL"]
pub mod app_spi_busif_clkctl;
#[doc = "APP_SPI_BUSIF_CLKSTAT (rw) register accessor: APP_SPI_BUSIF_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_spi_busif_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_spi_busif_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_spi_busif_clkstat`]
module"]
#[doc(alias = "APP_SPI_BUSIF_CLKSTAT")]
pub type AppSpiBusifClkstat = crate::Reg<app_spi_busif_clkstat::AppSpiBusifClkstatSpec>;
#[doc = "APP_SPI_BUSIF_CLKSTAT"]
pub mod app_spi_busif_clkstat;
#[doc = "APP_QSPI_CLKCTL (rw) register accessor: APP_QSPI_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_qspi_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_qspi_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_qspi_clkctl`]
module"]
#[doc(alias = "APP_QSPI_CLKCTL")]
pub type AppQspiClkctl = crate::Reg<app_qspi_clkctl::AppQspiClkctlSpec>;
#[doc = "APP_QSPI_CLKCTL"]
pub mod app_qspi_clkctl;
#[doc = "APP_QSPI_CLKSTAT (rw) register accessor: APP_QSPI_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_qspi_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_qspi_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_qspi_clkstat`]
module"]
#[doc(alias = "APP_QSPI_CLKSTAT")]
pub type AppQspiClkstat = crate::Reg<app_qspi_clkstat::AppQspiClkstatSpec>;
#[doc = "APP_QSPI_CLKSTAT"]
pub mod app_qspi_clkstat;
#[doc = "TOPSS_CLKCTL (rw) register accessor: TOPSS_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`topss_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topss_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@topss_clkctl`]
module"]
#[doc(alias = "TOPSS_CLKCTL")]
pub type TopssClkctl = crate::Reg<topss_clkctl::TopssClkctlSpec>;
#[doc = "TOPSS_CLKCTL"]
pub mod topss_clkctl;
#[doc = "TOPSS_CLKSTAT (rw) register accessor: TOPSS_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`topss_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topss_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@topss_clkstat`]
module"]
#[doc(alias = "TOPSS_CLKSTAT")]
pub type TopssClkstat = crate::Reg<topss_clkstat::TopssClkstatSpec>;
#[doc = "TOPSS_CLKSTAT"]
pub mod topss_clkstat;
#[doc = "APP_RTI_CLKCTL (rw) register accessor: APP_RTI_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_rti_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_rti_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rti_clkctl`]
module"]
#[doc(alias = "APP_RTI_CLKCTL")]
pub type AppRtiClkctl = crate::Reg<app_rti_clkctl::AppRtiClkctlSpec>;
#[doc = "APP_RTI_CLKCTL"]
pub mod app_rti_clkctl;
#[doc = "APP_RTI_CLKSTAT (rw) register accessor: APP_RTI_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_rti_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_rti_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rti_clkstat`]
module"]
#[doc(alias = "APP_RTI_CLKSTAT")]
pub type AppRtiClkstat = crate::Reg<app_rti_clkstat::AppRtiClkstatSpec>;
#[doc = "APP_RTI_CLKSTAT"]
pub mod app_rti_clkstat;
#[doc = "APP_WD_CLKCTL (rw) register accessor: APP_WD_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_wd_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_wd_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_wd_clkctl`]
module"]
#[doc(alias = "APP_WD_CLKCTL")]
pub type AppWdClkctl = crate::Reg<app_wd_clkctl::AppWdClkctlSpec>;
#[doc = "APP_WD_CLKCTL"]
pub mod app_wd_clkctl;
#[doc = "APP_WD_CLKSTAT (rw) register accessor: APP_WD_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_wd_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_wd_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_wd_clkstat`]
module"]
#[doc(alias = "APP_WD_CLKSTAT")]
pub type AppWdClkstat = crate::Reg<app_wd_clkstat::AppWdClkstatSpec>;
#[doc = "APP_WD_CLKSTAT"]
pub mod app_wd_clkstat;
#[doc = "APP_UART_0_CLKCTL (rw) register accessor: APP_UART_0_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_uart_0_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_uart_0_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uart_0_clkctl`]
module"]
#[doc(alias = "APP_UART_0_CLKCTL")]
pub type AppUart0Clkctl = crate::Reg<app_uart_0_clkctl::AppUart0ClkctlSpec>;
#[doc = "APP_UART_0_CLKCTL"]
pub mod app_uart_0_clkctl;
#[doc = "APP_UART_0_CLKSTAT (rw) register accessor: APP_UART_0_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_uart_0_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_uart_0_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uart_0_clkstat`]
module"]
#[doc(alias = "APP_UART_0_CLKSTAT")]
pub type AppUart0Clkstat = crate::Reg<app_uart_0_clkstat::AppUart0ClkstatSpec>;
#[doc = "APP_UART_0_CLKSTAT"]
pub mod app_uart_0_clkstat;
#[doc = "APP_UART_1_CLKCTL (rw) register accessor: APP_UART_1_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_uart_1_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_uart_1_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uart_1_clkctl`]
module"]
#[doc(alias = "APP_UART_1_CLKCTL")]
pub type AppUart1Clkctl = crate::Reg<app_uart_1_clkctl::AppUart1ClkctlSpec>;
#[doc = "APP_UART_1_CLKCTL"]
pub mod app_uart_1_clkctl;
#[doc = "APP_UART_1_CLKSTAT (rw) register accessor: APP_UART_1_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_uart_1_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_uart_1_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_uart_1_clkstat`]
module"]
#[doc(alias = "APP_UART_1_CLKSTAT")]
pub type AppUart1Clkstat = crate::Reg<app_uart_1_clkstat::AppUart1ClkstatSpec>;
#[doc = "APP_UART_1_CLKSTAT"]
pub mod app_uart_1_clkstat;
#[doc = "APP_I2C_CLKCTL (rw) register accessor: APP_I2C_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_i2c_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_i2c_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_i2c_clkctl`]
module"]
#[doc(alias = "APP_I2C_CLKCTL")]
pub type AppI2cClkctl = crate::Reg<app_i2c_clkctl::AppI2cClkctlSpec>;
#[doc = "APP_I2C_CLKCTL"]
pub mod app_i2c_clkctl;
#[doc = "APP_I2C_CLKSTAT (rw) register accessor: APP_I2C_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_i2c_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_i2c_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_i2c_clkstat`]
module"]
#[doc(alias = "APP_I2C_CLKSTAT")]
pub type AppI2cClkstat = crate::Reg<app_i2c_clkstat::AppI2cClkstatSpec>;
#[doc = "APP_I2C_CLKSTAT"]
pub mod app_i2c_clkstat;
#[doc = "APP_LIN_CLKCTL (rw) register accessor: APP_LIN_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_lin_clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_lin_clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_lin_clkctl`]
module"]
#[doc(alias = "APP_LIN_CLKCTL")]
pub type AppLinClkctl = crate::Reg<app_lin_clkctl::AppLinClkctlSpec>;
#[doc = "APP_LIN_CLKCTL"]
pub mod app_lin_clkctl;
#[doc = "APP_LIN_CLKSTAT (rw) register accessor: APP_LIN_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_lin_clkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_lin_clkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_lin_clkstat`]
module"]
#[doc(alias = "APP_LIN_CLKSTAT")]
pub type AppLinClkstat = crate::Reg<app_lin_clkstat::AppLinClkstatSpec>;
#[doc = "APP_LIN_CLKSTAT"]
pub mod app_lin_clkstat;
#[doc = "RESERVED0 (rw) register accessor: RESERVED0\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved0`]
module"]
#[doc(alias = "RESERVED0")]
pub type Reserved0 = crate::Reg<reserved0::Reserved0Spec>;
#[doc = "RESERVED0"]
pub mod reserved0;
#[doc = "RESERVED1 (rw) register accessor: RESERVED1\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved1`]
module"]
#[doc(alias = "RESERVED1")]
pub type Reserved1 = crate::Reg<reserved1::Reserved1Spec>;
#[doc = "RESERVED1"]
pub mod reserved1;
#[doc = "RESERVED2 (rw) register accessor: RESERVED2\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved2`]
module"]
#[doc(alias = "RESERVED2")]
pub type Reserved2 = crate::Reg<reserved2::Reserved2Spec>;
#[doc = "RESERVED2"]
pub mod reserved2;
#[doc = "RESERVED3 (rw) register accessor: RESERVED3\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved3`]
module"]
#[doc(alias = "RESERVED3")]
pub type Reserved3 = crate::Reg<reserved3::Reserved3Spec>;
#[doc = "RESERVED3"]
pub mod reserved3;
#[doc = "IPCFGCLKGATE0 (rw) register accessor: IPCFGCLKGATE0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcfgclkgate0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcfgclkgate0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcfgclkgate0`]
module"]
#[doc(alias = "IPCFGCLKGATE0")]
pub type Ipcfgclkgate0 = crate::Reg<ipcfgclkgate0::Ipcfgclkgate0Spec>;
#[doc = "IPCFGCLKGATE0"]
pub mod ipcfgclkgate0;
#[doc = "IPCFGCLKGATE1 (rw) register accessor: IPCFGCLKGATE1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcfgclkgate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcfgclkgate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcfgclkgate1`]
module"]
#[doc(alias = "IPCFGCLKGATE1")]
pub type Ipcfgclkgate1 = crate::Reg<ipcfgclkgate1::Ipcfgclkgate1Spec>;
#[doc = "IPCFGCLKGATE1"]
pub mod ipcfgclkgate1;
#[doc = "IPCFGCLKGATE2 (rw) register accessor: IPCFGCLKGATE2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcfgclkgate2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcfgclkgate2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcfgclkgate2`]
module"]
#[doc(alias = "IPCFGCLKGATE2")]
pub type Ipcfgclkgate2 = crate::Reg<ipcfgclkgate2::Ipcfgclkgate2Spec>;
#[doc = "IPCFGCLKGATE2"]
pub mod ipcfgclkgate2;
#[doc = "BLOCKRESET0 (rw) register accessor: BLOCKRESET0\n\nYou can [`read`](crate::Reg::read) this register and get [`blockreset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blockreset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blockreset0`]
module"]
#[doc(alias = "BLOCKRESET0")]
pub type Blockreset0 = crate::Reg<blockreset0::Blockreset0Spec>;
#[doc = "BLOCKRESET0"]
pub mod blockreset0;
#[doc = "BLOCKRESET1 (rw) register accessor: BLOCKRESET1\n\nYou can [`read`](crate::Reg::read) this register and get [`blockreset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blockreset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blockreset1`]
module"]
#[doc(alias = "BLOCKRESET1")]
pub type Blockreset1 = crate::Reg<blockreset1::Blockreset1Spec>;
#[doc = "BLOCKRESET1"]
pub mod blockreset1;
#[doc = "BLOCKRESET2 (rw) register accessor: BLOCKRESET2\n\nYou can [`read`](crate::Reg::read) this register and get [`blockreset2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blockreset2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blockreset2`]
module"]
#[doc(alias = "BLOCKRESET2")]
pub type Blockreset2 = crate::Reg<blockreset2::Blockreset2Spec>;
#[doc = "BLOCKRESET2"]
pub mod blockreset2;
#[doc = "PLATFORM_SIGNATURE (rw) register accessor: PLATFORM_SIGNATURE\n\nYou can [`read`](crate::Reg::read) this register and get [`platform_signature::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`platform_signature::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@platform_signature`]
module"]
#[doc(alias = "PLATFORM_SIGNATURE")]
pub type PlatformSignature = crate::Reg<platform_signature::PlatformSignatureSpec>;
#[doc = "PLATFORM_SIGNATURE"]
pub mod platform_signature;
#[doc = "POWERMODE (rw) register accessor: POWERMODE\n\nYou can [`read`](crate::Reg::read) this register and get [`powermode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powermode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powermode`]
module"]
#[doc(alias = "POWERMODE")]
pub type Powermode = crate::Reg<powermode::PowermodeSpec>;
#[doc = "POWERMODE"]
pub mod powermode;
#[doc = "RST_WFICHECK (rw) register accessor: RST_WFICHECK\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_wficheck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_wficheck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_wficheck`]
module"]
#[doc(alias = "RST_WFICHECK")]
pub type RstWficheck = crate::Reg<rst_wficheck::RstWficheckSpec>;
#[doc = "RST_WFICHECK"]
pub mod rst_wficheck;
#[doc = "RST_ASSERTDLY (rw) register accessor: RST_ASSERTDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_assertdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_assertdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_assertdly`]
module"]
#[doc(alias = "RST_ASSERTDLY")]
pub type RstAssertdly = crate::Reg<rst_assertdly::RstAssertdlySpec>;
#[doc = "RST_ASSERTDLY"]
pub mod rst_assertdly;
#[doc = "RST2ASSERTDLY (rw) register accessor: RST2ASSERTDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`rst2assertdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst2assertdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst2assertdly`]
module"]
#[doc(alias = "RST2ASSERTDLY")]
pub type Rst2assertdly = crate::Reg<rst2assertdly::Rst2assertdlySpec>;
#[doc = "RST2ASSERTDLY"]
pub mod rst2assertdly;
#[doc = "RST_FSM_TRIG (rw) register accessor: RST_FSM_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_fsm_trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_fsm_trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_fsm_trig`]
module"]
#[doc(alias = "RST_FSM_TRIG")]
pub type RstFsmTrig = crate::Reg<rst_fsm_trig::RstFsmTrigSpec>;
#[doc = "RST_FSM_TRIG"]
pub mod rst_fsm_trig;
#[doc = "RST_CAUSE (rw) register accessor: RST_CAUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_cause`]
module"]
#[doc(alias = "RST_CAUSE")]
pub type RstCause = crate::Reg<rst_cause::RstCauseSpec>;
#[doc = "RST_CAUSE"]
pub mod rst_cause;
#[doc = "RST_CAUSE_CLR (rw) register accessor: RST_CAUSE_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cause_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cause_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_cause_clr`]
module"]
#[doc(alias = "RST_CAUSE_CLR")]
pub type RstCauseClr = crate::Reg<rst_cause_clr::RstCauseClrSpec>;
#[doc = "RST_CAUSE_CLR"]
pub mod rst_cause_clr;
#[doc = "XTALCLK_CLK_GATE (rw) register accessor: XTALCLK_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalclk_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalclk_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalclk_clk_gate`]
module"]
#[doc(alias = "XTALCLK_CLK_GATE")]
pub type XtalclkClkGate = crate::Reg<xtalclk_clk_gate::XtalclkClkGateSpec>;
#[doc = "XTALCLK_CLK_GATE"]
pub mod xtalclk_clk_gate;
#[doc = "XTALCLKX2_CLK_GATE (rw) register accessor: XTALCLKX2_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalclkx2_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalclkx2_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalclkx2_clk_gate`]
module"]
#[doc(alias = "XTALCLKX2_CLK_GATE")]
pub type Xtalclkx2ClkGate = crate::Reg<xtalclkx2_clk_gate::Xtalclkx2ClkGateSpec>;
#[doc = "XTALCLKX2_CLK_GATE"]
pub mod xtalclkx2_clk_gate;
#[doc = "APLLDIV2_CLK_GATE (rw) register accessor: APLLDIV2_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`aplldiv2_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aplldiv2_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aplldiv2_clk_gate`]
module"]
#[doc(alias = "APLLDIV2_CLK_GATE")]
pub type Aplldiv2ClkGate = crate::Reg<aplldiv2_clk_gate::Aplldiv2ClkGateSpec>;
#[doc = "APLLDIV2_CLK_GATE"]
pub mod aplldiv2_clk_gate;
#[doc = "DFT_APPSS_LSTC_CLK_GATE (rw) register accessor: DFT_APPSS_LSTC_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_appss_lstc_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_appss_lstc_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_appss_lstc_clk_gate`]
module"]
#[doc(alias = "DFT_APPSS_LSTC_CLK_GATE")]
pub type DftAppssLstcClkGate = crate::Reg<dft_appss_lstc_clk_gate::DftAppssLstcClkGateSpec>;
#[doc = "DFT_APPSS_LSTC_CLK_GATE"]
pub mod dft_appss_lstc_clk_gate;
#[doc = "DFT_APPSS_LSTC_VBUSP_CLK_GATE (rw) register accessor: DFT_APPSS_LSTC_VBUSP_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_appss_lstc_vbusp_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_appss_lstc_vbusp_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_appss_lstc_vbusp_clk_gate`]
module"]
#[doc(alias = "DFT_APPSS_LSTC_VBUSP_CLK_GATE")]
pub type DftAppssLstcVbuspClkGate =
    crate::Reg<dft_appss_lstc_vbusp_clk_gate::DftAppssLstcVbuspClkGateSpec>;
#[doc = "DFT_APPSS_LSTC_VBUSP_CLK_GATE"]
pub mod dft_appss_lstc_vbusp_clk_gate;
#[doc = "APP_ROM_CLOCK_GATE (rw) register accessor: APP_ROM_CLOCK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_rom_clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_rom_clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_rom_clock_gate`]
module"]
#[doc(alias = "APP_ROM_CLOCK_GATE")]
pub type AppRomClockGate = crate::Reg<app_rom_clock_gate::AppRomClockGateSpec>;
#[doc = "APP_ROM_CLOCK_GATE"]
pub mod app_rom_clock_gate;
#[doc = "APP_RAM1_CLOCK_GATE (rw) register accessor: APP_RAM1_CLOCK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_ram1_clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_ram1_clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_ram1_clock_gate`]
module"]
#[doc(alias = "APP_RAM1_CLOCK_GATE")]
pub type AppRam1ClockGate = crate::Reg<app_ram1_clock_gate::AppRam1ClockGateSpec>;
#[doc = "APP_RAM1_CLOCK_GATE"]
pub mod app_ram1_clock_gate;
#[doc = "APP_RAM2_CLOCK_GATE (rw) register accessor: APP_RAM2_CLOCK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_ram2_clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_ram2_clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_ram2_clock_gate`]
module"]
#[doc(alias = "APP_RAM2_CLOCK_GATE")]
pub type AppRam2ClockGate = crate::Reg<app_ram2_clock_gate::AppRam2ClockGateSpec>;
#[doc = "APP_RAM2_CLOCK_GATE"]
pub mod app_ram2_clock_gate;
#[doc = "APP_RAM3_CLOCK_GATE (rw) register accessor: APP_RAM3_CLOCK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_ram3_clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_ram3_clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_ram3_clock_gate`]
module"]
#[doc(alias = "APP_RAM3_CLOCK_GATE")]
pub type AppRam3ClockGate = crate::Reg<app_ram3_clock_gate::AppRam3ClockGateSpec>;
#[doc = "APP_RAM3_CLOCK_GATE"]
pub mod app_ram3_clock_gate;
#[doc = "CFG_XBARA_DYNAMIC_CG (rw) register accessor: CFG_XBARA_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_xbara_dynamic_cg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_xbara_dynamic_cg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_xbara_dynamic_cg`]
module"]
#[doc(alias = "CFG_XBARA_DYNAMIC_CG")]
pub type CfgXbaraDynamicCg = crate::Reg<cfg_xbara_dynamic_cg::CfgXbaraDynamicCgSpec>;
#[doc = "CFG_XBARA_DYNAMIC_CG"]
pub mod cfg_xbara_dynamic_cg;
#[doc = "CFG_TPTC1_DYNAMIC_CG (rw) register accessor: CFG_TPTC1_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_tptc1_dynamic_cg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_tptc1_dynamic_cg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tptc1_dynamic_cg`]
module"]
#[doc(alias = "CFG_TPTC1_DYNAMIC_CG")]
pub type CfgTptc1DynamicCg = crate::Reg<cfg_tptc1_dynamic_cg::CfgTptc1DynamicCgSpec>;
#[doc = "CFG_TPTC1_DYNAMIC_CG"]
pub mod cfg_tptc1_dynamic_cg;
#[doc = "CFG_TPTC2_DYNAMIC_CG (rw) register accessor: CFG_TPTC2_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_tptc2_dynamic_cg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_tptc2_dynamic_cg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tptc2_dynamic_cg`]
module"]
#[doc(alias = "CFG_TPTC2_DYNAMIC_CG")]
pub type CfgTptc2DynamicCg = crate::Reg<cfg_tptc2_dynamic_cg::CfgTptc2DynamicCgSpec>;
#[doc = "CFG_TPTC2_DYNAMIC_CG"]
pub mod cfg_tptc2_dynamic_cg;
#[doc = "CFG_XBARA_SET_DYNAMIC_CG (rw) register accessor: CFG_XBARA_SET_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_xbara_set_dynamic_cg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_xbara_set_dynamic_cg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_xbara_set_dynamic_cg`]
module"]
#[doc(alias = "CFG_XBARA_SET_DYNAMIC_CG")]
pub type CfgXbaraSetDynamicCg = crate::Reg<cfg_xbara_set_dynamic_cg::CfgXbaraSetDynamicCgSpec>;
#[doc = "CFG_XBARA_SET_DYNAMIC_CG"]
pub mod cfg_xbara_set_dynamic_cg;
#[doc = "CFG_TPTC1_SET_DYNAMIC_CG (rw) register accessor: CFG_TPTC1_SET_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_tptc1_set_dynamic_cg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_tptc1_set_dynamic_cg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tptc1_set_dynamic_cg`]
module"]
#[doc(alias = "CFG_TPTC1_SET_DYNAMIC_CG")]
pub type CfgTptc1SetDynamicCg = crate::Reg<cfg_tptc1_set_dynamic_cg::CfgTptc1SetDynamicCgSpec>;
#[doc = "CFG_TPTC1_SET_DYNAMIC_CG"]
pub mod cfg_tptc1_set_dynamic_cg;
#[doc = "CFG_TPTC2_SET_DYNAMIC_CG (rw) register accessor: CFG_TPTC2_SET_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_tptc2_set_dynamic_cg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_tptc2_set_dynamic_cg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tptc2_set_dynamic_cg`]
module"]
#[doc(alias = "CFG_TPTC2_SET_DYNAMIC_CG")]
pub type CfgTptc2SetDynamicCg = crate::Reg<cfg_tptc2_set_dynamic_cg::CfgTptc2SetDynamicCgSpec>;
#[doc = "CFG_TPTC2_SET_DYNAMIC_CG"]
pub mod cfg_tptc2_set_dynamic_cg;
#[doc = "CM4_FORCE_HCLK_GATE (rw) register accessor: CM4_FORCE_HCLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_force_hclk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_force_hclk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_force_hclk_gate`]
module"]
#[doc(alias = "CM4_FORCE_HCLK_GATE")]
pub type Cm4ForceHclkGate = crate::Reg<cm4_force_hclk_gate::Cm4ForceHclkGateSpec>;
#[doc = "CM4_FORCE_HCLK_GATE"]
pub mod cm4_force_hclk_gate;
#[doc = "LIN_SCI_DIV (rw) register accessor: LIN_SCI_DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`lin_sci_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lin_sci_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_sci_div`]
module"]
#[doc(alias = "LIN_SCI_DIV")]
pub type LinSciDiv = crate::Reg<lin_sci_div::LinSciDivSpec>;
#[doc = "LIN_SCI_DIV"]
pub mod lin_sci_div;
#[doc = "APP_LSTC_EN (rw) register accessor: APP_LSTC_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`app_lstc_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_lstc_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_lstc_en`]
module"]
#[doc(alias = "APP_LSTC_EN")]
pub type AppLstcEn = crate::Reg<app_lstc_en::AppLstcEnSpec>;
#[doc = "APP_LSTC_EN"]
pub mod app_lstc_en;
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
