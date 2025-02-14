#[doc = "Register `ONEMCU_CTI_APPPULSE` reader"]
pub type R = crate::R<OnemcuCtiApppulseSpec>;
#[doc = "Register `ONEMCU_CTI_APPPULSE` writer"]
pub type W = crate::W<OnemcuCtiApppulseSpec>;
#[doc = "Field `ONEMCU_CTI_APPPULSE` reader - "]
pub type OnemcuCtiApppulseR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_APPPULSE` writer - "]
pub type OnemcuCtiApppulseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_apppulse(&self) -> OnemcuCtiApppulseR {
        OnemcuCtiApppulseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_apppulse(&mut self) -> OnemcuCtiApppulseW<OnemcuCtiApppulseSpec> {
        OnemcuCtiApppulseW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_APPPULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_apppulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_apppulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiApppulseSpec;
impl crate::RegisterSpec for OnemcuCtiApppulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_apppulse::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiApppulseSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_apppulse::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiApppulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_APPPULSE to value 0"]
impl crate::Resettable for OnemcuCtiApppulseSpec {
    const RESET_VALUE: u32 = 0;
}
