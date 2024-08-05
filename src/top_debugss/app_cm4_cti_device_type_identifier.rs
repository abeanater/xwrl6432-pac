#[doc = "Register `APP_CM4_CTI_Device_Type_Identifier` reader"]
pub type R = crate::R<AppCm4CtiDeviceTypeIdentifierSpec>;
#[doc = "Register `APP_CM4_CTI_Device_Type_Identifier` writer"]
pub type W = crate::W<AppCm4CtiDeviceTypeIdentifierSpec>;
#[doc = "Field `APP_CM4_CTI_Device_Type_Identifier` reader - "]
pub type AppCm4CtiDeviceTypeIdentifierR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_Device_Type_Identifier` writer - "]
pub type AppCm4CtiDeviceTypeIdentifierW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_device_type_identifier(&self) -> AppCm4CtiDeviceTypeIdentifierR {
        AppCm4CtiDeviceTypeIdentifierR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_device_type_identifier(
        &mut self,
    ) -> AppCm4CtiDeviceTypeIdentifierW<AppCm4CtiDeviceTypeIdentifierSpec> {
        AppCm4CtiDeviceTypeIdentifierW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_Device_Type_Identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_device_type_identifier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_device_type_identifier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiDeviceTypeIdentifierSpec;
impl crate::RegisterSpec for AppCm4CtiDeviceTypeIdentifierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_device_type_identifier::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiDeviceTypeIdentifierSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_device_type_identifier::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiDeviceTypeIdentifierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_Device_Type_Identifier to value 0"]
impl crate::Resettable for AppCm4CtiDeviceTypeIdentifierSpec {
    const RESET_VALUE: u32 = 0;
}
