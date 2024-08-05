#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    hw_reg0: HwReg0,
    adcbufcfg1: Adcbufcfg1,
    adcbufcfg1_extd: Adcbufcfg1Extd,
    adcbufcfg2: Adcbufcfg2,
    adcbufcfg3: Adcbufcfg3,
    adcbufcfg4: Adcbufcfg4,
    adcbufintgenditherdly: Adcbufintgenditherdly,
    adcbuff_ping_mem_init: AdcbuffPingMemInit,
    adcbuff_ping_mem_done: AdcbuffPingMemDone,
    adcbuff_ping_mem_status: AdcbuffPingMemStatus,
    adcbuff_pong_mem_init: AdcbuffPongMemInit,
    adcbuff_pong_mem_done: AdcbuffPongMemDone,
    adcbuff_pong_mem_status: AdcbuffPongMemStatus,
    hwass_shrd_ram_mem_init: HwassShrdRamMemInit,
    hwass_shrd_ram_mem_done: HwassShrdRamMemDone,
    hwass_shrd_ram_mem_status: HwassShrdRamMemStatus,
    hwass_shrd_ram_access_error_mask: HwassShrdRamAccessErrorMask,
    hwass_shrd_ram_access_error_status: HwassShrdRamAccessErrorStatus,
    hwass_shrd_ram_access_error_status_raw: HwassShrdRamAccessErrorStatusRaw,
    hwass_edma_clock_gate_control: HwassEdmaClockGateControl,
    hwass_ram_160kb_clock_gate: HwassRam160kbClockGate,
    _reserved22: [u8; 0x0fb0],
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
    #[doc = "0x08 - ADCBUFCFG1"]
    #[inline(always)]
    pub const fn adcbufcfg1(&self) -> &Adcbufcfg1 {
        &self.adcbufcfg1
    }
    #[doc = "0x0c - ADCBUFCFG1_EXTD"]
    #[inline(always)]
    pub const fn adcbufcfg1_extd(&self) -> &Adcbufcfg1Extd {
        &self.adcbufcfg1_extd
    }
    #[doc = "0x10 - ADCBUFCFG2"]
    #[inline(always)]
    pub const fn adcbufcfg2(&self) -> &Adcbufcfg2 {
        &self.adcbufcfg2
    }
    #[doc = "0x14 - ADCBUFCFG3"]
    #[inline(always)]
    pub const fn adcbufcfg3(&self) -> &Adcbufcfg3 {
        &self.adcbufcfg3
    }
    #[doc = "0x18 - ADCBUFCFG4"]
    #[inline(always)]
    pub const fn adcbufcfg4(&self) -> &Adcbufcfg4 {
        &self.adcbufcfg4
    }
    #[doc = "0x1c - ADCBUFINTGENDITHERDLY"]
    #[inline(always)]
    pub const fn adcbufintgenditherdly(&self) -> &Adcbufintgenditherdly {
        &self.adcbufintgenditherdly
    }
    #[doc = "0x20 - ADCBUFF_PING_MEM_INIT"]
    #[inline(always)]
    pub const fn adcbuff_ping_mem_init(&self) -> &AdcbuffPingMemInit {
        &self.adcbuff_ping_mem_init
    }
    #[doc = "0x24 - ADCBUFF_PING_MEM_DONE"]
    #[inline(always)]
    pub const fn adcbuff_ping_mem_done(&self) -> &AdcbuffPingMemDone {
        &self.adcbuff_ping_mem_done
    }
    #[doc = "0x28 - ADCBUFF_PING_MEM_STATUS"]
    #[inline(always)]
    pub const fn adcbuff_ping_mem_status(&self) -> &AdcbuffPingMemStatus {
        &self.adcbuff_ping_mem_status
    }
    #[doc = "0x2c - ADCBUFF_PONG_MEM_INIT"]
    #[inline(always)]
    pub const fn adcbuff_pong_mem_init(&self) -> &AdcbuffPongMemInit {
        &self.adcbuff_pong_mem_init
    }
    #[doc = "0x30 - ADCBUFF_PONG_MEM_DONE"]
    #[inline(always)]
    pub const fn adcbuff_pong_mem_done(&self) -> &AdcbuffPongMemDone {
        &self.adcbuff_pong_mem_done
    }
    #[doc = "0x34 - ADCBUFF_PONG_MEM_STATUS"]
    #[inline(always)]
    pub const fn adcbuff_pong_mem_status(&self) -> &AdcbuffPongMemStatus {
        &self.adcbuff_pong_mem_status
    }
    #[doc = "0x38 - HWASS_SHRD_RAM_MEM_INIT"]
    #[inline(always)]
    pub const fn hwass_shrd_ram_mem_init(&self) -> &HwassShrdRamMemInit {
        &self.hwass_shrd_ram_mem_init
    }
    #[doc = "0x3c - HWASS_SHRD_RAM_MEM_DONE"]
    #[inline(always)]
    pub const fn hwass_shrd_ram_mem_done(&self) -> &HwassShrdRamMemDone {
        &self.hwass_shrd_ram_mem_done
    }
    #[doc = "0x40 - HWASS_SHRD_RAM_MEM_STATUS"]
    #[inline(always)]
    pub const fn hwass_shrd_ram_mem_status(&self) -> &HwassShrdRamMemStatus {
        &self.hwass_shrd_ram_mem_status
    }
    #[doc = "0x44 - HWASS_SHRD_RAM_ACCESS_ERROR_MASK"]
    #[inline(always)]
    pub const fn hwass_shrd_ram_access_error_mask(&self) -> &HwassShrdRamAccessErrorMask {
        &self.hwass_shrd_ram_access_error_mask
    }
    #[doc = "0x48 - HWASS_SHRD_RAM_ACCESS_ERROR_STATUS"]
    #[inline(always)]
    pub const fn hwass_shrd_ram_access_error_status(&self) -> &HwassShrdRamAccessErrorStatus {
        &self.hwass_shrd_ram_access_error_status
    }
    #[doc = "0x4c - HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW"]
    #[inline(always)]
    pub const fn hwass_shrd_ram_access_error_status_raw(
        &self,
    ) -> &HwassShrdRamAccessErrorStatusRaw {
        &self.hwass_shrd_ram_access_error_status_raw
    }
    #[doc = "0x50 - HWASS_EDMA_CLOCK_GATE_CONTROL"]
    #[inline(always)]
    pub const fn hwass_edma_clock_gate_control(&self) -> &HwassEdmaClockGateControl {
        &self.hwass_edma_clock_gate_control
    }
    #[doc = "0x54 - HWASS_RAM_160KB_CLOCK_GATE"]
    #[inline(always)]
    pub const fn hwass_ram_160kb_clock_gate(&self) -> &HwassRam160kbClockGate {
        &self.hwass_ram_160kb_clock_gate
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
#[doc = "ADCBUFCFG1 (rw) register accessor: ADCBUFCFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg1`]
module"]
#[doc(alias = "ADCBUFCFG1")]
pub type Adcbufcfg1 = crate::Reg<adcbufcfg1::Adcbufcfg1Spec>;
#[doc = "ADCBUFCFG1"]
pub mod adcbufcfg1;
#[doc = "ADCBUFCFG1_EXTD (rw) register accessor: ADCBUFCFG1_EXTD\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg1_extd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg1_extd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg1_extd`]
module"]
#[doc(alias = "ADCBUFCFG1_EXTD")]
pub type Adcbufcfg1Extd = crate::Reg<adcbufcfg1_extd::Adcbufcfg1ExtdSpec>;
#[doc = "ADCBUFCFG1_EXTD"]
pub mod adcbufcfg1_extd;
#[doc = "ADCBUFCFG2 (rw) register accessor: ADCBUFCFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg2`]
module"]
#[doc(alias = "ADCBUFCFG2")]
pub type Adcbufcfg2 = crate::Reg<adcbufcfg2::Adcbufcfg2Spec>;
#[doc = "ADCBUFCFG2"]
pub mod adcbufcfg2;
#[doc = "ADCBUFCFG3 (rw) register accessor: ADCBUFCFG3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg3`]
module"]
#[doc(alias = "ADCBUFCFG3")]
pub type Adcbufcfg3 = crate::Reg<adcbufcfg3::Adcbufcfg3Spec>;
#[doc = "ADCBUFCFG3"]
pub mod adcbufcfg3;
#[doc = "ADCBUFCFG4 (rw) register accessor: ADCBUFCFG4\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufcfg4`]
module"]
#[doc(alias = "ADCBUFCFG4")]
pub type Adcbufcfg4 = crate::Reg<adcbufcfg4::Adcbufcfg4Spec>;
#[doc = "ADCBUFCFG4"]
pub mod adcbufcfg4;
#[doc = "ADCBUFINTGENDITHERDLY (rw) register accessor: ADCBUFINTGENDITHERDLY\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufintgenditherdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufintgenditherdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbufintgenditherdly`]
module"]
#[doc(alias = "ADCBUFINTGENDITHERDLY")]
pub type Adcbufintgenditherdly = crate::Reg<adcbufintgenditherdly::AdcbufintgenditherdlySpec>;
#[doc = "ADCBUFINTGENDITHERDLY"]
pub mod adcbufintgenditherdly;
#[doc = "ADCBUFF_PING_MEM_INIT (rw) register accessor: ADCBUFF_PING_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbuff_ping_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbuff_ping_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbuff_ping_mem_init`]
module"]
#[doc(alias = "ADCBUFF_PING_MEM_INIT")]
pub type AdcbuffPingMemInit = crate::Reg<adcbuff_ping_mem_init::AdcbuffPingMemInitSpec>;
#[doc = "ADCBUFF_PING_MEM_INIT"]
pub mod adcbuff_ping_mem_init;
#[doc = "ADCBUFF_PING_MEM_DONE (rw) register accessor: ADCBUFF_PING_MEM_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbuff_ping_mem_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbuff_ping_mem_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbuff_ping_mem_done`]
module"]
#[doc(alias = "ADCBUFF_PING_MEM_DONE")]
pub type AdcbuffPingMemDone = crate::Reg<adcbuff_ping_mem_done::AdcbuffPingMemDoneSpec>;
#[doc = "ADCBUFF_PING_MEM_DONE"]
pub mod adcbuff_ping_mem_done;
#[doc = "ADCBUFF_PING_MEM_STATUS (rw) register accessor: ADCBUFF_PING_MEM_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbuff_ping_mem_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbuff_ping_mem_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbuff_ping_mem_status`]
module"]
#[doc(alias = "ADCBUFF_PING_MEM_STATUS")]
pub type AdcbuffPingMemStatus = crate::Reg<adcbuff_ping_mem_status::AdcbuffPingMemStatusSpec>;
#[doc = "ADCBUFF_PING_MEM_STATUS"]
pub mod adcbuff_ping_mem_status;
#[doc = "ADCBUFF_PONG_MEM_INIT (rw) register accessor: ADCBUFF_PONG_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbuff_pong_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbuff_pong_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbuff_pong_mem_init`]
module"]
#[doc(alias = "ADCBUFF_PONG_MEM_INIT")]
pub type AdcbuffPongMemInit = crate::Reg<adcbuff_pong_mem_init::AdcbuffPongMemInitSpec>;
#[doc = "ADCBUFF_PONG_MEM_INIT"]
pub mod adcbuff_pong_mem_init;
#[doc = "ADCBUFF_PONG_MEM_DONE (rw) register accessor: ADCBUFF_PONG_MEM_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbuff_pong_mem_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbuff_pong_mem_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbuff_pong_mem_done`]
module"]
#[doc(alias = "ADCBUFF_PONG_MEM_DONE")]
pub type AdcbuffPongMemDone = crate::Reg<adcbuff_pong_mem_done::AdcbuffPongMemDoneSpec>;
#[doc = "ADCBUFF_PONG_MEM_DONE"]
pub mod adcbuff_pong_mem_done;
#[doc = "ADCBUFF_PONG_MEM_STATUS (rw) register accessor: ADCBUFF_PONG_MEM_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbuff_pong_mem_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbuff_pong_mem_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcbuff_pong_mem_status`]
module"]
#[doc(alias = "ADCBUFF_PONG_MEM_STATUS")]
pub type AdcbuffPongMemStatus = crate::Reg<adcbuff_pong_mem_status::AdcbuffPongMemStatusSpec>;
#[doc = "ADCBUFF_PONG_MEM_STATUS"]
pub mod adcbuff_pong_mem_status;
#[doc = "HWASS_SHRD_RAM_MEM_INIT (rw) register accessor: HWASS_SHRD_RAM_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_mem_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_mem_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram_mem_init`]
module"]
#[doc(alias = "HWASS_SHRD_RAM_MEM_INIT")]
pub type HwassShrdRamMemInit = crate::Reg<hwass_shrd_ram_mem_init::HwassShrdRamMemInitSpec>;
#[doc = "HWASS_SHRD_RAM_MEM_INIT"]
pub mod hwass_shrd_ram_mem_init;
#[doc = "HWASS_SHRD_RAM_MEM_DONE (rw) register accessor: HWASS_SHRD_RAM_MEM_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_mem_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_mem_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram_mem_done`]
module"]
#[doc(alias = "HWASS_SHRD_RAM_MEM_DONE")]
pub type HwassShrdRamMemDone = crate::Reg<hwass_shrd_ram_mem_done::HwassShrdRamMemDoneSpec>;
#[doc = "HWASS_SHRD_RAM_MEM_DONE"]
pub mod hwass_shrd_ram_mem_done;
#[doc = "HWASS_SHRD_RAM_MEM_STATUS (rw) register accessor: HWASS_SHRD_RAM_MEM_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_mem_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_mem_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram_mem_status`]
module"]
#[doc(alias = "HWASS_SHRD_RAM_MEM_STATUS")]
pub type HwassShrdRamMemStatus = crate::Reg<hwass_shrd_ram_mem_status::HwassShrdRamMemStatusSpec>;
#[doc = "HWASS_SHRD_RAM_MEM_STATUS"]
pub mod hwass_shrd_ram_mem_status;
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_MASK (rw) register accessor: HWASS_SHRD_RAM_ACCESS_ERROR_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_access_error_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_access_error_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram_access_error_mask`]
module"]
#[doc(alias = "HWASS_SHRD_RAM_ACCESS_ERROR_MASK")]
pub type HwassShrdRamAccessErrorMask =
    crate::Reg<hwass_shrd_ram_access_error_mask::HwassShrdRamAccessErrorMaskSpec>;
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_MASK"]
pub mod hwass_shrd_ram_access_error_mask;
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_STATUS (rw) register accessor: HWASS_SHRD_RAM_ACCESS_ERROR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_access_error_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_access_error_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram_access_error_status`]
module"]
#[doc(alias = "HWASS_SHRD_RAM_ACCESS_ERROR_STATUS")]
pub type HwassShrdRamAccessErrorStatus =
    crate::Reg<hwass_shrd_ram_access_error_status::HwassShrdRamAccessErrorStatusSpec>;
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_STATUS"]
pub mod hwass_shrd_ram_access_error_status;
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW (rw) register accessor: HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_access_error_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_access_error_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_shrd_ram_access_error_status_raw`]
module"]
#[doc(alias = "HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW")]
pub type HwassShrdRamAccessErrorStatusRaw =
    crate::Reg<hwass_shrd_ram_access_error_status_raw::HwassShrdRamAccessErrorStatusRawSpec>;
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW"]
pub mod hwass_shrd_ram_access_error_status_raw;
#[doc = "HWASS_EDMA_CLOCK_GATE_CONTROL (rw) register accessor: HWASS_EDMA_CLOCK_GATE_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_edma_clock_gate_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_edma_clock_gate_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_edma_clock_gate_control`]
module"]
#[doc(alias = "HWASS_EDMA_CLOCK_GATE_CONTROL")]
pub type HwassEdmaClockGateControl =
    crate::Reg<hwass_edma_clock_gate_control::HwassEdmaClockGateControlSpec>;
#[doc = "HWASS_EDMA_CLOCK_GATE_CONTROL"]
pub mod hwass_edma_clock_gate_control;
#[doc = "HWASS_RAM_160KB_CLOCK_GATE (rw) register accessor: HWASS_RAM_160KB_CLOCK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_ram_160kb_clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_ram_160kb_clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwass_ram_160kb_clock_gate`]
module"]
#[doc(alias = "HWASS_RAM_160KB_CLOCK_GATE")]
pub type HwassRam160kbClockGate =
    crate::Reg<hwass_ram_160kb_clock_gate::HwassRam160kbClockGateSpec>;
#[doc = "HWASS_RAM_160KB_CLOCK_GATE"]
pub mod hwass_ram_160kb_clock_gate;
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
