#[doc = "Register `HWASS_SHRD_RAM_MEM_STATUS` reader"]
pub type R = crate::R<HwassShrdRamMemStatusSpec>;
#[doc = "Register `HWASS_SHRD_RAM_MEM_STATUS` writer"]
pub type W = crate::W<HwassShrdRamMemStatusSpec>;
#[doc = "Field `mem_init_status` reader - 0:0\\]
1'b0: No initialization is happening for HWASS 160kb shared memory bank 1'b1: Initialization is in progress for HWASS 160kb shared memory bank"]
pub type MemInitStatusR = crate::BitReader;
#[doc = "Field `mem_init_status` writer - 0:0\\]
1'b0: No initialization is happening for HWASS 160kb shared memory bank 1'b1: Initialization is in progress for HWASS 160kb shared memory bank"]
pub type MemInitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for HWASS 160kb shared memory bank 1'b1: Initialization is in progress for HWASS 160kb shared memory bank"]
    #[inline(always)]
    pub fn mem_init_status(&self) -> MemInitStatusR {
        MemInitStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for HWASS 160kb shared memory bank 1'b1: Initialization is in progress for HWASS 160kb shared memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn mem_init_status(&mut self) -> MemInitStatusW<HwassShrdRamMemStatusSpec> {
        MemInitStatusW::new(self, 0)
    }
}
#[doc = "HWASS_SHRD_RAM_MEM_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_mem_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_mem_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassShrdRamMemStatusSpec;
impl crate::RegisterSpec for HwassShrdRamMemStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_shrd_ram_mem_status::R`](R) reader structure"]
impl crate::Readable for HwassShrdRamMemStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_shrd_ram_mem_status::W`](W) writer structure"]
impl crate::Writable for HwassShrdRamMemStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_SHRD_RAM_MEM_STATUS to value 0"]
impl crate::Resettable for HwassShrdRamMemStatusSpec {
    const RESET_VALUE: u32 = 0;
}
