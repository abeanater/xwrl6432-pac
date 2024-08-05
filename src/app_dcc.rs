#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dccgctrl: Dccgctrl,
    dccrev: Dccrev,
    dcccntseed0: Dcccntseed0,
    dccvalidseed0: Dccvalidseed0,
    dcccntseed1: Dcccntseed1,
    dccstat: Dccstat,
    dcccnt0: Dcccnt0,
    dccvalid0: Dccvalid0,
    dcccnt1: Dcccnt1,
    dccclkssrc1: Dccclkssrc1,
    dccclkssrc0: Dccclkssrc0,
    _reserved11: [u8; 0x04],
    dccgctrl2: Dccgctrl2,
    dccstatus2: Dccstatus2,
    dccerrcnt: Dccerrcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Starts / stops the counters clears the error signal"]
    #[inline(always)]
    pub const fn dccgctrl(&self) -> &Dccgctrl {
        &self.dccgctrl
    }
    #[doc = "0x04 - Module version"]
    #[inline(always)]
    pub const fn dccrev(&self) -> &Dccrev {
        &self.dccrev
    }
    #[doc = "0x08 - Seed value for the counter attached to clock source 0"]
    #[inline(always)]
    pub const fn dcccntseed0(&self) -> &Dcccntseed0 {
        &self.dcccntseed0
    }
    #[doc = "0x0c - Seed value for the timeout counter attached to clock source 0"]
    #[inline(always)]
    pub const fn dccvalidseed0(&self) -> &Dccvalidseed0 {
        &self.dccvalidseed0
    }
    #[doc = "0x10 - Seed value for the counter attached to clock source 1"]
    #[inline(always)]
    pub const fn dcccntseed1(&self) -> &Dcccntseed1 {
        &self.dcccntseed1
    }
    #[doc = "0x14 - Contains the error &amp; done flag bit"]
    #[inline(always)]
    pub const fn dccstat(&self) -> &Dccstat {
        &self.dccstat
    }
    #[doc = "0x18 - Value of the counter attached to clock source 0"]
    #[inline(always)]
    pub const fn dcccnt0(&self) -> &Dcccnt0 {
        &self.dcccnt0
    }
    #[doc = "0x1c - Value of the valid counter attached to clock source 0"]
    #[inline(always)]
    pub const fn dccvalid0(&self) -> &Dccvalid0 {
        &self.dccvalid0
    }
    #[doc = "0x20 - Value of the counter attached to clock source 1"]
    #[inline(always)]
    pub const fn dcccnt1(&self) -> &Dcccnt1 {
        &self.dcccnt1
    }
    #[doc = "0x24 - Clock source1 selection control"]
    #[inline(always)]
    pub const fn dccclkssrc1(&self) -> &Dccclkssrc1 {
        &self.dccclkssrc1
    }
    #[doc = "0x28 - Clock source0 selection control"]
    #[inline(always)]
    pub const fn dccclkssrc0(&self) -> &Dccclkssrc0 {
        &self.dccclkssrc0
    }
    #[doc = "0x30 - Global control register 2"]
    #[inline(always)]
    pub const fn dccgctrl2(&self) -> &Dccgctrl2 {
        &self.dccgctrl2
    }
    #[doc = "0x34 - FIFO status register"]
    #[inline(always)]
    pub const fn dccstatus2(&self) -> &Dccstatus2 {
        &self.dccstatus2
    }
    #[doc = "0x38 - Error count register"]
    #[inline(always)]
    pub const fn dccerrcnt(&self) -> &Dccerrcnt {
        &self.dccerrcnt
    }
}
#[doc = "DCCGCTRL (rw) register accessor: Starts / stops the counters clears the error signal\n\nYou can [`read`](crate::Reg::read) this register and get [`dccgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccgctrl`]
module"]
#[doc(alias = "DCCGCTRL")]
pub type Dccgctrl = crate::Reg<dccgctrl::DccgctrlSpec>;
#[doc = "Starts / stops the counters clears the error signal"]
pub mod dccgctrl;
#[doc = "DCCREV (rw) register accessor: Module version\n\nYou can [`read`](crate::Reg::read) this register and get [`dccrev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccrev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccrev`]
module"]
#[doc(alias = "DCCREV")]
pub type Dccrev = crate::Reg<dccrev::DccrevSpec>;
#[doc = "Module version"]
pub mod dccrev;
#[doc = "DCCCNTSEED0 (rw) register accessor: Seed value for the counter attached to clock source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcccntseed0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcccntseed0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcccntseed0`]
module"]
#[doc(alias = "DCCCNTSEED0")]
pub type Dcccntseed0 = crate::Reg<dcccntseed0::Dcccntseed0Spec>;
#[doc = "Seed value for the counter attached to clock source 0"]
pub mod dcccntseed0;
#[doc = "DCCVALIDSEED0 (rw) register accessor: Seed value for the timeout counter attached to clock source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dccvalidseed0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccvalidseed0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccvalidseed0`]
module"]
#[doc(alias = "DCCVALIDSEED0")]
pub type Dccvalidseed0 = crate::Reg<dccvalidseed0::Dccvalidseed0Spec>;
#[doc = "Seed value for the timeout counter attached to clock source 0"]
pub mod dccvalidseed0;
#[doc = "DCCCNTSEED1 (rw) register accessor: Seed value for the counter attached to clock source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcccntseed1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcccntseed1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcccntseed1`]
module"]
#[doc(alias = "DCCCNTSEED1")]
pub type Dcccntseed1 = crate::Reg<dcccntseed1::Dcccntseed1Spec>;
#[doc = "Seed value for the counter attached to clock source 1"]
pub mod dcccntseed1;
#[doc = "DCCSTAT (rw) register accessor: Contains the error &amp; done flag bit\n\nYou can [`read`](crate::Reg::read) this register and get [`dccstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccstat`]
module"]
#[doc(alias = "DCCSTAT")]
pub type Dccstat = crate::Reg<dccstat::DccstatSpec>;
#[doc = "Contains the error &amp; done flag bit"]
pub mod dccstat;
#[doc = "DCCCNT0 (rw) register accessor: Value of the counter attached to clock source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcccnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcccnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcccnt0`]
module"]
#[doc(alias = "DCCCNT0")]
pub type Dcccnt0 = crate::Reg<dcccnt0::Dcccnt0Spec>;
#[doc = "Value of the counter attached to clock source 0"]
pub mod dcccnt0;
#[doc = "DCCVALID0 (rw) register accessor: Value of the valid counter attached to clock source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dccvalid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccvalid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccvalid0`]
module"]
#[doc(alias = "DCCVALID0")]
pub type Dccvalid0 = crate::Reg<dccvalid0::Dccvalid0Spec>;
#[doc = "Value of the valid counter attached to clock source 0"]
pub mod dccvalid0;
#[doc = "DCCCNT1 (rw) register accessor: Value of the counter attached to clock source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcccnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcccnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcccnt1`]
module"]
#[doc(alias = "DCCCNT1")]
pub type Dcccnt1 = crate::Reg<dcccnt1::Dcccnt1Spec>;
#[doc = "Value of the counter attached to clock source 1"]
pub mod dcccnt1;
#[doc = "DCCCLKSSRC1 (rw) register accessor: Clock source1 selection control\n\nYou can [`read`](crate::Reg::read) this register and get [`dccclkssrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccclkssrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccclkssrc1`]
module"]
#[doc(alias = "DCCCLKSSRC1")]
pub type Dccclkssrc1 = crate::Reg<dccclkssrc1::Dccclkssrc1Spec>;
#[doc = "Clock source1 selection control"]
pub mod dccclkssrc1;
#[doc = "DCCCLKSSRC0 (rw) register accessor: Clock source0 selection control\n\nYou can [`read`](crate::Reg::read) this register and get [`dccclkssrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccclkssrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccclkssrc0`]
module"]
#[doc(alias = "DCCCLKSSRC0")]
pub type Dccclkssrc0 = crate::Reg<dccclkssrc0::Dccclkssrc0Spec>;
#[doc = "Clock source0 selection control"]
pub mod dccclkssrc0;
#[doc = "DCCGCTRL2 (rw) register accessor: Global control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dccgctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccgctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccgctrl2`]
module"]
#[doc(alias = "DCCGCTRL2")]
pub type Dccgctrl2 = crate::Reg<dccgctrl2::Dccgctrl2Spec>;
#[doc = "Global control register 2"]
pub mod dccgctrl2;
#[doc = "DCCSTATUS2 (rw) register accessor: FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dccstatus2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccstatus2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccstatus2`]
module"]
#[doc(alias = "DCCSTATUS2")]
pub type Dccstatus2 = crate::Reg<dccstatus2::Dccstatus2Spec>;
#[doc = "FIFO status register"]
pub mod dccstatus2;
#[doc = "DCCERRCNT (rw) register accessor: Error count register\n\nYou can [`read`](crate::Reg::read) this register and get [`dccerrcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccerrcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dccerrcnt`]
module"]
#[doc(alias = "DCCERRCNT")]
pub type Dccerrcnt = crate::Reg<dccerrcnt::DccerrcntSpec>;
#[doc = "Error count register"]
pub mod dccerrcnt;
