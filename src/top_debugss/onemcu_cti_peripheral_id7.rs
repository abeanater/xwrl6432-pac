#[doc = "Register `ONEMCU_CTI_PeripheralID7` reader"]
pub type R = crate::R<OnemcuCtiPeripheralId7Spec>;
#[doc = "Register `ONEMCU_CTI_PeripheralID7` writer"]
pub type W = crate::W<OnemcuCtiPeripheralId7Spec>;
#[doc = "Field `ONEMCU_CTI_PeripheralID7` reader - "]
pub type OnemcuCtiPeripheralId7R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_PeripheralID7` writer - "]
pub type OnemcuCtiPeripheralId7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_peripheral_id7(&self) -> OnemcuCtiPeripheralId7R {
        OnemcuCtiPeripheralId7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_peripheral_id7(
        &mut self,
    ) -> OnemcuCtiPeripheralId7W<OnemcuCtiPeripheralId7Spec> {
        OnemcuCtiPeripheralId7W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_PeripheralID7\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiPeripheralId7Spec;
impl crate::RegisterSpec for OnemcuCtiPeripheralId7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_peripheral_id7::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiPeripheralId7Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_peripheral_id7::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiPeripheralId7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_PeripheralID7 to value 0"]
impl crate::Resettable for OnemcuCtiPeripheralId7Spec {
    const RESET_VALUE: u32 = 0;
}
