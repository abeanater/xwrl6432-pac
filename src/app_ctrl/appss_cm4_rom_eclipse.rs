#[doc = "Register `APPSS_CM4_ROM_ECLIPSE` reader"]
pub type R = crate::R<AppssCm4RomEclipseSpec>;
#[doc = "Register `APPSS_CM4_ROM_ECLIPSE` writer"]
pub type W = crate::W<AppssCm4RomEclipseSpec>;
#[doc = "Field `memswap` reader - 2:0\\]
writing '111' ensures eclipsing of CPU_ROM immediately if memswap_wait is not set. If memswap_wait is set then ROM is eclipsed after CPU sys reset assertion."]
pub type MemswapR = crate::FieldReader;
#[doc = "Field `memswap` writer - 2:0\\]
writing '111' ensures eclipsing of CPU_ROM immediately if memswap_wait is not set. If memswap_wait is set then ROM is eclipsed after CPU sys reset assertion."]
pub type MemswapW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `memswap_wait` reader - 10:8\\]
writing 3'b111 ensures ROM-Eclipsing happens only after CPU sys reset. Orelse it will be a immediate change."]
pub type MemswapWaitR = crate::FieldReader;
#[doc = "Field `memswap_wait` writer - 10:8\\]
writing 3'b111 ensures ROM-Eclipsing happens only after CPU sys reset. Orelse it will be a immediate change."]
pub type MemswapWaitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' ensures eclipsing of CPU_ROM immediately if memswap_wait is not set. If memswap_wait is set then ROM is eclipsed after CPU sys reset assertion."]
    #[inline(always)]
    pub fn memswap(&self) -> MemswapR {
        MemswapR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
writing 3'b111 ensures ROM-Eclipsing happens only after CPU sys reset. Orelse it will be a immediate change."]
    #[inline(always)]
    pub fn memswap_wait(&self) -> MemswapWaitR {
        MemswapWaitR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' ensures eclipsing of CPU_ROM immediately if memswap_wait is not set. If memswap_wait is set then ROM is eclipsed after CPU sys reset assertion."]
    #[inline(always)]
    #[must_use]
    pub fn memswap(&mut self) -> MemswapW<AppssCm4RomEclipseSpec> {
        MemswapW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
writing 3'b111 ensures ROM-Eclipsing happens only after CPU sys reset. Orelse it will be a immediate change."]
    #[inline(always)]
    #[must_use]
    pub fn memswap_wait(&mut self) -> MemswapWaitW<AppssCm4RomEclipseSpec> {
        MemswapWaitW::new(self, 8)
    }
}
#[doc = "APPSS_CM4_ROM_ECLIPSE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_rom_eclipse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_rom_eclipse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssCm4RomEclipseSpec;
impl crate::RegisterSpec for AppssCm4RomEclipseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_cm4_rom_eclipse::R`](R) reader structure"]
impl crate::Readable for AppssCm4RomEclipseSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_cm4_rom_eclipse::W`](W) writer structure"]
impl crate::Writable for AppssCm4RomEclipseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_CM4_ROM_ECLIPSE to value 0"]
impl crate::Resettable for AppssCm4RomEclipseSpec {
    const RESET_VALUE: u32 = 0;
}
