#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    cpuclkctl: Cpuclkctl,
    canclkctl: Canclkctl,
    spiclkctl: Spiclkctl,
    qspiclkctl: Qspiclkctl,
    topssclkctl: Topssclkctl,
    rticlkctl: Rticlkctl,
    wdtclkctl: Wdtclkctl,
    uart1clkctl: Uart1clkctl,
    uart2clkctl: Uart2clkctl,
    i2cclkctl: I2cclkctl,
    linclkctl: Linclkctl,
    reserved0: Reserved0,
    reserved1: Reserved1,
    reserved2: Reserved2,
    reserved3: Reserved3,
    vbusclkgate0: Vbusclkgate0,
    vbusclkgate1: Vbusclkgate1,
    vbusclkgate2: Vbusclkgate2,
    blockreset0: Blockreset0,
    blockreset1: Blockreset1,
    _reserved21: [u8; 0x0fb4],
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
    #[doc = "0x04 - CPUCLKCTL"]
    #[inline(always)]
    pub const fn cpuclkctl(&self) -> &Cpuclkctl {
        &self.cpuclkctl
    }
    #[doc = "0x08 - CANCLKCTL"]
    #[inline(always)]
    pub const fn canclkctl(&self) -> &Canclkctl {
        &self.canclkctl
    }
    #[doc = "0x0c - SPICLKCTL"]
    #[inline(always)]
    pub const fn spiclkctl(&self) -> &Spiclkctl {
        &self.spiclkctl
    }
    #[doc = "0x10 - QSPICLKCTL"]
    #[inline(always)]
    pub const fn qspiclkctl(&self) -> &Qspiclkctl {
        &self.qspiclkctl
    }
    #[doc = "0x14 - TOPSSCLKCTL"]
    #[inline(always)]
    pub const fn topssclkctl(&self) -> &Topssclkctl {
        &self.topssclkctl
    }
    #[doc = "0x18 - RTICLKCTL"]
    #[inline(always)]
    pub const fn rticlkctl(&self) -> &Rticlkctl {
        &self.rticlkctl
    }
    #[doc = "0x1c - WDTCLKCTL"]
    #[inline(always)]
    pub const fn wdtclkctl(&self) -> &Wdtclkctl {
        &self.wdtclkctl
    }
    #[doc = "0x20 - UART1CLKCTL"]
    #[inline(always)]
    pub const fn uart1clkctl(&self) -> &Uart1clkctl {
        &self.uart1clkctl
    }
    #[doc = "0x24 - UART2CLKCTL"]
    #[inline(always)]
    pub const fn uart2clkctl(&self) -> &Uart2clkctl {
        &self.uart2clkctl
    }
    #[doc = "0x28 - I2CCLKCTL"]
    #[inline(always)]
    pub const fn i2cclkctl(&self) -> &I2cclkctl {
        &self.i2cclkctl
    }
    #[doc = "0x2c - LINCLKCTL"]
    #[inline(always)]
    pub const fn linclkctl(&self) -> &Linclkctl {
        &self.linclkctl
    }
    #[doc = "0x30 - RESERVED0"]
    #[inline(always)]
    pub const fn reserved0(&self) -> &Reserved0 {
        &self.reserved0
    }
    #[doc = "0x34 - RESERVED1"]
    #[inline(always)]
    pub const fn reserved1(&self) -> &Reserved1 {
        &self.reserved1
    }
    #[doc = "0x38 - RESERVED2"]
    #[inline(always)]
    pub const fn reserved2(&self) -> &Reserved2 {
        &self.reserved2
    }
    #[doc = "0x3c - RESERVED3"]
    #[inline(always)]
    pub const fn reserved3(&self) -> &Reserved3 {
        &self.reserved3
    }
    #[doc = "0x40 - VBUSCLKGATE0"]
    #[inline(always)]
    pub const fn vbusclkgate0(&self) -> &Vbusclkgate0 {
        &self.vbusclkgate0
    }
    #[doc = "0x44 - VBUSCLKGATE1"]
    #[inline(always)]
    pub const fn vbusclkgate1(&self) -> &Vbusclkgate1 {
        &self.vbusclkgate1
    }
    #[doc = "0x48 - VBUSCLKGATE2"]
    #[inline(always)]
    pub const fn vbusclkgate2(&self) -> &Vbusclkgate2 {
        &self.vbusclkgate2
    }
    #[doc = "0x4c - BLOCKRESET0"]
    #[inline(always)]
    pub const fn blockreset0(&self) -> &Blockreset0 {
        &self.blockreset0
    }
    #[doc = "0x50 - BLOCKRESET1"]
    #[inline(always)]
    pub const fn blockreset1(&self) -> &Blockreset1 {
        &self.blockreset1
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
#[doc = "CPUCLKCTL (rw) register accessor: CPUCLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuclkctl`]
module"]
#[doc(alias = "CPUCLKCTL")]
pub type Cpuclkctl = crate::Reg<cpuclkctl::CpuclkctlSpec>;
#[doc = "CPUCLKCTL"]
pub mod cpuclkctl;
#[doc = "CANCLKCTL (rw) register accessor: CANCLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`canclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canclkctl`]
module"]
#[doc(alias = "CANCLKCTL")]
pub type Canclkctl = crate::Reg<canclkctl::CanclkctlSpec>;
#[doc = "CANCLKCTL"]
pub mod canclkctl;
#[doc = "SPICLKCTL (rw) register accessor: SPICLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`spiclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spiclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spiclkctl`]
module"]
#[doc(alias = "SPICLKCTL")]
pub type Spiclkctl = crate::Reg<spiclkctl::SpiclkctlSpec>;
#[doc = "SPICLKCTL"]
pub mod spiclkctl;
#[doc = "QSPICLKCTL (rw) register accessor: QSPICLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`qspiclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspiclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspiclkctl`]
module"]
#[doc(alias = "QSPICLKCTL")]
pub type Qspiclkctl = crate::Reg<qspiclkctl::QspiclkctlSpec>;
#[doc = "QSPICLKCTL"]
pub mod qspiclkctl;
#[doc = "TOPSSCLKCTL (rw) register accessor: TOPSSCLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`topssclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topssclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@topssclkctl`]
module"]
#[doc(alias = "TOPSSCLKCTL")]
pub type Topssclkctl = crate::Reg<topssclkctl::TopssclkctlSpec>;
#[doc = "TOPSSCLKCTL"]
pub mod topssclkctl;
#[doc = "RTICLKCTL (rw) register accessor: RTICLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`rticlkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticlkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticlkctl`]
module"]
#[doc(alias = "RTICLKCTL")]
pub type Rticlkctl = crate::Reg<rticlkctl::RticlkctlSpec>;
#[doc = "RTICLKCTL"]
pub mod rticlkctl;
#[doc = "WDTCLKCTL (rw) register accessor: WDTCLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkctl`]
module"]
#[doc(alias = "WDTCLKCTL")]
pub type Wdtclkctl = crate::Reg<wdtclkctl::WdtclkctlSpec>;
#[doc = "WDTCLKCTL"]
pub mod wdtclkctl;
#[doc = "UART1CLKCTL (rw) register accessor: UART1CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1clkctl`]
module"]
#[doc(alias = "UART1CLKCTL")]
pub type Uart1clkctl = crate::Reg<uart1clkctl::Uart1clkctlSpec>;
#[doc = "UART1CLKCTL"]
pub mod uart1clkctl;
#[doc = "UART2CLKCTL (rw) register accessor: UART2CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2clkctl`]
module"]
#[doc(alias = "UART2CLKCTL")]
pub type Uart2clkctl = crate::Reg<uart2clkctl::Uart2clkctlSpec>;
#[doc = "UART2CLKCTL"]
pub mod uart2clkctl;
#[doc = "I2CCLKCTL (rw) register accessor: I2CCLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cclkctl`]
module"]
#[doc(alias = "I2CCLKCTL")]
pub type I2cclkctl = crate::Reg<i2cclkctl::I2cclkctlSpec>;
#[doc = "I2CCLKCTL"]
pub mod i2cclkctl;
#[doc = "LINCLKCTL (rw) register accessor: LINCLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`linclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linclkctl`]
module"]
#[doc(alias = "LINCLKCTL")]
pub type Linclkctl = crate::Reg<linclkctl::LinclkctlSpec>;
#[doc = "LINCLKCTL"]
pub mod linclkctl;
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
#[doc = "VBUSCLKGATE0 (rw) register accessor: VBUSCLKGATE0\n\nYou can [`read`](crate::Reg::read) this register and get [`vbusclkgate0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbusclkgate0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusclkgate0`]
module"]
#[doc(alias = "VBUSCLKGATE0")]
pub type Vbusclkgate0 = crate::Reg<vbusclkgate0::Vbusclkgate0Spec>;
#[doc = "VBUSCLKGATE0"]
pub mod vbusclkgate0;
#[doc = "VBUSCLKGATE1 (rw) register accessor: VBUSCLKGATE1\n\nYou can [`read`](crate::Reg::read) this register and get [`vbusclkgate1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbusclkgate1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusclkgate1`]
module"]
#[doc(alias = "VBUSCLKGATE1")]
pub type Vbusclkgate1 = crate::Reg<vbusclkgate1::Vbusclkgate1Spec>;
#[doc = "VBUSCLKGATE1"]
pub mod vbusclkgate1;
#[doc = "VBUSCLKGATE2 (rw) register accessor: VBUSCLKGATE2\n\nYou can [`read`](crate::Reg::read) this register and get [`vbusclkgate2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbusclkgate2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbusclkgate2`]
module"]
#[doc(alias = "VBUSCLKGATE2")]
pub type Vbusclkgate2 = crate::Reg<vbusclkgate2::Vbusclkgate2Spec>;
#[doc = "VBUSCLKGATE2"]
pub mod vbusclkgate2;
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
