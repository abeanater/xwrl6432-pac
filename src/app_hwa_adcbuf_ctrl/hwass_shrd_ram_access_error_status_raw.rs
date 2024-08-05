#[doc = "Register `HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW` reader"]
pub type R = crate::R<HwassShrdRamAccessErrorStatusRawSpec>;
#[doc = "Register `HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW` writer"]
pub type W = crate::W<HwassShrdRamAccessErrorStatusRawSpec>;
#[doc = "Field `shmem_access_errror_status_raw` reader - 0:0\\]
Indicates the shared ram access error (raw status). Set irrespective of HWASS_SHRD_RAM_ACCESS_ERROR_MASK bit"]
pub type ShmemAccessErrrorStatusRawR = crate::BitReader;
#[doc = "Field `shmem_access_errror_status_raw` writer - 0:0\\]
Indicates the shared ram access error (raw status). Set irrespective of HWASS_SHRD_RAM_ACCESS_ERROR_MASK bit"]
pub type ShmemAccessErrrorStatusRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the shared ram access error (raw status). Set irrespective of HWASS_SHRD_RAM_ACCESS_ERROR_MASK bit"]
    #[inline(always)]
    pub fn shmem_access_errror_status_raw(&self) -> ShmemAccessErrrorStatusRawR {
        ShmemAccessErrrorStatusRawR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the shared ram access error (raw status). Set irrespective of HWASS_SHRD_RAM_ACCESS_ERROR_MASK bit"]
    #[inline(always)]
    #[must_use]
    pub fn shmem_access_errror_status_raw(
        &mut self,
    ) -> ShmemAccessErrrorStatusRawW<HwassShrdRamAccessErrorStatusRawSpec> {
        ShmemAccessErrrorStatusRawW::new(self, 0)
    }
}
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_access_error_status_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_access_error_status_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassShrdRamAccessErrorStatusRawSpec;
impl crate::RegisterSpec for HwassShrdRamAccessErrorStatusRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_shrd_ram_access_error_status_raw::R`](R) reader structure"]
impl crate::Readable for HwassShrdRamAccessErrorStatusRawSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_shrd_ram_access_error_status_raw::W`](W) writer structure"]
impl crate::Writable for HwassShrdRamAccessErrorStatusRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_SHRD_RAM_ACCESS_ERROR_STATUS_RAW to value 0"]
impl crate::Resettable for HwassShrdRamAccessErrorStatusRawSpec {
    const RESET_VALUE: u32 = 0;
}
