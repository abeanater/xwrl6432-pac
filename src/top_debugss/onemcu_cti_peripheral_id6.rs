#[doc = "Register `ONEMCU_CTI_PeripheralID6` reader"]
pub type R = crate::R<OnemcuCtiPeripheralId6Spec>;
#[doc = "Register `ONEMCU_CTI_PeripheralID6` writer"]
pub type W = crate::W<OnemcuCtiPeripheralId6Spec>;
#[doc = "Field `ONEMCU_CTI_PeripheralID6` reader - "]
pub type OnemcuCtiPeripheralId6R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_PeripheralID6` writer - "]
pub type OnemcuCtiPeripheralId6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_peripheral_id6(&self) -> OnemcuCtiPeripheralId6R {
        OnemcuCtiPeripheralId6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_peripheral_id6(
        &mut self,
    ) -> OnemcuCtiPeripheralId6W<OnemcuCtiPeripheralId6Spec> {
        OnemcuCtiPeripheralId6W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_PeripheralID6\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiPeripheralId6Spec;
impl crate::RegisterSpec for OnemcuCtiPeripheralId6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_peripheral_id6::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiPeripheralId6Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_peripheral_id6::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiPeripheralId6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_PeripheralID6 to value 0"]
impl crate::Resettable for OnemcuCtiPeripheralId6Spec {
    const RESET_VALUE: u32 = 0;
}
