#[doc = "Register `ONEMCU_CTI_Device_ID` reader"]
pub type R = crate::R<OnemcuCtiDeviceIdSpec>;
#[doc = "Register `ONEMCU_CTI_Device_ID` writer"]
pub type W = crate::W<OnemcuCtiDeviceIdSpec>;
#[doc = "Field `ONEMCU_CTI_Device_ID` reader - "]
pub type OnemcuCtiDeviceIdR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_Device_ID` writer - "]
pub type OnemcuCtiDeviceIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_device_id(&self) -> OnemcuCtiDeviceIdR {
        OnemcuCtiDeviceIdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_device_id(&mut self) -> OnemcuCtiDeviceIdW<OnemcuCtiDeviceIdSpec> {
        OnemcuCtiDeviceIdW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_Device_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_device_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_device_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiDeviceIdSpec;
impl crate::RegisterSpec for OnemcuCtiDeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_device_id::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiDeviceIdSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_device_id::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiDeviceIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_Device_ID to value 0"]
impl crate::Resettable for OnemcuCtiDeviceIdSpec {
    const RESET_VALUE: u32 = 0;
}
