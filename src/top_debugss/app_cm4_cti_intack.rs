#[doc = "Register `APP_CM4_CTI_INTACK` reader"]
pub type R = crate::R<AppCm4CtiIntackSpec>;
#[doc = "Register `APP_CM4_CTI_INTACK` writer"]
pub type W = crate::W<AppCm4CtiIntackSpec>;
#[doc = "Field `APP_CM4_CTI_INTACK` reader - "]
pub type AppCm4CtiIntackR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_INTACK` writer - "]
pub type AppCm4CtiIntackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_intack(&self) -> AppCm4CtiIntackR {
        AppCm4CtiIntackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_intack(&mut self) -> AppCm4CtiIntackW<AppCm4CtiIntackSpec> {
        AppCm4CtiIntackW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_INTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_intack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_intack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiIntackSpec;
impl crate::RegisterSpec for AppCm4CtiIntackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_intack::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiIntackSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_intack::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiIntackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_INTACK to value 0"]
impl crate::Resettable for AppCm4CtiIntackSpec {
    const RESET_VALUE: u32 = 0;
}
