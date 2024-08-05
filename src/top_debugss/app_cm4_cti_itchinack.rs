#[doc = "Register `APP_CM4_CTI_ITCHINACK` reader"]
pub type R = crate::R<AppCm4CtiItchinackSpec>;
#[doc = "Register `APP_CM4_CTI_ITCHINACK` writer"]
pub type W = crate::W<AppCm4CtiItchinackSpec>;
#[doc = "Field `APP_CM4_CTI_ITCHINACK` reader - "]
pub type AppCm4CtiItchinackR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_ITCHINACK` writer - "]
pub type AppCm4CtiItchinackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_itchinack(&self) -> AppCm4CtiItchinackR {
        AppCm4CtiItchinackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_itchinack(&mut self) -> AppCm4CtiItchinackW<AppCm4CtiItchinackSpec> {
        AppCm4CtiItchinackW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_ITCHINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_itchinack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_itchinack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiItchinackSpec;
impl crate::RegisterSpec for AppCm4CtiItchinackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_itchinack::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiItchinackSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_itchinack::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiItchinackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_ITCHINACK to value 0"]
impl crate::Resettable for AppCm4CtiItchinackSpec {
    const RESET_VALUE: u32 = 0;
}
