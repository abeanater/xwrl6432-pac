#[doc = "Register `ONEMCU_CTI_INEN5` reader"]
pub type R = crate::R<OnemcuCtiInen5Spec>;
#[doc = "Register `ONEMCU_CTI_INEN5` writer"]
pub type W = crate::W<OnemcuCtiInen5Spec>;
#[doc = "Field `ONEMCU_CTI_INEN5` reader - "]
pub type OnemcuCtiInen5R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_INEN5` writer - "]
pub type OnemcuCtiInen5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_inen5(&self) -> OnemcuCtiInen5R {
        OnemcuCtiInen5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_inen5(&mut self) -> OnemcuCtiInen5W<OnemcuCtiInen5Spec> {
        OnemcuCtiInen5W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_INEN5\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiInen5Spec;
impl crate::RegisterSpec for OnemcuCtiInen5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_inen5::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiInen5Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_inen5::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiInen5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_INEN5 to value 0"]
impl crate::Resettable for OnemcuCtiInen5Spec {
    const RESET_VALUE: u32 = 0;
}
