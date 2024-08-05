#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    scigcr0: Scigcr0,
    scigcr1: Scigcr1,
    reserved1: Reserved1,
    scisetint: Scisetint,
    sciclearint: Sciclearint,
    scisetintlvl: Scisetintlvl,
    sciclearintlvl: Sciclearintlvl,
    sciflr: Sciflr,
    sciintvect0: Sciintvect0,
    sciintvect1: Sciintvect1,
    scichar: Scichar,
    scibaud: Scibaud,
    scied: Scied,
    scird: Scird,
    scitd: Scitd,
    scipio0: Scipio0,
    scipio1: Scipio1,
    scipio2: Scipio2,
    scipio3: Scipio3,
    scipio4: Scipio4,
    scipio5: Scipio5,
    scipio6: Scipio6,
    scipio7: Scipio7,
    scipio8: Scipio8,
    reserved2: Reserved2,
    reserved3: Reserved3,
    reserved4: Reserved4,
    reserved5: Reserved5,
    reserved6: Reserved6,
    reserved7: Reserved7,
    reserved8: Reserved8,
    reserved9: Reserved9,
    scipio9: Scipio9,
    _reserved33: [u8; 0x0c],
    sciiodctrl: Sciiodctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - The SCIGCR0 register defines the module reset"]
    #[inline(always)]
    pub const fn scigcr0(&self) -> &Scigcr0 {
        &self.scigcr0
    }
    #[doc = "0x04 - The SCIGCR1 register defines the frame format, protocol, and communication mode used by the SCI"]
    #[inline(always)]
    pub const fn scigcr1(&self) -> &Scigcr1 {
        &self.scigcr1
    }
    #[doc = "0x08 - Reserved"]
    #[inline(always)]
    pub const fn reserved1(&self) -> &Reserved1 {
        &self.reserved1
    }
    #[doc = "0x0c - SCI Set Interrupt Register"]
    #[inline(always)]
    pub const fn scisetint(&self) -> &Scisetint {
        &self.scisetint
    }
    #[doc = "0x10 - SCI Clear Interrupt Register"]
    #[inline(always)]
    pub const fn sciclearint(&self) -> &Sciclearint {
        &self.sciclearint
    }
    #[doc = "0x14 - SCI Set Interrupt Level Register"]
    #[inline(always)]
    pub const fn scisetintlvl(&self) -> &Scisetintlvl {
        &self.scisetintlvl
    }
    #[doc = "0x18 - SCI Clear Interrupt Level Register"]
    #[inline(always)]
    pub const fn sciclearintlvl(&self) -> &Sciclearintlvl {
        &self.sciclearintlvl
    }
    #[doc = "0x1c - SCI Flags Register"]
    #[inline(always)]
    pub const fn sciflr(&self) -> &Sciflr {
        &self.sciflr
    }
    #[doc = "0x20 - SCI Interrupt Offset Vector 0 Register"]
    #[inline(always)]
    pub const fn sciintvect0(&self) -> &Sciintvect0 {
        &self.sciintvect0
    }
    #[doc = "0x24 - SCI Interrupt Offset Vector 1 Register"]
    #[inline(always)]
    pub const fn sciintvect1(&self) -> &Sciintvect1 {
        &self.sciintvect1
    }
    #[doc = "0x28 - SCI Character Control Register"]
    #[inline(always)]
    pub const fn scichar(&self) -> &Scichar {
        &self.scichar
    }
    #[doc = "0x2c - SCI Baud Rate Selection Register"]
    #[inline(always)]
    pub const fn scibaud(&self) -> &Scibaud {
        &self.scibaud
    }
    #[doc = "0x30 - Receiver Emulation Data Buffer"]
    #[inline(always)]
    pub const fn scied(&self) -> &Scied {
        &self.scied
    }
    #[doc = "0x34 - Receiver Data Buffer"]
    #[inline(always)]
    pub const fn scird(&self) -> &Scird {
        &self.scird
    }
    #[doc = "0x38 - Transmit Data Buffer Register"]
    #[inline(always)]
    pub const fn scitd(&self) -> &Scitd {
        &self.scitd
    }
    #[doc = "0x3c - SCI Pin I/O Control Register 0"]
    #[inline(always)]
    pub const fn scipio0(&self) -> &Scipio0 {
        &self.scipio0
    }
    #[doc = "0x40 - SCI Pin I/O Control Register 1"]
    #[inline(always)]
    pub const fn scipio1(&self) -> &Scipio1 {
        &self.scipio1
    }
    #[doc = "0x44 - SCI Pin I/O Control Register 2"]
    #[inline(always)]
    pub const fn scipio2(&self) -> &Scipio2 {
        &self.scipio2
    }
    #[doc = "0x48 - SCI Pin I/O Control Register 3"]
    #[inline(always)]
    pub const fn scipio3(&self) -> &Scipio3 {
        &self.scipio3
    }
    #[doc = "0x4c - SCI Pin I/O Control Register 4"]
    #[inline(always)]
    pub const fn scipio4(&self) -> &Scipio4 {
        &self.scipio4
    }
    #[doc = "0x50 - SCI Pin I/O Control Register 5"]
    #[inline(always)]
    pub const fn scipio5(&self) -> &Scipio5 {
        &self.scipio5
    }
    #[doc = "0x54 - SCI Pin I/O Control Register 6"]
    #[inline(always)]
    pub const fn scipio6(&self) -> &Scipio6 {
        &self.scipio6
    }
    #[doc = "0x58 - SCI Pin I/O Control Register 7"]
    #[inline(always)]
    pub const fn scipio7(&self) -> &Scipio7 {
        &self.scipio7
    }
    #[doc = "0x5c - SCI Pin I/O Control Register 8"]
    #[inline(always)]
    pub const fn scipio8(&self) -> &Scipio8 {
        &self.scipio8
    }
    #[doc = "0x60 - Reserved"]
    #[inline(always)]
    pub const fn reserved2(&self) -> &Reserved2 {
        &self.reserved2
    }
    #[doc = "0x64 - Reserved"]
    #[inline(always)]
    pub const fn reserved3(&self) -> &Reserved3 {
        &self.reserved3
    }
    #[doc = "0x68 - Reserved"]
    #[inline(always)]
    pub const fn reserved4(&self) -> &Reserved4 {
        &self.reserved4
    }
    #[doc = "0x6c - Reserved"]
    #[inline(always)]
    pub const fn reserved5(&self) -> &Reserved5 {
        &self.reserved5
    }
    #[doc = "0x70 - Reserved"]
    #[inline(always)]
    pub const fn reserved6(&self) -> &Reserved6 {
        &self.reserved6
    }
    #[doc = "0x74 - Reserved"]
    #[inline(always)]
    pub const fn reserved7(&self) -> &Reserved7 {
        &self.reserved7
    }
    #[doc = "0x78 - Reserved"]
    #[inline(always)]
    pub const fn reserved8(&self) -> &Reserved8 {
        &self.reserved8
    }
    #[doc = "0x7c - Reserved"]
    #[inline(always)]
    pub const fn reserved9(&self) -> &Reserved9 {
        &self.reserved9
    }
    #[doc = "0x80 - SCI Pin I/O Control Register 9"]
    #[inline(always)]
    pub const fn scipio9(&self) -> &Scipio9 {
        &self.scipio9
    }
    #[doc = "0x90 - SCI IO DFT Control"]
    #[inline(always)]
    pub const fn sciiodctrl(&self) -> &Sciiodctrl {
        &self.sciiodctrl
    }
}
#[doc = "SCIGCR0 (rw) register accessor: The SCIGCR0 register defines the module reset\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scigcr0`]
module"]
#[doc(alias = "SCIGCR0")]
pub type Scigcr0 = crate::Reg<scigcr0::Scigcr0Spec>;
#[doc = "The SCIGCR0 register defines the module reset"]
pub mod scigcr0;
#[doc = "SCIGCR1 (rw) register accessor: The SCIGCR1 register defines the frame format, protocol, and communication mode used by the SCI\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scigcr1`]
module"]
#[doc(alias = "SCIGCR1")]
pub type Scigcr1 = crate::Reg<scigcr1::Scigcr1Spec>;
#[doc = "The SCIGCR1 register defines the frame format, protocol, and communication mode used by the SCI"]
pub mod scigcr1;
#[doc = "RESERVED1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved1`]
module"]
#[doc(alias = "RESERVED1")]
pub type Reserved1 = crate::Reg<reserved1::Reserved1Spec>;
#[doc = "Reserved"]
pub mod reserved1;
#[doc = "SCISETINT (rw) register accessor: SCI Set Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scisetint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scisetint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scisetint`]
module"]
#[doc(alias = "SCISETINT")]
pub type Scisetint = crate::Reg<scisetint::ScisetintSpec>;
#[doc = "SCI Set Interrupt Register"]
pub mod scisetint;
#[doc = "SCICLEARINT (rw) register accessor: SCI Clear Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciclearint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciclearint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciclearint`]
module"]
#[doc(alias = "SCICLEARINT")]
pub type Sciclearint = crate::Reg<sciclearint::SciclearintSpec>;
#[doc = "SCI Clear Interrupt Register"]
pub mod sciclearint;
#[doc = "SCISETINTLVL (rw) register accessor: SCI Set Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scisetintlvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scisetintlvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scisetintlvl`]
module"]
#[doc(alias = "SCISETINTLVL")]
pub type Scisetintlvl = crate::Reg<scisetintlvl::ScisetintlvlSpec>;
#[doc = "SCI Set Interrupt Level Register"]
pub mod scisetintlvl;
#[doc = "SCICLEARINTLVL (rw) register accessor: SCI Clear Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciclearintlvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciclearintlvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciclearintlvl`]
module"]
#[doc(alias = "SCICLEARINTLVL")]
pub type Sciclearintlvl = crate::Reg<sciclearintlvl::SciclearintlvlSpec>;
#[doc = "SCI Clear Interrupt Level Register"]
pub mod sciclearintlvl;
#[doc = "SCIFLR (rw) register accessor: SCI Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciflr`]
module"]
#[doc(alias = "SCIFLR")]
pub type Sciflr = crate::Reg<sciflr::SciflrSpec>;
#[doc = "SCI Flags Register"]
pub mod sciflr;
#[doc = "SCIINTVECT0 (rw) register accessor: SCI Interrupt Offset Vector 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciintvect0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciintvect0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciintvect0`]
module"]
#[doc(alias = "SCIINTVECT0")]
pub type Sciintvect0 = crate::Reg<sciintvect0::Sciintvect0Spec>;
#[doc = "SCI Interrupt Offset Vector 0 Register"]
pub mod sciintvect0;
#[doc = "SCIINTVECT1 (rw) register accessor: SCI Interrupt Offset Vector 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciintvect1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciintvect1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciintvect1`]
module"]
#[doc(alias = "SCIINTVECT1")]
pub type Sciintvect1 = crate::Reg<sciintvect1::Sciintvect1Spec>;
#[doc = "SCI Interrupt Offset Vector 1 Register"]
pub mod sciintvect1;
#[doc = "SCICHAR (rw) register accessor: SCI Character Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scichar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scichar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scichar`]
module"]
#[doc(alias = "SCICHAR")]
pub type Scichar = crate::Reg<scichar::ScicharSpec>;
#[doc = "SCI Character Control Register"]
pub mod scichar;
#[doc = "SCIBAUD (rw) register accessor: SCI Baud Rate Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scibaud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scibaud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scibaud`]
module"]
#[doc(alias = "SCIBAUD")]
pub type Scibaud = crate::Reg<scibaud::ScibaudSpec>;
#[doc = "SCI Baud Rate Selection Register"]
pub mod scibaud;
#[doc = "SCIED (rw) register accessor: Receiver Emulation Data Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`scied::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scied::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scied`]
module"]
#[doc(alias = "SCIED")]
pub type Scied = crate::Reg<scied::SciedSpec>;
#[doc = "Receiver Emulation Data Buffer"]
pub mod scied;
#[doc = "SCIRD (rw) register accessor: Receiver Data Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`scird::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scird::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scird`]
module"]
#[doc(alias = "SCIRD")]
pub type Scird = crate::Reg<scird::ScirdSpec>;
#[doc = "Receiver Data Buffer"]
pub mod scird;
#[doc = "SCITD (rw) register accessor: Transmit Data Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scitd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scitd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scitd`]
module"]
#[doc(alias = "SCITD")]
pub type Scitd = crate::Reg<scitd::ScitdSpec>;
#[doc = "Transmit Data Buffer Register"]
pub mod scitd;
#[doc = "SCIPIO0 (rw) register accessor: SCI Pin I/O Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio0`]
module"]
#[doc(alias = "SCIPIO0")]
pub type Scipio0 = crate::Reg<scipio0::Scipio0Spec>;
#[doc = "SCI Pin I/O Control Register 0"]
pub mod scipio0;
#[doc = "SCIPIO1 (rw) register accessor: SCI Pin I/O Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio1`]
module"]
#[doc(alias = "SCIPIO1")]
pub type Scipio1 = crate::Reg<scipio1::Scipio1Spec>;
#[doc = "SCI Pin I/O Control Register 1"]
pub mod scipio1;
#[doc = "SCIPIO2 (rw) register accessor: SCI Pin I/O Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio2`]
module"]
#[doc(alias = "SCIPIO2")]
pub type Scipio2 = crate::Reg<scipio2::Scipio2Spec>;
#[doc = "SCI Pin I/O Control Register 2"]
pub mod scipio2;
#[doc = "SCIPIO3 (rw) register accessor: SCI Pin I/O Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio3`]
module"]
#[doc(alias = "SCIPIO3")]
pub type Scipio3 = crate::Reg<scipio3::Scipio3Spec>;
#[doc = "SCI Pin I/O Control Register 3"]
pub mod scipio3;
#[doc = "SCIPIO4 (rw) register accessor: SCI Pin I/O Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio4`]
module"]
#[doc(alias = "SCIPIO4")]
pub type Scipio4 = crate::Reg<scipio4::Scipio4Spec>;
#[doc = "SCI Pin I/O Control Register 4"]
pub mod scipio4;
#[doc = "SCIPIO5 (rw) register accessor: SCI Pin I/O Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio5`]
module"]
#[doc(alias = "SCIPIO5")]
pub type Scipio5 = crate::Reg<scipio5::Scipio5Spec>;
#[doc = "SCI Pin I/O Control Register 5"]
pub mod scipio5;
#[doc = "SCIPIO6 (rw) register accessor: SCI Pin I/O Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio6`]
module"]
#[doc(alias = "SCIPIO6")]
pub type Scipio6 = crate::Reg<scipio6::Scipio6Spec>;
#[doc = "SCI Pin I/O Control Register 6"]
pub mod scipio6;
#[doc = "SCIPIO7 (rw) register accessor: SCI Pin I/O Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio7`]
module"]
#[doc(alias = "SCIPIO7")]
pub type Scipio7 = crate::Reg<scipio7::Scipio7Spec>;
#[doc = "SCI Pin I/O Control Register 7"]
pub mod scipio7;
#[doc = "SCIPIO8 (rw) register accessor: SCI Pin I/O Control Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio8`]
module"]
#[doc(alias = "SCIPIO8")]
pub type Scipio8 = crate::Reg<scipio8::Scipio8Spec>;
#[doc = "SCI Pin I/O Control Register 8"]
pub mod scipio8;
#[doc = "RESERVED2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved2`]
module"]
#[doc(alias = "RESERVED2")]
pub type Reserved2 = crate::Reg<reserved2::Reserved2Spec>;
#[doc = "Reserved"]
pub mod reserved2;
#[doc = "RESERVED3 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved3`]
module"]
#[doc(alias = "RESERVED3")]
pub type Reserved3 = crate::Reg<reserved3::Reserved3Spec>;
#[doc = "Reserved"]
pub mod reserved3;
#[doc = "RESERVED4 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved4`]
module"]
#[doc(alias = "RESERVED4")]
pub type Reserved4 = crate::Reg<reserved4::Reserved4Spec>;
#[doc = "Reserved"]
pub mod reserved4;
#[doc = "RESERVED5 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved5`]
module"]
#[doc(alias = "RESERVED5")]
pub type Reserved5 = crate::Reg<reserved5::Reserved5Spec>;
#[doc = "Reserved"]
pub mod reserved5;
#[doc = "RESERVED6 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved6`]
module"]
#[doc(alias = "RESERVED6")]
pub type Reserved6 = crate::Reg<reserved6::Reserved6Spec>;
#[doc = "Reserved"]
pub mod reserved6;
#[doc = "RESERVED7 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved7`]
module"]
#[doc(alias = "RESERVED7")]
pub type Reserved7 = crate::Reg<reserved7::Reserved7Spec>;
#[doc = "Reserved"]
pub mod reserved7;
#[doc = "RESERVED8 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved8`]
module"]
#[doc(alias = "RESERVED8")]
pub type Reserved8 = crate::Reg<reserved8::Reserved8Spec>;
#[doc = "Reserved"]
pub mod reserved8;
#[doc = "RESERVED9 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved9`]
module"]
#[doc(alias = "RESERVED9")]
pub type Reserved9 = crate::Reg<reserved9::Reserved9Spec>;
#[doc = "Reserved"]
pub mod reserved9;
#[doc = "SCIPIO9 (rw) register accessor: SCI Pin I/O Control Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio9`]
module"]
#[doc(alias = "SCIPIO9")]
pub type Scipio9 = crate::Reg<scipio9::Scipio9Spec>;
#[doc = "SCI Pin I/O Control Register 9"]
pub mod scipio9;
#[doc = "SCIIODCTRL (rw) register accessor: SCI IO DFT Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sciiodctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciiodctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciiodctrl`]
module"]
#[doc(alias = "SCIIODCTRL")]
pub type Sciiodctrl = crate::Reg<sciiodctrl::SciiodctrlSpec>;
#[doc = "SCI IO DFT Control"]
pub mod sciiodctrl;
