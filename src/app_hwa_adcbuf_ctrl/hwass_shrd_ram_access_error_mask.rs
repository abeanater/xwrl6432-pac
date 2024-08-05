#[doc = "Register `HWASS_SHRD_RAM_ACCESS_ERROR_MASK` reader"]
pub type R = crate::R<HwassShrdRamAccessErrorMaskSpec>;
#[doc = "Register `HWASS_SHRD_RAM_ACCESS_ERROR_MASK` writer"]
pub type W = crate::W<HwassShrdRamAccessErrorMaskSpec>;
#[doc = "Field `shmem_access_error_mask` reader - 0:0\\]
When 1'b1 : shared ram access error is masked. 1'b0 : shared ram access error is not masked."]
pub type ShmemAccessErrorMaskR = crate::BitReader;
#[doc = "Field `shmem_access_error_mask` writer - 0:0\\]
When 1'b1 : shared ram access error is masked. 1'b0 : shared ram access error is not masked."]
pub type ShmemAccessErrorMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When 1'b1 : shared ram access error is masked. 1'b0 : shared ram access error is not masked."]
    #[inline(always)]
    pub fn shmem_access_error_mask(&self) -> ShmemAccessErrorMaskR {
        ShmemAccessErrorMaskR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When 1'b1 : shared ram access error is masked. 1'b0 : shared ram access error is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn shmem_access_error_mask(
        &mut self,
    ) -> ShmemAccessErrorMaskW<HwassShrdRamAccessErrorMaskSpec> {
        ShmemAccessErrorMaskW::new(self, 0)
    }
}
#[doc = "HWASS_SHRD_RAM_ACCESS_ERROR_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_access_error_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_access_error_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassShrdRamAccessErrorMaskSpec;
impl crate::RegisterSpec for HwassShrdRamAccessErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_shrd_ram_access_error_mask::R`](R) reader structure"]
impl crate::Readable for HwassShrdRamAccessErrorMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_shrd_ram_access_error_mask::W`](W) writer structure"]
impl crate::Writable for HwassShrdRamAccessErrorMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_SHRD_RAM_ACCESS_ERROR_MASK to value 0"]
impl crate::Resettable for HwassShrdRamAccessErrorMaskSpec {
    const RESET_VALUE: u32 = 0;
}
