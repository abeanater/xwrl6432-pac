#[doc = "Register `ONEMCU_CTI_ITCHOUT` reader"]
pub type R = crate::R<OnemcuCtiItchoutSpec>;
#[doc = "Register `ONEMCU_CTI_ITCHOUT` writer"]
pub type W = crate::W<OnemcuCtiItchoutSpec>;
#[doc = "Field `ONEMCU_CTI_ITCHOUT` reader - "]
pub type OnemcuCtiItchoutR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITCHOUT` writer - "]
pub type OnemcuCtiItchoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_itchout(&self) -> OnemcuCtiItchoutR {
        OnemcuCtiItchoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_itchout(&mut self) -> OnemcuCtiItchoutW<OnemcuCtiItchoutSpec> {
        OnemcuCtiItchoutW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITCHOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itchout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itchout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiItchoutSpec;
impl crate::RegisterSpec for OnemcuCtiItchoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_itchout::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiItchoutSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_itchout::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiItchoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITCHOUT to value 0"]
impl crate::Resettable for OnemcuCtiItchoutSpec {
    const RESET_VALUE: u32 = 0;
}
