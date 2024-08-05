#[doc = "Register `APPSS_CM4_GLOBAL_CONFIG` reader"]
pub type R = crate::R<AppssCm4GlobalConfigSpec>;
#[doc = "Register `APPSS_CM4_GLOBAL_CONFIG` writer"]
pub type W = crate::W<AppssCm4GlobalConfigSpec>;
#[doc = "Field `teinit` reader - 0:0\\]
Reserved"]
pub type TeinitR = crate::BitReader;
#[doc = "Field `teinit` writer - 0:0\\]
Reserved"]
pub type TeinitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn teinit(&self) -> TeinitR {
        TeinitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn teinit(&mut self) -> TeinitW<AppssCm4GlobalConfigSpec> {
        TeinitW::new(self, 0)
    }
}
#[doc = "APPSS_CM4_GLOBAL_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_cm4_global_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_cm4_global_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssCm4GlobalConfigSpec;
impl crate::RegisterSpec for AppssCm4GlobalConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_cm4_global_config::R`](R) reader structure"]
impl crate::Readable for AppssCm4GlobalConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_cm4_global_config::W`](W) writer structure"]
impl crate::Writable for AppssCm4GlobalConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_CM4_GLOBAL_CONFIG to value 0"]
impl crate::Resettable for AppssCm4GlobalConfigSpec {
    const RESET_VALUE: u32 = 0;
}
