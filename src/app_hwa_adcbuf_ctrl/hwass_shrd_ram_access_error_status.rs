#[doc = "Register `HWASS_SHRD_RAM_ACCESS_ERROR_STATUS` reader"]
pub type R = crate::R<HwassShrdRamAccessErrorStatusSpec>;
#[doc = "Register `HWASS_SHRD_RAM_ACCESS_ERROR_STATUS` writer"]
pub type W = crate::W<HwassShrdRamAccessErrorStatusSpec>;
#[doc = "Field `shmem_access_error_status` reader - 0:0\\]
This field will be high whenever the invalid address of shared memory is accessed and the interrupt is not masked."]
pub type ShmemAccessErrorStatusR = crate::BitReader;
#[doc = "Field `shmem_access_error_status` writer - 0:0\\]
This field will be high whenever the invalid address of shared memory is accessed and the interrupt is not masked."]
pub type ShmemAccessErrorStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field will be high whenever the invalid address of shared memory is accessed and the interrupt is not masked."]
    #[inline(always)]
    pub fn shmem_access_error_status(&self) -> ShmemAccessErrorStatusR {
        ShmemAccessErrorStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field will be high whenever the invalid address of shared memory is accessed and the interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn shmem_access_error_status(
        &mut self,
    ) -> ShmemAccessErrorStatusW<HwassShrdRamAccessErrorStatusSpec> {
        ShmemAccessErrorStatusW::new(self, 0)
    }
}
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_access_error_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_access_error_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassShrdRamAccessErrorStatusSpec;
impl crate::RegisterSpec for HwassShrdRamAccessErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_shrd_ram_access_error_status::R`](R) reader structure"]
impl crate::Readable for HwassShrdRamAccessErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_shrd_ram_access_error_status::W`](W) writer structure"]
impl crate::Writable for HwassShrdRamAccessErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_SHRD_RAM_ACCESS_ERROR_STATUS to value 0"]
impl crate::Resettable for HwassShrdRamAccessErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
