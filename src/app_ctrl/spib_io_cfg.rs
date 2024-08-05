#[doc = "Register `SPIB_IO_CFG` reader"]
pub type R = crate::R<SpibIoCfgSpec>;
#[doc = "Register `SPIB_IO_CFG` writer"]
pub type W = crate::W<SpibIoCfgSpec>;
#[doc = "Field `cs_deact` reader - 0:0\\]
RESERVED"]
pub type CsDeactR = crate::BitReader;
#[doc = "Field `cs_deact` writer - 0:0\\]
RESERVED"]
pub type CsDeactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cs_pol` reader - 8:8\\]
RESERVED"]
pub type CsPolR = crate::BitReader;
#[doc = "Field `cs_pol` writer - 8:8\\]
RESERVED"]
pub type CsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `miso_oen_by_cs` reader - 16:16\\]
RESERVED"]
pub type MisoOenByCsR = crate::BitReader;
#[doc = "Field `miso_oen_by_cs` writer - 16:16\\]
RESERVED"]
pub type MisoOenByCsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RESERVED"]
    #[inline(always)]
    pub fn cs_deact(&self) -> CsDeactR {
        CsDeactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED"]
    #[inline(always)]
    pub fn cs_pol(&self) -> CsPolR {
        CsPolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RESERVED"]
    #[inline(always)]
    pub fn miso_oen_by_cs(&self) -> MisoOenByCsR {
        MisoOenByCsR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn cs_deact(&mut self) -> CsDeactW<SpibIoCfgSpec> {
        CsDeactW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn cs_pol(&mut self) -> CsPolW<SpibIoCfgSpec> {
        CsPolW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn miso_oen_by_cs(&mut self) -> MisoOenByCsW<SpibIoCfgSpec> {
        MisoOenByCsW::new(self, 16)
    }
}
#[doc = "SPIB_IO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spib_io_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spib_io_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpibIoCfgSpec;
impl crate::RegisterSpec for SpibIoCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spib_io_cfg::R`](R) reader structure"]
impl crate::Readable for SpibIoCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`spib_io_cfg::W`](W) writer structure"]
impl crate::Writable for SpibIoCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIB_IO_CFG to value 0"]
impl crate::Resettable for SpibIoCfgSpec {
    const RESET_VALUE: u32 = 0;
}
