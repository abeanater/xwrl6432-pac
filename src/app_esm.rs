#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    esmiepsr1: Esmiepsr1,
    esmiepcr1: Esmiepcr1,
    esmiesr1: Esmiesr1,
    esmiecr1: Esmiecr1,
    esmilsr1: Esmilsr1,
    esmilcr1: Esmilcr1,
    esmsr1: Esmsr1,
    esmsr2: Esmsr2,
    esmsr3: Esmsr3,
    esmepsr: Esmepsr,
    esmioffhr: Esmioffhr,
    esmiofflr: Esmiofflr,
    esmltcr: Esmltcr,
    esmltcpr: Esmltcpr,
    esmekr: Esmekr,
    esmssr2: Esmssr2,
    esmiepsr4: Esmiepsr4,
    esmiepcr4: Esmiepcr4,
    esmiesr4: Esmiesr4,
    esmiecr4: Esmiecr4,
    esmilsr4: Esmilsr4,
    esmilcr4: Esmilcr4,
    esmsr4: Esmsr4,
    _reserved23: [u8; 0x24],
    esmiepsr7: Esmiepsr7,
    esmiepcr7: Esmiepcr7,
    esmiesr7: Esmiesr7,
    esmiecr7: Esmiecr7,
    esmilsr7: Esmilsr7,
    esmilcr7: Esmilcr7,
    esmsr7: Esmsr7,
    _reserved30: [u8; 0x24],
    esmiepsr10: Esmiepsr10,
    esmiepcr10: Esmiepcr10,
    esmiesr10: Esmiesr10,
    esmiecr10: Esmiecr10,
    esmilsr10: Esmilsr10,
    esmilcr10: Esmilcr10,
    esmsr10: Esmsr10,
}
impl RegisterBlock {
    #[doc = "0x00 - ESM Enable ERROR Pin Action/Response Register 1"]
    #[inline(always)]
    pub const fn esmiepsr1(&self) -> &Esmiepsr1 {
        &self.esmiepsr1
    }
    #[doc = "0x04 - ESM Disable ERROR Pin Action/Response Register 1"]
    #[inline(always)]
    pub const fn esmiepcr1(&self) -> &Esmiepcr1 {
        &self.esmiepcr1
    }
    #[doc = "0x08 - ESM Interrupt Enable Set/Status Register 1"]
    #[inline(always)]
    pub const fn esmiesr1(&self) -> &Esmiesr1 {
        &self.esmiesr1
    }
    #[doc = "0x0c - ESM Interrupt Enable Clear/Status Register 1"]
    #[inline(always)]
    pub const fn esmiecr1(&self) -> &Esmiecr1 {
        &self.esmiecr1
    }
    #[doc = "0x10 - Interrupt Level Set/Status Register 1"]
    #[inline(always)]
    pub const fn esmilsr1(&self) -> &Esmilsr1 {
        &self.esmilsr1
    }
    #[doc = "0x14 - Interrupt Level Clear/Status Register 1"]
    #[inline(always)]
    pub const fn esmilcr1(&self) -> &Esmilcr1 {
        &self.esmilcr1
    }
    #[doc = "0x18 - ESM Status Register 1"]
    #[inline(always)]
    pub const fn esmsr1(&self) -> &Esmsr1 {
        &self.esmsr1
    }
    #[doc = "0x1c - ESM Status Register 2"]
    #[inline(always)]
    pub const fn esmsr2(&self) -> &Esmsr2 {
        &self.esmsr2
    }
    #[doc = "0x20 - ESM Status Register 3"]
    #[inline(always)]
    pub const fn esmsr3(&self) -> &Esmsr3 {
        &self.esmsr3
    }
    #[doc = "0x24 - ESM ERROR Pin Status Register"]
    #[inline(always)]
    pub const fn esmepsr(&self) -> &Esmepsr {
        &self.esmepsr
    }
    #[doc = "0x28 - ESM Interrupt Offset High Register"]
    #[inline(always)]
    pub const fn esmioffhr(&self) -> &Esmioffhr {
        &self.esmioffhr
    }
    #[doc = "0x2c - ESM Interrupt Offset Low Register"]
    #[inline(always)]
    pub const fn esmiofflr(&self) -> &Esmiofflr {
        &self.esmiofflr
    }
    #[doc = "0x30 - ESM Low-Time Counter Register"]
    #[inline(always)]
    pub const fn esmltcr(&self) -> &Esmltcr {
        &self.esmltcr
    }
    #[doc = "0x34 - ESM Low-Time Counter Preload Register"]
    #[inline(always)]
    pub const fn esmltcpr(&self) -> &Esmltcpr {
        &self.esmltcpr
    }
    #[doc = "0x38 - ESM Error Key Register"]
    #[inline(always)]
    pub const fn esmekr(&self) -> &Esmekr {
        &self.esmekr
    }
    #[doc = "0x3c - ESM Status Shadow Register 2"]
    #[inline(always)]
    pub const fn esmssr2(&self) -> &Esmssr2 {
        &self.esmssr2
    }
    #[doc = "0x40 - ESM Enable ERROR Pin Action/Response Register 4"]
    #[inline(always)]
    pub const fn esmiepsr4(&self) -> &Esmiepsr4 {
        &self.esmiepsr4
    }
    #[doc = "0x44 - ESM Disable ERROR Pin Action/Response Register 4"]
    #[inline(always)]
    pub const fn esmiepcr4(&self) -> &Esmiepcr4 {
        &self.esmiepcr4
    }
    #[doc = "0x48 - ESM Interrupt Enable Set/Status Register 4"]
    #[inline(always)]
    pub const fn esmiesr4(&self) -> &Esmiesr4 {
        &self.esmiesr4
    }
    #[doc = "0x4c - ESM Interrupt Enable Clear/Status Register 4"]
    #[inline(always)]
    pub const fn esmiecr4(&self) -> &Esmiecr4 {
        &self.esmiecr4
    }
    #[doc = "0x50 - Interrupt Level Set/Status Register 4"]
    #[inline(always)]
    pub const fn esmilsr4(&self) -> &Esmilsr4 {
        &self.esmilsr4
    }
    #[doc = "0x54 - Interrupt Level Clear/Status Register 4"]
    #[inline(always)]
    pub const fn esmilcr4(&self) -> &Esmilcr4 {
        &self.esmilcr4
    }
    #[doc = "0x58 - ESM Status Register 4"]
    #[inline(always)]
    pub const fn esmsr4(&self) -> &Esmsr4 {
        &self.esmsr4
    }
    #[doc = "0x80 - ESM Enable ERROR Pin Action/Response Register 7"]
    #[inline(always)]
    pub const fn esmiepsr7(&self) -> &Esmiepsr7 {
        &self.esmiepsr7
    }
    #[doc = "0x84 - ESM Disable ERROR Pin Action/Response Register 7"]
    #[inline(always)]
    pub const fn esmiepcr7(&self) -> &Esmiepcr7 {
        &self.esmiepcr7
    }
    #[doc = "0x88 - ESM Interrupt Enable Set/Status Register 7"]
    #[inline(always)]
    pub const fn esmiesr7(&self) -> &Esmiesr7 {
        &self.esmiesr7
    }
    #[doc = "0x8c - ESM Interrupt Enable Clear/Status Register 7"]
    #[inline(always)]
    pub const fn esmiecr7(&self) -> &Esmiecr7 {
        &self.esmiecr7
    }
    #[doc = "0x90 - Interrupt Level Set/Status Register 7"]
    #[inline(always)]
    pub const fn esmilsr7(&self) -> &Esmilsr7 {
        &self.esmilsr7
    }
    #[doc = "0x94 - Interrupt Level Clear/Status Register 7"]
    #[inline(always)]
    pub const fn esmilcr7(&self) -> &Esmilcr7 {
        &self.esmilcr7
    }
    #[doc = "0x98 - ESM Status Register 7"]
    #[inline(always)]
    pub const fn esmsr7(&self) -> &Esmsr7 {
        &self.esmsr7
    }
    #[doc = "0xc0 - ESM Enable ERROR Pin Action/Response Register 10"]
    #[inline(always)]
    pub const fn esmiepsr10(&self) -> &Esmiepsr10 {
        &self.esmiepsr10
    }
    #[doc = "0xc4 - ESM Disable ERROR Pin Action/Response Register 10"]
    #[inline(always)]
    pub const fn esmiepcr10(&self) -> &Esmiepcr10 {
        &self.esmiepcr10
    }
    #[doc = "0xc8 - ESM Interrupt Enable Set/Status Register 10"]
    #[inline(always)]
    pub const fn esmiesr10(&self) -> &Esmiesr10 {
        &self.esmiesr10
    }
    #[doc = "0xcc - ESM Interrupt Enable Clear/Status Register 10"]
    #[inline(always)]
    pub const fn esmiecr10(&self) -> &Esmiecr10 {
        &self.esmiecr10
    }
    #[doc = "0xd0 - Interrupt Level Set/Status Register 10"]
    #[inline(always)]
    pub const fn esmilsr10(&self) -> &Esmilsr10 {
        &self.esmilsr10
    }
    #[doc = "0xd4 - Interrupt Level Clear/Status Register 10"]
    #[inline(always)]
    pub const fn esmilcr10(&self) -> &Esmilcr10 {
        &self.esmilcr10
    }
    #[doc = "0xd8 - ESM Status Register 10"]
    #[inline(always)]
    pub const fn esmsr10(&self) -> &Esmsr10 {
        &self.esmsr10
    }
}
#[doc = "ESMIEPSR1 (rw) register accessor: ESM Enable ERROR Pin Action/Response Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiepsr1`]
module"]
#[doc(alias = "ESMIEPSR1")]
pub type Esmiepsr1 = crate::Reg<esmiepsr1::Esmiepsr1Spec>;
#[doc = "ESM Enable ERROR Pin Action/Response Register 1"]
pub mod esmiepsr1;
#[doc = "ESMIEPCR1 (rw) register accessor: ESM Disable ERROR Pin Action/Response Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiepcr1`]
module"]
#[doc(alias = "ESMIEPCR1")]
pub type Esmiepcr1 = crate::Reg<esmiepcr1::Esmiepcr1Spec>;
#[doc = "ESM Disable ERROR Pin Action/Response Register 1"]
pub mod esmiepcr1;
#[doc = "ESMIESR1 (rw) register accessor: ESM Interrupt Enable Set/Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiesr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiesr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiesr1`]
module"]
#[doc(alias = "ESMIESR1")]
pub type Esmiesr1 = crate::Reg<esmiesr1::Esmiesr1Spec>;
#[doc = "ESM Interrupt Enable Set/Status Register 1"]
pub mod esmiesr1;
#[doc = "ESMIECR1 (rw) register accessor: ESM Interrupt Enable Clear/Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiecr1`]
module"]
#[doc(alias = "ESMIECR1")]
pub type Esmiecr1 = crate::Reg<esmiecr1::Esmiecr1Spec>;
#[doc = "ESM Interrupt Enable Clear/Status Register 1"]
pub mod esmiecr1;
#[doc = "ESMILSR1 (rw) register accessor: Interrupt Level Set/Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmilsr1`]
module"]
#[doc(alias = "ESMILSR1")]
pub type Esmilsr1 = crate::Reg<esmilsr1::Esmilsr1Spec>;
#[doc = "Interrupt Level Set/Status Register 1"]
pub mod esmilsr1;
#[doc = "ESMILCR1 (rw) register accessor: Interrupt Level Clear/Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmilcr1`]
module"]
#[doc(alias = "ESMILCR1")]
pub type Esmilcr1 = crate::Reg<esmilcr1::Esmilcr1Spec>;
#[doc = "Interrupt Level Clear/Status Register 1"]
pub mod esmilcr1;
#[doc = "ESMSR1 (rw) register accessor: ESM Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmsr1`]
module"]
#[doc(alias = "ESMSR1")]
pub type Esmsr1 = crate::Reg<esmsr1::Esmsr1Spec>;
#[doc = "ESM Status Register 1"]
pub mod esmsr1;
#[doc = "ESMSR2 (rw) register accessor: ESM Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`esmsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmsr2`]
module"]
#[doc(alias = "ESMSR2")]
pub type Esmsr2 = crate::Reg<esmsr2::Esmsr2Spec>;
#[doc = "ESM Status Register 2"]
pub mod esmsr2;
#[doc = "ESMSR3 (rw) register accessor: ESM Status Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`esmsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmsr3`]
module"]
#[doc(alias = "ESMSR3")]
pub type Esmsr3 = crate::Reg<esmsr3::Esmsr3Spec>;
#[doc = "ESM Status Register 3"]
pub mod esmsr3;
#[doc = "ESMEPSR (rw) register accessor: ESM ERROR Pin Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmepsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmepsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmepsr`]
module"]
#[doc(alias = "ESMEPSR")]
pub type Esmepsr = crate::Reg<esmepsr::EsmepsrSpec>;
#[doc = "ESM ERROR Pin Status Register"]
pub mod esmepsr;
#[doc = "ESMIOFFHR (rw) register accessor: ESM Interrupt Offset High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmioffhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmioffhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmioffhr`]
module"]
#[doc(alias = "ESMIOFFHR")]
pub type Esmioffhr = crate::Reg<esmioffhr::EsmioffhrSpec>;
#[doc = "ESM Interrupt Offset High Register"]
pub mod esmioffhr;
#[doc = "ESMIOFFLR (rw) register accessor: ESM Interrupt Offset Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiofflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiofflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiofflr`]
module"]
#[doc(alias = "ESMIOFFLR")]
pub type Esmiofflr = crate::Reg<esmiofflr::EsmiofflrSpec>;
#[doc = "ESM Interrupt Offset Low Register"]
pub mod esmiofflr;
#[doc = "ESMLTCR (rw) register accessor: ESM Low-Time Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmltcr`]
module"]
#[doc(alias = "ESMLTCR")]
pub type Esmltcr = crate::Reg<esmltcr::EsmltcrSpec>;
#[doc = "ESM Low-Time Counter Register"]
pub mod esmltcr;
#[doc = "ESMLTCPR (rw) register accessor: ESM Low-Time Counter Preload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmltcpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmltcpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmltcpr`]
module"]
#[doc(alias = "ESMLTCPR")]
pub type Esmltcpr = crate::Reg<esmltcpr::EsmltcprSpec>;
#[doc = "ESM Low-Time Counter Preload Register"]
pub mod esmltcpr;
#[doc = "ESMEKR (rw) register accessor: ESM Error Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmekr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmekr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmekr`]
module"]
#[doc(alias = "ESMEKR")]
pub type Esmekr = crate::Reg<esmekr::EsmekrSpec>;
#[doc = "ESM Error Key Register"]
pub mod esmekr;
#[doc = "ESMSSR2 (rw) register accessor: ESM Status Shadow Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`esmssr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmssr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmssr2`]
module"]
#[doc(alias = "ESMSSR2")]
pub type Esmssr2 = crate::Reg<esmssr2::Esmssr2Spec>;
#[doc = "ESM Status Shadow Register 2"]
pub mod esmssr2;
#[doc = "ESMIEPSR4 (rw) register accessor: ESM Enable ERROR Pin Action/Response Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepsr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepsr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiepsr4`]
module"]
#[doc(alias = "ESMIEPSR4")]
pub type Esmiepsr4 = crate::Reg<esmiepsr4::Esmiepsr4Spec>;
#[doc = "ESM Enable ERROR Pin Action/Response Register 4"]
pub mod esmiepsr4;
#[doc = "ESMIEPCR4 (rw) register accessor: ESM Disable ERROR Pin Action/Response Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiepcr4`]
module"]
#[doc(alias = "ESMIEPCR4")]
pub type Esmiepcr4 = crate::Reg<esmiepcr4::Esmiepcr4Spec>;
#[doc = "ESM Disable ERROR Pin Action/Response Register 4"]
pub mod esmiepcr4;
#[doc = "ESMIESR4 (rw) register accessor: ESM Interrupt Enable Set/Status Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiesr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiesr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiesr4`]
module"]
#[doc(alias = "ESMIESR4")]
pub type Esmiesr4 = crate::Reg<esmiesr4::Esmiesr4Spec>;
#[doc = "ESM Interrupt Enable Set/Status Register 4"]
pub mod esmiesr4;
#[doc = "ESMIECR4 (rw) register accessor: ESM Interrupt Enable Clear/Status Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiecr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiecr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiecr4`]
module"]
#[doc(alias = "ESMIECR4")]
pub type Esmiecr4 = crate::Reg<esmiecr4::Esmiecr4Spec>;
#[doc = "ESM Interrupt Enable Clear/Status Register 4"]
pub mod esmiecr4;
#[doc = "ESMILSR4 (rw) register accessor: Interrupt Level Set/Status Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilsr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilsr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmilsr4`]
module"]
#[doc(alias = "ESMILSR4")]
pub type Esmilsr4 = crate::Reg<esmilsr4::Esmilsr4Spec>;
#[doc = "Interrupt Level Set/Status Register 4"]
pub mod esmilsr4;
#[doc = "ESMILCR4 (rw) register accessor: Interrupt Level Clear/Status Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmilcr4`]
module"]
#[doc(alias = "ESMILCR4")]
pub type Esmilcr4 = crate::Reg<esmilcr4::Esmilcr4Spec>;
#[doc = "Interrupt Level Clear/Status Register 4"]
pub mod esmilcr4;
#[doc = "ESMSR4 (rw) register accessor: ESM Status Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmsr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmsr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmsr4`]
module"]
#[doc(alias = "ESMSR4")]
pub type Esmsr4 = crate::Reg<esmsr4::Esmsr4Spec>;
#[doc = "ESM Status Register 4"]
pub mod esmsr4;
#[doc = "ESMIEPSR7 (rw) register accessor: ESM Enable ERROR Pin Action/Response Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepsr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepsr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiepsr7`]
module"]
#[doc(alias = "ESMIEPSR7")]
pub type Esmiepsr7 = crate::Reg<esmiepsr7::Esmiepsr7Spec>;
#[doc = "ESM Enable ERROR Pin Action/Response Register 7"]
pub mod esmiepsr7;
#[doc = "ESMIEPCR7 (rw) register accessor: ESM Disable ERROR Pin Action/Response Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepcr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepcr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiepcr7`]
module"]
#[doc(alias = "ESMIEPCR7")]
pub type Esmiepcr7 = crate::Reg<esmiepcr7::Esmiepcr7Spec>;
#[doc = "ESM Disable ERROR Pin Action/Response Register 7"]
pub mod esmiepcr7;
#[doc = "ESMIESR7 (rw) register accessor: ESM Interrupt Enable Set/Status Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiesr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiesr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiesr7`]
module"]
#[doc(alias = "ESMIESR7")]
pub type Esmiesr7 = crate::Reg<esmiesr7::Esmiesr7Spec>;
#[doc = "ESM Interrupt Enable Set/Status Register 7"]
pub mod esmiesr7;
#[doc = "ESMIECR7 (rw) register accessor: ESM Interrupt Enable Clear/Status Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiecr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiecr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiecr7`]
module"]
#[doc(alias = "ESMIECR7")]
pub type Esmiecr7 = crate::Reg<esmiecr7::Esmiecr7Spec>;
#[doc = "ESM Interrupt Enable Clear/Status Register 7"]
pub mod esmiecr7;
#[doc = "ESMILSR7 (rw) register accessor: Interrupt Level Set/Status Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilsr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilsr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmilsr7`]
module"]
#[doc(alias = "ESMILSR7")]
pub type Esmilsr7 = crate::Reg<esmilsr7::Esmilsr7Spec>;
#[doc = "Interrupt Level Set/Status Register 7"]
pub mod esmilsr7;
#[doc = "ESMILCR7 (rw) register accessor: Interrupt Level Clear/Status Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilcr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilcr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmilcr7`]
module"]
#[doc(alias = "ESMILCR7")]
pub type Esmilcr7 = crate::Reg<esmilcr7::Esmilcr7Spec>;
#[doc = "Interrupt Level Clear/Status Register 7"]
pub mod esmilcr7;
#[doc = "ESMSR7 (rw) register accessor: ESM Status Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`esmsr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmsr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmsr7`]
module"]
#[doc(alias = "ESMSR7")]
pub type Esmsr7 = crate::Reg<esmsr7::Esmsr7Spec>;
#[doc = "ESM Status Register 7"]
pub mod esmsr7;
#[doc = "ESMIEPSR10 (rw) register accessor: ESM Enable ERROR Pin Action/Response Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepsr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepsr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiepsr10`]
module"]
#[doc(alias = "ESMIEPSR10")]
pub type Esmiepsr10 = crate::Reg<esmiepsr10::Esmiepsr10Spec>;
#[doc = "ESM Enable ERROR Pin Action/Response Register 10"]
pub mod esmiepsr10;
#[doc = "ESMIEPCR10 (rw) register accessor: ESM Disable ERROR Pin Action/Response Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepcr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepcr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiepcr10`]
module"]
#[doc(alias = "ESMIEPCR10")]
pub type Esmiepcr10 = crate::Reg<esmiepcr10::Esmiepcr10Spec>;
#[doc = "ESM Disable ERROR Pin Action/Response Register 10"]
pub mod esmiepcr10;
#[doc = "ESMIESR10 (rw) register accessor: ESM Interrupt Enable Set/Status Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiesr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiesr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiesr10`]
module"]
#[doc(alias = "ESMIESR10")]
pub type Esmiesr10 = crate::Reg<esmiesr10::Esmiesr10Spec>;
#[doc = "ESM Interrupt Enable Set/Status Register 10"]
pub mod esmiesr10;
#[doc = "ESMIECR10 (rw) register accessor: ESM Interrupt Enable Clear/Status Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiecr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiecr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmiecr10`]
module"]
#[doc(alias = "ESMIECR10")]
pub type Esmiecr10 = crate::Reg<esmiecr10::Esmiecr10Spec>;
#[doc = "ESM Interrupt Enable Clear/Status Register 10"]
pub mod esmiecr10;
#[doc = "ESMILSR10 (rw) register accessor: Interrupt Level Set/Status Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilsr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilsr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmilsr10`]
module"]
#[doc(alias = "ESMILSR10")]
pub type Esmilsr10 = crate::Reg<esmilsr10::Esmilsr10Spec>;
#[doc = "Interrupt Level Set/Status Register 10"]
pub mod esmilsr10;
#[doc = "ESMILCR10 (rw) register accessor: Interrupt Level Clear/Status Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilcr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilcr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmilcr10`]
module"]
#[doc(alias = "ESMILCR10")]
pub type Esmilcr10 = crate::Reg<esmilcr10::Esmilcr10Spec>;
#[doc = "Interrupt Level Clear/Status Register 10"]
pub mod esmilcr10;
#[doc = "ESMSR10 (rw) register accessor: ESM Status Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmsr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmsr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esmsr10`]
module"]
#[doc(alias = "ESMSR10")]
pub type Esmsr10 = crate::Reg<esmsr10::Esmsr10Spec>;
#[doc = "ESM Status Register 10"]
pub mod esmsr10;
