#[doc = "Register `APP_CM4_CTI_ITCTRL` reader"]
pub type R = crate::R<AppCm4CtiItctrlSpec>;
#[doc = "Register `APP_CM4_CTI_ITCTRL` writer"]
pub type W = crate::W<AppCm4CtiItctrlSpec>;
#[doc = "Field `APP_CM4_CTI_ITCTRL` reader - "]
pub type AppCm4CtiItctrlR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_ITCTRL` writer - "]
pub type AppCm4CtiItctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_itctrl(&self) -> AppCm4CtiItctrlR {
        AppCm4CtiItctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_itctrl(&mut self) -> AppCm4CtiItctrlW<AppCm4CtiItctrlSpec> {
        AppCm4CtiItctrlW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_ITCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiItctrlSpec;
impl crate::RegisterSpec for AppCm4CtiItctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_itctrl::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiItctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_itctrl::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiItctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_ITCTRL to value 0"]
impl crate::Resettable for AppCm4CtiItctrlSpec {
    const RESET_VALUE: u32 = 0;
}
