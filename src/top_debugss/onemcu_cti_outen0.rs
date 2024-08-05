#[doc = "Register `ONEMCU_CTI_OUTEN0` reader"]
pub type R = crate::R<OnemcuCtiOuten0Spec>;
#[doc = "Register `ONEMCU_CTI_OUTEN0` writer"]
pub type W = crate::W<OnemcuCtiOuten0Spec>;
#[doc = "Field `ONEMCU_CTI_OUTEN0` reader - "]
pub type OnemcuCtiOuten0R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_OUTEN0` writer - "]
pub type OnemcuCtiOuten0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_outen0(&self) -> OnemcuCtiOuten0R {
        OnemcuCtiOuten0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_outen0(&mut self) -> OnemcuCtiOuten0W<OnemcuCtiOuten0Spec> {
        OnemcuCtiOuten0W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_OUTEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiOuten0Spec;
impl crate::RegisterSpec for OnemcuCtiOuten0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_outen0::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiOuten0Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_outen0::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiOuten0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_OUTEN0 to value 0"]
impl crate::Resettable for OnemcuCtiOuten0Spec {
    const RESET_VALUE: u32 = 0;
}
