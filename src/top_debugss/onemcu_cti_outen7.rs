#[doc = "Register `ONEMCU_CTI_OUTEN7` reader"]
pub type R = crate::R<OnemcuCtiOuten7Spec>;
#[doc = "Register `ONEMCU_CTI_OUTEN7` writer"]
pub type W = crate::W<OnemcuCtiOuten7Spec>;
#[doc = "Field `ONEMCU_CTI_OUTEN7` reader - "]
pub type OnemcuCtiOuten7R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_OUTEN7` writer - "]
pub type OnemcuCtiOuten7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_outen7(&self) -> OnemcuCtiOuten7R {
        OnemcuCtiOuten7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_outen7(&mut self) -> OnemcuCtiOuten7W<OnemcuCtiOuten7Spec> {
        OnemcuCtiOuten7W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_OUTEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiOuten7Spec;
impl crate::RegisterSpec for OnemcuCtiOuten7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_outen7::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiOuten7Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_outen7::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiOuten7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_OUTEN7 to value 0"]
impl crate::Resettable for OnemcuCtiOuten7Spec {
    const RESET_VALUE: u32 = 0;
}
