#[doc = "Register `APPSS_RAM3A_MEM_INIT_DONE` reader"]
pub type R = crate::R<AppssRam3aMemInitDoneSpec>;
#[doc = "Register `APPSS_RAM3A_MEM_INIT_DONE` writer"]
pub type W = crate::W<AppssRam3aMemInitDoneSpec>;
#[doc = "Field `mem_init_done` reader - 0:0\\]
This field will be high once initialization of APPSS_RAM3 partion0 banks is finished. Writing '1' would clear the bit."]
pub type MemInitDoneR = crate::BitReader;
#[doc = "Field `mem_init_done` writer - 0:0\\]
This field will be high once initialization of APPSS_RAM3 partion0 banks is finished. Writing '1' would clear the bit."]
pub type MemInitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once initialization of APPSS_RAM3 partion0 banks is finished. Writing '1' would clear the bit."]
    #[inline(always)]
    pub fn mem_init_done(&self) -> MemInitDoneR {
        MemInitDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once initialization of APPSS_RAM3 partion0 banks is finished. Writing '1' would clear the bit."]
    #[inline(always)]
    #[must_use]
    pub fn mem_init_done(&mut self) -> MemInitDoneW<AppssRam3aMemInitDoneSpec> {
        MemInitDoneW::new(self, 0)
    }
}
#[doc = "APPSS_RAM3A_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram3a_mem_init_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram3a_mem_init_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssRam3aMemInitDoneSpec;
impl crate::RegisterSpec for AppssRam3aMemInitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_ram3a_mem_init_done::R`](R) reader structure"]
impl crate::Readable for AppssRam3aMemInitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_ram3a_mem_init_done::W`](W) writer structure"]
impl crate::Writable for AppssRam3aMemInitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_RAM3A_MEM_INIT_DONE to value 0"]
impl crate::Resettable for AppssRam3aMemInitDoneSpec {
    const RESET_VALUE: u32 = 0;
}
