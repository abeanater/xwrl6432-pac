#[doc = "Register `APP_CM4_CTI_INEN3` reader"]
pub type R = crate::R<AppCm4CtiInen3Spec>;
#[doc = "Register `APP_CM4_CTI_INEN3` writer"]
pub type W = crate::W<AppCm4CtiInen3Spec>;
#[doc = "Field `APP_CM4_CTI_INEN3` reader - "]
pub type AppCm4CtiInen3R = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_INEN3` writer - "]
pub type AppCm4CtiInen3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_inen3(&self) -> AppCm4CtiInen3R {
        AppCm4CtiInen3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_inen3(&mut self) -> AppCm4CtiInen3W<AppCm4CtiInen3Spec> {
        AppCm4CtiInen3W::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_INEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiInen3Spec;
impl crate::RegisterSpec for AppCm4CtiInen3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_inen3::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiInen3Spec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_inen3::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiInen3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_INEN3 to value 0"]
impl crate::Resettable for AppCm4CtiInen3Spec {
    const RESET_VALUE: u32 = 0;
}
