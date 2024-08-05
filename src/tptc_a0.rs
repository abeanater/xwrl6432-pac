#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    tccfg: Tccfg,
    _reserved2: [u8; 0xf8],
    tcstat: Tcstat,
    intstat: Intstat,
    inten: Inten,
    intclr: Intclr,
    intcmd: Intcmd,
    _reserved7: [u8; 0x0c],
    errstat: Errstat,
    erren: Erren,
    errclr: Errclr,
    errdet: Errdet,
    errcmd: Errcmd,
    _reserved12: [u8; 0x0c],
    rdrate: Rdrate,
    _reserved13: [u8; 0xbc],
    popt: Popt,
    psrc: Psrc,
    pcnt: Pcnt,
    pdst: Pdst,
    pbidx: Pbidx,
    pmpprxy: Pmpprxy,
    _reserved19: [u8; 0x28],
    saopt: Saopt,
    sasrc: Sasrc,
    sacnt: Sacnt,
    sadst: Sadst,
    sabidx: Sabidx,
    sampprxy: Sampprxy,
    sacntrld: Sacntrld,
    sasrcbref: Sasrcbref,
    sadstbref: Sadstbref,
    sabcnt: Sabcnt,
    _reserved29: [u8; 0x18],
    dfcntrld: Dfcntrld,
    dfsrcbref: Dfsrcbref,
    _reserved31: [u8; 0x78],
    dfopt0: Dfopt0,
    dfsrc0: Dfsrc0,
    dfacnt0: Dfacnt0,
    dfdst0: Dfdst0,
    dfbidx0: Dfbidx0,
    dfmpprxy0: Dfmpprxy0,
    dfbcnt0: Dfbcnt0,
    _reserved38: [u8; 0x24],
    dfopt1: Dfopt1,
    dfsrc1: Dfsrc1,
    dfacnt1: Dfacnt1,
    dfdst1: Dfdst1,
    dfbidx1: Dfbidx1,
    dfmpprxy1: Dfmpprxy1,
    dfbcnt1: Dfbcnt1,
}
impl RegisterBlock {
    #[doc = "0x00 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
    #[doc = "0x04 - TC Configuration Register"]
    #[inline(always)]
    pub const fn tccfg(&self) -> &Tccfg {
        &self.tccfg
    }
    #[doc = "0x100 - TC Status Register"]
    #[inline(always)]
    pub const fn tcstat(&self) -> &Tcstat {
        &self.tcstat
    }
    #[doc = "0x104 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x108 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x10c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x110 - Interrupt Command Register"]
    #[inline(always)]
    pub const fn intcmd(&self) -> &Intcmd {
        &self.intcmd
    }
    #[doc = "0x120 - Error Status Register"]
    #[inline(always)]
    pub const fn errstat(&self) -> &Errstat {
        &self.errstat
    }
    #[doc = "0x124 - Error Enable Register"]
    #[inline(always)]
    pub const fn erren(&self) -> &Erren {
        &self.erren
    }
    #[doc = "0x128 - Error Clear Register"]
    #[inline(always)]
    pub const fn errclr(&self) -> &Errclr {
        &self.errclr
    }
    #[doc = "0x12c - Error Details Register"]
    #[inline(always)]
    pub const fn errdet(&self) -> &Errdet {
        &self.errdet
    }
    #[doc = "0x130 - Error Command Register"]
    #[inline(always)]
    pub const fn errcmd(&self) -> &Errcmd {
        &self.errcmd
    }
    #[doc = "0x140 - Read Rate Register"]
    #[inline(always)]
    pub const fn rdrate(&self) -> &Rdrate {
        &self.rdrate
    }
    #[doc = "0x200 - Prog Set Options"]
    #[inline(always)]
    pub const fn popt(&self) -> &Popt {
        &self.popt
    }
    #[doc = "0x204 - Prog Set Src Address"]
    #[inline(always)]
    pub const fn psrc(&self) -> &Psrc {
        &self.psrc
    }
    #[doc = "0x208 - Prog Set Count"]
    #[inline(always)]
    pub const fn pcnt(&self) -> &Pcnt {
        &self.pcnt
    }
    #[doc = "0x20c - Prog Set Dst Address"]
    #[inline(always)]
    pub const fn pdst(&self) -> &Pdst {
        &self.pdst
    }
    #[doc = "0x210 - Prog Set B-Dim Idx"]
    #[inline(always)]
    pub const fn pbidx(&self) -> &Pbidx {
        &self.pbidx
    }
    #[doc = "0x214 - Prog Set Mem Protect Proxy"]
    #[inline(always)]
    pub const fn pmpprxy(&self) -> &Pmpprxy {
        &self.pmpprxy
    }
    #[doc = "0x240 - Src Actv Set Options"]
    #[inline(always)]
    pub const fn saopt(&self) -> &Saopt {
        &self.saopt
    }
    #[doc = "0x244 - Src Actv Set Src Address"]
    #[inline(always)]
    pub const fn sasrc(&self) -> &Sasrc {
        &self.sasrc
    }
    #[doc = "0x248 - Src Actv Set A-Count"]
    #[inline(always)]
    pub const fn sacnt(&self) -> &Sacnt {
        &self.sacnt
    }
    #[doc = "0x24c - Src Actv Set Dst Address"]
    #[inline(always)]
    pub const fn sadst(&self) -> &Sadst {
        &self.sadst
    }
    #[doc = "0x250 - Src Actv Set B-Dim Idx"]
    #[inline(always)]
    pub const fn sabidx(&self) -> &Sabidx {
        &self.sabidx
    }
    #[doc = "0x254 - Src Actv Set Mem Protect Proxy"]
    #[inline(always)]
    pub const fn sampprxy(&self) -> &Sampprxy {
        &self.sampprxy
    }
    #[doc = "0x258 - Src Actv Set Cnt Reload"]
    #[inline(always)]
    pub const fn sacntrld(&self) -> &Sacntrld {
        &self.sacntrld
    }
    #[doc = "0x25c - Src Actv Set Src Addr B-Reference"]
    #[inline(always)]
    pub const fn sasrcbref(&self) -> &Sasrcbref {
        &self.sasrcbref
    }
    #[doc = "0x260 - Src Actv Set Dst Addr B-Reference"]
    #[inline(always)]
    pub const fn sadstbref(&self) -> &Sadstbref {
        &self.sadstbref
    }
    #[doc = "0x264 - Src Actv Set B-Count"]
    #[inline(always)]
    pub const fn sabcnt(&self) -> &Sabcnt {
        &self.sabcnt
    }
    #[doc = "0x280 - Dst FIFO Set Cnt Reload"]
    #[inline(always)]
    pub const fn dfcntrld(&self) -> &Dfcntrld {
        &self.dfcntrld
    }
    #[doc = "0x284 - Dst FIFO Set Src Addr B-Reference"]
    #[inline(always)]
    pub const fn dfsrcbref(&self) -> &Dfsrcbref {
        &self.dfsrcbref
    }
    #[doc = "0x300 - Dst FIFO Set Options"]
    #[inline(always)]
    pub const fn dfopt0(&self) -> &Dfopt0 {
        &self.dfopt0
    }
    #[doc = "0x304 - Dst FIFO Set Src Address"]
    #[inline(always)]
    pub const fn dfsrc0(&self) -> &Dfsrc0 {
        &self.dfsrc0
    }
    #[doc = "0x308 - Dst FIFO Set A-Count"]
    #[inline(always)]
    pub const fn dfacnt0(&self) -> &Dfacnt0 {
        &self.dfacnt0
    }
    #[doc = "0x30c - Dst FIFO Set Dst Address"]
    #[inline(always)]
    pub const fn dfdst0(&self) -> &Dfdst0 {
        &self.dfdst0
    }
    #[doc = "0x310 - Dst FIFO Set B-Dim Idx"]
    #[inline(always)]
    pub const fn dfbidx0(&self) -> &Dfbidx0 {
        &self.dfbidx0
    }
    #[doc = "0x314 - Dst FIFO Set Mem Protect Proxy"]
    #[inline(always)]
    pub const fn dfmpprxy0(&self) -> &Dfmpprxy0 {
        &self.dfmpprxy0
    }
    #[doc = "0x318 - Dst FIFO Set B-Count"]
    #[inline(always)]
    pub const fn dfbcnt0(&self) -> &Dfbcnt0 {
        &self.dfbcnt0
    }
    #[doc = "0x340 - Dst FIFO Set Options"]
    #[inline(always)]
    pub const fn dfopt1(&self) -> &Dfopt1 {
        &self.dfopt1
    }
    #[doc = "0x344 - Dst FIFO Set Src Address"]
    #[inline(always)]
    pub const fn dfsrc1(&self) -> &Dfsrc1 {
        &self.dfsrc1
    }
    #[doc = "0x348 - Dst FIFO Set A-Count"]
    #[inline(always)]
    pub const fn dfacnt1(&self) -> &Dfacnt1 {
        &self.dfacnt1
    }
    #[doc = "0x34c - Dst FIFO Set Dst Address"]
    #[inline(always)]
    pub const fn dfdst1(&self) -> &Dfdst1 {
        &self.dfdst1
    }
    #[doc = "0x350 - Dst FIFO Set B-Dim Idx"]
    #[inline(always)]
    pub const fn dfbidx1(&self) -> &Dfbidx1 {
        &self.dfbidx1
    }
    #[doc = "0x354 - Dst FIFO Set Mem Protect Proxy"]
    #[inline(always)]
    pub const fn dfmpprxy1(&self) -> &Dfmpprxy1 {
        &self.dfmpprxy1
    }
    #[doc = "0x358 - Dst FIFO Set B-Count"]
    #[inline(always)]
    pub const fn dfbcnt1(&self) -> &Dfbcnt1 {
        &self.dfbcnt1
    }
}
#[doc = "PID (rw) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "Peripheral ID Register"]
pub mod pid;
#[doc = "TCCFG (rw) register accessor: TC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccfg`]
module"]
#[doc(alias = "TCCFG")]
pub type Tccfg = crate::Reg<tccfg::TccfgSpec>;
#[doc = "TC Configuration Register"]
pub mod tccfg;
#[doc = "TCSTAT (rw) register accessor: TC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcstat`]
module"]
#[doc(alias = "TCSTAT")]
pub type Tcstat = crate::Reg<tcstat::TcstatSpec>;
#[doc = "TC Status Register"]
pub mod tcstat;
#[doc = "INTSTAT (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt Status Register"]
pub mod intstat;
#[doc = "INTEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "INTCLR (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod intclr;
#[doc = "INTCMD (rw) register accessor: Interrupt Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcmd`]
module"]
#[doc(alias = "INTCMD")]
pub type Intcmd = crate::Reg<intcmd::IntcmdSpec>;
#[doc = "Interrupt Command Register"]
pub mod intcmd;
#[doc = "ERRSTAT (rw) register accessor: Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errstat`]
module"]
#[doc(alias = "ERRSTAT")]
pub type Errstat = crate::Reg<errstat::ErrstatSpec>;
#[doc = "Error Status Register"]
pub mod errstat;
#[doc = "ERREN (rw) register accessor: Error Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erren`]
module"]
#[doc(alias = "ERREN")]
pub type Erren = crate::Reg<erren::ErrenSpec>;
#[doc = "Error Enable Register"]
pub mod erren;
#[doc = "ERRCLR (rw) register accessor: Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errclr`]
module"]
#[doc(alias = "ERRCLR")]
pub type Errclr = crate::Reg<errclr::ErrclrSpec>;
#[doc = "Error Clear Register"]
pub mod errclr;
#[doc = "ERRDET (rw) register accessor: Error Details Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errdet::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errdet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errdet`]
module"]
#[doc(alias = "ERRDET")]
pub type Errdet = crate::Reg<errdet::ErrdetSpec>;
#[doc = "Error Details Register"]
pub mod errdet;
#[doc = "ERRCMD (rw) register accessor: Error Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errcmd`]
module"]
#[doc(alias = "ERRCMD")]
pub type Errcmd = crate::Reg<errcmd::ErrcmdSpec>;
#[doc = "Error Command Register"]
pub mod errcmd;
#[doc = "RDRATE (rw) register accessor: Read Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdrate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdrate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdrate`]
module"]
#[doc(alias = "RDRATE")]
pub type Rdrate = crate::Reg<rdrate::RdrateSpec>;
#[doc = "Read Rate Register"]
pub mod rdrate;
#[doc = "POPT (rw) register accessor: Prog Set Options\n\nYou can [`read`](crate::Reg::read) this register and get [`popt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`popt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@popt`]
module"]
#[doc(alias = "POPT")]
pub type Popt = crate::Reg<popt::PoptSpec>;
#[doc = "Prog Set Options"]
pub mod popt;
#[doc = "PSRC (rw) register accessor: Prog Set Src Address\n\nYou can [`read`](crate::Reg::read) this register and get [`psrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psrc`]
module"]
#[doc(alias = "PSRC")]
pub type Psrc = crate::Reg<psrc::PsrcSpec>;
#[doc = "Prog Set Src Address"]
pub mod psrc;
#[doc = "PCNT (rw) register accessor: Prog Set Count\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt`]
module"]
#[doc(alias = "PCNT")]
pub type Pcnt = crate::Reg<pcnt::PcntSpec>;
#[doc = "Prog Set Count"]
pub mod pcnt;
#[doc = "PDST (rw) register accessor: Prog Set Dst Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pdst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdst`]
module"]
#[doc(alias = "PDST")]
pub type Pdst = crate::Reg<pdst::PdstSpec>;
#[doc = "Prog Set Dst Address"]
pub mod pdst;
#[doc = "PBIDX (rw) register accessor: Prog Set B-Dim Idx\n\nYou can [`read`](crate::Reg::read) this register and get [`pbidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbidx`]
module"]
#[doc(alias = "PBIDX")]
pub type Pbidx = crate::Reg<pbidx::PbidxSpec>;
#[doc = "Prog Set B-Dim Idx"]
pub mod pbidx;
#[doc = "PMPPRXY (rw) register accessor: Prog Set Mem Protect Proxy\n\nYou can [`read`](crate::Reg::read) this register and get [`pmpprxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpprxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmpprxy`]
module"]
#[doc(alias = "PMPPRXY")]
pub type Pmpprxy = crate::Reg<pmpprxy::PmpprxySpec>;
#[doc = "Prog Set Mem Protect Proxy"]
pub mod pmpprxy;
#[doc = "SAOPT (rw) register accessor: Src Actv Set Options\n\nYou can [`read`](crate::Reg::read) this register and get [`saopt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saopt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saopt`]
module"]
#[doc(alias = "SAOPT")]
pub type Saopt = crate::Reg<saopt::SaoptSpec>;
#[doc = "Src Actv Set Options"]
pub mod saopt;
#[doc = "SASRC (rw) register accessor: Src Actv Set Src Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sasrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sasrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sasrc`]
module"]
#[doc(alias = "SASRC")]
pub type Sasrc = crate::Reg<sasrc::SasrcSpec>;
#[doc = "Src Actv Set Src Address"]
pub mod sasrc;
#[doc = "SACNT (rw) register accessor: Src Actv Set A-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`sacnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sacnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sacnt`]
module"]
#[doc(alias = "SACNT")]
pub type Sacnt = crate::Reg<sacnt::SacntSpec>;
#[doc = "Src Actv Set A-Count"]
pub mod sacnt;
#[doc = "SADST (rw) register accessor: Src Actv Set Dst Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sadst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sadst`]
module"]
#[doc(alias = "SADST")]
pub type Sadst = crate::Reg<sadst::SadstSpec>;
#[doc = "Src Actv Set Dst Address"]
pub mod sadst;
#[doc = "SABIDX (rw) register accessor: Src Actv Set B-Dim Idx\n\nYou can [`read`](crate::Reg::read) this register and get [`sabidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sabidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sabidx`]
module"]
#[doc(alias = "SABIDX")]
pub type Sabidx = crate::Reg<sabidx::SabidxSpec>;
#[doc = "Src Actv Set B-Dim Idx"]
pub mod sabidx;
#[doc = "SAMPPRXY (rw) register accessor: Src Actv Set Mem Protect Proxy\n\nYou can [`read`](crate::Reg::read) this register and get [`sampprxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampprxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampprxy`]
module"]
#[doc(alias = "SAMPPRXY")]
pub type Sampprxy = crate::Reg<sampprxy::SampprxySpec>;
#[doc = "Src Actv Set Mem Protect Proxy"]
pub mod sampprxy;
#[doc = "SACNTRLD (rw) register accessor: Src Actv Set Cnt Reload\n\nYou can [`read`](crate::Reg::read) this register and get [`sacntrld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sacntrld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sacntrld`]
module"]
#[doc(alias = "SACNTRLD")]
pub type Sacntrld = crate::Reg<sacntrld::SacntrldSpec>;
#[doc = "Src Actv Set Cnt Reload"]
pub mod sacntrld;
#[doc = "SASRCBREF (rw) register accessor: Src Actv Set Src Addr B-Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`sasrcbref::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sasrcbref::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sasrcbref`]
module"]
#[doc(alias = "SASRCBREF")]
pub type Sasrcbref = crate::Reg<sasrcbref::SasrcbrefSpec>;
#[doc = "Src Actv Set Src Addr B-Reference"]
pub mod sasrcbref;
#[doc = "SADSTBREF (rw) register accessor: Src Actv Set Dst Addr B-Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`sadstbref::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadstbref::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sadstbref`]
module"]
#[doc(alias = "SADSTBREF")]
pub type Sadstbref = crate::Reg<sadstbref::SadstbrefSpec>;
#[doc = "Src Actv Set Dst Addr B-Reference"]
pub mod sadstbref;
#[doc = "SABCNT (rw) register accessor: Src Actv Set B-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`sabcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sabcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sabcnt`]
module"]
#[doc(alias = "SABCNT")]
pub type Sabcnt = crate::Reg<sabcnt::SabcntSpec>;
#[doc = "Src Actv Set B-Count"]
pub mod sabcnt;
#[doc = "DFCNTRLD (rw) register accessor: Dst FIFO Set Cnt Reload\n\nYou can [`read`](crate::Reg::read) this register and get [`dfcntrld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfcntrld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfcntrld`]
module"]
#[doc(alias = "DFCNTRLD")]
pub type Dfcntrld = crate::Reg<dfcntrld::DfcntrldSpec>;
#[doc = "Dst FIFO Set Cnt Reload"]
pub mod dfcntrld;
#[doc = "DFSRCBREF (rw) register accessor: Dst FIFO Set Src Addr B-Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsrcbref::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsrcbref::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsrcbref`]
module"]
#[doc(alias = "DFSRCBREF")]
pub type Dfsrcbref = crate::Reg<dfsrcbref::DfsrcbrefSpec>;
#[doc = "Dst FIFO Set Src Addr B-Reference"]
pub mod dfsrcbref;
#[doc = "DFOPT0 (rw) register accessor: Dst FIFO Set Options\n\nYou can [`read`](crate::Reg::read) this register and get [`dfopt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfopt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfopt0`]
module"]
#[doc(alias = "DFOPT0")]
pub type Dfopt0 = crate::Reg<dfopt0::Dfopt0Spec>;
#[doc = "Dst FIFO Set Options"]
pub mod dfopt0;
#[doc = "DFSRC0 (rw) register accessor: Dst FIFO Set Src Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsrc0`]
module"]
#[doc(alias = "DFSRC0")]
pub type Dfsrc0 = crate::Reg<dfsrc0::Dfsrc0Spec>;
#[doc = "Dst FIFO Set Src Address"]
pub mod dfsrc0;
#[doc = "DFACNT0 (rw) register accessor: Dst FIFO Set A-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dfacnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfacnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfacnt0`]
module"]
#[doc(alias = "DFACNT0")]
pub type Dfacnt0 = crate::Reg<dfacnt0::Dfacnt0Spec>;
#[doc = "Dst FIFO Set A-Count"]
pub mod dfacnt0;
#[doc = "DFDST0 (rw) register accessor: Dst FIFO Set Dst Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dfdst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfdst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfdst0`]
module"]
#[doc(alias = "DFDST0")]
pub type Dfdst0 = crate::Reg<dfdst0::Dfdst0Spec>;
#[doc = "Dst FIFO Set Dst Address"]
pub mod dfdst0;
#[doc = "DFBIDX0 (rw) register accessor: Dst FIFO Set B-Dim Idx\n\nYou can [`read`](crate::Reg::read) this register and get [`dfbidx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfbidx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfbidx0`]
module"]
#[doc(alias = "DFBIDX0")]
pub type Dfbidx0 = crate::Reg<dfbidx0::Dfbidx0Spec>;
#[doc = "Dst FIFO Set B-Dim Idx"]
pub mod dfbidx0;
#[doc = "DFMPPRXY0 (rw) register accessor: Dst FIFO Set Mem Protect Proxy\n\nYou can [`read`](crate::Reg::read) this register and get [`dfmpprxy0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfmpprxy0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfmpprxy0`]
module"]
#[doc(alias = "DFMPPRXY0")]
pub type Dfmpprxy0 = crate::Reg<dfmpprxy0::Dfmpprxy0Spec>;
#[doc = "Dst FIFO Set Mem Protect Proxy"]
pub mod dfmpprxy0;
#[doc = "DFBCNT0 (rw) register accessor: Dst FIFO Set B-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dfbcnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfbcnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfbcnt0`]
module"]
#[doc(alias = "DFBCNT0")]
pub type Dfbcnt0 = crate::Reg<dfbcnt0::Dfbcnt0Spec>;
#[doc = "Dst FIFO Set B-Count"]
pub mod dfbcnt0;
#[doc = "DFOPT1 (rw) register accessor: Dst FIFO Set Options\n\nYou can [`read`](crate::Reg::read) this register and get [`dfopt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfopt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfopt1`]
module"]
#[doc(alias = "DFOPT1")]
pub type Dfopt1 = crate::Reg<dfopt1::Dfopt1Spec>;
#[doc = "Dst FIFO Set Options"]
pub mod dfopt1;
#[doc = "DFSRC1 (rw) register accessor: Dst FIFO Set Src Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsrc1`]
module"]
#[doc(alias = "DFSRC1")]
pub type Dfsrc1 = crate::Reg<dfsrc1::Dfsrc1Spec>;
#[doc = "Dst FIFO Set Src Address"]
pub mod dfsrc1;
#[doc = "DFACNT1 (rw) register accessor: Dst FIFO Set A-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dfacnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfacnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfacnt1`]
module"]
#[doc(alias = "DFACNT1")]
pub type Dfacnt1 = crate::Reg<dfacnt1::Dfacnt1Spec>;
#[doc = "Dst FIFO Set A-Count"]
pub mod dfacnt1;
#[doc = "DFDST1 (rw) register accessor: Dst FIFO Set Dst Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dfdst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfdst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfdst1`]
module"]
#[doc(alias = "DFDST1")]
pub type Dfdst1 = crate::Reg<dfdst1::Dfdst1Spec>;
#[doc = "Dst FIFO Set Dst Address"]
pub mod dfdst1;
#[doc = "DFBIDX1 (rw) register accessor: Dst FIFO Set B-Dim Idx\n\nYou can [`read`](crate::Reg::read) this register and get [`dfbidx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfbidx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfbidx1`]
module"]
#[doc(alias = "DFBIDX1")]
pub type Dfbidx1 = crate::Reg<dfbidx1::Dfbidx1Spec>;
#[doc = "Dst FIFO Set B-Dim Idx"]
pub mod dfbidx1;
#[doc = "DFMPPRXY1 (rw) register accessor: Dst FIFO Set Mem Protect Proxy\n\nYou can [`read`](crate::Reg::read) this register and get [`dfmpprxy1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfmpprxy1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfmpprxy1`]
module"]
#[doc(alias = "DFMPPRXY1")]
pub type Dfmpprxy1 = crate::Reg<dfmpprxy1::Dfmpprxy1Spec>;
#[doc = "Dst FIFO Set Mem Protect Proxy"]
pub mod dfmpprxy1;
#[doc = "DFBCNT1 (rw) register accessor: Dst FIFO Set B-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dfbcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfbcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfbcnt1`]
module"]
#[doc(alias = "DFBCNT1")]
pub type Dfbcnt1 = crate::Reg<dfbcnt1::Dfbcnt1Spec>;
#[doc = "Dst FIFO Set B-Count"]
pub mod dfbcnt1;
