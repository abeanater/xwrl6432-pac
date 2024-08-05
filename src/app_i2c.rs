#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    icoar: Icoar,
    icimr: Icimr,
    icstr: Icstr,
    icclkl: Icclkl,
    icclkh: Icclkh,
    iccnt: Iccnt,
    icdrr: Icdrr,
    icsar: Icsar,
    icdxr: Icdxr,
    icmdr: Icmdr,
    icivr: Icivr,
    icemdr: Icemdr,
    icpsc: Icpsc,
    icpid1: Icpid1,
    icpid2: Icpid2,
    icdmac: Icdmac,
    i2c_reserved1: I2cReserved1,
    i2c_reserved2: I2cReserved2,
    icpfunc: Icpfunc,
    icpdir: Icpdir,
    icpdin: Icpdin,
    icpdout: Icpdout,
    icpdset: Icpdset,
    icpdclr: Icpdclr,
    icpdrv: Icpdrv,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C Own Address register"]
    #[inline(always)]
    pub const fn icoar(&self) -> &Icoar {
        &self.icoar
    }
    #[doc = "0x04 - I2C Interrupt Mask/Status register"]
    #[inline(always)]
    pub const fn icimr(&self) -> &Icimr {
        &self.icimr
    }
    #[doc = "0x08 - I2C Interrupt Status register"]
    #[inline(always)]
    pub const fn icstr(&self) -> &Icstr {
        &self.icstr
    }
    #[doc = "0x0c - I2C Clock Divider Low register"]
    #[inline(always)]
    pub const fn icclkl(&self) -> &Icclkl {
        &self.icclkl
    }
    #[doc = "0x10 - I2C Clock Divider High register"]
    #[inline(always)]
    pub const fn icclkh(&self) -> &Icclkh {
        &self.icclkh
    }
    #[doc = "0x14 - I2C Data Count register"]
    #[inline(always)]
    pub const fn iccnt(&self) -> &Iccnt {
        &self.iccnt
    }
    #[doc = "0x18 - I2C Data Receive register"]
    #[inline(always)]
    pub const fn icdrr(&self) -> &Icdrr {
        &self.icdrr
    }
    #[doc = "0x1c - I2C Slave Address register"]
    #[inline(always)]
    pub const fn icsar(&self) -> &Icsar {
        &self.icsar
    }
    #[doc = "0x20 - I2C Data Transmit register"]
    #[inline(always)]
    pub const fn icdxr(&self) -> &Icdxr {
        &self.icdxr
    }
    #[doc = "0x24 - I2C Mode register"]
    #[inline(always)]
    pub const fn icmdr(&self) -> &Icmdr {
        &self.icmdr
    }
    #[doc = "0x28 - I2C Interrupt Vector register"]
    #[inline(always)]
    pub const fn icivr(&self) -> &Icivr {
        &self.icivr
    }
    #[doc = "0x2c - I2C Extended Mode register"]
    #[inline(always)]
    pub const fn icemdr(&self) -> &Icemdr {
        &self.icemdr
    }
    #[doc = "0x30 - I2C Prescaler register"]
    #[inline(always)]
    pub const fn icpsc(&self) -> &Icpsc {
        &self.icpsc
    }
    #[doc = "0x34 - I2C Peripheral ID register 1"]
    #[inline(always)]
    pub const fn icpid1(&self) -> &Icpid1 {
        &self.icpid1
    }
    #[doc = "0x38 - I2C Peripheral ID register 2"]
    #[inline(always)]
    pub const fn icpid2(&self) -> &Icpid2 {
        &self.icpid2
    }
    #[doc = "0x3c - I2C DMA Control Register"]
    #[inline(always)]
    pub const fn icdmac(&self) -> &Icdmac {
        &self.icdmac
    }
    #[doc = "0x40 - Reserved"]
    #[inline(always)]
    pub const fn i2c_reserved1(&self) -> &I2cReserved1 {
        &self.i2c_reserved1
    }
    #[doc = "0x44 - Reserved"]
    #[inline(always)]
    pub const fn i2c_reserved2(&self) -> &I2cReserved2 {
        &self.i2c_reserved2
    }
    #[doc = "0x48 - I2C Pin Function register"]
    #[inline(always)]
    pub const fn icpfunc(&self) -> &Icpfunc {
        &self.icpfunc
    }
    #[doc = "0x4c - I2C Pin Direction register"]
    #[inline(always)]
    pub const fn icpdir(&self) -> &Icpdir {
        &self.icpdir
    }
    #[doc = "0x50 - I2C Pin Data In register"]
    #[inline(always)]
    pub const fn icpdin(&self) -> &Icpdin {
        &self.icpdin
    }
    #[doc = "0x54 - I2C Pin Data Out register"]
    #[inline(always)]
    pub const fn icpdout(&self) -> &Icpdout {
        &self.icpdout
    }
    #[doc = "0x58 - I2C Pin Data Set register"]
    #[inline(always)]
    pub const fn icpdset(&self) -> &Icpdset {
        &self.icpdset
    }
    #[doc = "0x5c - I2C Pin Data Clear register"]
    #[inline(always)]
    pub const fn icpdclr(&self) -> &Icpdclr {
        &self.icpdclr
    }
    #[doc = "0x60 - I2C Pin Driver Mode Register"]
    #[inline(always)]
    pub const fn icpdrv(&self) -> &Icpdrv {
        &self.icpdrv
    }
}
#[doc = "ICOAR (rw) register accessor: I2C Own Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`icoar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icoar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icoar`]
module"]
#[doc(alias = "ICOAR")]
pub type Icoar = crate::Reg<icoar::IcoarSpec>;
#[doc = "I2C Own Address register"]
pub mod icoar;
#[doc = "ICIMR (rw) register accessor: I2C Interrupt Mask/Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`icimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icimr`]
module"]
#[doc(alias = "ICIMR")]
pub type Icimr = crate::Reg<icimr::IcimrSpec>;
#[doc = "I2C Interrupt Mask/Status register"]
pub mod icimr;
#[doc = "ICSTR (rw) register accessor: I2C Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`icstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icstr`]
module"]
#[doc(alias = "ICSTR")]
pub type Icstr = crate::Reg<icstr::IcstrSpec>;
#[doc = "I2C Interrupt Status register"]
pub mod icstr;
#[doc = "ICCLKL (rw) register accessor: I2C Clock Divider Low register\n\nYou can [`read`](crate::Reg::read) this register and get [`icclkl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icclkl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icclkl`]
module"]
#[doc(alias = "ICCLKL")]
pub type Icclkl = crate::Reg<icclkl::IcclklSpec>;
#[doc = "I2C Clock Divider Low register"]
pub mod icclkl;
#[doc = "ICCLKH (rw) register accessor: I2C Clock Divider High register\n\nYou can [`read`](crate::Reg::read) this register and get [`icclkh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icclkh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icclkh`]
module"]
#[doc(alias = "ICCLKH")]
pub type Icclkh = crate::Reg<icclkh::IcclkhSpec>;
#[doc = "I2C Clock Divider High register"]
pub mod icclkh;
#[doc = "ICCNT (rw) register accessor: I2C Data Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`iccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iccnt`]
module"]
#[doc(alias = "ICCNT")]
pub type Iccnt = crate::Reg<iccnt::IccntSpec>;
#[doc = "I2C Data Count register"]
pub mod iccnt;
#[doc = "ICDRR (rw) register accessor: I2C Data Receive register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icdrr`]
module"]
#[doc(alias = "ICDRR")]
pub type Icdrr = crate::Reg<icdrr::IcdrrSpec>;
#[doc = "I2C Data Receive register"]
pub mod icdrr;
#[doc = "ICSAR (rw) register accessor: I2C Slave Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsar`]
module"]
#[doc(alias = "ICSAR")]
pub type Icsar = crate::Reg<icsar::IcsarSpec>;
#[doc = "I2C Slave Address register"]
pub mod icsar;
#[doc = "ICDXR (rw) register accessor: I2C Data Transmit register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdxr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdxr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icdxr`]
module"]
#[doc(alias = "ICDXR")]
pub type Icdxr = crate::Reg<icdxr::IcdxrSpec>;
#[doc = "I2C Data Transmit register"]
pub mod icdxr;
#[doc = "ICMDR (rw) register accessor: I2C Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`icmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icmdr`]
module"]
#[doc(alias = "ICMDR")]
pub type Icmdr = crate::Reg<icmdr::IcmdrSpec>;
#[doc = "I2C Mode register"]
pub mod icmdr;
#[doc = "ICIVR (rw) register accessor: I2C Interrupt Vector register\n\nYou can [`read`](crate::Reg::read) this register and get [`icivr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icivr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icivr`]
module"]
#[doc(alias = "ICIVR")]
pub type Icivr = crate::Reg<icivr::IcivrSpec>;
#[doc = "I2C Interrupt Vector register"]
pub mod icivr;
#[doc = "ICEMDR (rw) register accessor: I2C Extended Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`icemdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icemdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icemdr`]
module"]
#[doc(alias = "ICEMDR")]
pub type Icemdr = crate::Reg<icemdr::IcemdrSpec>;
#[doc = "I2C Extended Mode register"]
pub mod icemdr;
#[doc = "ICPSC (rw) register accessor: I2C Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpsc`]
module"]
#[doc(alias = "ICPSC")]
pub type Icpsc = crate::Reg<icpsc::IcpscSpec>;
#[doc = "I2C Prescaler register"]
pub mod icpsc;
#[doc = "ICPID1 (rw) register accessor: I2C Peripheral ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`icpid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpid1`]
module"]
#[doc(alias = "ICPID1")]
pub type Icpid1 = crate::Reg<icpid1::Icpid1Spec>;
#[doc = "I2C Peripheral ID register 1"]
pub mod icpid1;
#[doc = "ICPID2 (rw) register accessor: I2C Peripheral ID register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`icpid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpid2`]
module"]
#[doc(alias = "ICPID2")]
pub type Icpid2 = crate::Reg<icpid2::Icpid2Spec>;
#[doc = "I2C Peripheral ID register 2"]
pub mod icpid2;
#[doc = "ICDMAC (rw) register accessor: I2C DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icdmac`]
module"]
#[doc(alias = "ICDMAC")]
pub type Icdmac = crate::Reg<icdmac::IcdmacSpec>;
#[doc = "I2C DMA Control Register"]
pub mod icdmac;
#[doc = "I2C_RESERVED1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_reserved1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_reserved1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_reserved1`]
module"]
#[doc(alias = "I2C_RESERVED1")]
pub type I2cReserved1 = crate::Reg<i2c_reserved1::I2cReserved1Spec>;
#[doc = "Reserved"]
pub mod i2c_reserved1;
#[doc = "I2C_RESERVED2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_reserved2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_reserved2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_reserved2`]
module"]
#[doc(alias = "I2C_RESERVED2")]
pub type I2cReserved2 = crate::Reg<i2c_reserved2::I2cReserved2Spec>;
#[doc = "Reserved"]
pub mod i2c_reserved2;
#[doc = "ICPFUNC (rw) register accessor: I2C Pin Function register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpfunc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpfunc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpfunc`]
module"]
#[doc(alias = "ICPFUNC")]
pub type Icpfunc = crate::Reg<icpfunc::IcpfuncSpec>;
#[doc = "I2C Pin Function register"]
pub mod icpfunc;
#[doc = "ICPDIR (rw) register accessor: I2C Pin Direction register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpdir`]
module"]
#[doc(alias = "ICPDIR")]
pub type Icpdir = crate::Reg<icpdir::IcpdirSpec>;
#[doc = "I2C Pin Direction register"]
pub mod icpdir;
#[doc = "ICPDIN (rw) register accessor: I2C Pin Data In register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpdin`]
module"]
#[doc(alias = "ICPDIN")]
pub type Icpdin = crate::Reg<icpdin::IcpdinSpec>;
#[doc = "I2C Pin Data In register"]
pub mod icpdin;
#[doc = "ICPDOUT (rw) register accessor: I2C Pin Data Out register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpdout`]
module"]
#[doc(alias = "ICPDOUT")]
pub type Icpdout = crate::Reg<icpdout::IcpdoutSpec>;
#[doc = "I2C Pin Data Out register"]
pub mod icpdout;
#[doc = "ICPDSET (rw) register accessor: I2C Pin Data Set register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpdset`]
module"]
#[doc(alias = "ICPDSET")]
pub type Icpdset = crate::Reg<icpdset::IcpdsetSpec>;
#[doc = "I2C Pin Data Set register"]
pub mod icpdset;
#[doc = "ICPDCLR (rw) register accessor: I2C Pin Data Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpdclr`]
module"]
#[doc(alias = "ICPDCLR")]
pub type Icpdclr = crate::Reg<icpdclr::IcpdclrSpec>;
#[doc = "I2C Pin Data Clear register"]
pub mod icpdclr;
#[doc = "ICPDRV (rw) register accessor: I2C Pin Driver Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpdrv`]
module"]
#[doc(alias = "ICPDRV")]
pub type Icpdrv = crate::Reg<icpdrv::IcpdrvSpec>;
#[doc = "I2C Pin Driver Mode Register"]
pub mod icpdrv;
