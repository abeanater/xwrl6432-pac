#[doc = "Register `FEC_CM3_CTI_Device_Type_Identifier` reader"]
pub type R = crate::R<FecCm3CtiDeviceTypeIdentifierSpec>;
#[doc = "Register `FEC_CM3_CTI_Device_Type_Identifier` writer"]
pub type W = crate::W<FecCm3CtiDeviceTypeIdentifierSpec>;
#[doc = "Field `FEC_CM3_CTI_Device_Type_Identifier` reader - "]
pub type FecCm3CtiDeviceTypeIdentifierR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_Device_Type_Identifier` writer - "]
pub type FecCm3CtiDeviceTypeIdentifierW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_device_type_identifier(&self) -> FecCm3CtiDeviceTypeIdentifierR {
        FecCm3CtiDeviceTypeIdentifierR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_device_type_identifier(
        &mut self,
    ) -> FecCm3CtiDeviceTypeIdentifierW<FecCm3CtiDeviceTypeIdentifierSpec> {
        FecCm3CtiDeviceTypeIdentifierW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_Device_Type_Identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_device_type_identifier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_device_type_identifier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiDeviceTypeIdentifierSpec;
impl crate::RegisterSpec for FecCm3CtiDeviceTypeIdentifierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_device_type_identifier::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiDeviceTypeIdentifierSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_device_type_identifier::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiDeviceTypeIdentifierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_Device_Type_Identifier to value 0"]
impl crate::Resettable for FecCm3CtiDeviceTypeIdentifierSpec {
    const RESET_VALUE: u32 = 0;
}
