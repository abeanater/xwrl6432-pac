#[doc = "Register `HWASS_SHRD_RAM0_MEM_INIT_DONE` reader"]
pub type R = crate::R<HwassShrdRam0MemInitDoneSpec>;
#[doc = "Register `HWASS_SHRD_RAM0_MEM_INIT_DONE` writer"]
pub type W = crate::W<HwassShrdRam0MemInitDoneSpec>;
#[doc = "Field `mem_init_done` reader - 0:0\\]
This field will be high once intialization of HWASS Shared RAM parition0 is finished. Writing '1' would clear the bit"]
pub type MemInitDoneR = crate::BitReader;
#[doc = "Field `mem_init_done` writer - 0:0\\]
This field will be high once intialization of HWASS Shared RAM parition0 is finished. Writing '1' would clear the bit"]
pub type MemInitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once intialization of HWASS Shared RAM parition0 is finished. Writing '1' would clear the bit"]
    #[inline(always)]
    pub fn mem_init_done(&self) -> MemInitDoneR {
        MemInitDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once intialization of HWASS Shared RAM parition0 is finished. Writing '1' would clear the bit"]
    #[inline(always)]
    #[must_use]
    pub fn mem_init_done(&mut self) -> MemInitDoneW<HwassShrdRam0MemInitDoneSpec> {
        MemInitDoneW::new(self, 0)
    }
}
#[doc = "HWASS_SHRD_RAM0_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram0_mem_init_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram0_mem_init_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassShrdRam0MemInitDoneSpec;
impl crate::RegisterSpec for HwassShrdRam0MemInitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_shrd_ram0_mem_init_done::R`](R) reader structure"]
impl crate::Readable for HwassShrdRam0MemInitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_shrd_ram0_mem_init_done::W`](W) writer structure"]
impl crate::Writable for HwassShrdRam0MemInitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_SHRD_RAM0_MEM_INIT_DONE to value 0"]
impl crate::Resettable for HwassShrdRam0MemInitDoneSpec {
    const RESET_VALUE: u32 = 0;
}
