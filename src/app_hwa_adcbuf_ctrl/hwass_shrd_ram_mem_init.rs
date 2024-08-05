#[doc = "Register `HWASS_SHRD_RAM_MEM_INIT` reader"]
pub type R = crate::R<HwassShrdRamMemInitSpec>;
#[doc = "Register `HWASS_SHRD_RAM_MEM_INIT` writer"]
pub type W = crate::W<HwassShrdRamMemInitSpec>;
#[doc = "Field `mem_init` reader - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initialization of HWASS 160 kb shared memory bank . Value in each row is initialized to 0x00_0000_0000"]
pub type MemInitR = crate::BitReader;
#[doc = "Field `mem_init` writer - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initialization of HWASS 160 kb shared memory bank . Value in each row is initialized to 0x00_0000_0000"]
pub type MemInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initialization of HWASS 160 kb shared memory bank . Value in each row is initialized to 0x00_0000_0000"]
    #[inline(always)]
    pub fn mem_init(&self) -> MemInitR {
        MemInitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initialization of HWASS 160 kb shared memory bank . Value in each row is initialized to 0x00_0000_0000"]
    #[inline(always)]
    #[must_use]
    pub fn mem_init(&mut self) -> MemInitW<HwassShrdRamMemInitSpec> {
        MemInitW::new(self, 0)
    }
}
#[doc = "HWASS_SHRD_RAM_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`hwass_shrd_ram_mem_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwass_shrd_ram_mem_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwassShrdRamMemInitSpec;
impl crate::RegisterSpec for HwassShrdRamMemInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwass_shrd_ram_mem_init::R`](R) reader structure"]
impl crate::Readable for HwassShrdRamMemInitSpec {}
#[doc = "`write(|w| ..)` method takes [`hwass_shrd_ram_mem_init::W`](W) writer structure"]
impl crate::Writable for HwassShrdRamMemInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWASS_SHRD_RAM_MEM_INIT to value 0"]
impl crate::Resettable for HwassShrdRamMemInitSpec {
    const RESET_VALUE: u32 = 0;
}
