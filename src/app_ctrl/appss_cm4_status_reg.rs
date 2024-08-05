#[doc = "Register `APPSS_CM4_STATUS_REG` reader"]
pub type R = crate::R<AppssCm4StatusRegSpec>;
#[doc = "Register `APPSS_CM4_STATUS_REG` writer"]
pub type W = crate::W<AppssCm4StatusRegSpec>;
#[doc = "Field `memswap` reader - 0:0\\]
reading 1: confirms ROM is Eclipsed from with RAM for the CPU."]
pub type MemswapR = crate::BitReader;
#[doc = "Field `memswap` writer - 0:0\\]
reading 1: confirms ROM is Eclipsed from with RAM for the CPU."]
pub type MemswapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
reading 1: confirms ROM is Eclipsed from with RAM for the CPU."]
    #[inline(always)]
    pub fn memswap(&self) -> MemswapR {
        MemswapR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
reading 1: confirms ROM is Eclipsed from with RAM for the CPU."]
    #[inline(always)]
    #[must_use]
    pub fn memswap(&mut self) -> MemswapW<AppssCm4StatusRegSpec> {
        MemswapW::new(self, 0)
    }
}
#[doc = "APPSS_CM4_STATUS_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssCm4StatusRegSpec;
impl crate::RegisterSpec for AppssCm4StatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_cm4_status_reg::R`](R) reader structure"]
impl crate::Readable for AppssCm4StatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_cm4_status_reg::W`](W) writer structure"]
impl crate::Writable for AppssCm4StatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_CM4_STATUS_REG to value 0"]
impl crate::Resettable for AppssCm4StatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
