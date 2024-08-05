#[doc = "Register `ONEMCU_CTI_PeripheralID1` reader"]
pub type R = crate::R<OnemcuCtiPeripheralId1Spec>;
#[doc = "Register `ONEMCU_CTI_PeripheralID1` writer"]
pub type W = crate::W<OnemcuCtiPeripheralId1Spec>;
#[doc = "Field `ONEMCU_CTI_PeripheralID1` reader - "]
pub type OnemcuCtiPeripheralId1R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_PeripheralID1` writer - "]
pub type OnemcuCtiPeripheralId1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_peripheral_id1(&self) -> OnemcuCtiPeripheralId1R {
        OnemcuCtiPeripheralId1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_peripheral_id1(
        &mut self,
    ) -> OnemcuCtiPeripheralId1W<OnemcuCtiPeripheralId1Spec> {
        OnemcuCtiPeripheralId1W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_PeripheralID1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiPeripheralId1Spec;
impl crate::RegisterSpec for OnemcuCtiPeripheralId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_peripheral_id1::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiPeripheralId1Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_peripheral_id1::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiPeripheralId1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_PeripheralID1 to value 0"]
impl crate::Resettable for OnemcuCtiPeripheralId1Spec {
    const RESET_VALUE: u32 = 0;
}
