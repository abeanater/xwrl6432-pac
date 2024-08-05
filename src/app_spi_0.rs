#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hl_rev: HlRev,
    hl_hwinfo: HlHwinfo,
    _reserved2: [u8; 0x08],
    hl_sysconfig: HlSysconfig,
    _reserved3: [u8; 0xec],
    revision: Revision,
    _reserved4: [u8; 0x0c],
    sysconfig: Sysconfig,
    sysstatus: Sysstatus,
    irqstatus: Irqstatus,
    irqenable: Irqenable,
    wakeupenable: Wakeupenable,
    syst: Syst,
    modulctrl: Modulctrl,
    ch0conf: Ch0conf,
    ch0stat: Ch0stat,
    ch0ctrl: Ch0ctrl,
    tx0: Tx0,
    rx0: Rx0,
    ch1conf: Ch1conf,
    ch1stat: Ch1stat,
    ch1ctrl: Ch1ctrl,
    tx1: Tx1,
    rx1: Rx1,
    ch2conf: Ch2conf,
    ch2stat: Ch2stat,
    ch2ctrl: Ch2ctrl,
    tx2: Tx2,
    rx2: Rx2,
    ch3conf: Ch3conf,
    ch3stat: Ch3stat,
    ch3ctrl: Ch3ctrl,
    tx3: Tx3,
    rx3: Rx3,
    xferlevel: Xferlevel,
    daftx: Daftx,
    _reserved33: [u8; 0x1c],
    dafrx: Dafrx,
}
impl RegisterBlock {
    #[doc = "0x00 - IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility"]
    #[inline(always)]
    pub const fn hl_rev(&self) -> &HlRev {
        &self.hl_rev
    }
    #[doc = "0x04 - Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide."]
    #[inline(always)]
    pub const fn hl_hwinfo(&self) -> &HlHwinfo {
        &self.hl_hwinfo
    }
    #[doc = "0x10 - Clock management configuration"]
    #[inline(always)]
    pub const fn hl_sysconfig(&self) -> &HlSysconfig {
        &self.hl_sysconfig
    }
    #[doc = "0x100 - This register contains the hard coded RTL revision number."]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
    #[doc = "0x110 - This register allows controlling various parameters of the OCP interface."]
    #[inline(always)]
    pub const fn sysconfig(&self) -> &Sysconfig {
        &self.sysconfig
    }
    #[doc = "0x114 - This register provides status information about the module excluding the interrupt status information"]
    #[inline(always)]
    pub const fn sysstatus(&self) -> &Sysstatus {
        &self.sysstatus
    }
    #[doc = "0x118 - The interrupt status regroups all the status of the module internal events that can generate an interrupt"]
    #[inline(always)]
    pub const fn irqstatus(&self) -> &Irqstatus {
        &self.irqstatus
    }
    #[doc = "0x11c - This register allows to enable/disable the module internal sources of interrupt on an event-by-event basis."]
    #[inline(always)]
    pub const fn irqenable(&self) -> &Irqenable {
        &self.irqenable
    }
    #[doc = "0x120 - The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis."]
    #[inline(always)]
    pub const fn wakeupenable(&self) -> &Wakeupenable {
        &self.wakeupenable
    }
    #[doc = "0x124 - This register is used to check the correctness of the system interconnect either internally to peripheral bus or externally to device IO pads when the module is configured in system test (SYSTEST) mode."]
    #[inline(always)]
    pub const fn syst(&self) -> &Syst {
        &self.syst
    }
    #[doc = "0x128 - This register is dedicated to the configuration of the serial port interface."]
    #[inline(always)]
    pub const fn modulctrl(&self) -> &Modulctrl {
        &self.modulctrl
    }
    #[doc = "0x12c - This register is dedicated to the configuration of the channel 0"]
    #[inline(always)]
    pub const fn ch0conf(&self) -> &Ch0conf {
        &self.ch0conf
    }
    #[doc = "0x130 - This register provides status information about transmitter and receiver registers of channel 0"]
    #[inline(always)]
    pub const fn ch0stat(&self) -> &Ch0stat {
        &self.ch0stat
    }
    #[doc = "0x134 - This register is dedicated to enable the channel 0"]
    #[inline(always)]
    pub const fn ch0ctrl(&self) -> &Ch0ctrl {
        &self.ch0ctrl
    }
    #[doc = "0x138 - This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
    #[inline(always)]
    pub const fn tx0(&self) -> &Tx0 {
        &self.tx0
    }
    #[doc = "0x13c - This register contains a single SPI word received through the serial link what ever SPI word length is."]
    #[inline(always)]
    pub const fn rx0(&self) -> &Rx0 {
        &self.rx0
    }
    #[doc = "0x140 - This register is dedicated to the configuration of the channel."]
    #[inline(always)]
    pub const fn ch1conf(&self) -> &Ch1conf {
        &self.ch1conf
    }
    #[doc = "0x144 - This register provides status information about transmitter and receiver registers of channel 1"]
    #[inline(always)]
    pub const fn ch1stat(&self) -> &Ch1stat {
        &self.ch1stat
    }
    #[doc = "0x148 - This register is dedicated to enable the channel 1"]
    #[inline(always)]
    pub const fn ch1ctrl(&self) -> &Ch1ctrl {
        &self.ch1ctrl
    }
    #[doc = "0x14c - This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
    #[inline(always)]
    pub const fn tx1(&self) -> &Tx1 {
        &self.tx1
    }
    #[doc = "0x150 - This register contains a single SPI word received through the serial link what ever SPI word length is."]
    #[inline(always)]
    pub const fn rx1(&self) -> &Rx1 {
        &self.rx1
    }
    #[doc = "0x154 - This register is dedicated to the configuration of the channel 2"]
    #[inline(always)]
    pub const fn ch2conf(&self) -> &Ch2conf {
        &self.ch2conf
    }
    #[doc = "0x158 - This register provides status information about transmitter and receiver registers of channel 2"]
    #[inline(always)]
    pub const fn ch2stat(&self) -> &Ch2stat {
        &self.ch2stat
    }
    #[doc = "0x15c - This register is dedicated to enable the channel 2"]
    #[inline(always)]
    pub const fn ch2ctrl(&self) -> &Ch2ctrl {
        &self.ch2ctrl
    }
    #[doc = "0x160 - This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
    #[inline(always)]
    pub const fn tx2(&self) -> &Tx2 {
        &self.tx2
    }
    #[doc = "0x164 - This register contains a single SPI word received through the serial link what ever SPI word length is."]
    #[inline(always)]
    pub const fn rx2(&self) -> &Rx2 {
        &self.rx2
    }
    #[doc = "0x168 - This register is dedicated to the configuration of the channel 3"]
    #[inline(always)]
    pub const fn ch3conf(&self) -> &Ch3conf {
        &self.ch3conf
    }
    #[doc = "0x16c - This register provides status information about transmitter and receiver registers of channel 3"]
    #[inline(always)]
    pub const fn ch3stat(&self) -> &Ch3stat {
        &self.ch3stat
    }
    #[doc = "0x170 - This register is dedicated to enable the channel 3"]
    #[inline(always)]
    pub const fn ch3ctrl(&self) -> &Ch3ctrl {
        &self.ch3ctrl
    }
    #[doc = "0x174 - This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
    #[inline(always)]
    pub const fn tx3(&self) -> &Tx3 {
        &self.tx3
    }
    #[doc = "0x178 - This register contains a single SPI word received through the serial link what ever SPI word length is."]
    #[inline(always)]
    pub const fn rx3(&self) -> &Rx3 {
        &self.rx3
    }
    #[doc = "0x17c - This register provides transfer levels needed while using FIFO buffer during transfer."]
    #[inline(always)]
    pub const fn xferlevel(&self) -> &Xferlevel {
        &self.xferlevel
    }
    #[doc = "0x180 - This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled."]
    #[inline(always)]
    pub const fn daftx(&self) -> &Daftx {
        &self.daftx
    }
    #[doc = "0x1a0 - This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled."]
    #[inline(always)]
    pub const fn dafrx(&self) -> &Dafrx {
        &self.dafrx
    }
}
#[doc = "HL_REV (rw) register accessor: IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility\n\nYou can [`read`](crate::Reg::read) this register and get [`hl_rev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl_rev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hl_rev`]
module"]
#[doc(alias = "HL_REV")]
pub type HlRev = crate::Reg<hl_rev::HlRevSpec>;
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility"]
pub mod hl_rev;
#[doc = "HL_HWINFO (rw) register accessor: Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide.\n\nYou can [`read`](crate::Reg::read) this register and get [`hl_hwinfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl_hwinfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hl_hwinfo`]
module"]
#[doc(alias = "HL_HWINFO")]
pub type HlHwinfo = crate::Reg<hl_hwinfo::HlHwinfoSpec>;
#[doc = "Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide."]
pub mod hl_hwinfo;
#[doc = "HL_SYSCONFIG (rw) register accessor: Clock management configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hl_sysconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl_sysconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hl_sysconfig`]
module"]
#[doc(alias = "HL_SYSCONFIG")]
pub type HlSysconfig = crate::Reg<hl_sysconfig::HlSysconfigSpec>;
#[doc = "Clock management configuration"]
pub mod hl_sysconfig;
#[doc = "REVISION (rw) register accessor: This register contains the hard coded RTL revision number.\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revision::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`]
module"]
#[doc(alias = "REVISION")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "This register contains the hard coded RTL revision number."]
pub mod revision;
#[doc = "SYSCONFIG (rw) register accessor: This register allows controlling various parameters of the OCP interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysconfig`]
module"]
#[doc(alias = "SYSCONFIG")]
pub type Sysconfig = crate::Reg<sysconfig::SysconfigSpec>;
#[doc = "This register allows controlling various parameters of the OCP interface."]
pub mod sysconfig;
#[doc = "SYSSTATUS (rw) register accessor: This register provides status information about the module excluding the interrupt status information\n\nYou can [`read`](crate::Reg::read) this register and get [`sysstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysstatus`]
module"]
#[doc(alias = "SYSSTATUS")]
pub type Sysstatus = crate::Reg<sysstatus::SysstatusSpec>;
#[doc = "This register provides status information about the module excluding the interrupt status information"]
pub mod sysstatus;
#[doc = "IRQSTATUS (rw) register accessor: The interrupt status regroups all the status of the module internal events that can generate an interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`irqstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstatus`]
module"]
#[doc(alias = "IRQSTATUS")]
pub type Irqstatus = crate::Reg<irqstatus::IrqstatusSpec>;
#[doc = "The interrupt status regroups all the status of the module internal events that can generate an interrupt"]
pub mod irqstatus;
#[doc = "IRQENABLE (rw) register accessor: This register allows to enable/disable the module internal sources of interrupt on an event-by-event basis.\n\nYou can [`read`](crate::Reg::read) this register and get [`irqenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqenable`]
module"]
#[doc(alias = "IRQENABLE")]
pub type Irqenable = crate::Reg<irqenable::IrqenableSpec>;
#[doc = "This register allows to enable/disable the module internal sources of interrupt on an event-by-event basis."]
pub mod irqenable;
#[doc = "WAKEUPENABLE (rw) register accessor: The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeupenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeupenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeupenable`]
module"]
#[doc(alias = "WAKEUPENABLE")]
pub type Wakeupenable = crate::Reg<wakeupenable::WakeupenableSpec>;
#[doc = "The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis."]
pub mod wakeupenable;
#[doc = "SYST (rw) register accessor: This register is used to check the correctness of the system interconnect either internally to peripheral bus or externally to device IO pads when the module is configured in system test (SYSTEST) mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`syst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst`]
module"]
#[doc(alias = "SYST")]
pub type Syst = crate::Reg<syst::SystSpec>;
#[doc = "This register is used to check the correctness of the system interconnect either internally to peripheral bus or externally to device IO pads when the module is configured in system test (SYSTEST) mode."]
pub mod syst;
#[doc = "MODULCTRL (rw) register accessor: This register is dedicated to the configuration of the serial port interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`modulctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modulctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulctrl`]
module"]
#[doc(alias = "MODULCTRL")]
pub type Modulctrl = crate::Reg<modulctrl::ModulctrlSpec>;
#[doc = "This register is dedicated to the configuration of the serial port interface."]
pub mod modulctrl;
#[doc = "CH0CONF (rw) register accessor: This register is dedicated to the configuration of the channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0conf`]
module"]
#[doc(alias = "CH0CONF")]
pub type Ch0conf = crate::Reg<ch0conf::Ch0confSpec>;
#[doc = "This register is dedicated to the configuration of the channel 0"]
pub mod ch0conf;
#[doc = "CH0STAT (rw) register accessor: This register provides status information about transmitter and receiver registers of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0stat`]
module"]
#[doc(alias = "CH0STAT")]
pub type Ch0stat = crate::Reg<ch0stat::Ch0statSpec>;
#[doc = "This register provides status information about transmitter and receiver registers of channel 0"]
pub mod ch0stat;
#[doc = "CH0CTRL (rw) register accessor: This register is dedicated to enable the channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ctrl`]
module"]
#[doc(alias = "CH0CTRL")]
pub type Ch0ctrl = crate::Reg<ch0ctrl::Ch0ctrlSpec>;
#[doc = "This register is dedicated to enable the channel 0"]
pub mod ch0ctrl;
#[doc = "TX0 (rw) register accessor: This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx0`]
module"]
#[doc(alias = "TX0")]
pub type Tx0 = crate::Reg<tx0::Tx0Spec>;
#[doc = "This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
pub mod tx0;
#[doc = "RX0 (rw) register accessor: This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx0`]
module"]
#[doc(alias = "RX0")]
pub type Rx0 = crate::Reg<rx0::Rx0Spec>;
#[doc = "This register contains a single SPI word received through the serial link what ever SPI word length is."]
pub mod rx0;
#[doc = "CH1CONF (rw) register accessor: This register is dedicated to the configuration of the channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1conf`]
module"]
#[doc(alias = "CH1CONF")]
pub type Ch1conf = crate::Reg<ch1conf::Ch1confSpec>;
#[doc = "This register is dedicated to the configuration of the channel."]
pub mod ch1conf;
#[doc = "CH1STAT (rw) register accessor: This register provides status information about transmitter and receiver registers of channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1stat`]
module"]
#[doc(alias = "CH1STAT")]
pub type Ch1stat = crate::Reg<ch1stat::Ch1statSpec>;
#[doc = "This register provides status information about transmitter and receiver registers of channel 1"]
pub mod ch1stat;
#[doc = "CH1CTRL (rw) register accessor: This register is dedicated to enable the channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ctrl`]
module"]
#[doc(alias = "CH1CTRL")]
pub type Ch1ctrl = crate::Reg<ch1ctrl::Ch1ctrlSpec>;
#[doc = "This register is dedicated to enable the channel 1"]
pub mod ch1ctrl;
#[doc = "TX1 (rw) register accessor: This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx1`]
module"]
#[doc(alias = "TX1")]
pub type Tx1 = crate::Reg<tx1::Tx1Spec>;
#[doc = "This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
pub mod tx1;
#[doc = "RX1 (rw) register accessor: This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx1`]
module"]
#[doc(alias = "RX1")]
pub type Rx1 = crate::Reg<rx1::Rx1Spec>;
#[doc = "This register contains a single SPI word received through the serial link what ever SPI word length is."]
pub mod rx1;
#[doc = "CH2CONF (rw) register accessor: This register is dedicated to the configuration of the channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2conf`]
module"]
#[doc(alias = "CH2CONF")]
pub type Ch2conf = crate::Reg<ch2conf::Ch2confSpec>;
#[doc = "This register is dedicated to the configuration of the channel 2"]
pub mod ch2conf;
#[doc = "CH2STAT (rw) register accessor: This register provides status information about transmitter and receiver registers of channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2stat`]
module"]
#[doc(alias = "CH2STAT")]
pub type Ch2stat = crate::Reg<ch2stat::Ch2statSpec>;
#[doc = "This register provides status information about transmitter and receiver registers of channel 2"]
pub mod ch2stat;
#[doc = "CH2CTRL (rw) register accessor: This register is dedicated to enable the channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ctrl`]
module"]
#[doc(alias = "CH2CTRL")]
pub type Ch2ctrl = crate::Reg<ch2ctrl::Ch2ctrlSpec>;
#[doc = "This register is dedicated to enable the channel 2"]
pub mod ch2ctrl;
#[doc = "TX2 (rw) register accessor: This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx2`]
module"]
#[doc(alias = "TX2")]
pub type Tx2 = crate::Reg<tx2::Tx2Spec>;
#[doc = "This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
pub mod tx2;
#[doc = "RX2 (rw) register accessor: This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx2`]
module"]
#[doc(alias = "RX2")]
pub type Rx2 = crate::Reg<rx2::Rx2Spec>;
#[doc = "This register contains a single SPI word received through the serial link what ever SPI word length is."]
pub mod rx2;
#[doc = "CH3CONF (rw) register accessor: This register is dedicated to the configuration of the channel 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3conf`]
module"]
#[doc(alias = "CH3CONF")]
pub type Ch3conf = crate::Reg<ch3conf::Ch3confSpec>;
#[doc = "This register is dedicated to the configuration of the channel 3"]
pub mod ch3conf;
#[doc = "CH3STAT (rw) register accessor: This register provides status information about transmitter and receiver registers of channel 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3stat`]
module"]
#[doc(alias = "CH3STAT")]
pub type Ch3stat = crate::Reg<ch3stat::Ch3statSpec>;
#[doc = "This register provides status information about transmitter and receiver registers of channel 3"]
pub mod ch3stat;
#[doc = "CH3CTRL (rw) register accessor: This register is dedicated to enable the channel 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ctrl`]
module"]
#[doc(alias = "CH3CTRL")]
pub type Ch3ctrl = crate::Reg<ch3ctrl::Ch3ctrlSpec>;
#[doc = "This register is dedicated to enable the channel 3"]
pub mod ch3ctrl;
#[doc = "TX3 (rw) register accessor: This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx3`]
module"]
#[doc(alias = "TX3")]
pub type Tx3 = crate::Reg<tx3::Tx3Spec>;
#[doc = "This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
pub mod tx3;
#[doc = "RX3 (rw) register accessor: This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx3`]
module"]
#[doc(alias = "RX3")]
pub type Rx3 = crate::Reg<rx3::Rx3Spec>;
#[doc = "This register contains a single SPI word received through the serial link what ever SPI word length is."]
pub mod rx3;
#[doc = "XFERLEVEL (rw) register accessor: This register provides transfer levels needed while using FIFO buffer during transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`xferlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xferlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xferlevel`]
module"]
#[doc(alias = "XFERLEVEL")]
pub type Xferlevel = crate::Reg<xferlevel::XferlevelSpec>;
#[doc = "This register provides transfer levels needed while using FIFO buffer during transfer."]
pub mod xferlevel;
#[doc = "DAFTX (rw) register accessor: This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`daftx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daftx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daftx`]
module"]
#[doc(alias = "DAFTX")]
pub type Daftx = crate::Reg<daftx::DaftxSpec>;
#[doc = "This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled."]
pub mod daftx;
#[doc = "DAFRX (rw) register accessor: This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`dafrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dafrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dafrx`]
module"]
#[doc(alias = "DAFRX")]
pub type Dafrx = crate::Reg<dafrx::DafrxSpec>;
#[doc = "This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled."]
pub mod dafrx;
