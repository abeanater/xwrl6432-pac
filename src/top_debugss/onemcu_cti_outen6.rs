#[doc = "Register `ONEMCU_CTI_OUTEN6` reader"]
pub type R = crate::R<OnemcuCtiOuten6Spec>;
#[doc = "Register `ONEMCU_CTI_OUTEN6` writer"]
pub type W = crate::W<OnemcuCtiOuten6Spec>;
#[doc = "Field `ONEMCU_CTI_OUTEN6` reader - "]
pub type OnemcuCtiOuten6R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_OUTEN6` writer - "]
pub type OnemcuCtiOuten6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_outen6(&self) -> OnemcuCtiOuten6R {
        OnemcuCtiOuten6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_outen6(&mut self) -> OnemcuCtiOuten6W<OnemcuCtiOuten6Spec> {
        OnemcuCtiOuten6W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_OUTEN6\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiOuten6Spec;
impl crate::RegisterSpec for OnemcuCtiOuten6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_outen6::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiOuten6Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_outen6::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiOuten6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_OUTEN6 to value 0"]
impl crate::Resettable for OnemcuCtiOuten6Spec {
    const RESET_VALUE: u32 = 0;
}
