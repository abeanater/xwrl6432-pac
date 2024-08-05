#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtigctrl: Rtigctrl,
    rtitbctrl: Rtitbctrl,
    rticapctrl: Rticapctrl,
    rticompctrl: Rticompctrl,
    rtifrc0: Rtifrc0,
    rtiuc0: Rtiuc0,
    rticpuc0: Rticpuc0,
    _reserved7: [u8; 0x04],
    rticafrc0: Rticafrc0,
    rticauc0: Rticauc0,
    _reserved9: [u8; 0x08],
    rtifrc1: Rtifrc1,
    rtiuc1: Rtiuc1,
    rticpuc1: Rticpuc1,
    _reserved12: [u8; 0x04],
    rticafrc1: Rticafrc1,
    rticauc1: Rticauc1,
    _reserved14: [u8; 0x08],
    rticomp0: Rticomp0,
    rtiudcp0: Rtiudcp0,
    rticomp1: Rticomp1,
    rtiudcp1: Rtiudcp1,
    rticomp2: Rticomp2,
    rtiudcp2: Rtiudcp2,
    rticomp3: Rticomp3,
    rtiudcp3: Rtiudcp3,
    rtitblcomp: Rtitblcomp,
    rtitbhcomp: Rtitbhcomp,
    _reserved24: [u8; 0x08],
    rtisetint: Rtisetint,
    rticlearint: Rticlearint,
    rtiintflag: Rtiintflag,
    _reserved27: [u8; 0x04],
    rtidwdctrl: Rtidwdctrl,
    rtidwdprld: Rtidwdprld,
    rtiwdstatus: Rtiwdstatus,
    rtiwdkey: Rtiwdkey,
    rtidwdcntr: Rtidwdcntr,
    rtiwwdrxnctrl: Rtiwwdrxnctrl,
    rtiwwdsizectrl: Rtiwwdsizectrl,
    rtiintclrenable: Rtiintclrenable,
    rticomp0clr: Rticomp0clr,
    rticomp1clr: Rticomp1clr,
    rticomp2clr: Rticomp2clr,
    rticomp3clr: Rticomp3clr,
}
impl RegisterBlock {
    #[doc = "0x00 - Global Control Register starts / stops the counters"]
    #[inline(always)]
    pub const fn rtigctrl(&self) -> &Rtigctrl {
        &self.rtigctrl
    }
    #[doc = "0x04 - Timebase Control selection which source triggers free running counter 0"]
    #[inline(always)]
    pub const fn rtitbctrl(&self) -> &Rtitbctrl {
        &self.rtitbctrl
    }
    #[doc = "0x08 - Capture Control controls the capture source for the counters"]
    #[inline(always)]
    pub const fn rticapctrl(&self) -> &Rticapctrl {
        &self.rticapctrl
    }
    #[doc = "0x0c - Compare Control controls the source for the compare registers"]
    #[inline(always)]
    pub const fn rticompctrl(&self) -> &Rticompctrl {
        &self.rticompctrl
    }
    #[doc = "0x10 - Free Running Counter 0 current value of free running counter 0"]
    #[inline(always)]
    pub const fn rtifrc0(&self) -> &Rtifrc0 {
        &self.rtifrc0
    }
    #[doc = "0x14 - Up Counter 0 current value of prescale counter 0"]
    #[inline(always)]
    pub const fn rtiuc0(&self) -> &Rtiuc0 {
        &self.rtiuc0
    }
    #[doc = "0x18 - Compare Up Counter 0 compare value compared with prescale counter 0"]
    #[inline(always)]
    pub const fn rticpuc0(&self) -> &Rticpuc0 {
        &self.rticpuc0
    }
    #[doc = "0x20 - Capture Free Running Counter 0 current value of free running counter 0 on external event"]
    #[inline(always)]
    pub const fn rticafrc0(&self) -> &Rticafrc0 {
        &self.rticafrc0
    }
    #[doc = "0x24 - Capture Up Counter 0 current value of prescale counter 0 on external event"]
    #[inline(always)]
    pub const fn rticauc0(&self) -> &Rticauc0 {
        &self.rticauc0
    }
    #[doc = "0x30 - Free Running Counter 1 current value of free running counter 1"]
    #[inline(always)]
    pub const fn rtifrc1(&self) -> &Rtifrc1 {
        &self.rtifrc1
    }
    #[doc = "0x34 - Up Counter 1 current value of prescale counter 1"]
    #[inline(always)]
    pub const fn rtiuc1(&self) -> &Rtiuc1 {
        &self.rtiuc1
    }
    #[doc = "0x38 - Compare Up Counter 1 compare value compared with prescale counter 1"]
    #[inline(always)]
    pub const fn rticpuc1(&self) -> &Rticpuc1 {
        &self.rticpuc1
    }
    #[doc = "0x40 - Capture Free Running Counter 1 current value of free running counter 1 on external event"]
    #[inline(always)]
    pub const fn rticafrc1(&self) -> &Rticafrc1 {
        &self.rticafrc1
    }
    #[doc = "0x44 - Capture Up Counter 1 current value of prescale counter 1 on external event"]
    #[inline(always)]
    pub const fn rticauc1(&self) -> &Rticauc1 {
        &self.rticauc1
    }
    #[doc = "0x50 - Compare 0 compare value to be compared with the counters"]
    #[inline(always)]
    pub const fn rticomp0(&self) -> &Rticomp0 {
        &self.rticomp0
    }
    #[doc = "0x54 - Update Compare 0 value to be added to the compare register 0 value on compare match"]
    #[inline(always)]
    pub const fn rtiudcp0(&self) -> &Rtiudcp0 {
        &self.rtiudcp0
    }
    #[doc = "0x58 - Compare 1 compare value to be compared with the counters"]
    #[inline(always)]
    pub const fn rticomp1(&self) -> &Rticomp1 {
        &self.rticomp1
    }
    #[doc = "0x5c - Update Compare 1 value to be added to the compare register 1 value on compare match"]
    #[inline(always)]
    pub const fn rtiudcp1(&self) -> &Rtiudcp1 {
        &self.rtiudcp1
    }
    #[doc = "0x60 - Compare 2 compare value to be compared with the counters"]
    #[inline(always)]
    pub const fn rticomp2(&self) -> &Rticomp2 {
        &self.rticomp2
    }
    #[doc = "0x64 - Update Compare 2 value to be added to the compare register 2 value on compare match"]
    #[inline(always)]
    pub const fn rtiudcp2(&self) -> &Rtiudcp2 {
        &self.rtiudcp2
    }
    #[doc = "0x68 - Compare 3 compare value to be compared with the counters"]
    #[inline(always)]
    pub const fn rticomp3(&self) -> &Rticomp3 {
        &self.rticomp3
    }
    #[doc = "0x6c - Update Compare 3 value to be added to the compare register 3 value on compare match"]
    #[inline(always)]
    pub const fn rtiudcp3(&self) -> &Rtiudcp3 {
        &self.rtiudcp3
    }
    #[doc = "0x70 - Timebase Low Compare compare value to activate edge detection circuit"]
    #[inline(always)]
    pub const fn rtitblcomp(&self) -> &Rtitblcomp {
        &self.rtitblcomp
    }
    #[doc = "0x74 - Timebase High Compare compare value to deactivate edge detection circuit"]
    #[inline(always)]
    pub const fn rtitbhcomp(&self) -> &Rtitbhcomp {
        &self.rtitbhcomp
    }
    #[doc = "0x80 - Set Interrupt Enable sets interrupt enable bits int RTIINTCTRL without having to do a read-modify-write operation"]
    #[inline(always)]
    pub const fn rtisetint(&self) -> &Rtisetint {
        &self.rtisetint
    }
    #[doc = "0x84 - Clear Interrupt Enable clears interrupt enable bits int RTIINTCTRL without having to do a read-modify-write operation"]
    #[inline(always)]
    pub const fn rticlearint(&self) -> &Rticlearint {
        &self.rticlearint
    }
    #[doc = "0x88 - Interrupt Flags interrupt pending bits"]
    #[inline(always)]
    pub const fn rtiintflag(&self) -> &Rtiintflag {
        &self.rtiintflag
    }
    #[doc = "0x90 - Digital Watchdog Control Enables the Digital Watchdog"]
    #[inline(always)]
    pub const fn rtidwdctrl(&self) -> &Rtidwdctrl {
        &self.rtidwdctrl
    }
    #[doc = "0x94 - Digital Watchdog Preload sets the experation time of the Digital Watchdog"]
    #[inline(always)]
    pub const fn rtidwdprld(&self) -> &Rtidwdprld {
        &self.rtidwdprld
    }
    #[doc = "0x98 - Watchdog Status reflects the status of Analog and Digital Watchdog"]
    #[inline(always)]
    pub const fn rtiwdstatus(&self) -> &Rtiwdstatus {
        &self.rtiwdstatus
    }
    #[doc = "0x9c - Watchdog Key correct written key values discharge the external capacitor"]
    #[inline(always)]
    pub const fn rtiwdkey(&self) -> &Rtiwdkey {
        &self.rtiwdkey
    }
    #[doc = "0xa0 - Digital Watchdog Down Counter current value of DWD down counter"]
    #[inline(always)]
    pub const fn rtidwdcntr(&self) -> &Rtidwdcntr {
        &self.rtidwdcntr
    }
    #[doc = "0xa4 - Windowed Watchdog Reaction Control configures the windowed watchdog to either generate a non-maskable interrupt to the CPU or to generate a system reset"]
    #[inline(always)]
    pub const fn rtiwwdrxnctrl(&self) -> &Rtiwwdrxnctrl {
        &self.rtiwwdrxnctrl
    }
    #[doc = "0xa8 - Windowed Watchdog Size Control configures the size of the window for the digital windowed watchdog"]
    #[inline(always)]
    pub const fn rtiwwdsizectrl(&self) -> &Rtiwwdsizectrl {
        &self.rtiwwdsizectrl
    }
    #[doc = "0xac - RTI Compare Interrupt Clear Enable enable the auto clear functionality for each of the compare interrupts"]
    #[inline(always)]
    pub const fn rtiintclrenable(&self) -> &Rtiintclrenable {
        &self.rtiintclrenable
    }
    #[doc = "0xb0 - Compare 0 Clear compare value to be compared with the counter to clear the compare0 interrupt line"]
    #[inline(always)]
    pub const fn rticomp0clr(&self) -> &Rticomp0clr {
        &self.rticomp0clr
    }
    #[doc = "0xb4 - Compare 1 Clear compare value to be compared with the counter to clear the compare1 interrupt line"]
    #[inline(always)]
    pub const fn rticomp1clr(&self) -> &Rticomp1clr {
        &self.rticomp1clr
    }
    #[doc = "0xb8 - Compare 2 Clear compare value to be compared with the counter to clear the compare2 interrupt line"]
    #[inline(always)]
    pub const fn rticomp2clr(&self) -> &Rticomp2clr {
        &self.rticomp2clr
    }
    #[doc = "0xbc - Compare 3 Clear compare value to be compared with the counter to clear the compare3 interrupt line"]
    #[inline(always)]
    pub const fn rticomp3clr(&self) -> &Rticomp3clr {
        &self.rticomp3clr
    }
}
#[doc = "RTIGCTRL (rw) register accessor: Global Control Register starts / stops the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rtigctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtigctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtigctrl`]
module"]
#[doc(alias = "RTIGCTRL")]
pub type Rtigctrl = crate::Reg<rtigctrl::RtigctrlSpec>;
#[doc = "Global Control Register starts / stops the counters"]
pub mod rtigctrl;
#[doc = "RTITBCTRL (rw) register accessor: Timebase Control selection which source triggers free running counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rtitbctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtitbctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtitbctrl`]
module"]
#[doc(alias = "RTITBCTRL")]
pub type Rtitbctrl = crate::Reg<rtitbctrl::RtitbctrlSpec>;
#[doc = "Timebase Control selection which source triggers free running counter 0"]
pub mod rtitbctrl;
#[doc = "RTICAPCTRL (rw) register accessor: Capture Control controls the capture source for the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rticapctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticapctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticapctrl`]
module"]
#[doc(alias = "RTICAPCTRL")]
pub type Rticapctrl = crate::Reg<rticapctrl::RticapctrlSpec>;
#[doc = "Capture Control controls the capture source for the counters"]
pub mod rticapctrl;
#[doc = "RTICOMPCTRL (rw) register accessor: Compare Control controls the source for the compare registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rticompctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticompctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticompctrl`]
module"]
#[doc(alias = "RTICOMPCTRL")]
pub type Rticompctrl = crate::Reg<rticompctrl::RticompctrlSpec>;
#[doc = "Compare Control controls the source for the compare registers"]
pub mod rticompctrl;
#[doc = "RTIFRC0 (rw) register accessor: Free Running Counter 0 current value of free running counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rtifrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtifrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtifrc0`]
module"]
#[doc(alias = "RTIFRC0")]
pub type Rtifrc0 = crate::Reg<rtifrc0::Rtifrc0Spec>;
#[doc = "Free Running Counter 0 current value of free running counter 0"]
pub mod rtifrc0;
#[doc = "RTIUC0 (rw) register accessor: Up Counter 0 current value of prescale counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiuc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiuc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiuc0`]
module"]
#[doc(alias = "RTIUC0")]
pub type Rtiuc0 = crate::Reg<rtiuc0::Rtiuc0Spec>;
#[doc = "Up Counter 0 current value of prescale counter 0"]
pub mod rtiuc0;
#[doc = "RTICPUC0 (rw) register accessor: Compare Up Counter 0 compare value compared with prescale counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rticpuc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticpuc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticpuc0`]
module"]
#[doc(alias = "RTICPUC0")]
pub type Rticpuc0 = crate::Reg<rticpuc0::Rticpuc0Spec>;
#[doc = "Compare Up Counter 0 compare value compared with prescale counter 0"]
pub mod rticpuc0;
#[doc = "RTICAFRC0 (rw) register accessor: Capture Free Running Counter 0 current value of free running counter 0 on external event\n\nYou can [`read`](crate::Reg::read) this register and get [`rticafrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticafrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticafrc0`]
module"]
#[doc(alias = "RTICAFRC0")]
pub type Rticafrc0 = crate::Reg<rticafrc0::Rticafrc0Spec>;
#[doc = "Capture Free Running Counter 0 current value of free running counter 0 on external event"]
pub mod rticafrc0;
#[doc = "RTICAUC0 (rw) register accessor: Capture Up Counter 0 current value of prescale counter 0 on external event\n\nYou can [`read`](crate::Reg::read) this register and get [`rticauc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticauc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticauc0`]
module"]
#[doc(alias = "RTICAUC0")]
pub type Rticauc0 = crate::Reg<rticauc0::Rticauc0Spec>;
#[doc = "Capture Up Counter 0 current value of prescale counter 0 on external event"]
pub mod rticauc0;
#[doc = "RTIFRC1 (rw) register accessor: Free Running Counter 1 current value of free running counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtifrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtifrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtifrc1`]
module"]
#[doc(alias = "RTIFRC1")]
pub type Rtifrc1 = crate::Reg<rtifrc1::Rtifrc1Spec>;
#[doc = "Free Running Counter 1 current value of free running counter 1"]
pub mod rtifrc1;
#[doc = "RTIUC1 (rw) register accessor: Up Counter 1 current value of prescale counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiuc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiuc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiuc1`]
module"]
#[doc(alias = "RTIUC1")]
pub type Rtiuc1 = crate::Reg<rtiuc1::Rtiuc1Spec>;
#[doc = "Up Counter 1 current value of prescale counter 1"]
pub mod rtiuc1;
#[doc = "RTICPUC1 (rw) register accessor: Compare Up Counter 1 compare value compared with prescale counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rticpuc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticpuc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticpuc1`]
module"]
#[doc(alias = "RTICPUC1")]
pub type Rticpuc1 = crate::Reg<rticpuc1::Rticpuc1Spec>;
#[doc = "Compare Up Counter 1 compare value compared with prescale counter 1"]
pub mod rticpuc1;
#[doc = "RTICAFRC1 (rw) register accessor: Capture Free Running Counter 1 current value of free running counter 1 on external event\n\nYou can [`read`](crate::Reg::read) this register and get [`rticafrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticafrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticafrc1`]
module"]
#[doc(alias = "RTICAFRC1")]
pub type Rticafrc1 = crate::Reg<rticafrc1::Rticafrc1Spec>;
#[doc = "Capture Free Running Counter 1 current value of free running counter 1 on external event"]
pub mod rticafrc1;
#[doc = "RTICAUC1 (rw) register accessor: Capture Up Counter 1 current value of prescale counter 1 on external event\n\nYou can [`read`](crate::Reg::read) this register and get [`rticauc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticauc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticauc1`]
module"]
#[doc(alias = "RTICAUC1")]
pub type Rticauc1 = crate::Reg<rticauc1::Rticauc1Spec>;
#[doc = "Capture Up Counter 1 current value of prescale counter 1 on external event"]
pub mod rticauc1;
#[doc = "RTICOMP0 (rw) register accessor: Compare 0 compare value to be compared with the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticomp0`]
module"]
#[doc(alias = "RTICOMP0")]
pub type Rticomp0 = crate::Reg<rticomp0::Rticomp0Spec>;
#[doc = "Compare 0 compare value to be compared with the counters"]
pub mod rticomp0;
#[doc = "RTIUDCP0 (rw) register accessor: Update Compare 0 value to be added to the compare register 0 value on compare match\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiudcp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiudcp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiudcp0`]
module"]
#[doc(alias = "RTIUDCP0")]
pub type Rtiudcp0 = crate::Reg<rtiudcp0::Rtiudcp0Spec>;
#[doc = "Update Compare 0 value to be added to the compare register 0 value on compare match"]
pub mod rtiudcp0;
#[doc = "RTICOMP1 (rw) register accessor: Compare 1 compare value to be compared with the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticomp1`]
module"]
#[doc(alias = "RTICOMP1")]
pub type Rticomp1 = crate::Reg<rticomp1::Rticomp1Spec>;
#[doc = "Compare 1 compare value to be compared with the counters"]
pub mod rticomp1;
#[doc = "RTIUDCP1 (rw) register accessor: Update Compare 1 value to be added to the compare register 1 value on compare match\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiudcp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiudcp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiudcp1`]
module"]
#[doc(alias = "RTIUDCP1")]
pub type Rtiudcp1 = crate::Reg<rtiudcp1::Rtiudcp1Spec>;
#[doc = "Update Compare 1 value to be added to the compare register 1 value on compare match"]
pub mod rtiudcp1;
#[doc = "RTICOMP2 (rw) register accessor: Compare 2 compare value to be compared with the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticomp2`]
module"]
#[doc(alias = "RTICOMP2")]
pub type Rticomp2 = crate::Reg<rticomp2::Rticomp2Spec>;
#[doc = "Compare 2 compare value to be compared with the counters"]
pub mod rticomp2;
#[doc = "RTIUDCP2 (rw) register accessor: Update Compare 2 value to be added to the compare register 2 value on compare match\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiudcp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiudcp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiudcp2`]
module"]
#[doc(alias = "RTIUDCP2")]
pub type Rtiudcp2 = crate::Reg<rtiudcp2::Rtiudcp2Spec>;
#[doc = "Update Compare 2 value to be added to the compare register 2 value on compare match"]
pub mod rtiudcp2;
#[doc = "RTICOMP3 (rw) register accessor: Compare 3 compare value to be compared with the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticomp3`]
module"]
#[doc(alias = "RTICOMP3")]
pub type Rticomp3 = crate::Reg<rticomp3::Rticomp3Spec>;
#[doc = "Compare 3 compare value to be compared with the counters"]
pub mod rticomp3;
#[doc = "RTIUDCP3 (rw) register accessor: Update Compare 3 value to be added to the compare register 3 value on compare match\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiudcp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiudcp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiudcp3`]
module"]
#[doc(alias = "RTIUDCP3")]
pub type Rtiudcp3 = crate::Reg<rtiudcp3::Rtiudcp3Spec>;
#[doc = "Update Compare 3 value to be added to the compare register 3 value on compare match"]
pub mod rtiudcp3;
#[doc = "RTITBLCOMP (rw) register accessor: Timebase Low Compare compare value to activate edge detection circuit\n\nYou can [`read`](crate::Reg::read) this register and get [`rtitblcomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtitblcomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtitblcomp`]
module"]
#[doc(alias = "RTITBLCOMP")]
pub type Rtitblcomp = crate::Reg<rtitblcomp::RtitblcompSpec>;
#[doc = "Timebase Low Compare compare value to activate edge detection circuit"]
pub mod rtitblcomp;
#[doc = "RTITBHCOMP (rw) register accessor: Timebase High Compare compare value to deactivate edge detection circuit\n\nYou can [`read`](crate::Reg::read) this register and get [`rtitbhcomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtitbhcomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtitbhcomp`]
module"]
#[doc(alias = "RTITBHCOMP")]
pub type Rtitbhcomp = crate::Reg<rtitbhcomp::RtitbhcompSpec>;
#[doc = "Timebase High Compare compare value to deactivate edge detection circuit"]
pub mod rtitbhcomp;
#[doc = "RTISETINT (rw) register accessor: Set Interrupt Enable sets interrupt enable bits int RTIINTCTRL without having to do a read-modify-write operation\n\nYou can [`read`](crate::Reg::read) this register and get [`rtisetint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtisetint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtisetint`]
module"]
#[doc(alias = "RTISETINT")]
pub type Rtisetint = crate::Reg<rtisetint::RtisetintSpec>;
#[doc = "Set Interrupt Enable sets interrupt enable bits int RTIINTCTRL without having to do a read-modify-write operation"]
pub mod rtisetint;
#[doc = "RTICLEARINT (rw) register accessor: Clear Interrupt Enable clears interrupt enable bits int RTIINTCTRL without having to do a read-modify-write operation\n\nYou can [`read`](crate::Reg::read) this register and get [`rticlearint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticlearint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticlearint`]
module"]
#[doc(alias = "RTICLEARINT")]
pub type Rticlearint = crate::Reg<rticlearint::RticlearintSpec>;
#[doc = "Clear Interrupt Enable clears interrupt enable bits int RTIINTCTRL without having to do a read-modify-write operation"]
pub mod rticlearint;
#[doc = "RTIINTFLAG (rw) register accessor: Interrupt Flags interrupt pending bits\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiintflag`]
module"]
#[doc(alias = "RTIINTFLAG")]
pub type Rtiintflag = crate::Reg<rtiintflag::RtiintflagSpec>;
#[doc = "Interrupt Flags interrupt pending bits"]
pub mod rtiintflag;
#[doc = "RTIDWDCTRL (rw) register accessor: Digital Watchdog Control Enables the Digital Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rtidwdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtidwdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtidwdctrl`]
module"]
#[doc(alias = "RTIDWDCTRL")]
pub type Rtidwdctrl = crate::Reg<rtidwdctrl::RtidwdctrlSpec>;
#[doc = "Digital Watchdog Control Enables the Digital Watchdog"]
pub mod rtidwdctrl;
#[doc = "RTIDWDPRLD (rw) register accessor: Digital Watchdog Preload sets the experation time of the Digital Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rtidwdprld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtidwdprld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtidwdprld`]
module"]
#[doc(alias = "RTIDWDPRLD")]
pub type Rtidwdprld = crate::Reg<rtidwdprld::RtidwdprldSpec>;
#[doc = "Digital Watchdog Preload sets the experation time of the Digital Watchdog"]
pub mod rtidwdprld;
#[doc = "RTIWDSTATUS (rw) register accessor: Watchdog Status reflects the status of Analog and Digital Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiwdstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiwdstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiwdstatus`]
module"]
#[doc(alias = "RTIWDSTATUS")]
pub type Rtiwdstatus = crate::Reg<rtiwdstatus::RtiwdstatusSpec>;
#[doc = "Watchdog Status reflects the status of Analog and Digital Watchdog"]
pub mod rtiwdstatus;
#[doc = "RTIWDKEY (rw) register accessor: Watchdog Key correct written key values discharge the external capacitor\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiwdkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiwdkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiwdkey`]
module"]
#[doc(alias = "RTIWDKEY")]
pub type Rtiwdkey = crate::Reg<rtiwdkey::RtiwdkeySpec>;
#[doc = "Watchdog Key correct written key values discharge the external capacitor"]
pub mod rtiwdkey;
#[doc = "RTIDWDCNTR (rw) register accessor: Digital Watchdog Down Counter current value of DWD down counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rtidwdcntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtidwdcntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtidwdcntr`]
module"]
#[doc(alias = "RTIDWDCNTR")]
pub type Rtidwdcntr = crate::Reg<rtidwdcntr::RtidwdcntrSpec>;
#[doc = "Digital Watchdog Down Counter current value of DWD down counter"]
pub mod rtidwdcntr;
#[doc = "RTIWWDRXNCTRL (rw) register accessor: Windowed Watchdog Reaction Control configures the windowed watchdog to either generate a non-maskable interrupt to the CPU or to generate a system reset\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiwwdrxnctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiwwdrxnctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiwwdrxnctrl`]
module"]
#[doc(alias = "RTIWWDRXNCTRL")]
pub type Rtiwwdrxnctrl = crate::Reg<rtiwwdrxnctrl::RtiwwdrxnctrlSpec>;
#[doc = "Windowed Watchdog Reaction Control configures the windowed watchdog to either generate a non-maskable interrupt to the CPU or to generate a system reset"]
pub mod rtiwwdrxnctrl;
#[doc = "RTIWWDSIZECTRL (rw) register accessor: Windowed Watchdog Size Control configures the size of the window for the digital windowed watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiwwdsizectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiwwdsizectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiwwdsizectrl`]
module"]
#[doc(alias = "RTIWWDSIZECTRL")]
pub type Rtiwwdsizectrl = crate::Reg<rtiwwdsizectrl::RtiwwdsizectrlSpec>;
#[doc = "Windowed Watchdog Size Control configures the size of the window for the digital windowed watchdog"]
pub mod rtiwwdsizectrl;
#[doc = "RTIINTCLRENABLE (rw) register accessor: RTI Compare Interrupt Clear Enable enable the auto clear functionality for each of the compare interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiintclrenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiintclrenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtiintclrenable`]
module"]
#[doc(alias = "RTIINTCLRENABLE")]
pub type Rtiintclrenable = crate::Reg<rtiintclrenable::RtiintclrenableSpec>;
#[doc = "RTI Compare Interrupt Clear Enable enable the auto clear functionality for each of the compare interrupts"]
pub mod rtiintclrenable;
#[doc = "RTICOMP0CLR (rw) register accessor: Compare 0 Clear compare value to be compared with the counter to clear the compare0 interrupt line\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp0clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp0clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticomp0clr`]
module"]
#[doc(alias = "RTICOMP0CLR")]
pub type Rticomp0clr = crate::Reg<rticomp0clr::Rticomp0clrSpec>;
#[doc = "Compare 0 Clear compare value to be compared with the counter to clear the compare0 interrupt line"]
pub mod rticomp0clr;
#[doc = "RTICOMP1CLR (rw) register accessor: Compare 1 Clear compare value to be compared with the counter to clear the compare1 interrupt line\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp1clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp1clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticomp1clr`]
module"]
#[doc(alias = "RTICOMP1CLR")]
pub type Rticomp1clr = crate::Reg<rticomp1clr::Rticomp1clrSpec>;
#[doc = "Compare 1 Clear compare value to be compared with the counter to clear the compare1 interrupt line"]
pub mod rticomp1clr;
#[doc = "RTICOMP2CLR (rw) register accessor: Compare 2 Clear compare value to be compared with the counter to clear the compare2 interrupt line\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp2clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp2clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticomp2clr`]
module"]
#[doc(alias = "RTICOMP2CLR")]
pub type Rticomp2clr = crate::Reg<rticomp2clr::Rticomp2clrSpec>;
#[doc = "Compare 2 Clear compare value to be compared with the counter to clear the compare2 interrupt line"]
pub mod rticomp2clr;
#[doc = "RTICOMP3CLR (rw) register accessor: Compare 3 Clear compare value to be compared with the counter to clear the compare3 interrupt line\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp3clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp3clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rticomp3clr`]
module"]
#[doc(alias = "RTICOMP3CLR")]
pub type Rticomp3clr = crate::Reg<rticomp3clr::Rticomp3clrSpec>;
#[doc = "Compare 3 Clear compare value to be compared with the counter to clear the compare3 interrupt line"]
pub mod rticomp3clr;
