#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    scigcr0: Scigcr0,
    scigcr1: Scigcr1,
    scigcr2: Scigcr2,
    scisetint: Scisetint,
    sciclearint: Sciclearint,
    scisetintlvl: Scisetintlvl,
    sciclearintlvl: Sciclearintlvl,
    sciflr: Sciflr,
    sciintvect0: Sciintvect0,
    sciintvect1: Sciintvect1,
    sciformat: Sciformat,
    brsr: Brsr,
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
    lincomp: Lincomp,
    linrd0: Linrd0,
    linrd1: Linrd1,
    linmask: Linmask,
    linid: Linid,
    lintd0: Lintd0,
    lintd1: Lintd1,
    mbrsr: Mbrsr,
    scipio9: Scipio9,
    reserved: Reserved,
    reserved1: Reserved1,
    reserved2: Reserved2,
    iodftctrl: Iodftctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - The SCIGCR0 register defines the module reset."]
    #[inline(always)]
    pub const fn scigcr0(&self) -> &Scigcr0 {
        &self.scigcr0
    }
    #[doc = "0x04 - The SCIGCR1 register defines the frame format, protocol, and communication mode used by the SCI."]
    #[inline(always)]
    pub const fn scigcr1(&self) -> &Scigcr1 {
        &self.scigcr1
    }
    #[doc = "0x08 - The SCIGCR2 register is used to send or compare a checksum byte during extended frames, to generate a wakeup and for low-power mode control of the LIN module."]
    #[inline(always)]
    pub const fn scigcr2(&self) -> &Scigcr2 {
        &self.scigcr2
    }
    #[doc = "0x0c - The SCISETINT register is used to enable the various interrupts available in the LIN module."]
    #[inline(always)]
    pub const fn scisetint(&self) -> &Scisetint {
        &self.scisetint
    }
    #[doc = "0x10 - The SCICLEARINT register is used to disable the enabled interrupts without accessing the SCISETINT register."]
    #[inline(always)]
    pub const fn sciclearint(&self) -> &Sciclearint {
        &self.sciclearint
    }
    #[doc = "0x14 - The SCISETINTLVL register is used to map individual interrupt sources to the INT1 interrupt line."]
    #[inline(always)]
    pub const fn scisetintlvl(&self) -> &Scisetintlvl {
        &self.scisetintlvl
    }
    #[doc = "0x18 - The SCICLEARINTLVL register is used to map individual interrupt sources to the INT0 line."]
    #[inline(always)]
    pub const fn sciclearintlvl(&self) -> &Sciclearintlvl {
        &self.sciclearintlvl
    }
    #[doc = "0x1c - The SCIFLR register indicates the current status of the various interrupt sources of the LIN module."]
    #[inline(always)]
    pub const fn sciflr(&self) -> &Sciflr {
        &self.sciflr
    }
    #[doc = "0x20 - The SCIINTVECT0 register indicates the offset for the INT0 interrupt line."]
    #[inline(always)]
    pub const fn sciintvect0(&self) -> &Sciintvect0 {
        &self.sciintvect0
    }
    #[doc = "0x24 - The SCIINTVECT1 register indicates the offset for the INT1 interrupt line."]
    #[inline(always)]
    pub const fn sciintvect1(&self) -> &Sciintvect1 {
        &self.sciintvect1
    }
    #[doc = "0x28 - The SCIFORMAT register is used to set up the character and frame lengths."]
    #[inline(always)]
    pub const fn sciformat(&self) -> &Sciformat {
        &self.sciformat
    }
    #[doc = "0x2c - The BRSR register is used to configure the baud rate of the LIN module."]
    #[inline(always)]
    pub const fn brsr(&self) -> &Brsr {
        &self.brsr
    }
    #[doc = "0x30 - The SCIED register is a duplicate copy of SCIRD register that has no affect on the RXRDY flag for use with an emulator."]
    #[inline(always)]
    pub const fn scied(&self) -> &Scied {
        &self.scied
    }
    #[doc = "0x34 - The SCIRD register is where received data is stored and can be read from."]
    #[inline(always)]
    pub const fn scird(&self) -> &Scird {
        &self.scird
    }
    #[doc = "0x38 - The SCITD register is where data to be transmitted is written to by application software."]
    #[inline(always)]
    pub const fn scitd(&self) -> &Scitd {
        &self.scitd
    }
    #[doc = "0x3c - The SCIPIO0 register is used to enable the LINTX and LINRX pins."]
    #[inline(always)]
    pub const fn scipio0(&self) -> &Scipio0 {
        &self.scipio0
    }
    #[doc = "0x40 - SCIPIO1"]
    #[inline(always)]
    pub const fn scipio1(&self) -> &Scipio1 {
        &self.scipio1
    }
    #[doc = "0x44 - The SCIPIO2 register indicates the current status of the LINTX and LINRX pins."]
    #[inline(always)]
    pub const fn scipio2(&self) -> &Scipio2 {
        &self.scipio2
    }
    #[doc = "0x48 - SCIPIO3"]
    #[inline(always)]
    pub const fn scipio3(&self) -> &Scipio3 {
        &self.scipio3
    }
    #[doc = "0x4c - SCIPIO4"]
    #[inline(always)]
    pub const fn scipio4(&self) -> &Scipio4 {
        &self.scipio4
    }
    #[doc = "0x50 - SCIPIO5"]
    #[inline(always)]
    pub const fn scipio5(&self) -> &Scipio5 {
        &self.scipio5
    }
    #[doc = "0x54 - SCIPIO6"]
    #[inline(always)]
    pub const fn scipio6(&self) -> &Scipio6 {
        &self.scipio6
    }
    #[doc = "0x58 - SCIPIO7"]
    #[inline(always)]
    pub const fn scipio7(&self) -> &Scipio7 {
        &self.scipio7
    }
    #[doc = "0x5c - SCIPIO8"]
    #[inline(always)]
    pub const fn scipio8(&self) -> &Scipio8 {
        &self.scipio8
    }
    #[doc = "0x60 - The LINCOMPARE register is used to configure the sync delimeter and sync break extension."]
    #[inline(always)]
    pub const fn lincomp(&self) -> &Lincomp {
        &self.lincomp
    }
    #[doc = "0x64 - The LINRD0 register contains the lower 4 bytes of the received LIN frame data."]
    #[inline(always)]
    pub const fn linrd0(&self) -> &Linrd0 {
        &self.linrd0
    }
    #[doc = "0x68 - The LINRD1 regsiter contains the upper 4 bytes of the received LIN frame data."]
    #[inline(always)]
    pub const fn linrd1(&self) -> &Linrd1 {
        &self.linrd1
    }
    #[doc = "0x6c - The LINMASK register is used to configure the masks used for filtering incoming ID messages for receive and transmit frames."]
    #[inline(always)]
    pub const fn linmask(&self) -> &Linmask {
        &self.linmask
    }
    #[doc = "0x70 - The LINID register contains the identification fields for LIN communication. NOTE: For software compatibility with future LIN modules, the HGEN CTRL bit must be set to 1, the RX ID MASK field must be set to FFh, and the TX ID MASK field must be set to FFh."]
    #[inline(always)]
    pub const fn linid(&self) -> &Linid {
        &self.linid
    }
    #[doc = "0x74 - The LINTD0 register contains the lower 4 bytes of the data to be transmitted. NOTE: TD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame."]
    #[inline(always)]
    pub const fn lintd0(&self) -> &Lintd0 {
        &self.lintd0
    }
    #[doc = "0x78 - The LINTD1 register contains the upper 4 bytes of the data to be transmitted. NOTE: TD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame."]
    #[inline(always)]
    pub const fn lintd1(&self) -> &Lintd1 {
        &self.lintd1
    }
    #[doc = "0x7c - The MBRSR register is used to configure the expected maximum baud rate of the LIN network."]
    #[inline(always)]
    pub const fn mbrsr(&self) -> &Mbrsr {
        &self.mbrsr
    }
    #[doc = "0x80 - Couldn't find this register in spec. But it's mentioned in RTL."]
    #[inline(always)]
    pub const fn scipio9(&self) -> &Scipio9 {
        &self.scipio9
    }
    #[doc = "0x84 - Reserved"]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
    #[doc = "0x88 - Reserved1"]
    #[inline(always)]
    pub const fn reserved1(&self) -> &Reserved1 {
        &self.reserved1
    }
    #[doc = "0x8c - Reserved2"]
    #[inline(always)]
    pub const fn reserved2(&self) -> &Reserved2 {
        &self.reserved2
    }
    #[doc = "0x90 - The IODFTCTRL register is used to emulate various error and test conditions."]
    #[inline(always)]
    pub const fn iodftctrl(&self) -> &Iodftctrl {
        &self.iodftctrl
    }
}
#[doc = "SCIGCR0 (rw) register accessor: The SCIGCR0 register defines the module reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scigcr0`]
module"]
#[doc(alias = "SCIGCR0")]
pub type Scigcr0 = crate::Reg<scigcr0::Scigcr0Spec>;
#[doc = "The SCIGCR0 register defines the module reset."]
pub mod scigcr0;
#[doc = "SCIGCR1 (rw) register accessor: The SCIGCR1 register defines the frame format, protocol, and communication mode used by the SCI.\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scigcr1`]
module"]
#[doc(alias = "SCIGCR1")]
pub type Scigcr1 = crate::Reg<scigcr1::Scigcr1Spec>;
#[doc = "The SCIGCR1 register defines the frame format, protocol, and communication mode used by the SCI."]
pub mod scigcr1;
#[doc = "SCIGCR2 (rw) register accessor: The SCIGCR2 register is used to send or compare a checksum byte during extended frames, to generate a wakeup and for low-power mode control of the LIN module.\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scigcr2`]
module"]
#[doc(alias = "SCIGCR2")]
pub type Scigcr2 = crate::Reg<scigcr2::Scigcr2Spec>;
#[doc = "The SCIGCR2 register is used to send or compare a checksum byte during extended frames, to generate a wakeup and for low-power mode control of the LIN module."]
pub mod scigcr2;
#[doc = "SCISETINT (rw) register accessor: The SCISETINT register is used to enable the various interrupts available in the LIN module.\n\nYou can [`read`](crate::Reg::read) this register and get [`scisetint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scisetint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scisetint`]
module"]
#[doc(alias = "SCISETINT")]
pub type Scisetint = crate::Reg<scisetint::ScisetintSpec>;
#[doc = "The SCISETINT register is used to enable the various interrupts available in the LIN module."]
pub mod scisetint;
#[doc = "SCICLEARINT (rw) register accessor: The SCICLEARINT register is used to disable the enabled interrupts without accessing the SCISETINT register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciclearint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciclearint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciclearint`]
module"]
#[doc(alias = "SCICLEARINT")]
pub type Sciclearint = crate::Reg<sciclearint::SciclearintSpec>;
#[doc = "The SCICLEARINT register is used to disable the enabled interrupts without accessing the SCISETINT register."]
pub mod sciclearint;
#[doc = "SCISETINTLVL (rw) register accessor: The SCISETINTLVL register is used to map individual interrupt sources to the INT1 interrupt line.\n\nYou can [`read`](crate::Reg::read) this register and get [`scisetintlvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scisetintlvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scisetintlvl`]
module"]
#[doc(alias = "SCISETINTLVL")]
pub type Scisetintlvl = crate::Reg<scisetintlvl::ScisetintlvlSpec>;
#[doc = "The SCISETINTLVL register is used to map individual interrupt sources to the INT1 interrupt line."]
pub mod scisetintlvl;
#[doc = "SCICLEARINTLVL (rw) register accessor: The SCICLEARINTLVL register is used to map individual interrupt sources to the INT0 line.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciclearintlvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciclearintlvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciclearintlvl`]
module"]
#[doc(alias = "SCICLEARINTLVL")]
pub type Sciclearintlvl = crate::Reg<sciclearintlvl::SciclearintlvlSpec>;
#[doc = "The SCICLEARINTLVL register is used to map individual interrupt sources to the INT0 line."]
pub mod sciclearintlvl;
#[doc = "SCIFLR (rw) register accessor: The SCIFLR register indicates the current status of the various interrupt sources of the LIN module.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciflr`]
module"]
#[doc(alias = "SCIFLR")]
pub type Sciflr = crate::Reg<sciflr::SciflrSpec>;
#[doc = "The SCIFLR register indicates the current status of the various interrupt sources of the LIN module."]
pub mod sciflr;
#[doc = "SCIINTVECT0 (rw) register accessor: The SCIINTVECT0 register indicates the offset for the INT0 interrupt line.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciintvect0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciintvect0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciintvect0`]
module"]
#[doc(alias = "SCIINTVECT0")]
pub type Sciintvect0 = crate::Reg<sciintvect0::Sciintvect0Spec>;
#[doc = "The SCIINTVECT0 register indicates the offset for the INT0 interrupt line."]
pub mod sciintvect0;
#[doc = "SCIINTVECT1 (rw) register accessor: The SCIINTVECT1 register indicates the offset for the INT1 interrupt line.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciintvect1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciintvect1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciintvect1`]
module"]
#[doc(alias = "SCIINTVECT1")]
pub type Sciintvect1 = crate::Reg<sciintvect1::Sciintvect1Spec>;
#[doc = "The SCIINTVECT1 register indicates the offset for the INT1 interrupt line."]
pub mod sciintvect1;
#[doc = "SCIFORMAT (rw) register accessor: The SCIFORMAT register is used to set up the character and frame lengths.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciformat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciformat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sciformat`]
module"]
#[doc(alias = "SCIFORMAT")]
pub type Sciformat = crate::Reg<sciformat::SciformatSpec>;
#[doc = "The SCIFORMAT register is used to set up the character and frame lengths."]
pub mod sciformat;
#[doc = "BRSR (rw) register accessor: The BRSR register is used to configure the baud rate of the LIN module.\n\nYou can [`read`](crate::Reg::read) this register and get [`brsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brsr`]
module"]
#[doc(alias = "BRSR")]
pub type Brsr = crate::Reg<brsr::BrsrSpec>;
#[doc = "The BRSR register is used to configure the baud rate of the LIN module."]
pub mod brsr;
#[doc = "SCIED (rw) register accessor: The SCIED register is a duplicate copy of SCIRD register that has no affect on the RXRDY flag for use with an emulator.\n\nYou can [`read`](crate::Reg::read) this register and get [`scied::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scied::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scied`]
module"]
#[doc(alias = "SCIED")]
pub type Scied = crate::Reg<scied::SciedSpec>;
#[doc = "The SCIED register is a duplicate copy of SCIRD register that has no affect on the RXRDY flag for use with an emulator."]
pub mod scied;
#[doc = "SCIRD (rw) register accessor: The SCIRD register is where received data is stored and can be read from.\n\nYou can [`read`](crate::Reg::read) this register and get [`scird::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scird::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scird`]
module"]
#[doc(alias = "SCIRD")]
pub type Scird = crate::Reg<scird::ScirdSpec>;
#[doc = "The SCIRD register is where received data is stored and can be read from."]
pub mod scird;
#[doc = "SCITD (rw) register accessor: The SCITD register is where data to be transmitted is written to by application software.\n\nYou can [`read`](crate::Reg::read) this register and get [`scitd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scitd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scitd`]
module"]
#[doc(alias = "SCITD")]
pub type Scitd = crate::Reg<scitd::ScitdSpec>;
#[doc = "The SCITD register is where data to be transmitted is written to by application software."]
pub mod scitd;
#[doc = "SCIPIO0 (rw) register accessor: The SCIPIO0 register is used to enable the LINTX and LINRX pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio0`]
module"]
#[doc(alias = "SCIPIO0")]
pub type Scipio0 = crate::Reg<scipio0::Scipio0Spec>;
#[doc = "The SCIPIO0 register is used to enable the LINTX and LINRX pins."]
pub mod scipio0;
#[doc = "SCIPIO1 (rw) register accessor: SCIPIO1\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio1`]
module"]
#[doc(alias = "SCIPIO1")]
pub type Scipio1 = crate::Reg<scipio1::Scipio1Spec>;
#[doc = "SCIPIO1"]
pub mod scipio1;
#[doc = "SCIPIO2 (rw) register accessor: The SCIPIO2 register indicates the current status of the LINTX and LINRX pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio2`]
module"]
#[doc(alias = "SCIPIO2")]
pub type Scipio2 = crate::Reg<scipio2::Scipio2Spec>;
#[doc = "The SCIPIO2 register indicates the current status of the LINTX and LINRX pins."]
pub mod scipio2;
#[doc = "SCIPIO3 (rw) register accessor: SCIPIO3\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio3`]
module"]
#[doc(alias = "SCIPIO3")]
pub type Scipio3 = crate::Reg<scipio3::Scipio3Spec>;
#[doc = "SCIPIO3"]
pub mod scipio3;
#[doc = "SCIPIO4 (rw) register accessor: SCIPIO4\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio4`]
module"]
#[doc(alias = "SCIPIO4")]
pub type Scipio4 = crate::Reg<scipio4::Scipio4Spec>;
#[doc = "SCIPIO4"]
pub mod scipio4;
#[doc = "SCIPIO5 (rw) register accessor: SCIPIO5\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio5`]
module"]
#[doc(alias = "SCIPIO5")]
pub type Scipio5 = crate::Reg<scipio5::Scipio5Spec>;
#[doc = "SCIPIO5"]
pub mod scipio5;
#[doc = "SCIPIO6 (rw) register accessor: SCIPIO6\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio6`]
module"]
#[doc(alias = "SCIPIO6")]
pub type Scipio6 = crate::Reg<scipio6::Scipio6Spec>;
#[doc = "SCIPIO6"]
pub mod scipio6;
#[doc = "SCIPIO7 (rw) register accessor: SCIPIO7\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio7`]
module"]
#[doc(alias = "SCIPIO7")]
pub type Scipio7 = crate::Reg<scipio7::Scipio7Spec>;
#[doc = "SCIPIO7"]
pub mod scipio7;
#[doc = "SCIPIO8 (rw) register accessor: SCIPIO8\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio8`]
module"]
#[doc(alias = "SCIPIO8")]
pub type Scipio8 = crate::Reg<scipio8::Scipio8Spec>;
#[doc = "SCIPIO8"]
pub mod scipio8;
#[doc = "LINCOMP (rw) register accessor: The LINCOMPARE register is used to configure the sync delimeter and sync break extension.\n\nYou can [`read`](crate::Reg::read) this register and get [`lincomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lincomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lincomp`]
module"]
#[doc(alias = "LINCOMP")]
pub type Lincomp = crate::Reg<lincomp::LincompSpec>;
#[doc = "The LINCOMPARE register is used to configure the sync delimeter and sync break extension."]
pub mod lincomp;
#[doc = "LINRD0 (rw) register accessor: The LINRD0 register contains the lower 4 bytes of the received LIN frame data.\n\nYou can [`read`](crate::Reg::read) this register and get [`linrd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linrd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linrd0`]
module"]
#[doc(alias = "LINRD0")]
pub type Linrd0 = crate::Reg<linrd0::Linrd0Spec>;
#[doc = "The LINRD0 register contains the lower 4 bytes of the received LIN frame data."]
pub mod linrd0;
#[doc = "LINRD1 (rw) register accessor: The LINRD1 regsiter contains the upper 4 bytes of the received LIN frame data.\n\nYou can [`read`](crate::Reg::read) this register and get [`linrd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linrd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linrd1`]
module"]
#[doc(alias = "LINRD1")]
pub type Linrd1 = crate::Reg<linrd1::Linrd1Spec>;
#[doc = "The LINRD1 regsiter contains the upper 4 bytes of the received LIN frame data."]
pub mod linrd1;
#[doc = "LINMASK (rw) register accessor: The LINMASK register is used to configure the masks used for filtering incoming ID messages for receive and transmit frames.\n\nYou can [`read`](crate::Reg::read) this register and get [`linmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linmask`]
module"]
#[doc(alias = "LINMASK")]
pub type Linmask = crate::Reg<linmask::LinmaskSpec>;
#[doc = "The LINMASK register is used to configure the masks used for filtering incoming ID messages for receive and transmit frames."]
pub mod linmask;
#[doc = "LINID (rw) register accessor: The LINID register contains the identification fields for LIN communication. NOTE: For software compatibility with future LIN modules, the HGEN CTRL bit must be set to 1, the RX ID MASK field must be set to FFh, and the TX ID MASK field must be set to FFh.\n\nYou can [`read`](crate::Reg::read) this register and get [`linid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linid`]
module"]
#[doc(alias = "LINID")]
pub type Linid = crate::Reg<linid::LinidSpec>;
#[doc = "The LINID register contains the identification fields for LIN communication. NOTE: For software compatibility with future LIN modules, the HGEN CTRL bit must be set to 1, the RX ID MASK field must be set to FFh, and the TX ID MASK field must be set to FFh."]
pub mod linid;
#[doc = "LINTD0 (rw) register accessor: The LINTD0 register contains the lower 4 bytes of the data to be transmitted. NOTE: TD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame.\n\nYou can [`read`](crate::Reg::read) this register and get [`lintd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lintd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lintd0`]
module"]
#[doc(alias = "LINTD0")]
pub type Lintd0 = crate::Reg<lintd0::Lintd0Spec>;
#[doc = "The LINTD0 register contains the lower 4 bytes of the data to be transmitted. NOTE: TD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame."]
pub mod lintd0;
#[doc = "LINTD1 (rw) register accessor: The LINTD1 register contains the upper 4 bytes of the data to be transmitted. NOTE: TD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame.\n\nYou can [`read`](crate::Reg::read) this register and get [`lintd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lintd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lintd1`]
module"]
#[doc(alias = "LINTD1")]
pub type Lintd1 = crate::Reg<lintd1::Lintd1Spec>;
#[doc = "The LINTD1 register contains the upper 4 bytes of the data to be transmitted. NOTE: TD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame."]
pub mod lintd1;
#[doc = "MBRSR (rw) register accessor: The MBRSR register is used to configure the expected maximum baud rate of the LIN network.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbrsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbrsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbrsr`]
module"]
#[doc(alias = "MBRSR")]
pub type Mbrsr = crate::Reg<mbrsr::MbrsrSpec>;
#[doc = "The MBRSR register is used to configure the expected maximum baud rate of the LIN network."]
pub mod mbrsr;
#[doc = "SCIPIO9 (rw) register accessor: Couldn't find this register in spec. But it's mentioned in RTL.\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scipio9`]
module"]
#[doc(alias = "SCIPIO9")]
pub type Scipio9 = crate::Reg<scipio9::Scipio9Spec>;
#[doc = "Couldn't find this register in spec. But it's mentioned in RTL."]
pub mod scipio9;
#[doc = "Reserved (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`]
module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = "Reserved"]
pub mod reserved;
#[doc = "Reserved1 (rw) register accessor: Reserved1\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved1`]
module"]
pub type Reserved1 = crate::Reg<reserved1::Reserved1Spec>;
#[doc = "Reserved1"]
pub mod reserved1;
#[doc = "Reserved2 (rw) register accessor: Reserved2\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved2`]
module"]
pub type Reserved2 = crate::Reg<reserved2::Reserved2Spec>;
#[doc = "Reserved2"]
pub mod reserved2;
#[doc = "IODFTCTRL (rw) register accessor: The IODFTCTRL register is used to emulate various error and test conditions.\n\nYou can [`read`](crate::Reg::read) this register and get [`iodftctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodftctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodftctrl`]
module"]
#[doc(alias = "IODFTCTRL")]
pub type Iodftctrl = crate::Reg<iodftctrl::IodftctrlSpec>;
#[doc = "The IODFTCTRL register is used to emulate various error and test conditions."]
pub mod iodftctrl;
