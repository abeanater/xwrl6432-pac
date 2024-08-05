#[doc = "Register `ONEMCU_CTI_PeripheralID2` reader"]
pub type R = crate::R<OnemcuCtiPeripheralId2Spec>;
#[doc = "Register `ONEMCU_CTI_PeripheralID2` writer"]
pub type W = crate::W<OnemcuCtiPeripheralId2Spec>;
#[doc = "Field `ONEMCU_CTI_PeripheralID2` reader - "]
pub type OnemcuCtiPeripheralId2R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_PeripheralID2` writer - "]
pub type OnemcuCtiPeripheralId2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_peripheral_id2(&self) -> OnemcuCtiPeripheralId2R {
        OnemcuCtiPeripheralId2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_peripheral_id2(
        &mut self,
    ) -> OnemcuCtiPeripheralId2W<OnemcuCtiPeripheralId2Spec> {
        OnemcuCtiPeripheralId2W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_PeripheralID2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiPeripheralId2Spec;
impl crate::RegisterSpec for OnemcuCtiPeripheralId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_peripheral_id2::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiPeripheralId2Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_peripheral_id2::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiPeripheralId2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_PeripheralID2 to value 0"]
impl crate::Resettable for OnemcuCtiPeripheralId2Spec {
    const RESET_VALUE: u32 = 0;
}
