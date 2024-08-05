#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    giogcr: Giogcr,
    giopwdn: Giopwdn,
    giointdet: Giointdet,
    giopol: Giopol,
    gioenaset: Gioenaset,
    gioenaclr: Gioenaclr,
    giolvlset: Giolvlset,
    giolvlclr: Giolvlclr,
    gioflg: Gioflg,
    giooffa: Giooffa,
    giooffb: Giooffb,
    gioemua: Gioemua,
    gioemub: Gioemub,
    giodira: Giodira,
    giodina: Giodina,
    giodouta: Giodouta,
    gioseta: Gioseta,
    gioclra: Gioclra,
    giopdra: Giopdra,
    giopuldisa: Giopuldisa,
    giopsla: Giopsla,
    giodirb: Giodirb,
    giodinb: Giodinb,
    giodoutb: Giodoutb,
    giosetb: Giosetb,
    gioclrb: Gioclrb,
    giopdrb: Giopdrb,
    giopuldisb: Giopuldisb,
    giopslb: Giopslb,
    giodirc: Giodirc,
    giodinc: Giodinc,
    giodoutc: Giodoutc,
    giosetc: Giosetc,
    gioclrc: Gioclrc,
    giopdrc: Giopdrc,
    giopuldisc: Giopuldisc,
    giopslc: Giopslc,
    giodird: Giodird,
    giodind: Giodind,
    giodoutd: Giodoutd,
    giosetd: Giosetd,
    gioclrd: Gioclrd,
    giopdrd: Giopdrd,
    giopuldisd: Giopuldisd,
    giopsld: Giopsld,
    giodire: Giodire,
    giodine: Giodine,
    giodoute: Giodoute,
    giosete: Giosete,
    gioclre: Gioclre,
    giopdre: Giopdre,
    giopuldise: Giopuldise,
    giopsle: Giopsle,
    giodirf: Giodirf,
    giodinf: Giodinf,
    giodoutf: Giodoutf,
    giosetf: Giosetf,
    gioclrf: Gioclrf,
    giopdrf: Giopdrf,
    giopuldisf: Giopuldisf,
    giopslf: Giopslf,
    giodirg: Giodirg,
    gioding: Gioding,
    giodoutg: Giodoutg,
    giosetg: Giosetg,
    gioclrg: Gioclrg,
    giopdrg: Giopdrg,
    giopuldisg: Giopuldisg,
    giopslg: Giopslg,
    giodirh: Giodirh,
    giodinh: Giodinh,
    giodouth: Giodouth,
    gioseth: Gioseth,
    gioclrh: Gioclrh,
    giopdrh: Giopdrh,
    giopuldish: Giopuldish,
    giopslh: Giopslh,
    giosrca: Giosrca,
    giosrcb: Giosrcb,
    giosrcc: Giosrcc,
    giosrcd: Giosrcd,
    giosrce: Giosrce,
    giosrcf: Giosrcf,
    giosrcg: Giosrcg,
    giosrch: Giosrch,
}
impl RegisterBlock {
    #[doc = "0x00 - GIO reset"]
    #[inline(always)]
    pub const fn giogcr(&self) -> &Giogcr {
        &self.giogcr
    }
    #[doc = "0x04 - GIO power down mode register"]
    #[inline(always)]
    pub const fn giopwdn(&self) -> &Giopwdn {
        &self.giopwdn
    }
    #[doc = "0x08 - Interrupt detection select for pins \\[0:1\\]
GIO\\[7:0\\]."]
    #[inline(always)]
    pub const fn giointdet(&self) -> &Giointdet {
        &self.giointdet
    }
    #[doc = "0x0c - Interrupt polarity select for pins \\[0:1\\]
GIO\\[7:0\\]."]
    #[inline(always)]
    pub const fn giopol(&self) -> &Giopol {
        &self.giopol
    }
    #[doc = "0x10 - Interrupt enable for pins \\[0:1\\]
GIO\\[7:0\\]."]
    #[inline(always)]
    pub const fn gioenaset(&self) -> &Gioenaset {
        &self.gioenaset
    }
    #[doc = "0x14 - Interrupt enable for pins \\[0:1\\]
GIO\\[7:0\\]."]
    #[inline(always)]
    pub const fn gioenaclr(&self) -> &Gioenaclr {
        &self.gioenaclr
    }
    #[doc = "0x18 - GIO high priority interrupt for pins \\[0:1\\]
GIO\\[7:0\\]."]
    #[inline(always)]
    pub const fn giolvlset(&self) -> &Giolvlset {
        &self.giolvlset
    }
    #[doc = "0x1c - GIO low priority interrupt for pins \\[0:1\\]
GIO\\[7:0\\]."]
    #[inline(always)]
    pub const fn giolvlclr(&self) -> &Giolvlclr {
        &self.giolvlclr
    }
    #[doc = "0x20 - GIO flag for pins \\[0:1\\]
GIO\\[7:0\\]."]
    #[inline(always)]
    pub const fn gioflg(&self) -> &Gioflg {
        &self.gioflg
    }
    #[doc = "0x24 - Index bits for currently pending high-priority interrupt Register A"]
    #[inline(always)]
    pub const fn giooffa(&self) -> &Giooffa {
        &self.giooffa
    }
    #[doc = "0x28 - Index bits for currently pending high-priority interrupt Register B"]
    #[inline(always)]
    pub const fn giooffb(&self) -> &Giooffb {
        &self.giooffb
    }
    #[doc = "0x2c - GIO emulation register A"]
    #[inline(always)]
    pub const fn gioemua(&self) -> &Gioemua {
        &self.gioemua
    }
    #[doc = "0x30 - GIO emulation register B"]
    #[inline(always)]
    pub const fn gioemub(&self) -> &Gioemub {
        &self.gioemub
    }
    #[doc = "0x34 - GIO data direction of pins in Port A"]
    #[inline(always)]
    pub const fn giodira(&self) -> &Giodira {
        &self.giodira
    }
    #[doc = "0x38 - GIO data input for pins in port A"]
    #[inline(always)]
    pub const fn giodina(&self) -> &Giodina {
        &self.giodina
    }
    #[doc = "0x3c - GIO data output for pins in port A"]
    #[inline(always)]
    pub const fn giodouta(&self) -> &Giodouta {
        &self.giodouta
    }
    #[doc = "0x40 - GIO data set for port A"]
    #[inline(always)]
    pub const fn gioseta(&self) -> &Gioseta {
        &self.gioseta
    }
    #[doc = "0x44 - GIO data clear for port A"]
    #[inline(always)]
    pub const fn gioclra(&self) -> &Gioclra {
        &self.gioclra
    }
    #[doc = "0x48 - GIO open drain for port A"]
    #[inline(always)]
    pub const fn giopdra(&self) -> &Giopdra {
        &self.giopdra
    }
    #[doc = "0x4c - GIO pul disable for port A"]
    #[inline(always)]
    pub const fn giopuldisa(&self) -> &Giopuldisa {
        &self.giopuldisa
    }
    #[doc = "0x50 - GIO pul select for port A"]
    #[inline(always)]
    pub const fn giopsla(&self) -> &Giopsla {
        &self.giopsla
    }
    #[doc = "0x54 - GIO data direction of pins in Port B"]
    #[inline(always)]
    pub const fn giodirb(&self) -> &Giodirb {
        &self.giodirb
    }
    #[doc = "0x58 - GIO data input for pins in port B"]
    #[inline(always)]
    pub const fn giodinb(&self) -> &Giodinb {
        &self.giodinb
    }
    #[doc = "0x5c - GIO data output for pins in port B"]
    #[inline(always)]
    pub const fn giodoutb(&self) -> &Giodoutb {
        &self.giodoutb
    }
    #[doc = "0x60 - GIO data set for port B"]
    #[inline(always)]
    pub const fn giosetb(&self) -> &Giosetb {
        &self.giosetb
    }
    #[doc = "0x64 - GIO data clear for port B"]
    #[inline(always)]
    pub const fn gioclrb(&self) -> &Gioclrb {
        &self.gioclrb
    }
    #[doc = "0x68 - GIO open drain for port B"]
    #[inline(always)]
    pub const fn giopdrb(&self) -> &Giopdrb {
        &self.giopdrb
    }
    #[doc = "0x6c - GIO pul disable for port B"]
    #[inline(always)]
    pub const fn giopuldisb(&self) -> &Giopuldisb {
        &self.giopuldisb
    }
    #[doc = "0x70 - GIO pul select for port B"]
    #[inline(always)]
    pub const fn giopslb(&self) -> &Giopslb {
        &self.giopslb
    }
    #[doc = "0x74 - GIO data direction of pins in Port C"]
    #[inline(always)]
    pub const fn giodirc(&self) -> &Giodirc {
        &self.giodirc
    }
    #[doc = "0x78 - GIO data input for pins in port C"]
    #[inline(always)]
    pub const fn giodinc(&self) -> &Giodinc {
        &self.giodinc
    }
    #[doc = "0x7c - GIO data output for pins in port C"]
    #[inline(always)]
    pub const fn giodoutc(&self) -> &Giodoutc {
        &self.giodoutc
    }
    #[doc = "0x80 - GIO data set for port C"]
    #[inline(always)]
    pub const fn giosetc(&self) -> &Giosetc {
        &self.giosetc
    }
    #[doc = "0x84 - GIO data clear for port C"]
    #[inline(always)]
    pub const fn gioclrc(&self) -> &Gioclrc {
        &self.gioclrc
    }
    #[doc = "0x88 - GIO open drain for port C"]
    #[inline(always)]
    pub const fn giopdrc(&self) -> &Giopdrc {
        &self.giopdrc
    }
    #[doc = "0x8c - GIO pul disable for port C"]
    #[inline(always)]
    pub const fn giopuldisc(&self) -> &Giopuldisc {
        &self.giopuldisc
    }
    #[doc = "0x90 - GIO pul select for port C"]
    #[inline(always)]
    pub const fn giopslc(&self) -> &Giopslc {
        &self.giopslc
    }
    #[doc = "0x94 - GIO data direction of pins in Port D"]
    #[inline(always)]
    pub const fn giodird(&self) -> &Giodird {
        &self.giodird
    }
    #[doc = "0x98 - GIO data input for pins in port D"]
    #[inline(always)]
    pub const fn giodind(&self) -> &Giodind {
        &self.giodind
    }
    #[doc = "0x9c - GIO data output for pins in port D"]
    #[inline(always)]
    pub const fn giodoutd(&self) -> &Giodoutd {
        &self.giodoutd
    }
    #[doc = "0xa0 - GIO data set for port D"]
    #[inline(always)]
    pub const fn giosetd(&self) -> &Giosetd {
        &self.giosetd
    }
    #[doc = "0xa4 - GIO data clear for port D"]
    #[inline(always)]
    pub const fn gioclrd(&self) -> &Gioclrd {
        &self.gioclrd
    }
    #[doc = "0xa8 - GIO open drain for port D"]
    #[inline(always)]
    pub const fn giopdrd(&self) -> &Giopdrd {
        &self.giopdrd
    }
    #[doc = "0xac - GIO pul disable for port D"]
    #[inline(always)]
    pub const fn giopuldisd(&self) -> &Giopuldisd {
        &self.giopuldisd
    }
    #[doc = "0xb0 - GIO pul select for port D"]
    #[inline(always)]
    pub const fn giopsld(&self) -> &Giopsld {
        &self.giopsld
    }
    #[doc = "0xb4 - GIO data direction of pins in Port E"]
    #[inline(always)]
    pub const fn giodire(&self) -> &Giodire {
        &self.giodire
    }
    #[doc = "0xb8 - GIO data input for pins in port E"]
    #[inline(always)]
    pub const fn giodine(&self) -> &Giodine {
        &self.giodine
    }
    #[doc = "0xbc - GIO data output for pins in port E"]
    #[inline(always)]
    pub const fn giodoute(&self) -> &Giodoute {
        &self.giodoute
    }
    #[doc = "0xc0 - GIO data set for port E"]
    #[inline(always)]
    pub const fn giosete(&self) -> &Giosete {
        &self.giosete
    }
    #[doc = "0xc4 - GIO data clear for port E"]
    #[inline(always)]
    pub const fn gioclre(&self) -> &Gioclre {
        &self.gioclre
    }
    #[doc = "0xc8 - GIO open drain for port E"]
    #[inline(always)]
    pub const fn giopdre(&self) -> &Giopdre {
        &self.giopdre
    }
    #[doc = "0xcc - GIO pul disable for port E"]
    #[inline(always)]
    pub const fn giopuldise(&self) -> &Giopuldise {
        &self.giopuldise
    }
    #[doc = "0xd0 - GIO pul select for port E"]
    #[inline(always)]
    pub const fn giopsle(&self) -> &Giopsle {
        &self.giopsle
    }
    #[doc = "0xd4 - GIO data direction of pins in Port F"]
    #[inline(always)]
    pub const fn giodirf(&self) -> &Giodirf {
        &self.giodirf
    }
    #[doc = "0xd8 - GIO data input for pins in Port F"]
    #[inline(always)]
    pub const fn giodinf(&self) -> &Giodinf {
        &self.giodinf
    }
    #[doc = "0xdc - GIO data output for pins in Port F"]
    #[inline(always)]
    pub const fn giodoutf(&self) -> &Giodoutf {
        &self.giodoutf
    }
    #[doc = "0xe0 - GIO data set for Port F"]
    #[inline(always)]
    pub const fn giosetf(&self) -> &Giosetf {
        &self.giosetf
    }
    #[doc = "0xe4 - GIO data clear for Port F"]
    #[inline(always)]
    pub const fn gioclrf(&self) -> &Gioclrf {
        &self.gioclrf
    }
    #[doc = "0xe8 - GIO open drain for Port F"]
    #[inline(always)]
    pub const fn giopdrf(&self) -> &Giopdrf {
        &self.giopdrf
    }
    #[doc = "0xec - GIO pul disable for port F"]
    #[inline(always)]
    pub const fn giopuldisf(&self) -> &Giopuldisf {
        &self.giopuldisf
    }
    #[doc = "0xf0 - GIO pul select for port F"]
    #[inline(always)]
    pub const fn giopslf(&self) -> &Giopslf {
        &self.giopslf
    }
    #[doc = "0xf4 - GIO data direction of pins in Port G"]
    #[inline(always)]
    pub const fn giodirg(&self) -> &Giodirg {
        &self.giodirg
    }
    #[doc = "0xf8 - GIO data input for pins in port G"]
    #[inline(always)]
    pub const fn gioding(&self) -> &Gioding {
        &self.gioding
    }
    #[doc = "0xfc - GIO data output for pins in port G"]
    #[inline(always)]
    pub const fn giodoutg(&self) -> &Giodoutg {
        &self.giodoutg
    }
    #[doc = "0x100 - GIO data set for port G"]
    #[inline(always)]
    pub const fn giosetg(&self) -> &Giosetg {
        &self.giosetg
    }
    #[doc = "0x104 - GIO data clear for port G"]
    #[inline(always)]
    pub const fn gioclrg(&self) -> &Gioclrg {
        &self.gioclrg
    }
    #[doc = "0x108 - GIO open drain for port G"]
    #[inline(always)]
    pub const fn giopdrg(&self) -> &Giopdrg {
        &self.giopdrg
    }
    #[doc = "0x10c - GIO pul disable for port G"]
    #[inline(always)]
    pub const fn giopuldisg(&self) -> &Giopuldisg {
        &self.giopuldisg
    }
    #[doc = "0x110 - GIO pul select for port G"]
    #[inline(always)]
    pub const fn giopslg(&self) -> &Giopslg {
        &self.giopslg
    }
    #[doc = "0x114 - GIO data direction of pins in Port H"]
    #[inline(always)]
    pub const fn giodirh(&self) -> &Giodirh {
        &self.giodirh
    }
    #[doc = "0x118 - GIO data input for pins in Port H"]
    #[inline(always)]
    pub const fn giodinh(&self) -> &Giodinh {
        &self.giodinh
    }
    #[doc = "0x11c - GIO data output for pins in Port H"]
    #[inline(always)]
    pub const fn giodouth(&self) -> &Giodouth {
        &self.giodouth
    }
    #[doc = "0x120 - GIO data set for Port H"]
    #[inline(always)]
    pub const fn gioseth(&self) -> &Gioseth {
        &self.gioseth
    }
    #[doc = "0x124 - GIO data clear for Port H"]
    #[inline(always)]
    pub const fn gioclrh(&self) -> &Gioclrh {
        &self.gioclrh
    }
    #[doc = "0x128 - GIO open drain for Port H"]
    #[inline(always)]
    pub const fn giopdrh(&self) -> &Giopdrh {
        &self.giopdrh
    }
    #[doc = "0x12c - GIO pul disable for port H"]
    #[inline(always)]
    pub const fn giopuldish(&self) -> &Giopuldish {
        &self.giopuldish
    }
    #[doc = "0x130 - GIO pul select for port H"]
    #[inline(always)]
    pub const fn giopslh(&self) -> &Giopslh {
        &self.giopslh
    }
    #[doc = "0x134 - GIO slew rate select for port A"]
    #[inline(always)]
    pub const fn giosrca(&self) -> &Giosrca {
        &self.giosrca
    }
    #[doc = "0x138 - GIO slew rate select for port B"]
    #[inline(always)]
    pub const fn giosrcb(&self) -> &Giosrcb {
        &self.giosrcb
    }
    #[doc = "0x13c - GIO slew rate select for port C"]
    #[inline(always)]
    pub const fn giosrcc(&self) -> &Giosrcc {
        &self.giosrcc
    }
    #[doc = "0x140 - GIO slew rate select for port D"]
    #[inline(always)]
    pub const fn giosrcd(&self) -> &Giosrcd {
        &self.giosrcd
    }
    #[doc = "0x144 - GIO slew rate select for port E"]
    #[inline(always)]
    pub const fn giosrce(&self) -> &Giosrce {
        &self.giosrce
    }
    #[doc = "0x148 - GIO slew rate select for port F"]
    #[inline(always)]
    pub const fn giosrcf(&self) -> &Giosrcf {
        &self.giosrcf
    }
    #[doc = "0x14c - GIO slew rate select for port G"]
    #[inline(always)]
    pub const fn giosrcg(&self) -> &Giosrcg {
        &self.giosrcg
    }
    #[doc = "0x150 - GIO slew rate select for port H"]
    #[inline(always)]
    pub const fn giosrch(&self) -> &Giosrch {
        &self.giosrch
    }
}
#[doc = "GIOGCR (rw) register accessor: GIO reset\n\nYou can [`read`](crate::Reg::read) this register and get [`giogcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giogcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giogcr`]
module"]
#[doc(alias = "GIOGCR")]
pub type Giogcr = crate::Reg<giogcr::GiogcrSpec>;
#[doc = "GIO reset"]
pub mod giogcr;
#[doc = "GIOPWDN (rw) register accessor: GIO power down mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`giopwdn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopwdn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopwdn`]
module"]
#[doc(alias = "GIOPWDN")]
pub type Giopwdn = crate::Reg<giopwdn::GiopwdnSpec>;
#[doc = "GIO power down mode register"]
pub mod giopwdn;
#[doc = "GIOINTDET (rw) register accessor: Interrupt detection select for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`giointdet::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giointdet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giointdet`]
module"]
#[doc(alias = "GIOINTDET")]
pub type Giointdet = crate::Reg<giointdet::GiointdetSpec>;
#[doc = "Interrupt detection select for pins \\[0:1\\]
GIO\\[7:0\\]."]
pub mod giointdet;
#[doc = "GIOPOL (rw) register accessor: Interrupt polarity select for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`giopol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopol`]
module"]
#[doc(alias = "GIOPOL")]
pub type Giopol = crate::Reg<giopol::GiopolSpec>;
#[doc = "Interrupt polarity select for pins \\[0:1\\]
GIO\\[7:0\\]."]
pub mod giopol;
#[doc = "GIOENASET (rw) register accessor: Interrupt enable for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`gioenaset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioenaset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioenaset`]
module"]
#[doc(alias = "GIOENASET")]
pub type Gioenaset = crate::Reg<gioenaset::GioenasetSpec>;
#[doc = "Interrupt enable for pins \\[0:1\\]
GIO\\[7:0\\]."]
pub mod gioenaset;
#[doc = "GIOENACLR (rw) register accessor: Interrupt enable for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`gioenaclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioenaclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioenaclr`]
module"]
#[doc(alias = "GIOENACLR")]
pub type Gioenaclr = crate::Reg<gioenaclr::GioenaclrSpec>;
#[doc = "Interrupt enable for pins \\[0:1\\]
GIO\\[7:0\\]."]
pub mod gioenaclr;
#[doc = "GIOLVLSET (rw) register accessor: GIO high priority interrupt for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`giolvlset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giolvlset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giolvlset`]
module"]
#[doc(alias = "GIOLVLSET")]
pub type Giolvlset = crate::Reg<giolvlset::GiolvlsetSpec>;
#[doc = "GIO high priority interrupt for pins \\[0:1\\]
GIO\\[7:0\\]."]
pub mod giolvlset;
#[doc = "GIOLVLCLR (rw) register accessor: GIO low priority interrupt for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`giolvlclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giolvlclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giolvlclr`]
module"]
#[doc(alias = "GIOLVLCLR")]
pub type Giolvlclr = crate::Reg<giolvlclr::GiolvlclrSpec>;
#[doc = "GIO low priority interrupt for pins \\[0:1\\]
GIO\\[7:0\\]."]
pub mod giolvlclr;
#[doc = "GIOFLG (rw) register accessor: GIO flag for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`gioflg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioflg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioflg`]
module"]
#[doc(alias = "GIOFLG")]
pub type Gioflg = crate::Reg<gioflg::GioflgSpec>;
#[doc = "GIO flag for pins \\[0:1\\]
GIO\\[7:0\\]."]
pub mod gioflg;
#[doc = "GIOOFFA (rw) register accessor: Index bits for currently pending high-priority interrupt Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`giooffa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giooffa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giooffa`]
module"]
#[doc(alias = "GIOOFFA")]
pub type Giooffa = crate::Reg<giooffa::GiooffaSpec>;
#[doc = "Index bits for currently pending high-priority interrupt Register A"]
pub mod giooffa;
#[doc = "GIOOFFB (rw) register accessor: Index bits for currently pending high-priority interrupt Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`giooffb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giooffb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giooffb`]
module"]
#[doc(alias = "GIOOFFB")]
pub type Giooffb = crate::Reg<giooffb::GiooffbSpec>;
#[doc = "Index bits for currently pending high-priority interrupt Register B"]
pub mod giooffb;
#[doc = "GIOEMUA (rw) register accessor: GIO emulation register A\n\nYou can [`read`](crate::Reg::read) this register and get [`gioemua::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioemua::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioemua`]
module"]
#[doc(alias = "GIOEMUA")]
pub type Gioemua = crate::Reg<gioemua::GioemuaSpec>;
#[doc = "GIO emulation register A"]
pub mod gioemua;
#[doc = "GIOEMUB (rw) register accessor: GIO emulation register B\n\nYou can [`read`](crate::Reg::read) this register and get [`gioemub::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioemub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioemub`]
module"]
#[doc(alias = "GIOEMUB")]
pub type Gioemub = crate::Reg<gioemub::GioemubSpec>;
#[doc = "GIO emulation register B"]
pub mod gioemub;
#[doc = "GIODIRA (rw) register accessor: GIO data direction of pins in Port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giodira::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodira::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodira`]
module"]
#[doc(alias = "GIODIRA")]
pub type Giodira = crate::Reg<giodira::GiodiraSpec>;
#[doc = "GIO data direction of pins in Port A"]
pub mod giodira;
#[doc = "GIODINA (rw) register accessor: GIO data input for pins in port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giodina::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodina::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodina`]
module"]
#[doc(alias = "GIODINA")]
pub type Giodina = crate::Reg<giodina::GiodinaSpec>;
#[doc = "GIO data input for pins in port A"]
pub mod giodina;
#[doc = "GIODOUTA (rw) register accessor: GIO data output for pins in port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giodouta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodouta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodouta`]
module"]
#[doc(alias = "GIODOUTA")]
pub type Giodouta = crate::Reg<giodouta::GiodoutaSpec>;
#[doc = "GIO data output for pins in port A"]
pub mod giodouta;
#[doc = "GIOSETA (rw) register accessor: GIO data set for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`gioseta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioseta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioseta`]
module"]
#[doc(alias = "GIOSETA")]
pub type Gioseta = crate::Reg<gioseta::GiosetaSpec>;
#[doc = "GIO data set for port A"]
pub mod gioseta;
#[doc = "GIOCLRA (rw) register accessor: GIO data clear for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioclra`]
module"]
#[doc(alias = "GIOCLRA")]
pub type Gioclra = crate::Reg<gioclra::GioclraSpec>;
#[doc = "GIO data clear for port A"]
pub mod gioclra;
#[doc = "GIOPDRA (rw) register accessor: GIO open drain for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopdra`]
module"]
#[doc(alias = "GIOPDRA")]
pub type Giopdra = crate::Reg<giopdra::GiopdraSpec>;
#[doc = "GIO open drain for port A"]
pub mod giopdra;
#[doc = "GIOPULDISA (rw) register accessor: GIO pul disable for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopuldisa`]
module"]
#[doc(alias = "GIOPULDISA")]
pub type Giopuldisa = crate::Reg<giopuldisa::GiopuldisaSpec>;
#[doc = "GIO pul disable for port A"]
pub mod giopuldisa;
#[doc = "GIOPSLA (rw) register accessor: GIO pul select for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giopsla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopsla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopsla`]
module"]
#[doc(alias = "GIOPSLA")]
pub type Giopsla = crate::Reg<giopsla::GiopslaSpec>;
#[doc = "GIO pul select for port A"]
pub mod giopsla;
#[doc = "GIODIRB (rw) register accessor: GIO data direction of pins in Port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodirb`]
module"]
#[doc(alias = "GIODIRB")]
pub type Giodirb = crate::Reg<giodirb::GiodirbSpec>;
#[doc = "GIO data direction of pins in Port B"]
pub mod giodirb;
#[doc = "GIODINB (rw) register accessor: GIO data input for pins in port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giodinb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodinb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodinb`]
module"]
#[doc(alias = "GIODINB")]
pub type Giodinb = crate::Reg<giodinb::GiodinbSpec>;
#[doc = "GIO data input for pins in port B"]
pub mod giodinb;
#[doc = "GIODOUTB (rw) register accessor: GIO data output for pins in port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodoutb`]
module"]
#[doc(alias = "GIODOUTB")]
pub type Giodoutb = crate::Reg<giodoutb::GiodoutbSpec>;
#[doc = "GIO data output for pins in port B"]
pub mod giodoutb;
#[doc = "GIOSETB (rw) register accessor: GIO data set for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosetb`]
module"]
#[doc(alias = "GIOSETB")]
pub type Giosetb = crate::Reg<giosetb::GiosetbSpec>;
#[doc = "GIO data set for port B"]
pub mod giosetb;
#[doc = "GIOCLRB (rw) register accessor: GIO data clear for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioclrb`]
module"]
#[doc(alias = "GIOCLRB")]
pub type Gioclrb = crate::Reg<gioclrb::GioclrbSpec>;
#[doc = "GIO data clear for port B"]
pub mod gioclrb;
#[doc = "GIOPDRB (rw) register accessor: GIO open drain for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopdrb`]
module"]
#[doc(alias = "GIOPDRB")]
pub type Giopdrb = crate::Reg<giopdrb::GiopdrbSpec>;
#[doc = "GIO open drain for port B"]
pub mod giopdrb;
#[doc = "GIOPULDISB (rw) register accessor: GIO pul disable for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopuldisb`]
module"]
#[doc(alias = "GIOPULDISB")]
pub type Giopuldisb = crate::Reg<giopuldisb::GiopuldisbSpec>;
#[doc = "GIO pul disable for port B"]
pub mod giopuldisb;
#[doc = "GIOPSLB (rw) register accessor: GIO pul select for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopslb`]
module"]
#[doc(alias = "GIOPSLB")]
pub type Giopslb = crate::Reg<giopslb::GiopslbSpec>;
#[doc = "GIO pul select for port B"]
pub mod giopslb;
#[doc = "GIODIRC (rw) register accessor: GIO data direction of pins in Port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodirc`]
module"]
#[doc(alias = "GIODIRC")]
pub type Giodirc = crate::Reg<giodirc::GiodircSpec>;
#[doc = "GIO data direction of pins in Port C"]
pub mod giodirc;
#[doc = "GIODINC (rw) register accessor: GIO data input for pins in port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giodinc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodinc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodinc`]
module"]
#[doc(alias = "GIODINC")]
pub type Giodinc = crate::Reg<giodinc::GiodincSpec>;
#[doc = "GIO data input for pins in port C"]
pub mod giodinc;
#[doc = "GIODOUTC (rw) register accessor: GIO data output for pins in port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodoutc`]
module"]
#[doc(alias = "GIODOUTC")]
pub type Giodoutc = crate::Reg<giodoutc::GiodoutcSpec>;
#[doc = "GIO data output for pins in port C"]
pub mod giodoutc;
#[doc = "GIOSETC (rw) register accessor: GIO data set for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosetc`]
module"]
#[doc(alias = "GIOSETC")]
pub type Giosetc = crate::Reg<giosetc::GiosetcSpec>;
#[doc = "GIO data set for port C"]
pub mod giosetc;
#[doc = "GIOCLRC (rw) register accessor: GIO data clear for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioclrc`]
module"]
#[doc(alias = "GIOCLRC")]
pub type Gioclrc = crate::Reg<gioclrc::GioclrcSpec>;
#[doc = "GIO data clear for port C"]
pub mod gioclrc;
#[doc = "GIOPDRC (rw) register accessor: GIO open drain for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopdrc`]
module"]
#[doc(alias = "GIOPDRC")]
pub type Giopdrc = crate::Reg<giopdrc::GiopdrcSpec>;
#[doc = "GIO open drain for port C"]
pub mod giopdrc;
#[doc = "GIOPULDISC (rw) register accessor: GIO pul disable for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopuldisc`]
module"]
#[doc(alias = "GIOPULDISC")]
pub type Giopuldisc = crate::Reg<giopuldisc::GiopuldiscSpec>;
#[doc = "GIO pul disable for port C"]
pub mod giopuldisc;
#[doc = "GIOPSLC (rw) register accessor: GIO pul select for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopslc`]
module"]
#[doc(alias = "GIOPSLC")]
pub type Giopslc = crate::Reg<giopslc::GiopslcSpec>;
#[doc = "GIO pul select for port C"]
pub mod giopslc;
#[doc = "GIODIRD (rw) register accessor: GIO data direction of pins in Port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giodird::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodird::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodird`]
module"]
#[doc(alias = "GIODIRD")]
pub type Giodird = crate::Reg<giodird::GiodirdSpec>;
#[doc = "GIO data direction of pins in Port D"]
pub mod giodird;
#[doc = "GIODIND (rw) register accessor: GIO data input for pins in port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giodind::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodind::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodind`]
module"]
#[doc(alias = "GIODIND")]
pub type Giodind = crate::Reg<giodind::GiodindSpec>;
#[doc = "GIO data input for pins in port D"]
pub mod giodind;
#[doc = "GIODOUTD (rw) register accessor: GIO data output for pins in port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodoutd`]
module"]
#[doc(alias = "GIODOUTD")]
pub type Giodoutd = crate::Reg<giodoutd::GiodoutdSpec>;
#[doc = "GIO data output for pins in port D"]
pub mod giodoutd;
#[doc = "GIOSETD (rw) register accessor: GIO data set for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosetd`]
module"]
#[doc(alias = "GIOSETD")]
pub type Giosetd = crate::Reg<giosetd::GiosetdSpec>;
#[doc = "GIO data set for port D"]
pub mod giosetd;
#[doc = "GIOCLRD (rw) register accessor: GIO data clear for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioclrd`]
module"]
#[doc(alias = "GIOCLRD")]
pub type Gioclrd = crate::Reg<gioclrd::GioclrdSpec>;
#[doc = "GIO data clear for port D"]
pub mod gioclrd;
#[doc = "GIOPDRD (rw) register accessor: GIO open drain for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopdrd`]
module"]
#[doc(alias = "GIOPDRD")]
pub type Giopdrd = crate::Reg<giopdrd::GiopdrdSpec>;
#[doc = "GIO open drain for port D"]
pub mod giopdrd;
#[doc = "GIOPULDISD (rw) register accessor: GIO pul disable for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopuldisd`]
module"]
#[doc(alias = "GIOPULDISD")]
pub type Giopuldisd = crate::Reg<giopuldisd::GiopuldisdSpec>;
#[doc = "GIO pul disable for port D"]
pub mod giopuldisd;
#[doc = "GIOPSLD (rw) register accessor: GIO pul select for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giopsld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopsld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopsld`]
module"]
#[doc(alias = "GIOPSLD")]
pub type Giopsld = crate::Reg<giopsld::GiopsldSpec>;
#[doc = "GIO pul select for port D"]
pub mod giopsld;
#[doc = "GIODIRE (rw) register accessor: GIO data direction of pins in Port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giodire::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodire::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodire`]
module"]
#[doc(alias = "GIODIRE")]
pub type Giodire = crate::Reg<giodire::GiodireSpec>;
#[doc = "GIO data direction of pins in Port E"]
pub mod giodire;
#[doc = "GIODINE (rw) register accessor: GIO data input for pins in port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giodine::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodine::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodine`]
module"]
#[doc(alias = "GIODINE")]
pub type Giodine = crate::Reg<giodine::GiodineSpec>;
#[doc = "GIO data input for pins in port E"]
pub mod giodine;
#[doc = "GIODOUTE (rw) register accessor: GIO data output for pins in port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodoute`]
module"]
#[doc(alias = "GIODOUTE")]
pub type Giodoute = crate::Reg<giodoute::GiodouteSpec>;
#[doc = "GIO data output for pins in port E"]
pub mod giodoute;
#[doc = "GIOSETE (rw) register accessor: GIO data set for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giosete::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosete::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosete`]
module"]
#[doc(alias = "GIOSETE")]
pub type Giosete = crate::Reg<giosete::GioseteSpec>;
#[doc = "GIO data set for port E"]
pub mod giosete;
#[doc = "GIOCLRE (rw) register accessor: GIO data clear for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioclre`]
module"]
#[doc(alias = "GIOCLRE")]
pub type Gioclre = crate::Reg<gioclre::GioclreSpec>;
#[doc = "GIO data clear for port E"]
pub mod gioclre;
#[doc = "GIOPDRE (rw) register accessor: GIO open drain for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopdre`]
module"]
#[doc(alias = "GIOPDRE")]
pub type Giopdre = crate::Reg<giopdre::GiopdreSpec>;
#[doc = "GIO open drain for port E"]
pub mod giopdre;
#[doc = "GIOPULDISE (rw) register accessor: GIO pul disable for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopuldise`]
module"]
#[doc(alias = "GIOPULDISE")]
pub type Giopuldise = crate::Reg<giopuldise::GiopuldiseSpec>;
#[doc = "GIO pul disable for port E"]
pub mod giopuldise;
#[doc = "GIOPSLE (rw) register accessor: GIO pul select for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giopsle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopsle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopsle`]
module"]
#[doc(alias = "GIOPSLE")]
pub type Giopsle = crate::Reg<giopsle::GiopsleSpec>;
#[doc = "GIO pul select for port E"]
pub mod giopsle;
#[doc = "GIODIRF (rw) register accessor: GIO data direction of pins in Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodirf`]
module"]
#[doc(alias = "GIODIRF")]
pub type Giodirf = crate::Reg<giodirf::GiodirfSpec>;
#[doc = "GIO data direction of pins in Port F"]
pub mod giodirf;
#[doc = "GIODINF (rw) register accessor: GIO data input for pins in Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giodinf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodinf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodinf`]
module"]
#[doc(alias = "GIODINF")]
pub type Giodinf = crate::Reg<giodinf::GiodinfSpec>;
#[doc = "GIO data input for pins in Port F"]
pub mod giodinf;
#[doc = "GIODOUTF (rw) register accessor: GIO data output for pins in Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodoutf`]
module"]
#[doc(alias = "GIODOUTF")]
pub type Giodoutf = crate::Reg<giodoutf::GiodoutfSpec>;
#[doc = "GIO data output for pins in Port F"]
pub mod giodoutf;
#[doc = "GIOSETF (rw) register accessor: GIO data set for Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosetf`]
module"]
#[doc(alias = "GIOSETF")]
pub type Giosetf = crate::Reg<giosetf::GiosetfSpec>;
#[doc = "GIO data set for Port F"]
pub mod giosetf;
#[doc = "GIOCLRF (rw) register accessor: GIO data clear for Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioclrf`]
module"]
#[doc(alias = "GIOCLRF")]
pub type Gioclrf = crate::Reg<gioclrf::GioclrfSpec>;
#[doc = "GIO data clear for Port F"]
pub mod gioclrf;
#[doc = "GIOPDRF (rw) register accessor: GIO open drain for Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopdrf`]
module"]
#[doc(alias = "GIOPDRF")]
pub type Giopdrf = crate::Reg<giopdrf::GiopdrfSpec>;
#[doc = "GIO open drain for Port F"]
pub mod giopdrf;
#[doc = "GIOPULDISF (rw) register accessor: GIO pul disable for port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopuldisf`]
module"]
#[doc(alias = "GIOPULDISF")]
pub type Giopuldisf = crate::Reg<giopuldisf::GiopuldisfSpec>;
#[doc = "GIO pul disable for port F"]
pub mod giopuldisf;
#[doc = "GIOPSLF (rw) register accessor: GIO pul select for port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopslf`]
module"]
#[doc(alias = "GIOPSLF")]
pub type Giopslf = crate::Reg<giopslf::GiopslfSpec>;
#[doc = "GIO pul select for port F"]
pub mod giopslf;
#[doc = "GIODIRG (rw) register accessor: GIO data direction of pins in Port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodirg`]
module"]
#[doc(alias = "GIODIRG")]
pub type Giodirg = crate::Reg<giodirg::GiodirgSpec>;
#[doc = "GIO data direction of pins in Port G"]
pub mod giodirg;
#[doc = "GIODING (rw) register accessor: GIO data input for pins in port G\n\nYou can [`read`](crate::Reg::read) this register and get [`gioding::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioding::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioding`]
module"]
#[doc(alias = "GIODING")]
pub type Gioding = crate::Reg<gioding::GiodingSpec>;
#[doc = "GIO data input for pins in port G"]
pub mod gioding;
#[doc = "GIODOUTG (rw) register accessor: GIO data output for pins in port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodoutg`]
module"]
#[doc(alias = "GIODOUTG")]
pub type Giodoutg = crate::Reg<giodoutg::GiodoutgSpec>;
#[doc = "GIO data output for pins in port G"]
pub mod giodoutg;
#[doc = "GIOSETG (rw) register accessor: GIO data set for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giosetg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosetg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosetg`]
module"]
#[doc(alias = "GIOSETG")]
pub type Giosetg = crate::Reg<giosetg::GiosetgSpec>;
#[doc = "GIO data set for port G"]
pub mod giosetg;
#[doc = "GIOCLRG (rw) register accessor: GIO data clear for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioclrg`]
module"]
#[doc(alias = "GIOCLRG")]
pub type Gioclrg = crate::Reg<gioclrg::GioclrgSpec>;
#[doc = "GIO data clear for port G"]
pub mod gioclrg;
#[doc = "GIOPDRG (rw) register accessor: GIO open drain for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopdrg`]
module"]
#[doc(alias = "GIOPDRG")]
pub type Giopdrg = crate::Reg<giopdrg::GiopdrgSpec>;
#[doc = "GIO open drain for port G"]
pub mod giopdrg;
#[doc = "GIOPULDISG (rw) register accessor: GIO pul disable for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopuldisg`]
module"]
#[doc(alias = "GIOPULDISG")]
pub type Giopuldisg = crate::Reg<giopuldisg::GiopuldisgSpec>;
#[doc = "GIO pul disable for port G"]
pub mod giopuldisg;
#[doc = "GIOPSLG (rw) register accessor: GIO pul select for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopslg`]
module"]
#[doc(alias = "GIOPSLG")]
pub type Giopslg = crate::Reg<giopslg::GiopslgSpec>;
#[doc = "GIO pul select for port G"]
pub mod giopslg;
#[doc = "GIODIRH (rw) register accessor: GIO data direction of pins in Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodirh`]
module"]
#[doc(alias = "GIODIRH")]
pub type Giodirh = crate::Reg<giodirh::GiodirhSpec>;
#[doc = "GIO data direction of pins in Port H"]
pub mod giodirh;
#[doc = "GIODINH (rw) register accessor: GIO data input for pins in Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giodinh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodinh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodinh`]
module"]
#[doc(alias = "GIODINH")]
pub type Giodinh = crate::Reg<giodinh::GiodinhSpec>;
#[doc = "GIO data input for pins in Port H"]
pub mod giodinh;
#[doc = "GIODOUTH (rw) register accessor: GIO data output for pins in Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giodouth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodouth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giodouth`]
module"]
#[doc(alias = "GIODOUTH")]
pub type Giodouth = crate::Reg<giodouth::GiodouthSpec>;
#[doc = "GIO data output for pins in Port H"]
pub mod giodouth;
#[doc = "GIOSETH (rw) register accessor: GIO data set for Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`gioseth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioseth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioseth`]
module"]
#[doc(alias = "GIOSETH")]
pub type Gioseth = crate::Reg<gioseth::GiosethSpec>;
#[doc = "GIO data set for Port H"]
pub mod gioseth;
#[doc = "GIOCLRH (rw) register accessor: GIO data clear for Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gioclrh`]
module"]
#[doc(alias = "GIOCLRH")]
pub type Gioclrh = crate::Reg<gioclrh::GioclrhSpec>;
#[doc = "GIO data clear for Port H"]
pub mod gioclrh;
#[doc = "GIOPDRH (rw) register accessor: GIO open drain for Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giopdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopdrh`]
module"]
#[doc(alias = "GIOPDRH")]
pub type Giopdrh = crate::Reg<giopdrh::GiopdrhSpec>;
#[doc = "GIO open drain for Port H"]
pub mod giopdrh;
#[doc = "GIOPULDISH (rw) register accessor: GIO pul disable for port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldish::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldish::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopuldish`]
module"]
#[doc(alias = "GIOPULDISH")]
pub type Giopuldish = crate::Reg<giopuldish::GiopuldishSpec>;
#[doc = "GIO pul disable for port H"]
pub mod giopuldish;
#[doc = "GIOPSLH (rw) register accessor: GIO pul select for port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giopslh`]
module"]
#[doc(alias = "GIOPSLH")]
pub type Giopslh = crate::Reg<giopslh::GiopslhSpec>;
#[doc = "GIO pul select for port H"]
pub mod giopslh;
#[doc = "GIOSRCA (rw) register accessor: GIO slew rate select for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosrca`]
module"]
#[doc(alias = "GIOSRCA")]
pub type Giosrca = crate::Reg<giosrca::GiosrcaSpec>;
#[doc = "GIO slew rate select for port A"]
pub mod giosrca;
#[doc = "GIOSRCB (rw) register accessor: GIO slew rate select for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrcb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrcb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosrcb`]
module"]
#[doc(alias = "GIOSRCB")]
pub type Giosrcb = crate::Reg<giosrcb::GiosrcbSpec>;
#[doc = "GIO slew rate select for port B"]
pub mod giosrcb;
#[doc = "GIOSRCC (rw) register accessor: GIO slew rate select for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosrcc`]
module"]
#[doc(alias = "GIOSRCC")]
pub type Giosrcc = crate::Reg<giosrcc::GiosrccSpec>;
#[doc = "GIO slew rate select for port C"]
pub mod giosrcc;
#[doc = "GIOSRCD (rw) register accessor: GIO slew rate select for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosrcd`]
module"]
#[doc(alias = "GIOSRCD")]
pub type Giosrcd = crate::Reg<giosrcd::GiosrcdSpec>;
#[doc = "GIO slew rate select for port D"]
pub mod giosrcd;
#[doc = "GIOSRCE (rw) register accessor: GIO slew rate select for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosrce`]
module"]
#[doc(alias = "GIOSRCE")]
pub type Giosrce = crate::Reg<giosrce::GiosrceSpec>;
#[doc = "GIO slew rate select for port E"]
pub mod giosrce;
#[doc = "GIOSRCF (rw) register accessor: GIO slew rate select for port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosrcf`]
module"]
#[doc(alias = "GIOSRCF")]
pub type Giosrcf = crate::Reg<giosrcf::GiosrcfSpec>;
#[doc = "GIO slew rate select for port F"]
pub mod giosrcf;
#[doc = "GIOSRCG (rw) register accessor: GIO slew rate select for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrcg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrcg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosrcg`]
module"]
#[doc(alias = "GIOSRCG")]
pub type Giosrcg = crate::Reg<giosrcg::GiosrcgSpec>;
#[doc = "GIO slew rate select for port G"]
pub mod giosrcg;
#[doc = "GIOSRCH (rw) register accessor: GIO slew rate select for port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giosrch`]
module"]
#[doc(alias = "GIOSRCH")]
pub type Giosrch = crate::Reg<giosrch::GiosrchSpec>;
#[doc = "GIO slew rate select for port H"]
pub mod giosrch;
