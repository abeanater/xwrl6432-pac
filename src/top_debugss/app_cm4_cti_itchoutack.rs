#[doc = "Register `APP_CM4_CTI_ITCHOUTACK` reader"]
pub type R = crate::R<AppCm4CtiItchoutackSpec>;
#[doc = "Register `APP_CM4_CTI_ITCHOUTACK` writer"]
pub type W = crate::W<AppCm4CtiItchoutackSpec>;
#[doc = "Field `APP_CM4_CTI_ITCHOUTACK` reader - "]
pub type AppCm4CtiItchoutackR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_ITCHOUTACK` writer - "]
pub type AppCm4CtiItchoutackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_itchoutack(&self) -> AppCm4CtiItchoutackR {
        AppCm4CtiItchoutackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_itchoutack(&mut self) -> AppCm4CtiItchoutackW<AppCm4CtiItchoutackSpec> {
        AppCm4CtiItchoutackW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_ITCHOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_itchoutack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_itchoutack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiItchoutackSpec;
impl crate::RegisterSpec for AppCm4CtiItchoutackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_itchoutack::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiItchoutackSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_itchoutack::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiItchoutackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_ITCHOUTACK to value 0"]
impl crate::Resettable for AppCm4CtiItchoutackSpec {
    const RESET_VALUE: u32 = 0;
}
