#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    hw_reg0: HwReg0,
    hw_reg1: HwReg1,
    previous_name: PreviousName,
    hw_reg3: HwReg3,
    hw_reg4: HwReg4,
    hw_reg5: HwReg5,
    hw_reg6: HwReg6,
    hw_reg7: HwReg7,
    appss_sw_int: AppssSwInt,
    appss_ipc_rfs: AppssIpcRfs,
    appss_capevnt_sel: AppssCapevntSel,
    appss_dma_req_sel: AppssDmaReqSel,
    appss_dma1_req_sel: AppssDma1ReqSel,
    appss_irq_req_sel: AppssIrqReqSel,
    appss_spi_trig_src: AppssSpiTrigSrc,
    appss_ram1a_mem_init: AppssRam1aMemInit,
    appss_ram1a_mem_init_done: AppssRam1aMemInitDone,
    appss_ram1a_mem_init_status: AppssRam1aMemInitStatus,
    appss_ram2a_mem_init: AppssRam2aMemInit,
    appss_ram2a_mem_init_done: AppssRam2aMemInitDone,
    appss_ram2a_mem_init_status: AppssRam2aMemInitStatus,
    appss_ram3a_mem_init: AppssRam3aMemInit,
    appss_ram3a_mem_init_done: AppssRam3aMemInitDone,
    appss_ram3a_mem_init_status: AppssRam3aMemInitStatus,
    hwass_shrd_ram0_mem_init: HwassShrdRam0MemInit,
    hwass_shrd_ram0_mem_init_done: HwassShrdRam0MemInitDone,
    hwass_shrd_ram0_mem_init_status: HwassShrdRam0MemInitStatus,
    hwass_shrd_ram1_mem_init: HwassShrdRam1MemInit,
    hwass_shrd_ram1_mem_init_done: HwassShrdRam1MemInitDone,
    hwass_shrd_ram1_mem_init_status: HwassShrdRam1MemInitStatus,
    appss_tpcc_meminit_start: AppssTpccMeminitStart,
    appss_tpcc_meminit_done: AppssTpccMeminitDone,
    appss_tpcc_meminit_status: AppssTpccMeminitStatus,
    appss_spia_cfg: AppssSpiaCfg,
    appss_spib_cfg: AppssSpibCfg,
    appss_epwm_cfg: AppssEpwmCfg,
    reserved: Reserved,
    appss_mcan_fe_and_lin_intr_sel: AppssMcanFeAndLinIntrSel,
    appss_mcana_int_clr: AppssMcanaIntClr,
    appss_mcana_int_mask: AppssMcanaIntMask,
    appss_mcana_int_stat: AppssMcanaIntStat,
    appss_cm4_global_config: AppssCm4GlobalConfig,
    reserved1: Reserved1,
    appss_cm4_rom_eclipse: AppssCm4RomEclipse,
    appss_cm4_status_reg: AppssCm4StatusReg,
    appss_ahb_ctrl: AppssAhbCtrl,
    esm_gating0: EsmGating0,
    esm_gating1: EsmGating1,
    esm_gating2: EsmGating2,
    esm_gating3: EsmGating3,
    esm_gating4: EsmGating4,
    esm_gating5: EsmGating5,
    esm_gating6: EsmGating6,
    esm_gating7: EsmGating7,
    appss_cm4_halt: AppssCm4Halt,
    appss_cm4_event: AppssCm4Event,
    spia_io_cfg: SpiaIoCfg,
    spib_io_cfg: SpibIoCfg,
    spi_host_irq: SpiHostIrq,
    tptc_dbs_config: TptcDbsConfig,
    tpcc_parity_ctrl: TpccParityCtrl,
    tpcc_parity_status: TpccParityStatus,
    appss_dbg_ack_ctl0: AppssDbgAckCtl0,
    debugss_csetb_flush: DebugssCsetbFlush,
    cpsw_control: CpswControl,
    appss_erragg_mask0: AppssErraggMask0,
    appss_erragg_status0: AppssErraggStatus0,
    _reserved68: [u8; 0x80],
    appss_tpcc_a_erragg_mask: AppssTpccAErraggMask,
    appss_tpcc_a_erragg_status: AppssTpccAErraggStatus,
    appss_tpcc_a_erragg_status_raw: AppssTpccAErraggStatusRaw,
    _reserved71: [u8; 0x78],
    appss_tpcc_a_intagg_mask: AppssTpccAIntaggMask,
    hw_spare_wph: HwSpareWph,
    appss_tpcc_a_intagg_status_raw: AppssTpccAIntaggStatusRaw,
    _reserved74: [u8; 0x54],
    appss_tpcc_b_erragg_mask: AppssTpccBErraggMask,
    appss_tpcc_b_erragg_status: AppssTpccBErraggStatus,
    appss_tpcc_b_erragg_status_raw: AppssTpccBErraggStatusRaw,
    _reserved77: [u8; 0x6c],
    appss_tpcc_b_intagg_mask: AppssTpccBIntaggMask,
    appss_tpcc_b_intagg_status: AppssTpccBIntaggStatus,
    appss_tpcc_b_intagg_status_raw: AppssTpccBIntaggStatusRaw,
    appss_mpu_erragg_mask: AppssMpuErraggMask,
    appss_mpu_erragg_status: AppssMpuErraggStatus,
    appss_mpu_erragg_status_raw: AppssMpuErraggStatusRaw,
    appss_qspi_config: AppssQspiConfig,
    appss_cti_trig_sel: AppssCtiTrigSel,
    appss_dbgss_cti_trig_sel: AppssDbgssCtiTrigSel,
    appss_boot_info_reg0: AppssBootInfoReg0,
    appss_boot_info_reg1: AppssBootInfoReg1,
    appss_boot_info_reg2: AppssBootInfoReg2,
    appss_boot_info_reg3: AppssBootInfoReg3,
    appss_boot_info_reg4: AppssBootInfoReg4,
    appss_boot_info_reg5: AppssBootInfoReg5,
    appss_boot_info_reg6: AppssBootInfoReg6,
    appss_boot_info_reg7: AppssBootInfoReg7,
    appss_tptc_eccaggr_clk_cntrl: AppssTptcEccaggrClkCntrl,
    appss_tptc_boundary_cfg: AppssTptcBoundaryCfg,
    appss_tptc_xid_reorder_cfg: AppssTptcXidReorderCfg,
    hw_sync_fe_ctrl: HwSyncFeCtrl,
    hw_spare_reg1: HwSpareReg1,
    hw_spare_reg2: HwSpareReg2,
    hw_spare_reg3: HwSpareReg3,
    nerror_mask: NerrorMask,
    hw_spare_rw0: HwSpareRw0,
    hw_spare_rw1: HwSpareRw1,
    hw_spare_rw2: HwSpareRw2,
    hw_spare_rw3: HwSpareRw3,
    hw_spare_rw4: HwSpareRw4,
    hw_spare_rw5: HwSpareRw5,
    hw_spare_ro0: HwSpareRo0,
    hw_spare_ro1: HwSpareRo1,
    hw_spare_ro2: HwSpareRo2,
    hw_spare_ro3: HwSpareRo3,
    hw_spare_rec: HwSpareRec,
    app_ctrl: AppCtrl,
    wic_ctrl: WicCtrl,
    wic_stat_clr: WicStatClr,
    wic_stat: WicStat,
    wicen: Wicen,
    forcefclkactive: Forcefclkactive,
    fecss_clk_gate: FecssClkGate,
    appss_shared_mem_clk_gate: AppssSharedMemClkGate,
    appss_mem_init_slice_sel: AppssMemInitSliceSel,
    appss_qspi_char_ext_clk_en: AppssQspiCharExtClkEn,
    appss_qspi_ext_clk_en: AppssQspiExtClkEn,
    spi1_smart_idle: Spi1SmartIdle,
    spi2_smart_idle: Spi2SmartIdle,
    can_smart_idle: CanSmartIdle,
    lin_smart_idle: LinSmartIdle,
    hwass_clk_gate: HwassClkGate,
    cfg_timeout_pcr3: CfgTimeoutPcr3,
    reserved0: Reserved0,
    appss_erragg_mask1: AppssErraggMask1,
    appss_erragg_status1: AppssErraggStatus1,
    forcehclkactive: Forcehclkactive,
    appss_ram1_owrite_err: AppssRam1OwriteErr,
    appss_ram1_owrite_err_addr: AppssRam1OwriteErrAddr,
    appss_ram2_owrite_err: AppssRam2OwriteErr,
    appss_ram2_owrite_err_addr: AppssRam2OwriteErrAddr,
    appss_ram3_owrite_err: AppssRam3OwriteErr,
    appss_ram3_owrite_err_addr: AppssRam3OwriteErrAddr,
    appss_shrd_ram_owrite_err: AppssShrdRamOwriteErr,
    appss_shrd_ram_owrite_err_addr: AppssShrdRamOwriteErrAddr,
    appss_owrite_err_aggr: AppssOwriteErrAggr,
    hw_spare_rw6: HwSpareRw6,
    hw_spare_rw7: HwSpareRw7,
    hw_spare_rw8: HwSpareRw8,
    hw_spare_rw9: HwSpareRw9,
    hw_spare_hwa_rw0: HwSpareHwaRw0,
    _reserved148: [u8; 0x0c00],
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
    #[doc = "0x04 - HW_REG0"]
    #[inline(always)]
    pub const fn hw_reg0(&self) -> &HwReg0 {
        &self.hw_reg0
    }
    #[doc = "0x08 - HW_REG1"]
    #[inline(always)]
    pub const fn hw_reg1(&self) -> &HwReg1 {
        &self.hw_reg1
    }
    #[doc = "0x0c - PREVIOUS_NAME"]
    #[inline(always)]
    pub const fn previous_name(&self) -> &PreviousName {
        &self.previous_name
    }
    #[doc = "0x10 - HW_REG3"]
    #[inline(always)]
    pub const fn hw_reg3(&self) -> &HwReg3 {
        &self.hw_reg3
    }
    #[doc = "0x14 - HW_REG4"]
    #[inline(always)]
    pub const fn hw_reg4(&self) -> &HwReg4 {
        &self.hw_reg4
    }
    #[doc = "0x18 - HW_REG5"]
    #[inline(always)]
    pub const fn hw_reg5(&self) -> &HwReg5 {
        &self.hw_reg5
    }
    #[doc = "0x1c - HW_REG6"]
    #[inline(always)]
    pub const fn hw_reg6(&self) -> &HwReg6 {
        &self.hw_reg6
    }
    #[doc = "0x20 - HW_REG7"]
    #[inline(always)]
    pub const fn hw_reg7(&self) -> &HwReg7 {
        &self.hw_reg7
    }
    #[doc = "0x24 - APPSS_SW_INT"]
    #[inline(always)]
    pub const fn appss_sw_int(&self) -> &AppssSwInt {
        &self.appss_sw_int
    }
    #[doc = "0x28 - APPSS_IPC_RFS"]
    #[inline(always)]
    pub const fn appss_ipc_rfs(&self) -> &AppssIpcRfs {
        &self.appss_ipc_rfs
    }
    #[doc = "0x2c - APPSS_CAPEVNT_SEL"]
    #[inline(always)]
    pub const fn appss_capevnt_sel(&self) -> &AppssCapevntSel {
        &self.appss_capevnt_sel
    }
    #[doc = "0x30 - APPSS_DMA_REQ_SEL"]
    #[inline(always)]
    pub const fn appss_dma_req_sel(&self) -> &AppssDmaReqSel {
        &self.appss_dma_req_sel
    }
    #[doc = "0x34 - APPSS_DMA1_REQ_SEL"]
    #[inline(always)]
    pub const fn appss_dma1_req_sel(&self) -> &AppssDma1ReqSel {
        &self.appss_dma1_req_sel
    }
    #[doc = "0x38 - APPSS_IRQ_REQ_SEL"]
    #[inline(always)]
    pub const fn appss_irq_req_sel(&self) -> &AppssIrqReqSel {
        &self.appss_irq_req_sel
    }
    #[doc = "0x3c - APPSS_SPI_TRIG_SRC"]
    #[inline(always)]
    pub const fn appss_spi_trig_src(&self) -> &AppssSpiTrigSrc {
        &self.appss_spi_trig_src
    }
    #[doc = "0x40 - APPSS_RAM1A_MEM_INIT"]
    #[inline(always)]
    pub const fn appss_ram1a_mem_init(&self) -> &AppssRam1aMemInit {
        &self.appss_ram1a_mem_init
    }
    #[doc = "0x44 - APPSS_RAM1A_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn appss_ram1a_mem_init_done(&self) -> &AppssRam1aMemInitDone {
        &self.appss_ram1a_mem_init_done
    }
    #[doc = "0x48 - APPSS_RAM1A_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn appss_ram1a_mem_init_status(&self) -> &AppssRam1aMemInitStatus {
        &self.appss_ram1a_mem_init_status
    }
    #[doc = "0x4c - APPSS_RAM2A_MEM_INIT"]
    #[inline(always)]
    pub const fn appss_ram2a_mem_init(&self) -> &AppssRam2aMemInit {
        &self.appss_ram2a_mem_init
    }
    #[doc = "0x50 - APPSS_RAM2A_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn appss_ram2a_mem_init_done(&self) -> &AppssRam2aMemInitDone {
        &self.appss_ram2a_mem_init_done
    }
    #[doc = "0x54 - APPSS_RAM2A_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn appss_ram2a_mem_init_status(&self) -> &AppssRam2aMemInitStatus {
        &self.appss_ram2a_mem_init_status
    }
    #[doc = "0x58 - APPSS_RAM3A_MEM_INIT"]
    #[inline(always)]
    pub const fn appss_ram3a_mem_init(&self) -> &AppssRam3aMemInit {
        &self.appss_ram3a_mem_init
    }
    #[doc = "0x5c - APPSS_RAM3A_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn appss_ram3a_mem_init_done(&self) -> &AppssRam3aMemInitDone {
        &self.appss_ram3a_mem_init_done
    }
    #[doc = "0x60 - APPSS_RAM3A_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn appss_ram3a_mem_init_status(&self) -> &AppssRam3aMemInitStatus {
        &self.appss_ram3a_mem_init_status
    }
    #[doc = "0x64 - HWASS_SHRD_RAM0_MEM_INIT"]
    #[inline(always)]
    pub const fn hwass_shrd_ram0_mem_init(&self) -> &HwassShrdRam0MemInit {
        &self.hwass_shrd_ram0_mem_init
    }
    #[doc = "0x68 - HWASS_SHRD_RAM0_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn hwass_shrd_ram0_mem_init_done(&self) -> &HwassShrdRam0MemInitDone {
        &self.hwass_shrd_ram0_mem_init_done
    }
    #[doc = "0x6c - HWASS_SHRD_RAM0_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn hwass_shrd_ram0_mem_init_status(&self) -> &HwassShrdRam0MemInitStatus {
        &self.hwass_shrd_ram0_mem_init_status
    }
    #[doc = "0x70 - HWASS_SHRD_RAM1_MEM_INIT"]
    #[inline(always)]
    pub const fn hwass_shrd_ram1_mem_init(&self) -> &HwassShrdRam1MemInit {
        &self.hwass_shrd_ram1_mem_init
    }
    #[doc = "0x74 - HWASS_SHRD_RAM1_MEM_INIT_DONE"]
    #[inline(always)]
    pub const fn hwass_shrd_ram1_mem_init_done(&self) -> &HwassShrdRam1MemInitDone {
        &self.hwass_shrd_ram1_mem_init_done
    }
    #[doc = "0x78 - HWASS_SHRD_RAM1_MEM_INIT_STATUS"]
    #[inline(always)]
    pub const fn hwass_shrd_ram1_mem_init_status(&self) -> &HwassShrdRam1MemInitStatus {
        &self.hwass_shrd_ram1_mem_init_status
    }
    #[doc = "0x7c - APPSS_TPCC_MEMINIT_START"]
    #[inline(always)]
    pub const fn appss_tpcc_meminit_start(&self) -> &AppssTpccMeminitStart {
        &self.appss_tpcc_meminit_start
    }
    #[doc = "0x80 - APPSS_TPCC_MEMINIT_DONE"]
    #[inline(always)]
    pub const fn appss_tpcc_meminit_done(&self) -> &AppssTpccMeminitDone {
        &self.appss_tpcc_meminit_done
    }
    #[doc = "0x84 - APPSS_TPCC_MEMINIT_STATUS"]
    #[inline(always)]
    pub const fn appss_tpcc_meminit_status(&self) -> &AppssTpccMeminitStatus {
        &self.appss_tpcc_meminit_status
    }
    #[doc = "0x88 - APPSS_SPIA_CFG"]
    #[inline(always)]
    pub const fn appss_spia_cfg(&self) -> &AppssSpiaCfg {
        &self.appss_spia_cfg
    }
    #[doc = "0x8c - APPSS_SPIB_CFG"]
    #[inline(always)]
    pub const fn appss_spib_cfg(&self) -> &AppssSpibCfg {
        &self.appss_spib_cfg
    }
    #[doc = "0x90 - APPSS_EPWM_CFG"]
    #[inline(always)]
    pub const fn appss_epwm_cfg(&self) -> &AppssEpwmCfg {
        &self.appss_epwm_cfg
    }
    #[doc = "0x94 - RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
    #[doc = "0x98 - APPSS_MCAN_FE_AND_LIN_INTR_SEL"]
    #[inline(always)]
    pub const fn appss_mcan_fe_and_lin_intr_sel(&self) -> &AppssMcanFeAndLinIntrSel {
        &self.appss_mcan_fe_and_lin_intr_sel
    }
    #[doc = "0x9c - APPSS_MCANA_INT_CLR"]
    #[inline(always)]
    pub const fn appss_mcana_int_clr(&self) -> &AppssMcanaIntClr {
        &self.appss_mcana_int_clr
    }
    #[doc = "0xa0 - APPSS_MCANA_INT_MASK"]
    #[inline(always)]
    pub const fn appss_mcana_int_mask(&self) -> &AppssMcanaIntMask {
        &self.appss_mcana_int_mask
    }
    #[doc = "0xa4 - APPSS_MCANA_INT_STAT"]
    #[inline(always)]
    pub const fn appss_mcana_int_stat(&self) -> &AppssMcanaIntStat {
        &self.appss_mcana_int_stat
    }
    #[doc = "0xa8 - APPSS_CM4_GLOBAL_CONFIG"]
    #[inline(always)]
    pub const fn appss_cm4_global_config(&self) -> &AppssCm4GlobalConfig {
        &self.appss_cm4_global_config
    }
    #[doc = "0xac - RESERVED1"]
    #[inline(always)]
    pub const fn reserved1(&self) -> &Reserved1 {
        &self.reserved1
    }
    #[doc = "0xb0 - APPSS_CM4_ROM_ECLIPSE"]
    #[inline(always)]
    pub const fn appss_cm4_rom_eclipse(&self) -> &AppssCm4RomEclipse {
        &self.appss_cm4_rom_eclipse
    }
    #[doc = "0xb4 - APPSS_CM4_STATUS_REG"]
    #[inline(always)]
    pub const fn appss_cm4_status_reg(&self) -> &AppssCm4StatusReg {
        &self.appss_cm4_status_reg
    }
    #[doc = "0xb8 - APPSS_AHB_CTRL"]
    #[inline(always)]
    pub const fn appss_ahb_ctrl(&self) -> &AppssAhbCtrl {
        &self.appss_ahb_ctrl
    }
    #[doc = "0xbc - ESM_GATING0"]
    #[inline(always)]
    pub const fn esm_gating0(&self) -> &EsmGating0 {
        &self.esm_gating0
    }
    #[doc = "0xc0 - ESM_GATING1"]
    #[inline(always)]
    pub const fn esm_gating1(&self) -> &EsmGating1 {
        &self.esm_gating1
    }
    #[doc = "0xc4 - ESM_GATING2"]
    #[inline(always)]
    pub const fn esm_gating2(&self) -> &EsmGating2 {
        &self.esm_gating2
    }
    #[doc = "0xc8 - ESM_GATING3"]
    #[inline(always)]
    pub const fn esm_gating3(&self) -> &EsmGating3 {
        &self.esm_gating3
    }
    #[doc = "0xcc - ESM_GATING4"]
    #[inline(always)]
    pub const fn esm_gating4(&self) -> &EsmGating4 {
        &self.esm_gating4
    }
    #[doc = "0xd0 - ESM_GATING5"]
    #[inline(always)]
    pub const fn esm_gating5(&self) -> &EsmGating5 {
        &self.esm_gating5
    }
    #[doc = "0xd4 - ESM_GATING6"]
    #[inline(always)]
    pub const fn esm_gating6(&self) -> &EsmGating6 {
        &self.esm_gating6
    }
    #[doc = "0xd8 - ESM_GATING7"]
    #[inline(always)]
    pub const fn esm_gating7(&self) -> &EsmGating7 {
        &self.esm_gating7
    }
    #[doc = "0xdc - APPSS_CM4_HALT"]
    #[inline(always)]
    pub const fn appss_cm4_halt(&self) -> &AppssCm4Halt {
        &self.appss_cm4_halt
    }
    #[doc = "0xe0 - APPSS_CM4_EVENT"]
    #[inline(always)]
    pub const fn appss_cm4_event(&self) -> &AppssCm4Event {
        &self.appss_cm4_event
    }
    #[doc = "0xe4 - SPIA_IO_CFG"]
    #[inline(always)]
    pub const fn spia_io_cfg(&self) -> &SpiaIoCfg {
        &self.spia_io_cfg
    }
    #[doc = "0xe8 - SPIB_IO_CFG"]
    #[inline(always)]
    pub const fn spib_io_cfg(&self) -> &SpibIoCfg {
        &self.spib_io_cfg
    }
    #[doc = "0xec - SPI_HOST_IRQ"]
    #[inline(always)]
    pub const fn spi_host_irq(&self) -> &SpiHostIrq {
        &self.spi_host_irq
    }
    #[doc = "0xf0 - TPTC_DBS_CONFIG"]
    #[inline(always)]
    pub const fn tptc_dbs_config(&self) -> &TptcDbsConfig {
        &self.tptc_dbs_config
    }
    #[doc = "0xf4 - TPCC_PARITY_CTRL"]
    #[inline(always)]
    pub const fn tpcc_parity_ctrl(&self) -> &TpccParityCtrl {
        &self.tpcc_parity_ctrl
    }
    #[doc = "0xf8 - TPCC_PARITY_STATUS"]
    #[inline(always)]
    pub const fn tpcc_parity_status(&self) -> &TpccParityStatus {
        &self.tpcc_parity_status
    }
    #[doc = "0xfc - APPSS_DBG_ACK_CTL0"]
    #[inline(always)]
    pub const fn appss_dbg_ack_ctl0(&self) -> &AppssDbgAckCtl0 {
        &self.appss_dbg_ack_ctl0
    }
    #[doc = "0x100 - DEBUGSS_CSETB_FLUSH"]
    #[inline(always)]
    pub const fn debugss_csetb_flush(&self) -> &DebugssCsetbFlush {
        &self.debugss_csetb_flush
    }
    #[doc = "0x104 - CPSW_CONTROL"]
    #[inline(always)]
    pub const fn cpsw_control(&self) -> &CpswControl {
        &self.cpsw_control
    }
    #[doc = "0x108 - APPSS_ERRAGG_MASK0"]
    #[inline(always)]
    pub const fn appss_erragg_mask0(&self) -> &AppssErraggMask0 {
        &self.appss_erragg_mask0
    }
    #[doc = "0x10c - APPSS_ERRAGG_STATUS0"]
    #[inline(always)]
    pub const fn appss_erragg_status0(&self) -> &AppssErraggStatus0 {
        &self.appss_erragg_status0
    }
    #[doc = "0x190 - APPSS_TPCC_A_ERRAGG_MASK"]
    #[inline(always)]
    pub const fn appss_tpcc_a_erragg_mask(&self) -> &AppssTpccAErraggMask {
        &self.appss_tpcc_a_erragg_mask
    }
    #[doc = "0x194 - APPSS_TPCC_A_ERRAGG_STATUS"]
    #[inline(always)]
    pub const fn appss_tpcc_a_erragg_status(&self) -> &AppssTpccAErraggStatus {
        &self.appss_tpcc_a_erragg_status
    }
    #[doc = "0x198 - APPSS_TPCC_A_ERRAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn appss_tpcc_a_erragg_status_raw(&self) -> &AppssTpccAErraggStatusRaw {
        &self.appss_tpcc_a_erragg_status_raw
    }
    #[doc = "0x214 - APPSS_TPCC_A_INTAGG_MASK"]
    #[inline(always)]
    pub const fn appss_tpcc_a_intagg_mask(&self) -> &AppssTpccAIntaggMask {
        &self.appss_tpcc_a_intagg_mask
    }
    #[doc = "0x218 - HW_SPARE_WPH"]
    #[inline(always)]
    pub const fn hw_spare_wph(&self) -> &HwSpareWph {
        &self.hw_spare_wph
    }
    #[doc = "0x21c - APPSS_TPCC_A_INTAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn appss_tpcc_a_intagg_status_raw(&self) -> &AppssTpccAIntaggStatusRaw {
        &self.appss_tpcc_a_intagg_status_raw
    }
    #[doc = "0x274 - APPSS_TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub const fn appss_tpcc_b_erragg_mask(&self) -> &AppssTpccBErraggMask {
        &self.appss_tpcc_b_erragg_mask
    }
    #[doc = "0x278 - APPSS_TPCC_B_ERRAGG_STATUS"]
    #[inline(always)]
    pub const fn appss_tpcc_b_erragg_status(&self) -> &AppssTpccBErraggStatus {
        &self.appss_tpcc_b_erragg_status
    }
    #[doc = "0x27c - APPSS_TPCC_B_ERRAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn appss_tpcc_b_erragg_status_raw(&self) -> &AppssTpccBErraggStatusRaw {
        &self.appss_tpcc_b_erragg_status_raw
    }
    #[doc = "0x2ec - APPSS_TPCC_B_INTAGG_MASK"]
    #[inline(always)]
    pub const fn appss_tpcc_b_intagg_mask(&self) -> &AppssTpccBIntaggMask {
        &self.appss_tpcc_b_intagg_mask
    }
    #[doc = "0x2f0 - APPSS_TPCC_B_INTAGG_STATUS"]
    #[inline(always)]
    pub const fn appss_tpcc_b_intagg_status(&self) -> &AppssTpccBIntaggStatus {
        &self.appss_tpcc_b_intagg_status
    }
    #[doc = "0x2f4 - APPSS_TPCC_B_INTAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn appss_tpcc_b_intagg_status_raw(&self) -> &AppssTpccBIntaggStatusRaw {
        &self.appss_tpcc_b_intagg_status_raw
    }
    #[doc = "0x2f8 - APPSS_MPU_ERRAGG_MASK"]
    #[inline(always)]
    pub const fn appss_mpu_erragg_mask(&self) -> &AppssMpuErraggMask {
        &self.appss_mpu_erragg_mask
    }
    #[doc = "0x2fc - APPSS_MPU_ERRAGG_STATUS"]
    #[inline(always)]
    pub const fn appss_mpu_erragg_status(&self) -> &AppssMpuErraggStatus {
        &self.appss_mpu_erragg_status
    }
    #[doc = "0x300 - APPSS_MPU_ERRAGG_STATUS_RAW"]
    #[inline(always)]
    pub const fn appss_mpu_erragg_status_raw(&self) -> &AppssMpuErraggStatusRaw {
        &self.appss_mpu_erragg_status_raw
    }
    #[doc = "0x304 - APPSS_QSPI_CONFIG"]
    #[inline(always)]
    pub const fn appss_qspi_config(&self) -> &AppssQspiConfig {
        &self.appss_qspi_config
    }
    #[doc = "0x308 - APPSS_CTI_TRIG_SEL"]
    #[inline(always)]
    pub const fn appss_cti_trig_sel(&self) -> &AppssCtiTrigSel {
        &self.appss_cti_trig_sel
    }
    #[doc = "0x30c - APPSS_DBGSS_CTI_TRIG_SEL"]
    #[inline(always)]
    pub const fn appss_dbgss_cti_trig_sel(&self) -> &AppssDbgssCtiTrigSel {
        &self.appss_dbgss_cti_trig_sel
    }
    #[doc = "0x310 - APPSS_BOOT_INFO_REG0"]
    #[inline(always)]
    pub const fn appss_boot_info_reg0(&self) -> &AppssBootInfoReg0 {
        &self.appss_boot_info_reg0
    }
    #[doc = "0x314 - APPSS_BOOT_INFO_REG1"]
    #[inline(always)]
    pub const fn appss_boot_info_reg1(&self) -> &AppssBootInfoReg1 {
        &self.appss_boot_info_reg1
    }
    #[doc = "0x318 - APPSS_BOOT_INFO_REG2"]
    #[inline(always)]
    pub const fn appss_boot_info_reg2(&self) -> &AppssBootInfoReg2 {
        &self.appss_boot_info_reg2
    }
    #[doc = "0x31c - APPSS_BOOT_INFO_REG3"]
    #[inline(always)]
    pub const fn appss_boot_info_reg3(&self) -> &AppssBootInfoReg3 {
        &self.appss_boot_info_reg3
    }
    #[doc = "0x320 - APPSS_BOOT_INFO_REG4"]
    #[inline(always)]
    pub const fn appss_boot_info_reg4(&self) -> &AppssBootInfoReg4 {
        &self.appss_boot_info_reg4
    }
    #[doc = "0x324 - APPSS_BOOT_INFO_REG5"]
    #[inline(always)]
    pub const fn appss_boot_info_reg5(&self) -> &AppssBootInfoReg5 {
        &self.appss_boot_info_reg5
    }
    #[doc = "0x328 - APPSS_BOOT_INFO_REG6"]
    #[inline(always)]
    pub const fn appss_boot_info_reg6(&self) -> &AppssBootInfoReg6 {
        &self.appss_boot_info_reg6
    }
    #[doc = "0x32c - APPSS_BOOT_INFO_REG7"]
    #[inline(always)]
    pub const fn appss_boot_info_reg7(&self) -> &AppssBootInfoReg7 {
        &self.appss_boot_info_reg7
    }
    #[doc = "0x330 - APPSS_TPTC_ECCAGGR_CLK_CNTRL"]
    #[inline(always)]
    pub const fn appss_tptc_eccaggr_clk_cntrl(&self) -> &AppssTptcEccaggrClkCntrl {
        &self.appss_tptc_eccaggr_clk_cntrl
    }
    #[doc = "0x334 - APPSS_TPTC_BOUNDARY_CFG"]
    #[inline(always)]
    pub const fn appss_tptc_boundary_cfg(&self) -> &AppssTptcBoundaryCfg {
        &self.appss_tptc_boundary_cfg
    }
    #[doc = "0x338 - APPSS_TPTC_XID_REORDER_CFG"]
    #[inline(always)]
    pub const fn appss_tptc_xid_reorder_cfg(&self) -> &AppssTptcXidReorderCfg {
        &self.appss_tptc_xid_reorder_cfg
    }
    #[doc = "0x33c - HW_Sync_FE_CTRL"]
    #[inline(always)]
    pub const fn hw_sync_fe_ctrl(&self) -> &HwSyncFeCtrl {
        &self.hw_sync_fe_ctrl
    }
    #[doc = "0x340 - HW_SPARE_REG1"]
    #[inline(always)]
    pub const fn hw_spare_reg1(&self) -> &HwSpareReg1 {
        &self.hw_spare_reg1
    }
    #[doc = "0x344 - HW_SPARE_REG2"]
    #[inline(always)]
    pub const fn hw_spare_reg2(&self) -> &HwSpareReg2 {
        &self.hw_spare_reg2
    }
    #[doc = "0x348 - HW_SPARE_REG3"]
    #[inline(always)]
    pub const fn hw_spare_reg3(&self) -> &HwSpareReg3 {
        &self.hw_spare_reg3
    }
    #[doc = "0x34c - NERROR_MASK"]
    #[inline(always)]
    pub const fn nerror_mask(&self) -> &NerrorMask {
        &self.nerror_mask
    }
    #[doc = "0x350 - HW_SPARE_RW0"]
    #[inline(always)]
    pub const fn hw_spare_rw0(&self) -> &HwSpareRw0 {
        &self.hw_spare_rw0
    }
    #[doc = "0x354 - HW_SPARE_RW1"]
    #[inline(always)]
    pub const fn hw_spare_rw1(&self) -> &HwSpareRw1 {
        &self.hw_spare_rw1
    }
    #[doc = "0x358 - HW_SPARE_RW2"]
    #[inline(always)]
    pub const fn hw_spare_rw2(&self) -> &HwSpareRw2 {
        &self.hw_spare_rw2
    }
    #[doc = "0x35c - HW_SPARE_RW3"]
    #[inline(always)]
    pub const fn hw_spare_rw3(&self) -> &HwSpareRw3 {
        &self.hw_spare_rw3
    }
    #[doc = "0x360 - HW_SPARE_RW4"]
    #[inline(always)]
    pub const fn hw_spare_rw4(&self) -> &HwSpareRw4 {
        &self.hw_spare_rw4
    }
    #[doc = "0x364 - HW_SPARE_RW5"]
    #[inline(always)]
    pub const fn hw_spare_rw5(&self) -> &HwSpareRw5 {
        &self.hw_spare_rw5
    }
    #[doc = "0x368 - HW_SPARE_RO0"]
    #[inline(always)]
    pub const fn hw_spare_ro0(&self) -> &HwSpareRo0 {
        &self.hw_spare_ro0
    }
    #[doc = "0x36c - HW_SPARE_RO1"]
    #[inline(always)]
    pub const fn hw_spare_ro1(&self) -> &HwSpareRo1 {
        &self.hw_spare_ro1
    }
    #[doc = "0x370 - HW_SPARE_RO2"]
    #[inline(always)]
    pub const fn hw_spare_ro2(&self) -> &HwSpareRo2 {
        &self.hw_spare_ro2
    }
    #[doc = "0x374 - HW_SPARE_RO3"]
    #[inline(always)]
    pub const fn hw_spare_ro3(&self) -> &HwSpareRo3 {
        &self.hw_spare_ro3
    }
    #[doc = "0x378 - HW_SPARE_REC"]
    #[inline(always)]
    pub const fn hw_spare_rec(&self) -> &HwSpareRec {
        &self.hw_spare_rec
    }
    #[doc = "0x37c - APP_CTRL"]
    #[inline(always)]
    pub const fn app_ctrl(&self) -> &AppCtrl {
        &self.app_ctrl
    }
    #[doc = "0x380 - WIC_CTRL"]
    #[inline(always)]
    pub const fn wic_ctrl(&self) -> &WicCtrl {
        &self.wic_ctrl
    }
    #[doc = "0x384 - WIC_STAT_CLR"]
    #[inline(always)]
    pub const fn wic_stat_clr(&self) -> &WicStatClr {
        &self.wic_stat_clr
    }
    #[doc = "0x388 - WIC_STAT"]
    #[inline(always)]
    pub const fn wic_stat(&self) -> &WicStat {
        &self.wic_stat
    }
    #[doc = "0x38c - WICEN"]
    #[inline(always)]
    pub const fn wicen(&self) -> &Wicen {
        &self.wicen
    }
    #[doc = "0x390 - FORCEFCLKACTIVE"]
    #[inline(always)]
    pub const fn forcefclkactive(&self) -> &Forcefclkactive {
        &self.forcefclkactive
    }
    #[doc = "0x394 - FECSS_CLK_GATE"]
    #[inline(always)]
    pub const fn fecss_clk_gate(&self) -> &FecssClkGate {
        &self.fecss_clk_gate
    }
    #[doc = "0x398 - APPSS_SHARED_MEM_CLK_GATE"]
    #[inline(always)]
    pub const fn appss_shared_mem_clk_gate(&self) -> &AppssSharedMemClkGate {
        &self.appss_shared_mem_clk_gate
    }
    #[doc = "0x39c - APPSS_MEM_INIT_SLICE_SEL"]
    #[inline(always)]
    pub const fn appss_mem_init_slice_sel(&self) -> &AppssMemInitSliceSel {
        &self.appss_mem_init_slice_sel
    }
    #[doc = "0x3a0 - APPSS_QSPI_CHAR_EXT_CLK_EN"]
    #[inline(always)]
    pub const fn appss_qspi_char_ext_clk_en(&self) -> &AppssQspiCharExtClkEn {
        &self.appss_qspi_char_ext_clk_en
    }
    #[doc = "0x3a4 - APPSS_QSPI_EXT_CLK_EN"]
    #[inline(always)]
    pub const fn appss_qspi_ext_clk_en(&self) -> &AppssQspiExtClkEn {
        &self.appss_qspi_ext_clk_en
    }
    #[doc = "0x3a8 - SPI1_SMART_IDLE"]
    #[inline(always)]
    pub const fn spi1_smart_idle(&self) -> &Spi1SmartIdle {
        &self.spi1_smart_idle
    }
    #[doc = "0x3ac - SPI2_SMART_IDLE"]
    #[inline(always)]
    pub const fn spi2_smart_idle(&self) -> &Spi2SmartIdle {
        &self.spi2_smart_idle
    }
    #[doc = "0x3b0 - CAN_SMART_IDLE"]
    #[inline(always)]
    pub const fn can_smart_idle(&self) -> &CanSmartIdle {
        &self.can_smart_idle
    }
    #[doc = "0x3b4 - LIN_SMART_IDLE"]
    #[inline(always)]
    pub const fn lin_smart_idle(&self) -> &LinSmartIdle {
        &self.lin_smart_idle
    }
    #[doc = "0x3b8 - HWASS_CLK_GATE"]
    #[inline(always)]
    pub const fn hwass_clk_gate(&self) -> &HwassClkGate {
        &self.hwass_clk_gate
    }
    #[doc = "0x3bc - CFG_TIMEOUT_PCR3"]
    #[inline(always)]
    pub const fn cfg_timeout_pcr3(&self) -> &CfgTimeoutPcr3 {
        &self.cfg_timeout_pcr3
    }
    #[doc = "0x3c0 - RESERVED0"]
    #[inline(always)]
    pub const fn reserved0(&self) -> &Reserved0 {
        &self.reserved0
    }
    #[doc = "0x3c4 - APPSS_ERRAGG_MASK1"]
    #[inline(always)]
    pub const fn appss_erragg_mask1(&self) -> &AppssErraggMask1 {
        &self.appss_erragg_mask1
    }
    #[doc = "0x3c8 - APPSS_ERRAGG_STATUS1"]
    #[inline(always)]
    pub const fn appss_erragg_status1(&self) -> &AppssErraggStatus1 {
        &self.appss_erragg_status1
    }
    #[doc = "0x3cc - FORCEHCLKACTIVE"]
    #[inline(always)]
    pub const fn forcehclkactive(&self) -> &Forcehclkactive {
        &self.forcehclkactive
    }
    #[doc = "0x3d0 - APPSS_RAM1_OWRITE_ERR"]
    #[inline(always)]
    pub const fn appss_ram1_owrite_err(&self) -> &AppssRam1OwriteErr {
        &self.appss_ram1_owrite_err
    }
    #[doc = "0x3d4 - APPSS_RAM1_OWRITE_ERR_ADDR"]
    #[inline(always)]
    pub const fn appss_ram1_owrite_err_addr(&self) -> &AppssRam1OwriteErrAddr {
        &self.appss_ram1_owrite_err_addr
    }
    #[doc = "0x3d8 - APPSS_RAM2_OWRITE_ERR"]
    #[inline(always)]
    pub const fn appss_ram2_owrite_err(&self) -> &AppssRam2OwriteErr {
        &self.appss_ram2_owrite_err
    }
    #[doc = "0x3dc - APPSS_RAM2_OWRITE_ERR_ADDR"]
    #[inline(always)]
    pub const fn appss_ram2_owrite_err_addr(&self) -> &AppssRam2OwriteErrAddr {
        &self.appss_ram2_owrite_err_addr
    }
    #[doc = "0x3e0 - APPSS_RAM3_OWRITE_ERR"]
    #[inline(always)]
    pub const fn appss_ram3_owrite_err(&self) -> &AppssRam3OwriteErr {
        &self.appss_ram3_owrite_err
    }
    #[doc = "0x3e4 - APPSS_RAM3_OWRITE_ERR_ADDR"]
    #[inline(always)]
    pub const fn appss_ram3_owrite_err_addr(&self) -> &AppssRam3OwriteErrAddr {
        &self.appss_ram3_owrite_err_addr
    }
    #[doc = "0x3e8 - APPSS_SHRD_RAM_OWRITE_ERR"]
    #[inline(always)]
    pub const fn appss_shrd_ram_owrite_err(&self) -> &AppssShrdRamOwriteErr {
        &self.appss_shrd_ram_owrite_err
    }
    #[doc = "0x3ec - APPSS_SHRD_RAM_OWRITE_ERR_ADDR"]
    #[inline(always)]
    pub const fn appss_shrd_ram_owrite_err_addr(&self) -> &AppssShrdRamOwriteErrAddr {
        &self.appss_shrd_ram_owrite_err_addr
    }
    #[doc = "0x3f0 - APPSS_OWRITE_ERR_AGGR"]
    #[inline(always)]
    pub const fn appss_owrite_err_aggr(&self) -> &AppssOwriteErrAggr {
        &self.appss_owrite_err_aggr
    }
    #[doc = "0x3f4 - HW_SPARE_RW6"]
    #[inline(always)]
    pub const fn hw_spare_rw6(&self) -> &HwSpareRw6 {
        &self.hw_spare_rw6
    }
    #[doc = "0x3f8 - HW_SPARE_RW7"]
    #[inline(always)]
    pub const fn hw_spare_rw7(&self) -> &HwSpareRw7 {
        &self.hw_spare_rw7
    }
    #[doc = "0x3fc - HW_SPARE_RW8"]
    #[inline(always)]
    pub const fn hw_spare_rw8(&self) -> &HwSpareRw8 {
        &self.hw_spare_rw8
    }
    #[doc = "0x400 - HW_SPARE_RW9"]
    #[inline(always)]
    pub const fn hw_spare_rw9(&self) -> &HwSpareRw9 {
        &self.hw_spare_rw9
    }
    #[doc = "0x404 - HW_SPARE_HWA_RW0"]
    #[inline(always)]
    pub const fn hw_spare_hwa_rw0(&self) -> &HwSpareHwaRw0 {
        &self.hw_spare_hwa_rw0
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
#[doc = "HW_REG0 (rw) register accessor: HW_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg0`]
module"]
#[doc(alias = "HW_REG0")]
pub type HwReg0 = crate::Reg<hw_reg0::HwReg0Spec>;
#[doc = "HW_REG0"]
pub mod hw_reg0;
#[doc = "HW_REG1 (rw) register accessor: HW_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg1`]
module"]
#[doc(alias = "HW_REG1")]
pub type HwReg1 = crate::Reg<hw_reg1::HwReg1Spec>;
#[doc = "HW_REG1"]
pub mod hw_reg1;
#[doc = "PREVIOUS_NAME (rw) register accessor: PREVIOUS_NAME\n\nYou can [`read`](crate::Reg::read) this register and get [`previous_name::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`previous_name::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@previous_name`]
module"]
#[doc(alias = "PREVIOUS_NAME")]
pub type PreviousName = crate::Reg<previous_name::PreviousNameSpec>;
#[doc = "PREVIOUS_NAME"]
pub mod previous_name;
#[doc = "HW_REG3 (rw) register accessor: HW_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg3`]
module"]
#[doc(alias = "HW_REG3")]
pub type HwReg3 = crate::Reg<hw_reg3::HwReg3Spec>;
#[doc = "HW_REG3"]
pub mod hw_reg3;
#[doc = "HW_REG4 (rw) register accessor: HW_REG4\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg4`]
module"]
#[doc(alias = "HW_REG4")]
pub type HwReg4 = crate::Reg<hw_reg4::HwReg4Spec>;
#[doc = "HW_REG4"]
pub mod hw_reg4;
#[doc = "HW_REG5 (rw) register accessor: HW_REG5\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg5`]
module"]
#[doc(alias = "HW_REG5")]
pub type HwReg5 = crate::Reg<hw_reg5::HwReg5Spec>;
#[doc = "HW_REG5"]
pub mod hw_reg5;
#[doc = "HW_REG6 (rw) register accessor: HW_REG6\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg6`]
module"]
#[doc(alias = "HW_REG6")]
pub type HwReg6 = crate::Reg<hw_reg6::HwReg6Spec>;
#[doc = "HW_REG6"]
pub mod hw_reg6;
#[doc = "HW_REG7 (rw) register accessor: HW_REG7\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_reg7`]
module"]
#[doc(alias = "HW_REG7")]
pub type HwReg7 = crate::Reg<hw_reg7::HwReg7Spec>;
#[doc = "HW_REG7"]
pub mod hw_reg7;
#[doc = "APPSS_SW_INT (rw) register accessor: APPSS_SW_INT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_sw_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_sw_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_sw_int`]
module"]
#[doc(alias = "APPSS_SW_INT")]
pub type AppssSwInt = crate::Reg<appss_sw_int::AppssSwIntSpec>;
#[doc = "APPSS_SW_INT"]
pub mod appss_sw_int;
#[doc = "APPSS_IPC_RFS (rw) register accessor: APPSS_IPC_RFS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ipc_rfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ipc_rfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ipc_rfs`]
module"]
#[doc(alias = "APPSS_IPC_RFS")]
pub type AppssIpcRfs = crate::Reg<appss_ipc_rfs::AppssIpcRfsSpec>;
#[doc = "APPSS_IPC_RFS"]
pub mod appss_ipc_rfs;
#[doc = "APPSS_CAPEVNT_SEL (rw) register accessor: APPSS_CAPEVNT_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_capevnt_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_capevnt_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_capevnt_sel`]
module"]
#[doc(alias = "APPSS_CAPEVNT_SEL")]
pub type AppssCapevntSel = crate::Reg<appss_capevnt_sel::AppssCapevntSelSpec>;
#[doc = "APPSS_CAPEVNT_SEL"]
pub mod appss_capevnt_sel;
#[doc = "APPSS_DMA_REQ_SEL (rw) register accessor: APPSS_DMA_REQ_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_dma_req_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_dma_req_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_dma_req_sel`]
module"]
#[doc(alias = "APPSS_DMA_REQ_SEL")]
pub type AppssDmaReqSel = crate::Reg<appss_dma_req_sel::AppssDmaReqSelSpec>;
#[doc = "APPSS_DMA_REQ_SEL"]
pub mod appss_dma_req_sel;
#[doc = "APPSS_DMA1_REQ_SEL (rw) register accessor: APPSS_DMA1_REQ_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_dma1_req_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_dma1_req_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_dma1_req_sel`]
module"]
#[doc(alias = "APPSS_DMA1_REQ_SEL")]
pub type AppssDma1ReqSel = crate::Reg<appss_dma1_req_sel::AppssDma1ReqSelSpec>;
#[doc = "APPSS_DMA1_REQ_SEL"]
pub mod appss_dma1_req_sel;
#[doc = "APPSS_IRQ_REQ_SEL (rw) register accessor: APPSS_IRQ_REQ_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_irq_req_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_irq_req_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_irq_req_sel`]
module"]
#[doc(alias = "APPSS_IRQ_REQ_SEL")]
pub type AppssIrqReqSel = crate::Reg<appss_irq_req_sel::AppssIrqReqSelSpec>;
#[doc = "APPSS_IRQ_REQ_SEL"]
pub mod appss_irq_req_sel;
#[doc = "APPSS_SPI_TRIG_SRC (rw) register accessor: APPSS_SPI_TRIG_SRC\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_spi_trig_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_spi_trig_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_spi_trig_src`]
module"]
#[doc(alias = "APPSS_SPI_TRIG_SRC")]
pub type AppssSpiTrigSrc = crate::Reg<appss_spi_trig_src::AppssSpiTrigSrcSpec>;
#[doc = "APPSS_SPI_TRIG_SRC"]
pub mod appss_spi_trig_src;
#[doc = "APPSS_RAM1A_MEM_INIT (rw) register accessor: APPSS_RAM1A_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram1a_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram1a_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram1a_mem_init`]
module"]
#[doc(alias = "APPSS_RAM1A_MEM_INIT")]
pub type AppssRam1aMemInit = crate::Reg<appss_ram1a_mem_init::AppssRam1aMemInitSpec>;
#[doc = "APPSS_RAM1A_MEM_INIT"]
pub mod appss_ram1a_mem_init;
#[doc = "APPSS_RAM1A_MEM_INIT_DONE (rw) register accessor: APPSS_RAM1A_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram1a_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram1a_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram1a_mem_init_done`]
module"]
#[doc(alias = "APPSS_RAM1A_MEM_INIT_DONE")]
pub type AppssRam1aMemInitDone = crate::Reg<appss_ram1a_mem_init_done::AppssRam1aMemInitDoneSpec>;
#[doc = "APPSS_RAM1A_MEM_INIT_DONE"]
pub mod appss_ram1a_mem_init_done;
#[doc = "APPSS_RAM1A_MEM_INIT_STATUS (rw) register accessor: APPSS_RAM1A_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram1a_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram1a_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram1a_mem_init_status`]
module"]
#[doc(alias = "APPSS_RAM1A_MEM_INIT_STATUS")]
pub type AppssRam1aMemInitStatus =
    crate::Reg<appss_ram1a_mem_init_status::AppssRam1aMemInitStatusSpec>;
#[doc = "APPSS_RAM1A_MEM_INIT_STATUS"]
pub mod appss_ram1a_mem_init_status;
#[doc = "APPSS_RAM2A_MEM_INIT (rw) register accessor: APPSS_RAM2A_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram2a_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram2a_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram2a_mem_init`]
module"]
#[doc(alias = "APPSS_RAM2A_MEM_INIT")]
pub type AppssRam2aMemInit = crate::Reg<appss_ram2a_mem_init::AppssRam2aMemInitSpec>;
#[doc = "APPSS_RAM2A_MEM_INIT"]
pub mod appss_ram2a_mem_init;
#[doc = "APPSS_RAM2A_MEM_INIT_DONE (rw) register accessor: APPSS_RAM2A_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram2a_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram2a_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram2a_mem_init_done`]
module"]
#[doc(alias = "APPSS_RAM2A_MEM_INIT_DONE")]
pub type AppssRam2aMemInitDone = crate::Reg<appss_ram2a_mem_init_done::AppssRam2aMemInitDoneSpec>;
#[doc = "APPSS_RAM2A_MEM_INIT_DONE"]
pub mod appss_ram2a_mem_init_done;
#[doc = "APPSS_RAM2A_MEM_INIT_STATUS (rw) register accessor: APPSS_RAM2A_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram2a_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram2a_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram2a_mem_init_status`]
module"]
#[doc(alias = "APPSS_RAM2A_MEM_INIT_STATUS")]
pub type AppssRam2aMemInitStatus =
    crate::Reg<appss_ram2a_mem_init_status::AppssRam2aMemInitStatusSpec>;
#[doc = "APPSS_RAM2A_MEM_INIT_STATUS"]
pub mod appss_ram2a_mem_init_status;
#[doc = "APPSS_RAM3A_MEM_INIT (rw) register accessor: APPSS_RAM3A_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram3a_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram3a_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram3a_mem_init`]
module"]
#[doc(alias = "APPSS_RAM3A_MEM_INIT")]
pub type AppssRam3aMemInit = crate::Reg<appss_ram3a_mem_init::AppssRam3aMemInitSpec>;
#[doc = "APPSS_RAM3A_MEM_INIT"]
pub mod appss_ram3a_mem_init;
#[doc = "APPSS_RAM3A_MEM_INIT_DONE (rw) register accessor: APPSS_RAM3A_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram3a_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram3a_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram3a_mem_init_done`]
module"]
#[doc(alias = "APPSS_RAM3A_MEM_INIT_DONE")]
pub type AppssRam3aMemInitDone = crate::Reg<appss_ram3a_mem_init_done::AppssRam3aMemInitDoneSpec>;
#[doc = "APPSS_RAM3A_MEM_INIT_DONE"]
pub mod appss_ram3a_mem_init_done;
#[doc = "APPSS_RAM3A_MEM_INIT_STATUS (rw) register accessor: APPSS_RAM3A_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram3a_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram3a_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram3a_mem_init_status`]
module"]
#[doc(alias = "APPSS_RAM3A_MEM_INIT_STATUS")]
pub type AppssRam3aMemInitStatus =
    crate::Reg<appss_ram3a_mem_init_status::AppssRam3aMemInitStatusSpec>;
#[doc = "APPSS_RAM3A_MEM_INIT_STATUS"]
pub mod appss_ram3a_mem_init_status;
#[doc = "HWASS_SHRD_RAM0_MEM_INIT (rw) register accessor: HWASS_SHRD_RAM0_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram0_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram0_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram0_mem_init`]
module"]
#[doc(alias = "HWASS_SHRD_RAM0_MEM_INIT")]
pub type HwassShrdRam0MemInit = crate::Reg<hwass_shrd_ram0_mem_init::HwassShrdRam0MemInitSpec>;
#[doc = "HWASS_SHRD_RAM0_MEM_INIT"]
pub mod hwass_shrd_ram0_mem_init;
#[doc = "HWASS_SHRD_RAM0_MEM_INIT_DONE (rw) register accessor: HWASS_SHRD_RAM0_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram0_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram0_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram0_mem_init_done`]
module"]
#[doc(alias = "HWASS_SHRD_RAM0_MEM_INIT_DONE")]
pub type HwassShrdRam0MemInitDone =
    crate::Reg<hwass_shrd_ram0_mem_init_done::HwassShrdRam0MemInitDoneSpec>;
#[doc = "HWASS_SHRD_RAM0_MEM_INIT_DONE"]
pub mod hwass_shrd_ram0_mem_init_done;
#[doc = "HWASS_SHRD_RAM0_MEM_INIT_STATUS (rw) register accessor: HWASS_SHRD_RAM0_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram0_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram0_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram0_mem_init_status`]
module"]
#[doc(alias = "HWASS_SHRD_RAM0_MEM_INIT_STATUS")]
pub type HwassShrdRam0MemInitStatus =
    crate::Reg<hwass_shrd_ram0_mem_init_status::HwassShrdRam0MemInitStatusSpec>;
#[doc = "HWASS_SHRD_RAM0_MEM_INIT_STATUS"]
pub mod hwass_shrd_ram0_mem_init_status;
#[doc = "HWASS_SHRD_RAM1_MEM_INIT (rw) register accessor: HWASS_SHRD_RAM1_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram1_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram1_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram1_mem_init`]
module"]
#[doc(alias = "HWASS_SHRD_RAM1_MEM_INIT")]
pub type HwassShrdRam1MemInit = crate::Reg<hwass_shrd_ram1_mem_init::HwassShrdRam1MemInitSpec>;
#[doc = "HWASS_SHRD_RAM1_MEM_INIT"]
pub mod hwass_shrd_ram1_mem_init;
#[doc = "HWASS_SHRD_RAM1_MEM_INIT_DONE (rw) register accessor: HWASS_SHRD_RAM1_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram1_mem_init_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram1_mem_init_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram1_mem_init_done`]
module"]
#[doc(alias = "HWASS_SHRD_RAM1_MEM_INIT_DONE")]
pub type HwassShrdRam1MemInitDone =
    crate::Reg<hwass_shrd_ram1_mem_init_done::HwassShrdRam1MemInitDoneSpec>;
#[doc = "HWASS_SHRD_RAM1_MEM_INIT_DONE"]
pub mod hwass_shrd_ram1_mem_init_done;
#[doc = "HWASS_SHRD_RAM1_MEM_INIT_STATUS (rw) register accessor: HWASS_SHRD_RAM1_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram1_mem_init_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram1_mem_init_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram1_mem_init_status`]
module"]
#[doc(alias = "HWASS_SHRD_RAM1_MEM_INIT_STATUS")]
pub type HwassShrdRam1MemInitStatus =
    crate::Reg<hwass_shrd_ram1_mem_init_status::HwassShrdRam1MemInitStatusSpec>;
#[doc = "HWASS_SHRD_RAM1_MEM_INIT_STATUS"]
pub mod hwass_shrd_ram1_mem_init_status;
#[doc = "APPSS_TPCC_MEMINIT_START (rw) register accessor: APPSS_TPCC_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_meminit_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_meminit_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_meminit_start`]
module"]
#[doc(alias = "APPSS_TPCC_MEMINIT_START")]
pub type AppssTpccMeminitStart = crate::Reg<appss_tpcc_meminit_start::AppssTpccMeminitStartSpec>;
#[doc = "APPSS_TPCC_MEMINIT_START"]
pub mod appss_tpcc_meminit_start;
#[doc = "APPSS_TPCC_MEMINIT_DONE (rw) register accessor: APPSS_TPCC_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_meminit_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_meminit_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_meminit_done`]
module"]
#[doc(alias = "APPSS_TPCC_MEMINIT_DONE")]
pub type AppssTpccMeminitDone = crate::Reg<appss_tpcc_meminit_done::AppssTpccMeminitDoneSpec>;
#[doc = "APPSS_TPCC_MEMINIT_DONE"]
pub mod appss_tpcc_meminit_done;
#[doc = "APPSS_TPCC_MEMINIT_STATUS (rw) register accessor: APPSS_TPCC_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_meminit_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_meminit_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_meminit_status`]
module"]
#[doc(alias = "APPSS_TPCC_MEMINIT_STATUS")]
pub type AppssTpccMeminitStatus = crate::Reg<appss_tpcc_meminit_status::AppssTpccMeminitStatusSpec>;
#[doc = "APPSS_TPCC_MEMINIT_STATUS"]
pub mod appss_tpcc_meminit_status;
#[doc = "APPSS_SPIA_CFG (rw) register accessor: APPSS_SPIA_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_spia_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_spia_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_spia_cfg`]
module"]
#[doc(alias = "APPSS_SPIA_CFG")]
pub type AppssSpiaCfg = crate::Reg<appss_spia_cfg::AppssSpiaCfgSpec>;
#[doc = "APPSS_SPIA_CFG"]
pub mod appss_spia_cfg;
#[doc = "APPSS_SPIB_CFG (rw) register accessor: APPSS_SPIB_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_spib_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_spib_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_spib_cfg`]
module"]
#[doc(alias = "APPSS_SPIB_CFG")]
pub type AppssSpibCfg = crate::Reg<appss_spib_cfg::AppssSpibCfgSpec>;
#[doc = "APPSS_SPIB_CFG"]
pub mod appss_spib_cfg;
#[doc = "APPSS_EPWM_CFG (rw) register accessor: APPSS_EPWM_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_epwm_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_epwm_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_epwm_cfg`]
module"]
#[doc(alias = "APPSS_EPWM_CFG")]
pub type AppssEpwmCfg = crate::Reg<appss_epwm_cfg::AppssEpwmCfgSpec>;
#[doc = "APPSS_EPWM_CFG"]
pub mod appss_epwm_cfg;
#[doc = "RESERVED (rw) register accessor: RESERVED\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`]
module"]
#[doc(alias = "RESERVED")]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = "RESERVED"]
pub mod reserved;
#[doc = "APPSS_MCAN_FE_AND_LIN_INTR_SEL (rw) register accessor: APPSS_MCAN_FE_AND_LIN_INTR_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mcan_fe_and_lin_intr_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mcan_fe_and_lin_intr_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_mcan_fe_and_lin_intr_sel`]
module"]
#[doc(alias = "APPSS_MCAN_FE_AND_LIN_INTR_SEL")]
pub type AppssMcanFeAndLinIntrSel =
    crate::Reg<appss_mcan_fe_and_lin_intr_sel::AppssMcanFeAndLinIntrSelSpec>;
#[doc = "APPSS_MCAN_FE_AND_LIN_INTR_SEL"]
pub mod appss_mcan_fe_and_lin_intr_sel;
#[doc = "APPSS_MCANA_INT_CLR (rw) register accessor: APPSS_MCANA_INT_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mcana_int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mcana_int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_mcana_int_clr`]
module"]
#[doc(alias = "APPSS_MCANA_INT_CLR")]
pub type AppssMcanaIntClr = crate::Reg<appss_mcana_int_clr::AppssMcanaIntClrSpec>;
#[doc = "APPSS_MCANA_INT_CLR"]
pub mod appss_mcana_int_clr;
#[doc = "APPSS_MCANA_INT_MASK (rw) register accessor: APPSS_MCANA_INT_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mcana_int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mcana_int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_mcana_int_mask`]
module"]
#[doc(alias = "APPSS_MCANA_INT_MASK")]
pub type AppssMcanaIntMask = crate::Reg<appss_mcana_int_mask::AppssMcanaIntMaskSpec>;
#[doc = "APPSS_MCANA_INT_MASK"]
pub mod appss_mcana_int_mask;
#[doc = "APPSS_MCANA_INT_STAT (rw) register accessor: APPSS_MCANA_INT_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mcana_int_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mcana_int_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_mcana_int_stat`]
module"]
#[doc(alias = "APPSS_MCANA_INT_STAT")]
pub type AppssMcanaIntStat = crate::Reg<appss_mcana_int_stat::AppssMcanaIntStatSpec>;
#[doc = "APPSS_MCANA_INT_STAT"]
pub mod appss_mcana_int_stat;
#[doc = "APPSS_CM4_GLOBAL_CONFIG (rw) register accessor: APPSS_CM4_GLOBAL_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_global_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_global_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_cm4_global_config`]
module"]
#[doc(alias = "APPSS_CM4_GLOBAL_CONFIG")]
pub type AppssCm4GlobalConfig = crate::Reg<appss_cm4_global_config::AppssCm4GlobalConfigSpec>;
#[doc = "APPSS_CM4_GLOBAL_CONFIG"]
pub mod appss_cm4_global_config;
#[doc = "RESERVED1 (rw) register accessor: RESERVED1\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved1`]
module"]
#[doc(alias = "RESERVED1")]
pub type Reserved1 = crate::Reg<reserved1::Reserved1Spec>;
#[doc = "RESERVED1"]
pub mod reserved1;
#[doc = "APPSS_CM4_ROM_ECLIPSE (rw) register accessor: APPSS_CM4_ROM_ECLIPSE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_rom_eclipse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_rom_eclipse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_cm4_rom_eclipse`]
module"]
#[doc(alias = "APPSS_CM4_ROM_ECLIPSE")]
pub type AppssCm4RomEclipse = crate::Reg<appss_cm4_rom_eclipse::AppssCm4RomEclipseSpec>;
#[doc = "APPSS_CM4_ROM_ECLIPSE"]
pub mod appss_cm4_rom_eclipse;
#[doc = "APPSS_CM4_STATUS_REG (rw) register accessor: APPSS_CM4_STATUS_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_cm4_status_reg`]
module"]
#[doc(alias = "APPSS_CM4_STATUS_REG")]
pub type AppssCm4StatusReg = crate::Reg<appss_cm4_status_reg::AppssCm4StatusRegSpec>;
#[doc = "APPSS_CM4_STATUS_REG"]
pub mod appss_cm4_status_reg;
#[doc = "APPSS_AHB_CTRL (rw) register accessor: APPSS_AHB_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ahb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ahb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ahb_ctrl`]
module"]
#[doc(alias = "APPSS_AHB_CTRL")]
pub type AppssAhbCtrl = crate::Reg<appss_ahb_ctrl::AppssAhbCtrlSpec>;
#[doc = "APPSS_AHB_CTRL"]
pub mod appss_ahb_ctrl;
#[doc = "ESM_GATING0 (rw) register accessor: ESM_GATING0\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating0`]
module"]
#[doc(alias = "ESM_GATING0")]
pub type EsmGating0 = crate::Reg<esm_gating0::EsmGating0Spec>;
#[doc = "ESM_GATING0"]
pub mod esm_gating0;
#[doc = "ESM_GATING1 (rw) register accessor: ESM_GATING1\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating1`]
module"]
#[doc(alias = "ESM_GATING1")]
pub type EsmGating1 = crate::Reg<esm_gating1::EsmGating1Spec>;
#[doc = "ESM_GATING1"]
pub mod esm_gating1;
#[doc = "ESM_GATING2 (rw) register accessor: ESM_GATING2\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating2`]
module"]
#[doc(alias = "ESM_GATING2")]
pub type EsmGating2 = crate::Reg<esm_gating2::EsmGating2Spec>;
#[doc = "ESM_GATING2"]
pub mod esm_gating2;
#[doc = "ESM_GATING3 (rw) register accessor: ESM_GATING3\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating3`]
module"]
#[doc(alias = "ESM_GATING3")]
pub type EsmGating3 = crate::Reg<esm_gating3::EsmGating3Spec>;
#[doc = "ESM_GATING3"]
pub mod esm_gating3;
#[doc = "ESM_GATING4 (rw) register accessor: ESM_GATING4\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating4`]
module"]
#[doc(alias = "ESM_GATING4")]
pub type EsmGating4 = crate::Reg<esm_gating4::EsmGating4Spec>;
#[doc = "ESM_GATING4"]
pub mod esm_gating4;
#[doc = "ESM_GATING5 (rw) register accessor: ESM_GATING5\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating5`]
module"]
#[doc(alias = "ESM_GATING5")]
pub type EsmGating5 = crate::Reg<esm_gating5::EsmGating5Spec>;
#[doc = "ESM_GATING5"]
pub mod esm_gating5;
#[doc = "ESM_GATING6 (rw) register accessor: ESM_GATING6\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating6`]
module"]
#[doc(alias = "ESM_GATING6")]
pub type EsmGating6 = crate::Reg<esm_gating6::EsmGating6Spec>;
#[doc = "ESM_GATING6"]
pub mod esm_gating6;
#[doc = "ESM_GATING7 (rw) register accessor: ESM_GATING7\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esm_gating7`]
module"]
#[doc(alias = "ESM_GATING7")]
pub type EsmGating7 = crate::Reg<esm_gating7::EsmGating7Spec>;
#[doc = "ESM_GATING7"]
pub mod esm_gating7;
#[doc = "APPSS_CM4_HALT (rw) register accessor: APPSS_CM4_HALT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_halt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_halt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_cm4_halt`]
module"]
#[doc(alias = "APPSS_CM4_HALT")]
pub type AppssCm4Halt = crate::Reg<appss_cm4_halt::AppssCm4HaltSpec>;
#[doc = "APPSS_CM4_HALT"]
pub mod appss_cm4_halt;
#[doc = "APPSS_CM4_EVENT (rw) register accessor: APPSS_CM4_EVENT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_event::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_event::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_cm4_event`]
module"]
#[doc(alias = "APPSS_CM4_EVENT")]
pub type AppssCm4Event = crate::Reg<appss_cm4_event::AppssCm4EventSpec>;
#[doc = "APPSS_CM4_EVENT"]
pub mod appss_cm4_event;
#[doc = "SPIA_IO_CFG (rw) register accessor: SPIA_IO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spia_io_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spia_io_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spia_io_cfg`]
module"]
#[doc(alias = "SPIA_IO_CFG")]
pub type SpiaIoCfg = crate::Reg<spia_io_cfg::SpiaIoCfgSpec>;
#[doc = "SPIA_IO_CFG"]
pub mod spia_io_cfg;
#[doc = "SPIB_IO_CFG (rw) register accessor: SPIB_IO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spib_io_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spib_io_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spib_io_cfg`]
module"]
#[doc(alias = "SPIB_IO_CFG")]
pub type SpibIoCfg = crate::Reg<spib_io_cfg::SpibIoCfgSpec>;
#[doc = "SPIB_IO_CFG"]
pub mod spib_io_cfg;
#[doc = "SPI_HOST_IRQ (rw) register accessor: SPI_HOST_IRQ\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_host_irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_host_irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_host_irq`]
module"]
#[doc(alias = "SPI_HOST_IRQ")]
pub type SpiHostIrq = crate::Reg<spi_host_irq::SpiHostIrqSpec>;
#[doc = "SPI_HOST_IRQ"]
pub mod spi_host_irq;
#[doc = "TPTC_DBS_CONFIG (rw) register accessor: TPTC_DBS_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tptc_dbs_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tptc_dbs_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tptc_dbs_config`]
module"]
#[doc(alias = "TPTC_DBS_CONFIG")]
pub type TptcDbsConfig = crate::Reg<tptc_dbs_config::TptcDbsConfigSpec>;
#[doc = "TPTC_DBS_CONFIG"]
pub mod tptc_dbs_config;
#[doc = "TPCC_PARITY_CTRL (rw) register accessor: TPCC_PARITY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`tpcc_parity_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpcc_parity_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpcc_parity_ctrl`]
module"]
#[doc(alias = "TPCC_PARITY_CTRL")]
pub type TpccParityCtrl = crate::Reg<tpcc_parity_ctrl::TpccParityCtrlSpec>;
#[doc = "TPCC_PARITY_CTRL"]
pub mod tpcc_parity_ctrl;
#[doc = "TPCC_PARITY_STATUS (rw) register accessor: TPCC_PARITY_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`tpcc_parity_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpcc_parity_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpcc_parity_status`]
module"]
#[doc(alias = "TPCC_PARITY_STATUS")]
pub type TpccParityStatus = crate::Reg<tpcc_parity_status::TpccParityStatusSpec>;
#[doc = "TPCC_PARITY_STATUS"]
pub mod tpcc_parity_status;
#[doc = "APPSS_DBG_ACK_CTL0 (rw) register accessor: APPSS_DBG_ACK_CTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_dbg_ack_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_dbg_ack_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_dbg_ack_ctl0`]
module"]
#[doc(alias = "APPSS_DBG_ACK_CTL0")]
pub type AppssDbgAckCtl0 = crate::Reg<appss_dbg_ack_ctl0::AppssDbgAckCtl0Spec>;
#[doc = "APPSS_DBG_ACK_CTL0"]
pub mod appss_dbg_ack_ctl0;
#[doc = "DEBUGSS_CSETB_FLUSH (rw) register accessor: DEBUGSS_CSETB_FLUSH\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_csetb_flush::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_csetb_flush::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debugss_csetb_flush`]
module"]
#[doc(alias = "DEBUGSS_CSETB_FLUSH")]
pub type DebugssCsetbFlush = crate::Reg<debugss_csetb_flush::DebugssCsetbFlushSpec>;
#[doc = "DEBUGSS_CSETB_FLUSH"]
pub mod debugss_csetb_flush;
#[doc = "CPSW_CONTROL (rw) register accessor: CPSW_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_control`]
module"]
#[doc(alias = "CPSW_CONTROL")]
pub type CpswControl = crate::Reg<cpsw_control::CpswControlSpec>;
#[doc = "CPSW_CONTROL"]
pub mod cpsw_control;
#[doc = "APPSS_ERRAGG_MASK0 (rw) register accessor: APPSS_ERRAGG_MASK0\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_erragg_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_erragg_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_erragg_mask0`]
module"]
#[doc(alias = "APPSS_ERRAGG_MASK0")]
pub type AppssErraggMask0 = crate::Reg<appss_erragg_mask0::AppssErraggMask0Spec>;
#[doc = "APPSS_ERRAGG_MASK0"]
pub mod appss_erragg_mask0;
#[doc = "APPSS_ERRAGG_STATUS0 (rw) register accessor: APPSS_ERRAGG_STATUS0\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_erragg_status0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_erragg_status0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_erragg_status0`]
module"]
#[doc(alias = "APPSS_ERRAGG_STATUS0")]
pub type AppssErraggStatus0 = crate::Reg<appss_erragg_status0::AppssErraggStatus0Spec>;
#[doc = "APPSS_ERRAGG_STATUS0"]
pub mod appss_erragg_status0;
#[doc = "APPSS_TPCC_A_ERRAGG_MASK (rw) register accessor: APPSS_TPCC_A_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_a_erragg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_a_erragg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_a_erragg_mask`]
module"]
#[doc(alias = "APPSS_TPCC_A_ERRAGG_MASK")]
pub type AppssTpccAErraggMask = crate::Reg<appss_tpcc_a_erragg_mask::AppssTpccAErraggMaskSpec>;
#[doc = "APPSS_TPCC_A_ERRAGG_MASK"]
pub mod appss_tpcc_a_erragg_mask;
#[doc = "APPSS_TPCC_A_ERRAGG_STATUS (rw) register accessor: APPSS_TPCC_A_ERRAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_a_erragg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_a_erragg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_a_erragg_status`]
module"]
#[doc(alias = "APPSS_TPCC_A_ERRAGG_STATUS")]
pub type AppssTpccAErraggStatus =
    crate::Reg<appss_tpcc_a_erragg_status::AppssTpccAErraggStatusSpec>;
#[doc = "APPSS_TPCC_A_ERRAGG_STATUS"]
pub mod appss_tpcc_a_erragg_status;
#[doc = "APPSS_TPCC_A_ERRAGG_STATUS_RAW (rw) register accessor: APPSS_TPCC_A_ERRAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_a_erragg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_a_erragg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_a_erragg_status_raw`]
module"]
#[doc(alias = "APPSS_TPCC_A_ERRAGG_STATUS_RAW")]
pub type AppssTpccAErraggStatusRaw =
    crate::Reg<appss_tpcc_a_erragg_status_raw::AppssTpccAErraggStatusRawSpec>;
#[doc = "APPSS_TPCC_A_ERRAGG_STATUS_RAW"]
pub mod appss_tpcc_a_erragg_status_raw;
#[doc = "APPSS_TPCC_A_INTAGG_MASK (rw) register accessor: APPSS_TPCC_A_INTAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_a_intagg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_a_intagg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_a_intagg_mask`]
module"]
#[doc(alias = "APPSS_TPCC_A_INTAGG_MASK")]
pub type AppssTpccAIntaggMask = crate::Reg<appss_tpcc_a_intagg_mask::AppssTpccAIntaggMaskSpec>;
#[doc = "APPSS_TPCC_A_INTAGG_MASK"]
pub mod appss_tpcc_a_intagg_mask;
#[doc = "HW_SPARE_WPH (rw) register accessor: HW_SPARE_WPH\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_wph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_wph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_wph`]
module"]
#[doc(alias = "HW_SPARE_WPH")]
pub type HwSpareWph = crate::Reg<hw_spare_wph::HwSpareWphSpec>;
#[doc = "HW_SPARE_WPH"]
pub mod hw_spare_wph;
#[doc = "APPSS_TPCC_A_INTAGG_STATUS_RAW (rw) register accessor: APPSS_TPCC_A_INTAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_a_intagg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_a_intagg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_a_intagg_status_raw`]
module"]
#[doc(alias = "APPSS_TPCC_A_INTAGG_STATUS_RAW")]
pub type AppssTpccAIntaggStatusRaw =
    crate::Reg<appss_tpcc_a_intagg_status_raw::AppssTpccAIntaggStatusRawSpec>;
#[doc = "APPSS_TPCC_A_INTAGG_STATUS_RAW"]
pub mod appss_tpcc_a_intagg_status_raw;
#[doc = "APPSS_TPCC_B_ERRAGG_MASK (rw) register accessor: APPSS_TPCC_B_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_erragg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_erragg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_b_erragg_mask`]
module"]
#[doc(alias = "APPSS_TPCC_B_ERRAGG_MASK")]
pub type AppssTpccBErraggMask = crate::Reg<appss_tpcc_b_erragg_mask::AppssTpccBErraggMaskSpec>;
#[doc = "APPSS_TPCC_B_ERRAGG_MASK"]
pub mod appss_tpcc_b_erragg_mask;
#[doc = "APPSS_TPCC_B_ERRAGG_STATUS (rw) register accessor: APPSS_TPCC_B_ERRAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_erragg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_erragg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_b_erragg_status`]
module"]
#[doc(alias = "APPSS_TPCC_B_ERRAGG_STATUS")]
pub type AppssTpccBErraggStatus =
    crate::Reg<appss_tpcc_b_erragg_status::AppssTpccBErraggStatusSpec>;
#[doc = "APPSS_TPCC_B_ERRAGG_STATUS"]
pub mod appss_tpcc_b_erragg_status;
#[doc = "APPSS_TPCC_B_ERRAGG_STATUS_RAW (rw) register accessor: APPSS_TPCC_B_ERRAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_erragg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_erragg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_b_erragg_status_raw`]
module"]
#[doc(alias = "APPSS_TPCC_B_ERRAGG_STATUS_RAW")]
pub type AppssTpccBErraggStatusRaw =
    crate::Reg<appss_tpcc_b_erragg_status_raw::AppssTpccBErraggStatusRawSpec>;
#[doc = "APPSS_TPCC_B_ERRAGG_STATUS_RAW"]
pub mod appss_tpcc_b_erragg_status_raw;
#[doc = "APPSS_TPCC_B_INTAGG_MASK (rw) register accessor: APPSS_TPCC_B_INTAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_intagg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_intagg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_b_intagg_mask`]
module"]
#[doc(alias = "APPSS_TPCC_B_INTAGG_MASK")]
pub type AppssTpccBIntaggMask = crate::Reg<appss_tpcc_b_intagg_mask::AppssTpccBIntaggMaskSpec>;
#[doc = "APPSS_TPCC_B_INTAGG_MASK"]
pub mod appss_tpcc_b_intagg_mask;
#[doc = "APPSS_TPCC_B_INTAGG_STATUS (rw) register accessor: APPSS_TPCC_B_INTAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_intagg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_intagg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_b_intagg_status`]
module"]
#[doc(alias = "APPSS_TPCC_B_INTAGG_STATUS")]
pub type AppssTpccBIntaggStatus =
    crate::Reg<appss_tpcc_b_intagg_status::AppssTpccBIntaggStatusSpec>;
#[doc = "APPSS_TPCC_B_INTAGG_STATUS"]
pub mod appss_tpcc_b_intagg_status;
#[doc = "APPSS_TPCC_B_INTAGG_STATUS_RAW (rw) register accessor: APPSS_TPCC_B_INTAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_intagg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_intagg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tpcc_b_intagg_status_raw`]
module"]
#[doc(alias = "APPSS_TPCC_B_INTAGG_STATUS_RAW")]
pub type AppssTpccBIntaggStatusRaw =
    crate::Reg<appss_tpcc_b_intagg_status_raw::AppssTpccBIntaggStatusRawSpec>;
#[doc = "APPSS_TPCC_B_INTAGG_STATUS_RAW"]
pub mod appss_tpcc_b_intagg_status_raw;
#[doc = "APPSS_MPU_ERRAGG_MASK (rw) register accessor: APPSS_MPU_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mpu_erragg_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mpu_erragg_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_mpu_erragg_mask`]
module"]
#[doc(alias = "APPSS_MPU_ERRAGG_MASK")]
pub type AppssMpuErraggMask = crate::Reg<appss_mpu_erragg_mask::AppssMpuErraggMaskSpec>;
#[doc = "APPSS_MPU_ERRAGG_MASK"]
pub mod appss_mpu_erragg_mask;
#[doc = "APPSS_MPU_ERRAGG_STATUS (rw) register accessor: APPSS_MPU_ERRAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mpu_erragg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mpu_erragg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_mpu_erragg_status`]
module"]
#[doc(alias = "APPSS_MPU_ERRAGG_STATUS")]
pub type AppssMpuErraggStatus = crate::Reg<appss_mpu_erragg_status::AppssMpuErraggStatusSpec>;
#[doc = "APPSS_MPU_ERRAGG_STATUS"]
pub mod appss_mpu_erragg_status;
#[doc = "APPSS_MPU_ERRAGG_STATUS_RAW (rw) register accessor: APPSS_MPU_ERRAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mpu_erragg_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mpu_erragg_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_mpu_erragg_status_raw`]
module"]
#[doc(alias = "APPSS_MPU_ERRAGG_STATUS_RAW")]
pub type AppssMpuErraggStatusRaw =
    crate::Reg<appss_mpu_erragg_status_raw::AppssMpuErraggStatusRawSpec>;
#[doc = "APPSS_MPU_ERRAGG_STATUS_RAW"]
pub mod appss_mpu_erragg_status_raw;
#[doc = "APPSS_QSPI_CONFIG (rw) register accessor: APPSS_QSPI_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_qspi_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_qspi_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_qspi_config`]
module"]
#[doc(alias = "APPSS_QSPI_CONFIG")]
pub type AppssQspiConfig = crate::Reg<appss_qspi_config::AppssQspiConfigSpec>;
#[doc = "APPSS_QSPI_CONFIG"]
pub mod appss_qspi_config;
#[doc = "APPSS_CTI_TRIG_SEL (rw) register accessor: APPSS_CTI_TRIG_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cti_trig_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cti_trig_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_cti_trig_sel`]
module"]
#[doc(alias = "APPSS_CTI_TRIG_SEL")]
pub type AppssCtiTrigSel = crate::Reg<appss_cti_trig_sel::AppssCtiTrigSelSpec>;
#[doc = "APPSS_CTI_TRIG_SEL"]
pub mod appss_cti_trig_sel;
#[doc = "APPSS_DBGSS_CTI_TRIG_SEL (rw) register accessor: APPSS_DBGSS_CTI_TRIG_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_dbgss_cti_trig_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_dbgss_cti_trig_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_dbgss_cti_trig_sel`]
module"]
#[doc(alias = "APPSS_DBGSS_CTI_TRIG_SEL")]
pub type AppssDbgssCtiTrigSel = crate::Reg<appss_dbgss_cti_trig_sel::AppssDbgssCtiTrigSelSpec>;
#[doc = "APPSS_DBGSS_CTI_TRIG_SEL"]
pub mod appss_dbgss_cti_trig_sel;
#[doc = "APPSS_BOOT_INFO_REG0 (rw) register accessor: APPSS_BOOT_INFO_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_boot_info_reg0`]
module"]
#[doc(alias = "APPSS_BOOT_INFO_REG0")]
pub type AppssBootInfoReg0 = crate::Reg<appss_boot_info_reg0::AppssBootInfoReg0Spec>;
#[doc = "APPSS_BOOT_INFO_REG0"]
pub mod appss_boot_info_reg0;
#[doc = "APPSS_BOOT_INFO_REG1 (rw) register accessor: APPSS_BOOT_INFO_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_boot_info_reg1`]
module"]
#[doc(alias = "APPSS_BOOT_INFO_REG1")]
pub type AppssBootInfoReg1 = crate::Reg<appss_boot_info_reg1::AppssBootInfoReg1Spec>;
#[doc = "APPSS_BOOT_INFO_REG1"]
pub mod appss_boot_info_reg1;
#[doc = "APPSS_BOOT_INFO_REG2 (rw) register accessor: APPSS_BOOT_INFO_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_boot_info_reg2`]
module"]
#[doc(alias = "APPSS_BOOT_INFO_REG2")]
pub type AppssBootInfoReg2 = crate::Reg<appss_boot_info_reg2::AppssBootInfoReg2Spec>;
#[doc = "APPSS_BOOT_INFO_REG2"]
pub mod appss_boot_info_reg2;
#[doc = "APPSS_BOOT_INFO_REG3 (rw) register accessor: APPSS_BOOT_INFO_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_boot_info_reg3`]
module"]
#[doc(alias = "APPSS_BOOT_INFO_REG3")]
pub type AppssBootInfoReg3 = crate::Reg<appss_boot_info_reg3::AppssBootInfoReg3Spec>;
#[doc = "APPSS_BOOT_INFO_REG3"]
pub mod appss_boot_info_reg3;
#[doc = "APPSS_BOOT_INFO_REG4 (rw) register accessor: APPSS_BOOT_INFO_REG4\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_boot_info_reg4`]
module"]
#[doc(alias = "APPSS_BOOT_INFO_REG4")]
pub type AppssBootInfoReg4 = crate::Reg<appss_boot_info_reg4::AppssBootInfoReg4Spec>;
#[doc = "APPSS_BOOT_INFO_REG4"]
pub mod appss_boot_info_reg4;
#[doc = "APPSS_BOOT_INFO_REG5 (rw) register accessor: APPSS_BOOT_INFO_REG5\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_boot_info_reg5`]
module"]
#[doc(alias = "APPSS_BOOT_INFO_REG5")]
pub type AppssBootInfoReg5 = crate::Reg<appss_boot_info_reg5::AppssBootInfoReg5Spec>;
#[doc = "APPSS_BOOT_INFO_REG5"]
pub mod appss_boot_info_reg5;
#[doc = "APPSS_BOOT_INFO_REG6 (rw) register accessor: APPSS_BOOT_INFO_REG6\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_boot_info_reg6`]
module"]
#[doc(alias = "APPSS_BOOT_INFO_REG6")]
pub type AppssBootInfoReg6 = crate::Reg<appss_boot_info_reg6::AppssBootInfoReg6Spec>;
#[doc = "APPSS_BOOT_INFO_REG6"]
pub mod appss_boot_info_reg6;
#[doc = "APPSS_BOOT_INFO_REG7 (rw) register accessor: APPSS_BOOT_INFO_REG7\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_boot_info_reg7`]
module"]
#[doc(alias = "APPSS_BOOT_INFO_REG7")]
pub type AppssBootInfoReg7 = crate::Reg<appss_boot_info_reg7::AppssBootInfoReg7Spec>;
#[doc = "APPSS_BOOT_INFO_REG7"]
pub mod appss_boot_info_reg7;
#[doc = "APPSS_TPTC_ECCAGGR_CLK_CNTRL (rw) register accessor: APPSS_TPTC_ECCAGGR_CLK_CNTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tptc_eccaggr_clk_cntrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tptc_eccaggr_clk_cntrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tptc_eccaggr_clk_cntrl`]
module"]
#[doc(alias = "APPSS_TPTC_ECCAGGR_CLK_CNTRL")]
pub type AppssTptcEccaggrClkCntrl =
    crate::Reg<appss_tptc_eccaggr_clk_cntrl::AppssTptcEccaggrClkCntrlSpec>;
#[doc = "APPSS_TPTC_ECCAGGR_CLK_CNTRL"]
pub mod appss_tptc_eccaggr_clk_cntrl;
#[doc = "APPSS_TPTC_BOUNDARY_CFG (rw) register accessor: APPSS_TPTC_BOUNDARY_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tptc_boundary_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tptc_boundary_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tptc_boundary_cfg`]
module"]
#[doc(alias = "APPSS_TPTC_BOUNDARY_CFG")]
pub type AppssTptcBoundaryCfg = crate::Reg<appss_tptc_boundary_cfg::AppssTptcBoundaryCfgSpec>;
#[doc = "APPSS_TPTC_BOUNDARY_CFG"]
pub mod appss_tptc_boundary_cfg;
#[doc = "APPSS_TPTC_XID_REORDER_CFG (rw) register accessor: APPSS_TPTC_XID_REORDER_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tptc_xid_reorder_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tptc_xid_reorder_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_tptc_xid_reorder_cfg`]
module"]
#[doc(alias = "APPSS_TPTC_XID_REORDER_CFG")]
pub type AppssTptcXidReorderCfg =
    crate::Reg<appss_tptc_xid_reorder_cfg::AppssTptcXidReorderCfgSpec>;
#[doc = "APPSS_TPTC_XID_REORDER_CFG"]
pub mod appss_tptc_xid_reorder_cfg;
#[doc = "HW_Sync_FE_CTRL (rw) register accessor: HW_Sync_FE_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_sync_fe_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_sync_fe_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_sync_fe_ctrl`]
module"]
#[doc(alias = "HW_Sync_FE_CTRL")]
pub type HwSyncFeCtrl = crate::Reg<hw_sync_fe_ctrl::HwSyncFeCtrlSpec>;
#[doc = "HW_Sync_FE_CTRL"]
pub mod hw_sync_fe_ctrl;
#[doc = "HW_SPARE_REG1 (rw) register accessor: HW_SPARE_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_reg1`]
module"]
#[doc(alias = "HW_SPARE_REG1")]
pub type HwSpareReg1 = crate::Reg<hw_spare_reg1::HwSpareReg1Spec>;
#[doc = "HW_SPARE_REG1"]
pub mod hw_spare_reg1;
#[doc = "HW_SPARE_REG2 (rw) register accessor: HW_SPARE_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_reg2`]
module"]
#[doc(alias = "HW_SPARE_REG2")]
pub type HwSpareReg2 = crate::Reg<hw_spare_reg2::HwSpareReg2Spec>;
#[doc = "HW_SPARE_REG2"]
pub mod hw_spare_reg2;
#[doc = "HW_SPARE_REG3 (rw) register accessor: HW_SPARE_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_reg3`]
module"]
#[doc(alias = "HW_SPARE_REG3")]
pub type HwSpareReg3 = crate::Reg<hw_spare_reg3::HwSpareReg3Spec>;
#[doc = "HW_SPARE_REG3"]
pub mod hw_spare_reg3;
#[doc = "NERROR_MASK (rw) register accessor: NERROR_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`nerror_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nerror_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nerror_mask`]
module"]
#[doc(alias = "NERROR_MASK")]
pub type NerrorMask = crate::Reg<nerror_mask::NerrorMaskSpec>;
#[doc = "NERROR_MASK"]
pub mod nerror_mask;
#[doc = "HW_SPARE_RW0 (rw) register accessor: HW_SPARE_RW0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw0`]
module"]
#[doc(alias = "HW_SPARE_RW0")]
pub type HwSpareRw0 = crate::Reg<hw_spare_rw0::HwSpareRw0Spec>;
#[doc = "HW_SPARE_RW0"]
pub mod hw_spare_rw0;
#[doc = "HW_SPARE_RW1 (rw) register accessor: HW_SPARE_RW1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw1`]
module"]
#[doc(alias = "HW_SPARE_RW1")]
pub type HwSpareRw1 = crate::Reg<hw_spare_rw1::HwSpareRw1Spec>;
#[doc = "HW_SPARE_RW1"]
pub mod hw_spare_rw1;
#[doc = "HW_SPARE_RW2 (rw) register accessor: HW_SPARE_RW2\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw2`]
module"]
#[doc(alias = "HW_SPARE_RW2")]
pub type HwSpareRw2 = crate::Reg<hw_spare_rw2::HwSpareRw2Spec>;
#[doc = "HW_SPARE_RW2"]
pub mod hw_spare_rw2;
#[doc = "HW_SPARE_RW3 (rw) register accessor: HW_SPARE_RW3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw3`]
module"]
#[doc(alias = "HW_SPARE_RW3")]
pub type HwSpareRw3 = crate::Reg<hw_spare_rw3::HwSpareRw3Spec>;
#[doc = "HW_SPARE_RW3"]
pub mod hw_spare_rw3;
#[doc = "HW_SPARE_RW4 (rw) register accessor: HW_SPARE_RW4\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw4`]
module"]
#[doc(alias = "HW_SPARE_RW4")]
pub type HwSpareRw4 = crate::Reg<hw_spare_rw4::HwSpareRw4Spec>;
#[doc = "HW_SPARE_RW4"]
pub mod hw_spare_rw4;
#[doc = "HW_SPARE_RW5 (rw) register accessor: HW_SPARE_RW5\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw5`]
module"]
#[doc(alias = "HW_SPARE_RW5")]
pub type HwSpareRw5 = crate::Reg<hw_spare_rw5::HwSpareRw5Spec>;
#[doc = "HW_SPARE_RW5"]
pub mod hw_spare_rw5;
#[doc = "HW_SPARE_RO0 (rw) register accessor: HW_SPARE_RO0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_ro0`]
module"]
#[doc(alias = "HW_SPARE_RO0")]
pub type HwSpareRo0 = crate::Reg<hw_spare_ro0::HwSpareRo0Spec>;
#[doc = "HW_SPARE_RO0"]
pub mod hw_spare_ro0;
#[doc = "HW_SPARE_RO1 (rw) register accessor: HW_SPARE_RO1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_ro1`]
module"]
#[doc(alias = "HW_SPARE_RO1")]
pub type HwSpareRo1 = crate::Reg<hw_spare_ro1::HwSpareRo1Spec>;
#[doc = "HW_SPARE_RO1"]
pub mod hw_spare_ro1;
#[doc = "HW_SPARE_RO2 (rw) register accessor: HW_SPARE_RO2\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_ro2`]
module"]
#[doc(alias = "HW_SPARE_RO2")]
pub type HwSpareRo2 = crate::Reg<hw_spare_ro2::HwSpareRo2Spec>;
#[doc = "HW_SPARE_RO2"]
pub mod hw_spare_ro2;
#[doc = "HW_SPARE_RO3 (rw) register accessor: HW_SPARE_RO3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_ro3`]
module"]
#[doc(alias = "HW_SPARE_RO3")]
pub type HwSpareRo3 = crate::Reg<hw_spare_ro3::HwSpareRo3Spec>;
#[doc = "HW_SPARE_RO3"]
pub mod hw_spare_ro3;
#[doc = "HW_SPARE_REC (rw) register accessor: HW_SPARE_REC\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rec`]
module"]
#[doc(alias = "HW_SPARE_REC")]
pub type HwSpareRec = crate::Reg<hw_spare_rec::HwSpareRecSpec>;
#[doc = "HW_SPARE_REC"]
pub mod hw_spare_rec;
#[doc = "APP_CTRL (rw) register accessor: APP_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_ctrl`]
module"]
#[doc(alias = "APP_CTRL")]
pub type AppCtrl = crate::Reg<app_ctrl::AppCtrlSpec>;
#[doc = "APP_CTRL"]
pub mod app_ctrl;
#[doc = "WIC_CTRL (rw) register accessor: WIC_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`wic_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wic_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wic_ctrl`]
module"]
#[doc(alias = "WIC_CTRL")]
pub type WicCtrl = crate::Reg<wic_ctrl::WicCtrlSpec>;
#[doc = "WIC_CTRL"]
pub mod wic_ctrl;
#[doc = "WIC_STAT_CLR (rw) register accessor: WIC_STAT_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`wic_stat_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wic_stat_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wic_stat_clr`]
module"]
#[doc(alias = "WIC_STAT_CLR")]
pub type WicStatClr = crate::Reg<wic_stat_clr::WicStatClrSpec>;
#[doc = "WIC_STAT_CLR"]
pub mod wic_stat_clr;
#[doc = "WIC_STAT (rw) register accessor: WIC_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`wic_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wic_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wic_stat`]
module"]
#[doc(alias = "WIC_STAT")]
pub type WicStat = crate::Reg<wic_stat::WicStatSpec>;
#[doc = "WIC_STAT"]
pub mod wic_stat;
#[doc = "WICEN (rw) register accessor: WICEN\n\nYou can [`read`](crate::Reg::read) this register and get [`wicen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wicen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wicen`]
module"]
#[doc(alias = "WICEN")]
pub type Wicen = crate::Reg<wicen::WicenSpec>;
#[doc = "WICEN"]
pub mod wicen;
#[doc = "FORCEFCLKACTIVE (rw) register accessor: FORCEFCLKACTIVE\n\nYou can [`read`](crate::Reg::read) this register and get [`forcefclkactive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`forcefclkactive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@forcefclkactive`]
module"]
#[doc(alias = "FORCEFCLKACTIVE")]
pub type Forcefclkactive = crate::Reg<forcefclkactive::ForcefclkactiveSpec>;
#[doc = "FORCEFCLKACTIVE"]
pub mod forcefclkactive;
#[doc = "FECSS_CLK_GATE (rw) register accessor: FECSS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`fecss_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fecss_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fecss_clk_gate`]
module"]
#[doc(alias = "FECSS_CLK_GATE")]
pub type FecssClkGate = crate::Reg<fecss_clk_gate::FecssClkGateSpec>;
#[doc = "FECSS_CLK_GATE"]
pub mod fecss_clk_gate;
#[doc = "APPSS_SHARED_MEM_CLK_GATE (rw) register accessor: APPSS_SHARED_MEM_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_shared_mem_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_shared_mem_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_shared_mem_clk_gate`]
module"]
#[doc(alias = "APPSS_SHARED_MEM_CLK_GATE")]
pub type AppssSharedMemClkGate = crate::Reg<appss_shared_mem_clk_gate::AppssSharedMemClkGateSpec>;
#[doc = "APPSS_SHARED_MEM_CLK_GATE"]
pub mod appss_shared_mem_clk_gate;
#[doc = "APPSS_MEM_INIT_SLICE_SEL (rw) register accessor: APPSS_MEM_INIT_SLICE_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mem_init_slice_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mem_init_slice_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_mem_init_slice_sel`]
module"]
#[doc(alias = "APPSS_MEM_INIT_SLICE_SEL")]
pub type AppssMemInitSliceSel = crate::Reg<appss_mem_init_slice_sel::AppssMemInitSliceSelSpec>;
#[doc = "APPSS_MEM_INIT_SLICE_SEL"]
pub mod appss_mem_init_slice_sel;
#[doc = "APPSS_QSPI_CHAR_EXT_CLK_EN (rw) register accessor: APPSS_QSPI_CHAR_EXT_CLK_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_qspi_char_ext_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_qspi_char_ext_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_qspi_char_ext_clk_en`]
module"]
#[doc(alias = "APPSS_QSPI_CHAR_EXT_CLK_EN")]
pub type AppssQspiCharExtClkEn = crate::Reg<appss_qspi_char_ext_clk_en::AppssQspiCharExtClkEnSpec>;
#[doc = "APPSS_QSPI_CHAR_EXT_CLK_EN"]
pub mod appss_qspi_char_ext_clk_en;
#[doc = "APPSS_QSPI_EXT_CLK_EN (rw) register accessor: APPSS_QSPI_EXT_CLK_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_qspi_ext_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_qspi_ext_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_qspi_ext_clk_en`]
module"]
#[doc(alias = "APPSS_QSPI_EXT_CLK_EN")]
pub type AppssQspiExtClkEn = crate::Reg<appss_qspi_ext_clk_en::AppssQspiExtClkEnSpec>;
#[doc = "APPSS_QSPI_EXT_CLK_EN"]
pub mod appss_qspi_ext_clk_en;
#[doc = "SPI1_SMART_IDLE (rw) register accessor: SPI1_SMART_IDLE\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1_smart_idle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1_smart_idle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1_smart_idle`]
module"]
#[doc(alias = "SPI1_SMART_IDLE")]
pub type Spi1SmartIdle = crate::Reg<spi1_smart_idle::Spi1SmartIdleSpec>;
#[doc = "SPI1_SMART_IDLE"]
pub mod spi1_smart_idle;
#[doc = "SPI2_SMART_IDLE (rw) register accessor: SPI2_SMART_IDLE\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2_smart_idle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2_smart_idle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2_smart_idle`]
module"]
#[doc(alias = "SPI2_SMART_IDLE")]
pub type Spi2SmartIdle = crate::Reg<spi2_smart_idle::Spi2SmartIdleSpec>;
#[doc = "SPI2_SMART_IDLE"]
pub mod spi2_smart_idle;
#[doc = "CAN_SMART_IDLE (rw) register accessor: CAN_SMART_IDLE\n\nYou can [`read`](crate::Reg::read) this register and get [`can_smart_idle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_smart_idle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_smart_idle`]
module"]
#[doc(alias = "CAN_SMART_IDLE")]
pub type CanSmartIdle = crate::Reg<can_smart_idle::CanSmartIdleSpec>;
#[doc = "CAN_SMART_IDLE"]
pub mod can_smart_idle;
#[doc = "LIN_SMART_IDLE (rw) register accessor: LIN_SMART_IDLE\n\nYou can [`read`](crate::Reg::read) this register and get [`lin_smart_idle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lin_smart_idle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_smart_idle`]
module"]
#[doc(alias = "LIN_SMART_IDLE")]
pub type LinSmartIdle = crate::Reg<lin_smart_idle::LinSmartIdleSpec>;
#[doc = "LIN_SMART_IDLE"]
pub mod lin_smart_idle;
#[doc = "HWASS_CLK_GATE (rw) register accessor: HWASS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_clk_gate`]
module"]
#[doc(alias = "HWASS_CLK_GATE")]
pub type HwassClkGate = crate::Reg<hwass_clk_gate::HwassClkGateSpec>;
#[doc = "HWASS_CLK_GATE"]
pub mod hwass_clk_gate;
#[doc = "CFG_TIMEOUT_PCR3 (rw) register accessor: CFG_TIMEOUT_PCR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_timeout_pcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_timeout_pcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_timeout_pcr3`]
module"]
#[doc(alias = "CFG_TIMEOUT_PCR3")]
pub type CfgTimeoutPcr3 = crate::Reg<cfg_timeout_pcr3::CfgTimeoutPcr3Spec>;
#[doc = "CFG_TIMEOUT_PCR3"]
pub mod cfg_timeout_pcr3;
#[doc = "RESERVED0 (rw) register accessor: RESERVED0\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved0`]
module"]
#[doc(alias = "RESERVED0")]
pub type Reserved0 = crate::Reg<reserved0::Reserved0Spec>;
#[doc = "RESERVED0"]
pub mod reserved0;
#[doc = "APPSS_ERRAGG_MASK1 (rw) register accessor: APPSS_ERRAGG_MASK1\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_erragg_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_erragg_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_erragg_mask1`]
module"]
#[doc(alias = "APPSS_ERRAGG_MASK1")]
pub type AppssErraggMask1 = crate::Reg<appss_erragg_mask1::AppssErraggMask1Spec>;
#[doc = "APPSS_ERRAGG_MASK1"]
pub mod appss_erragg_mask1;
#[doc = "APPSS_ERRAGG_STATUS1 (rw) register accessor: APPSS_ERRAGG_STATUS1\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_erragg_status1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_erragg_status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_erragg_status1`]
module"]
#[doc(alias = "APPSS_ERRAGG_STATUS1")]
pub type AppssErraggStatus1 = crate::Reg<appss_erragg_status1::AppssErraggStatus1Spec>;
#[doc = "APPSS_ERRAGG_STATUS1"]
pub mod appss_erragg_status1;
#[doc = "FORCEHCLKACTIVE (rw) register accessor: FORCEHCLKACTIVE\n\nYou can [`read`](crate::Reg::read) this register and get [`forcehclkactive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`forcehclkactive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@forcehclkactive`]
module"]
#[doc(alias = "FORCEHCLKACTIVE")]
pub type Forcehclkactive = crate::Reg<forcehclkactive::ForcehclkactiveSpec>;
#[doc = "FORCEHCLKACTIVE"]
pub mod forcehclkactive;
#[doc = "APPSS_RAM1_OWRITE_ERR (rw) register accessor: APPSS_RAM1_OWRITE_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram1_owrite_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram1_owrite_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram1_owrite_err`]
module"]
#[doc(alias = "APPSS_RAM1_OWRITE_ERR")]
pub type AppssRam1OwriteErr = crate::Reg<appss_ram1_owrite_err::AppssRam1OwriteErrSpec>;
#[doc = "APPSS_RAM1_OWRITE_ERR"]
pub mod appss_ram1_owrite_err;
#[doc = "APPSS_RAM1_OWRITE_ERR_ADDR (rw) register accessor: APPSS_RAM1_OWRITE_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram1_owrite_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram1_owrite_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram1_owrite_err_addr`]
module"]
#[doc(alias = "APPSS_RAM1_OWRITE_ERR_ADDR")]
pub type AppssRam1OwriteErrAddr =
    crate::Reg<appss_ram1_owrite_err_addr::AppssRam1OwriteErrAddrSpec>;
#[doc = "APPSS_RAM1_OWRITE_ERR_ADDR"]
pub mod appss_ram1_owrite_err_addr;
#[doc = "APPSS_RAM2_OWRITE_ERR (rw) register accessor: APPSS_RAM2_OWRITE_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram2_owrite_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram2_owrite_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram2_owrite_err`]
module"]
#[doc(alias = "APPSS_RAM2_OWRITE_ERR")]
pub type AppssRam2OwriteErr = crate::Reg<appss_ram2_owrite_err::AppssRam2OwriteErrSpec>;
#[doc = "APPSS_RAM2_OWRITE_ERR"]
pub mod appss_ram2_owrite_err;
#[doc = "APPSS_RAM2_OWRITE_ERR_ADDR (rw) register accessor: APPSS_RAM2_OWRITE_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram2_owrite_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram2_owrite_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram2_owrite_err_addr`]
module"]
#[doc(alias = "APPSS_RAM2_OWRITE_ERR_ADDR")]
pub type AppssRam2OwriteErrAddr =
    crate::Reg<appss_ram2_owrite_err_addr::AppssRam2OwriteErrAddrSpec>;
#[doc = "APPSS_RAM2_OWRITE_ERR_ADDR"]
pub mod appss_ram2_owrite_err_addr;
#[doc = "APPSS_RAM3_OWRITE_ERR (rw) register accessor: APPSS_RAM3_OWRITE_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram3_owrite_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram3_owrite_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram3_owrite_err`]
module"]
#[doc(alias = "APPSS_RAM3_OWRITE_ERR")]
pub type AppssRam3OwriteErr = crate::Reg<appss_ram3_owrite_err::AppssRam3OwriteErrSpec>;
#[doc = "APPSS_RAM3_OWRITE_ERR"]
pub mod appss_ram3_owrite_err;
#[doc = "APPSS_RAM3_OWRITE_ERR_ADDR (rw) register accessor: APPSS_RAM3_OWRITE_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram3_owrite_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram3_owrite_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_ram3_owrite_err_addr`]
module"]
#[doc(alias = "APPSS_RAM3_OWRITE_ERR_ADDR")]
pub type AppssRam3OwriteErrAddr =
    crate::Reg<appss_ram3_owrite_err_addr::AppssRam3OwriteErrAddrSpec>;
#[doc = "APPSS_RAM3_OWRITE_ERR_ADDR"]
pub mod appss_ram3_owrite_err_addr;
#[doc = "APPSS_SHRD_RAM_OWRITE_ERR (rw) register accessor: APPSS_SHRD_RAM_OWRITE_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_shrd_ram_owrite_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_shrd_ram_owrite_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_shrd_ram_owrite_err`]
module"]
#[doc(alias = "APPSS_SHRD_RAM_OWRITE_ERR")]
pub type AppssShrdRamOwriteErr = crate::Reg<appss_shrd_ram_owrite_err::AppssShrdRamOwriteErrSpec>;
#[doc = "APPSS_SHRD_RAM_OWRITE_ERR"]
pub mod appss_shrd_ram_owrite_err;
#[doc = "APPSS_SHRD_RAM_OWRITE_ERR_ADDR (rw) register accessor: APPSS_SHRD_RAM_OWRITE_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_shrd_ram_owrite_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_shrd_ram_owrite_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_shrd_ram_owrite_err_addr`]
module"]
#[doc(alias = "APPSS_SHRD_RAM_OWRITE_ERR_ADDR")]
pub type AppssShrdRamOwriteErrAddr =
    crate::Reg<appss_shrd_ram_owrite_err_addr::AppssShrdRamOwriteErrAddrSpec>;
#[doc = "APPSS_SHRD_RAM_OWRITE_ERR_ADDR"]
pub mod appss_shrd_ram_owrite_err_addr;
#[doc = "APPSS_OWRITE_ERR_AGGR (rw) register accessor: APPSS_OWRITE_ERR_AGGR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_owrite_err_aggr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_owrite_err_aggr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appss_owrite_err_aggr`]
module"]
#[doc(alias = "APPSS_OWRITE_ERR_AGGR")]
pub type AppssOwriteErrAggr = crate::Reg<appss_owrite_err_aggr::AppssOwriteErrAggrSpec>;
#[doc = "APPSS_OWRITE_ERR_AGGR"]
pub mod appss_owrite_err_aggr;
#[doc = "HW_SPARE_RW6 (rw) register accessor: HW_SPARE_RW6\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw6`]
module"]
#[doc(alias = "HW_SPARE_RW6")]
pub type HwSpareRw6 = crate::Reg<hw_spare_rw6::HwSpareRw6Spec>;
#[doc = "HW_SPARE_RW6"]
pub mod hw_spare_rw6;
#[doc = "HW_SPARE_RW7 (rw) register accessor: HW_SPARE_RW7\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw7`]
module"]
#[doc(alias = "HW_SPARE_RW7")]
pub type HwSpareRw7 = crate::Reg<hw_spare_rw7::HwSpareRw7Spec>;
#[doc = "HW_SPARE_RW7"]
pub mod hw_spare_rw7;
#[doc = "HW_SPARE_RW8 (rw) register accessor: HW_SPARE_RW8\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw8`]
module"]
#[doc(alias = "HW_SPARE_RW8")]
pub type HwSpareRw8 = crate::Reg<hw_spare_rw8::HwSpareRw8Spec>;
#[doc = "HW_SPARE_RW8"]
pub mod hw_spare_rw8;
#[doc = "HW_SPARE_RW9 (rw) register accessor: HW_SPARE_RW9\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_rw9`]
module"]
#[doc(alias = "HW_SPARE_RW9")]
pub type HwSpareRw9 = crate::Reg<hw_spare_rw9::HwSpareRw9Spec>;
#[doc = "HW_SPARE_RW9"]
pub mod hw_spare_rw9;
#[doc = "HW_SPARE_HWA_RW0 (rw) register accessor: HW_SPARE_HWA_RW0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_hwa_rw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_hwa_rw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_spare_hwa_rw0`]
module"]
#[doc(alias = "HW_SPARE_HWA_RW0")]
pub type HwSpareHwaRw0 = crate::Reg<hw_spare_hwa_rw0::HwSpareHwaRw0Spec>;
#[doc = "HW_SPARE_HWA_RW0"]
pub mod hw_spare_hwa_rw0;
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
