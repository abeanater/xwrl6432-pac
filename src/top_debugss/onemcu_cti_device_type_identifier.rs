#[doc = "Register `ONEMCU_CTI_Device_Type_Identifier` reader"]
pub type R = crate::R<OnemcuCtiDeviceTypeIdentifierSpec>;
#[doc = "Register `ONEMCU_CTI_Device_Type_Identifier` writer"]
pub type W = crate::W<OnemcuCtiDeviceTypeIdentifierSpec>;
#[doc = "Field `ONEMCU_CTI_Device_Type_Identifier` reader - "]
pub type OnemcuCtiDeviceTypeIdentifierR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_Device_Type_Identifier` writer - "]
pub type OnemcuCtiDeviceTypeIdentifierW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_device_type_identifier(&self) -> OnemcuCtiDeviceTypeIdentifierR {
        OnemcuCtiDeviceTypeIdentifierR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_device_type_identifier(
        &mut self,
    ) -> OnemcuCtiDeviceTypeIdentifierW<OnemcuCtiDeviceTypeIdentifierSpec> {
        OnemcuCtiDeviceTypeIdentifierW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_Device_Type_Identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_device_type_identifier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_device_type_identifier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiDeviceTypeIdentifierSpec;
impl crate::RegisterSpec for OnemcuCtiDeviceTypeIdentifierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_device_type_identifier::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiDeviceTypeIdentifierSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_device_type_identifier::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiDeviceTypeIdentifierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_Device_Type_Identifier to value 0"]
impl crate::Resettable for OnemcuCtiDeviceTypeIdentifierSpec {
    const RESET_VALUE: u32 = 0;
}
