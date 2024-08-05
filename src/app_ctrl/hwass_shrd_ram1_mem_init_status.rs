#[doc = "Register `HWASS_SHRD_RAM1_MEM_INIT_STATUS` reader"]
pub type R = crate::R<HwassShrdRam1MemInitStatusSpec>;
#[doc = "Register `HWASS_SHRD_RAM1_MEM_INIT_STATUS` writer"]
pub type W = crate::W<HwassShrdRam1MemInitStatusSpec>;
#[doc = "Field `mem_status` reader - 0:0\\]
1'b0: No initialization is happening for HWASS Shared RAM parition1 1'b1: Initialization is in progress for HWASS Shared RAM parition1"]
pub type MemStatusR = crate::BitReader;
#[doc = "Field `mem_status` writer - 0:0\\]
1'b0: No initialization is happening for HWASS Shared RAM parition1 1'b1: Initialization is in progress for HWASS Shared RAM parition1"]
pub type MemStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for HWASS Shared RAM parition1 1'b1: Initialization is in progress for HWASS Shared RAM parition1"]
    #[inline(always)]
    pub fn mem_status(&self) -> MemStatusR {
        MemStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for HWASS Shared RAM parition1 1'b1: Initialization is in progress for HWASS Shared RAM parition1"]
    #[inline(always)]
    #[must_use]
    pub fn mem_status(&mut self) -> MemStatusW<HwassShrdRam1MemInitStatusSpec> {
        MemStatusW::new(self, 0)
    }
}
#[doc = "HWASS_SHRD_RAM1_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram1_mem_init_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram1_mem_init_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassShrdRam1MemInitStatusSpec;
impl crate::RegisterSpec for HwassShrdRam1MemInitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_shrd_ram1_mem_init_status::R`](R) reader structure"]
impl crate::Readable for HwassShrdRam1MemInitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_shrd_ram1_mem_init_status::W`](W) writer structure"]
impl crate::Writable for HwassShrdRam1MemInitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_SHRD_RAM1_MEM_INIT_STATUS to value 0"]
impl crate::Resettable for HwassShrdRam1MemInitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
