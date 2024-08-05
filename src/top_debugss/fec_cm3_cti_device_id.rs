#[doc = "Register `FEC_CM3_CTI_Device_ID` reader"]
pub type R = crate::R<FecCm3CtiDeviceIdSpec>;
#[doc = "Register `FEC_CM3_CTI_Device_ID` writer"]
pub type W = crate::W<FecCm3CtiDeviceIdSpec>;
#[doc = "Field `FEC_CM3_CTI_Device_ID` reader - "]
pub type FecCm3CtiDeviceIdR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_Device_ID` writer - "]
pub type FecCm3CtiDeviceIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_device_id(&self) -> FecCm3CtiDeviceIdR {
        FecCm3CtiDeviceIdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_device_id(&mut self) -> FecCm3CtiDeviceIdW<FecCm3CtiDeviceIdSpec> {
        FecCm3CtiDeviceIdW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_Device_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_device_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_device_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiDeviceIdSpec;
impl crate::RegisterSpec for FecCm3CtiDeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_device_id::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiDeviceIdSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_device_id::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiDeviceIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_Device_ID to value 0"]
impl crate::Resettable for FecCm3CtiDeviceIdSpec {
    const RESET_VALUE: u32 = 0;
}
