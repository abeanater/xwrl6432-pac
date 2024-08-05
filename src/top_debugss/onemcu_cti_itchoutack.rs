#[doc = "Register `ONEMCU_CTI_ITCHOUTACK` reader"]
pub type R = crate::R<OnemcuCtiItchoutackSpec>;
#[doc = "Register `ONEMCU_CTI_ITCHOUTACK` writer"]
pub type W = crate::W<OnemcuCtiItchoutackSpec>;
#[doc = "Field `ONEMCU_CTI_ITCHOUTACK` reader - "]
pub type OnemcuCtiItchoutackR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITCHOUTACK` writer - "]
pub type OnemcuCtiItchoutackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_itchoutack(&self) -> OnemcuCtiItchoutackR {
        OnemcuCtiItchoutackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_itchoutack(&mut self) -> OnemcuCtiItchoutackW<OnemcuCtiItchoutackSpec> {
        OnemcuCtiItchoutackW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITCHOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itchoutack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itchoutack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiItchoutackSpec;
impl crate::RegisterSpec for OnemcuCtiItchoutackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_itchoutack::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiItchoutackSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_itchoutack::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiItchoutackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITCHOUTACK to value 0"]
impl crate::Resettable for OnemcuCtiItchoutackSpec {
    const RESET_VALUE: u32 = 0;
}
