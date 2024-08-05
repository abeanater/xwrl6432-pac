#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    mss_qspi_reserved1: MssQspiReserved1,
    mss_qspi_reserved2: MssQspiReserved2,
    mss_qspi_reserved3: MssQspiReserved3,
    sysconfig: Sysconfig,
    mss_qspi_reserved4: MssQspiReserved4,
    mss_qspi_reserved5: MssQspiReserved5,
    mss_qspi_reserved6: MssQspiReserved6,
    intr_status_raw_set: IntrStatusRawSet,
    intr_status_enabled_clear: IntrStatusEnabledClear,
    intr_enable_set: IntrEnableSet,
    intr_enable_clear: IntrEnableClear,
    intc_eoi: IntcEoi,
    mss_qspi_reserved7: MssQspiReserved7,
    mss_qspi_reserved8: MssQspiReserved8,
    mss_qspi_reserved9: MssQspiReserved9,
    spi_clock_cntrl: SpiClockCntrl,
    spi_dc: SpiDc,
    spi_cmd: SpiCmd,
    spi_status: SpiStatus,
    spi_data: SpiData,
    spi_setup0: SpiSetup0,
    spi_setup1: SpiSetup1,
    spi_setup2: SpiSetup2,
    spi_setup3: SpiSetup3,
    spi_switch: SpiSwitch,
    spi_data1: SpiData1,
    spi_data2: SpiData2,
    spi_data3: SpiData3,
}
impl RegisterBlock {
    #[doc = "0x00 - PID"]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
    #[doc = "0x04 - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved1(&self) -> &MssQspiReserved1 {
        &self.mss_qspi_reserved1
    }
    #[doc = "0x08 - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved2(&self) -> &MssQspiReserved2 {
        &self.mss_qspi_reserved2
    }
    #[doc = "0x0c - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved3(&self) -> &MssQspiReserved3 {
        &self.mss_qspi_reserved3
    }
    #[doc = "0x10 - SYSCONFIG"]
    #[inline(always)]
    pub const fn sysconfig(&self) -> &Sysconfig {
        &self.sysconfig
    }
    #[doc = "0x14 - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved4(&self) -> &MssQspiReserved4 {
        &self.mss_qspi_reserved4
    }
    #[doc = "0x18 - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved5(&self) -> &MssQspiReserved5 {
        &self.mss_qspi_reserved5
    }
    #[doc = "0x1c - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved6(&self) -> &MssQspiReserved6 {
        &self.mss_qspi_reserved6
    }
    #[doc = "0x20 - INTR Interrupt Status Raw/Set Register"]
    #[inline(always)]
    pub const fn intr_status_raw_set(&self) -> &IntrStatusRawSet {
        &self.intr_status_raw_set
    }
    #[doc = "0x24 - INTR Interrupt Status Enabled/Clear Register"]
    #[inline(always)]
    pub const fn intr_status_enabled_clear(&self) -> &IntrStatusEnabledClear {
        &self.intr_status_enabled_clear
    }
    #[doc = "0x28 - INTR Interrupt Enable/Set Register"]
    #[inline(always)]
    pub const fn intr_enable_set(&self) -> &IntrEnableSet {
        &self.intr_enable_set
    }
    #[doc = "0x2c - INTR Interrupt Enable/Clear Register"]
    #[inline(always)]
    pub const fn intr_enable_clear(&self) -> &IntrEnableClear {
        &self.intr_enable_clear
    }
    #[doc = "0x30 - EOI Register"]
    #[inline(always)]
    pub const fn intc_eoi(&self) -> &IntcEoi {
        &self.intc_eoi
    }
    #[doc = "0x34 - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved7(&self) -> &MssQspiReserved7 {
        &self.mss_qspi_reserved7
    }
    #[doc = "0x38 - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved8(&self) -> &MssQspiReserved8 {
        &self.mss_qspi_reserved8
    }
    #[doc = "0x3c - Reserved"]
    #[inline(always)]
    pub const fn mss_qspi_reserved9(&self) -> &MssQspiReserved9 {
        &self.mss_qspi_reserved9
    }
    #[doc = "0x40 - SPI Clock Control Register (SPICC)"]
    #[inline(always)]
    pub const fn spi_clock_cntrl(&self) -> &SpiClockCntrl {
        &self.spi_clock_cntrl
    }
    #[doc = "0x44 - SPI Data Control Register (SPIDC)"]
    #[inline(always)]
    pub const fn spi_dc(&self) -> &SpiDc {
        &self.spi_dc
    }
    #[doc = "0x48 - SPI Command Register (SPICR)"]
    #[inline(always)]
    pub const fn spi_cmd(&self) -> &SpiCmd {
        &self.spi_cmd
    }
    #[doc = "0x4c - SPI Status Register (SPISR)"]
    #[inline(always)]
    pub const fn spi_status(&self) -> &SpiStatus {
        &self.spi_status
    }
    #[doc = "0x50 - SPI Data Register (SPIDR)"]
    #[inline(always)]
    pub const fn spi_data(&self) -> &SpiData {
        &self.spi_data
    }
    #[doc = "0x54 - Memory Mapped SPI Setup0 Register"]
    #[inline(always)]
    pub const fn spi_setup0(&self) -> &SpiSetup0 {
        &self.spi_setup0
    }
    #[doc = "0x58 - Memory Mapped SPI Setup1 Register"]
    #[inline(always)]
    pub const fn spi_setup1(&self) -> &SpiSetup1 {
        &self.spi_setup1
    }
    #[doc = "0x5c - Memory Mapped SPI Setup2 Register"]
    #[inline(always)]
    pub const fn spi_setup2(&self) -> &SpiSetup2 {
        &self.spi_setup2
    }
    #[doc = "0x60 - Memory Mapped SPI Setup3 Register"]
    #[inline(always)]
    pub const fn spi_setup3(&self) -> &SpiSetup3 {
        &self.spi_setup3
    }
    #[doc = "0x64 - Memory Mapped SPI Switch Register"]
    #[inline(always)]
    pub const fn spi_switch(&self) -> &SpiSwitch {
        &self.spi_switch
    }
    #[doc = "0x68 - SPI Data Register (SPIDR1)"]
    #[inline(always)]
    pub const fn spi_data1(&self) -> &SpiData1 {
        &self.spi_data1
    }
    #[doc = "0x6c - SPI Data Register (SPIDR2)"]
    #[inline(always)]
    pub const fn spi_data2(&self) -> &SpiData2 {
        &self.spi_data2
    }
    #[doc = "0x70 - SPI Data Register (SPIDR3)"]
    #[inline(always)]
    pub const fn spi_data3(&self) -> &SpiData3 {
        &self.spi_data3
    }
}
#[doc = "PID (rw) register accessor: PID\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "PID"]
pub mod pid;
#[doc = "MSS_QSPI_Reserved1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved1`]
module"]
#[doc(alias = "MSS_QSPI_Reserved1")]
pub type MssQspiReserved1 = crate::Reg<mss_qspi_reserved1::MssQspiReserved1Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved1;
#[doc = "MSS_QSPI_Reserved2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved2`]
module"]
#[doc(alias = "MSS_QSPI_Reserved2")]
pub type MssQspiReserved2 = crate::Reg<mss_qspi_reserved2::MssQspiReserved2Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved2;
#[doc = "MSS_QSPI_Reserved3 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved3`]
module"]
#[doc(alias = "MSS_QSPI_Reserved3")]
pub type MssQspiReserved3 = crate::Reg<mss_qspi_reserved3::MssQspiReserved3Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved3;
#[doc = "SYSCONFIG (rw) register accessor: SYSCONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`sysconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysconfig`]
module"]
#[doc(alias = "SYSCONFIG")]
pub type Sysconfig = crate::Reg<sysconfig::SysconfigSpec>;
#[doc = "SYSCONFIG"]
pub mod sysconfig;
#[doc = "MSS_QSPI_Reserved4 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved4`]
module"]
#[doc(alias = "MSS_QSPI_Reserved4")]
pub type MssQspiReserved4 = crate::Reg<mss_qspi_reserved4::MssQspiReserved4Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved4;
#[doc = "MSS_QSPI_Reserved5 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved5`]
module"]
#[doc(alias = "MSS_QSPI_Reserved5")]
pub type MssQspiReserved5 = crate::Reg<mss_qspi_reserved5::MssQspiReserved5Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved5;
#[doc = "MSS_QSPI_Reserved6 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved6`]
module"]
#[doc(alias = "MSS_QSPI_Reserved6")]
pub type MssQspiReserved6 = crate::Reg<mss_qspi_reserved6::MssQspiReserved6Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved6;
#[doc = "INTR_STATUS_RAW_SET (rw) register accessor: INTR Interrupt Status Raw/Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_status_raw_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_status_raw_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_raw_set`]
module"]
#[doc(alias = "INTR_STATUS_RAW_SET")]
pub type IntrStatusRawSet = crate::Reg<intr_status_raw_set::IntrStatusRawSetSpec>;
#[doc = "INTR Interrupt Status Raw/Set Register"]
pub mod intr_status_raw_set;
#[doc = "INTR_STATUS_ENABLED_CLEAR (rw) register accessor: INTR Interrupt Status Enabled/Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_status_enabled_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_status_enabled_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_enabled_clear`]
module"]
#[doc(alias = "INTR_STATUS_ENABLED_CLEAR")]
pub type IntrStatusEnabledClear = crate::Reg<intr_status_enabled_clear::IntrStatusEnabledClearSpec>;
#[doc = "INTR Interrupt Status Enabled/Clear Register"]
pub mod intr_status_enabled_clear;
#[doc = "INTR_ENABLE_SET (rw) register accessor: INTR Interrupt Enable/Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enable_set`]
module"]
#[doc(alias = "INTR_ENABLE_SET")]
pub type IntrEnableSet = crate::Reg<intr_enable_set::IntrEnableSetSpec>;
#[doc = "INTR Interrupt Enable/Set Register"]
pub mod intr_enable_set;
#[doc = "INTR_ENABLE_CLEAR (rw) register accessor: INTR Interrupt Enable/Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_enable_clear`]
module"]
#[doc(alias = "INTR_ENABLE_CLEAR")]
pub type IntrEnableClear = crate::Reg<intr_enable_clear::IntrEnableClearSpec>;
#[doc = "INTR Interrupt Enable/Clear Register"]
pub mod intr_enable_clear;
#[doc = "INTC_EOI (rw) register accessor: EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_eoi`]
module"]
#[doc(alias = "INTC_EOI")]
pub type IntcEoi = crate::Reg<intc_eoi::IntcEoiSpec>;
#[doc = "EOI Register"]
pub mod intc_eoi;
#[doc = "MSS_QSPI_Reserved7 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved7`]
module"]
#[doc(alias = "MSS_QSPI_Reserved7")]
pub type MssQspiReserved7 = crate::Reg<mss_qspi_reserved7::MssQspiReserved7Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved7;
#[doc = "MSS_QSPI_Reserved8 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved8`]
module"]
#[doc(alias = "MSS_QSPI_Reserved8")]
pub type MssQspiReserved8 = crate::Reg<mss_qspi_reserved8::MssQspiReserved8Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved8;
#[doc = "MSS_QSPI_Reserved9 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_qspi_reserved9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_qspi_reserved9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mss_qspi_reserved9`]
module"]
#[doc(alias = "MSS_QSPI_Reserved9")]
pub type MssQspiReserved9 = crate::Reg<mss_qspi_reserved9::MssQspiReserved9Spec>;
#[doc = "Reserved"]
pub mod mss_qspi_reserved9;
#[doc = "SPI_CLOCK_CNTRL (rw) register accessor: SPI Clock Control Register (SPICC)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_clock_cntrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_clock_cntrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_clock_cntrl`]
module"]
#[doc(alias = "SPI_CLOCK_CNTRL")]
pub type SpiClockCntrl = crate::Reg<spi_clock_cntrl::SpiClockCntrlSpec>;
#[doc = "SPI Clock Control Register (SPICC)"]
pub mod spi_clock_cntrl;
#[doc = "SPI_DC (rw) register accessor: SPI Data Control Register (SPIDC)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dc`]
module"]
#[doc(alias = "SPI_DC")]
pub type SpiDc = crate::Reg<spi_dc::SpiDcSpec>;
#[doc = "SPI Data Control Register (SPIDC)"]
pub mod spi_dc;
#[doc = "SPI_CMD (rw) register accessor: SPI Command Register (SPICR)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cmd`]
module"]
#[doc(alias = "SPI_CMD")]
pub type SpiCmd = crate::Reg<spi_cmd::SpiCmdSpec>;
#[doc = "SPI Command Register (SPICR)"]
pub mod spi_cmd;
#[doc = "SPI_STATUS (rw) register accessor: SPI Status Register (SPISR)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_status`]
module"]
#[doc(alias = "SPI_STATUS")]
pub type SpiStatus = crate::Reg<spi_status::SpiStatusSpec>;
#[doc = "SPI Status Register (SPISR)"]
pub mod spi_status;
#[doc = "SPI_DATA (rw) register accessor: SPI Data Register (SPIDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_data`]
module"]
#[doc(alias = "SPI_DATA")]
pub type SpiData = crate::Reg<spi_data::SpiDataSpec>;
#[doc = "SPI Data Register (SPIDR)"]
pub mod spi_data;
#[doc = "SPI_SETUP0 (rw) register accessor: Memory Mapped SPI Setup0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_setup0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_setup0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_setup0`]
module"]
#[doc(alias = "SPI_SETUP0")]
pub type SpiSetup0 = crate::Reg<spi_setup0::SpiSetup0Spec>;
#[doc = "Memory Mapped SPI Setup0 Register"]
pub mod spi_setup0;
#[doc = "SPI_SETUP1 (rw) register accessor: Memory Mapped SPI Setup1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_setup1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_setup1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_setup1`]
module"]
#[doc(alias = "SPI_SETUP1")]
pub type SpiSetup1 = crate::Reg<spi_setup1::SpiSetup1Spec>;
#[doc = "Memory Mapped SPI Setup1 Register"]
pub mod spi_setup1;
#[doc = "SPI_SETUP2 (rw) register accessor: Memory Mapped SPI Setup2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_setup2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_setup2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_setup2`]
module"]
#[doc(alias = "SPI_SETUP2")]
pub type SpiSetup2 = crate::Reg<spi_setup2::SpiSetup2Spec>;
#[doc = "Memory Mapped SPI Setup2 Register"]
pub mod spi_setup2;
#[doc = "SPI_SETUP3 (rw) register accessor: Memory Mapped SPI Setup3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_setup3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_setup3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_setup3`]
module"]
#[doc(alias = "SPI_SETUP3")]
pub type SpiSetup3 = crate::Reg<spi_setup3::SpiSetup3Spec>;
#[doc = "Memory Mapped SPI Setup3 Register"]
pub mod spi_setup3;
#[doc = "SPI_SWITCH (rw) register accessor: Memory Mapped SPI Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_switch`]
module"]
#[doc(alias = "SPI_SWITCH")]
pub type SpiSwitch = crate::Reg<spi_switch::SpiSwitchSpec>;
#[doc = "Memory Mapped SPI Switch Register"]
pub mod spi_switch;
#[doc = "SPI_DATA1 (rw) register accessor: SPI Data Register (SPIDR1)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_data1`]
module"]
#[doc(alias = "SPI_DATA1")]
pub type SpiData1 = crate::Reg<spi_data1::SpiData1Spec>;
#[doc = "SPI Data Register (SPIDR1)"]
pub mod spi_data1;
#[doc = "SPI_DATA2 (rw) register accessor: SPI Data Register (SPIDR2)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_data2`]
module"]
#[doc(alias = "SPI_DATA2")]
pub type SpiData2 = crate::Reg<spi_data2::SpiData2Spec>;
#[doc = "SPI Data Register (SPIDR2)"]
pub mod spi_data2;
#[doc = "SPI_DATA3 (rw) register accessor: SPI Data Register (SPIDR3)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_data3`]
module"]
#[doc(alias = "SPI_DATA3")]
pub type SpiData3 = crate::Reg<spi_data3::SpiData3Spec>;
#[doc = "SPI Data Register (SPIDR3)"]
pub mod spi_data3;
