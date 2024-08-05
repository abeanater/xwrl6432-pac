#[doc = "Register `AETSTAT` reader"]
pub type R = crate::R<AetstatSpec>;
#[doc = "Register `AETSTAT` writer"]
pub type W = crate::W<AetstatSpec>;
#[doc = "Field `STAT` reader - 0:0\\]
AET Status: AETSTAT = 0 : tpcc_aet is currently low. AETSTAT = 1 : tpcc_aet is currently high."]
pub type StatR = crate::BitReader;
#[doc = "Field `STAT` writer - 0:0\\]
AET Status: AETSTAT = 0 : tpcc_aet is currently low. AETSTAT = 1 : tpcc_aet is currently high."]
pub type StatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES67` reader - 31:1\\]
RESERVE FIELD"]
pub type Res67R = crate::FieldReader<u32>;
#[doc = "Field `RES67` writer - 31:1\\]
RESERVE FIELD"]
pub type Res67W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AET Status: AETSTAT = 0 : tpcc_aet is currently low. AETSTAT = 1 : tpcc_aet is currently high."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res67(&self) -> Res67R {
        Res67R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
AET Status: AETSTAT = 0 : tpcc_aet is currently low. AETSTAT = 1 : tpcc_aet is currently high."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<AetstatSpec> {
        StatW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res67(&mut self) -> Res67W<AetstatSpec> {
        Res67W::new(self, 1)
    }
}
#[doc = "Advanced Event Trigger Stat\n\nYou can [`read`](crate::Reg::read) this register and get [`aetstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aetstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AetstatSpec;
impl crate::RegisterSpec for AetstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aetstat::R`](R) reader structure"]
impl crate::Readable for AetstatSpec {}
#[doc = "`write(|w| ..)` method takes [`aetstat::W`](W) writer structure"]
impl crate::Writable for AetstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AETSTAT to value 0"]
impl crate::Resettable for AetstatSpec {
    const RESET_VALUE: u32 = 0;
}
