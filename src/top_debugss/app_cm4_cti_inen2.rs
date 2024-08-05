#[doc = "Register `APP_CM4_CTI_INEN2` reader"]
pub type R = crate::R<AppCm4CtiInen2Spec>;
#[doc = "Register `APP_CM4_CTI_INEN2` writer"]
pub type W = crate::W<AppCm4CtiInen2Spec>;
#[doc = "Field `APP_CM4_CTI_INEN2` reader - "]
pub type AppCm4CtiInen2R = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_INEN2` writer - "]
pub type AppCm4CtiInen2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_inen2(&self) -> AppCm4CtiInen2R {
        AppCm4CtiInen2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_inen2(&mut self) -> AppCm4CtiInen2W<AppCm4CtiInen2Spec> {
        AppCm4CtiInen2W::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_INEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiInen2Spec;
impl crate::RegisterSpec for AppCm4CtiInen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_inen2::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiInen2Spec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_inen2::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiInen2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_INEN2 to value 0"]
impl crate::Resettable for AppCm4CtiInen2Spec {
    const RESET_VALUE: u32 = 0;
}
