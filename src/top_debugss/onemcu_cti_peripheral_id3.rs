#[doc = "Register `ONEMCU_CTI_PeripheralID3` reader"]
pub type R = crate::R<OnemcuCtiPeripheralId3Spec>;
#[doc = "Register `ONEMCU_CTI_PeripheralID3` writer"]
pub type W = crate::W<OnemcuCtiPeripheralId3Spec>;
#[doc = "Field `ONEMCU_CTI_PeripheralID3` reader - "]
pub type OnemcuCtiPeripheralId3R = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_PeripheralID3` writer - "]
pub type OnemcuCtiPeripheralId3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_peripheral_id3(&self) -> OnemcuCtiPeripheralId3R {
        OnemcuCtiPeripheralId3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_peripheral_id3(
        &mut self,
    ) -> OnemcuCtiPeripheralId3W<OnemcuCtiPeripheralId3Spec> {
        OnemcuCtiPeripheralId3W::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_PeripheralID3\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiPeripheralId3Spec;
impl crate::RegisterSpec for OnemcuCtiPeripheralId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_peripheral_id3::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiPeripheralId3Spec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_peripheral_id3::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiPeripheralId3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_PeripheralID3 to value 0"]
impl crate::Resettable for OnemcuCtiPeripheralId3Spec {
    const RESET_VALUE: u32 = 0;
}
