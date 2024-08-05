#[doc = "Register `APPSS_RAM1A_MEM_INIT_STATUS` reader"]
pub type R = crate::R<AppssRam1aMemInitStatusSpec>;
#[doc = "Register `APPSS_RAM1A_MEM_INIT_STATUS` writer"]
pub type W = crate::W<AppssRam1aMemInitStatusSpec>;
#[doc = "Field `mem_status` reader - 0:0\\]
1'b0: No initialization is happening for APPSS_RAM1 partion0 bank 1'b1: Initialization is in progress for APPSS_RAM1 partion0 bank"]
pub type MemStatusR = crate::BitReader;
#[doc = "Field `mem_status` writer - 0:0\\]
1'b0: No initialization is happening for APPSS_RAM1 partion0 bank 1'b1: Initialization is in progress for APPSS_RAM1 partion0 bank"]
pub type MemStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for APPSS_RAM1 partion0 bank 1'b1: Initialization is in progress for APPSS_RAM1 partion0 bank"]
    #[inline(always)]
    pub fn mem_status(&self) -> MemStatusR {
        MemStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for APPSS_RAM1 partion0 bank 1'b1: Initialization is in progress for APPSS_RAM1 partion0 bank"]
    #[inline(always)]
    #[must_use]
    pub fn mem_status(&mut self) -> MemStatusW<AppssRam1aMemInitStatusSpec> {
        MemStatusW::new(self, 0)
    }
}
#[doc = "APPSS_RAM1A_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram1a_mem_init_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram1a_mem_init_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssRam1aMemInitStatusSpec;
impl crate::RegisterSpec for AppssRam1aMemInitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_ram1a_mem_init_status::R`](R) reader structure"]
impl crate::Readable for AppssRam1aMemInitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_ram1a_mem_init_status::W`](W) writer structure"]
impl crate::Writable for AppssRam1aMemInitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_RAM1A_MEM_INIT_STATUS to value 0"]
impl crate::Resettable for AppssRam1aMemInitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
