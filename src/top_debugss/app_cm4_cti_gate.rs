#[doc = "Register `APP_CM4_CTI_GATE` reader"]
pub type R = crate::R<AppCm4CtiGateSpec>;
#[doc = "Register `APP_CM4_CTI_GATE` writer"]
pub type W = crate::W<AppCm4CtiGateSpec>;
#[doc = "Field `APP_CM4_CTI_GATE` reader - "]
pub type AppCm4CtiGateR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_GATE` writer - "]
pub type AppCm4CtiGateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_gate(&self) -> AppCm4CtiGateR {
        AppCm4CtiGateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_gate(&mut self) -> AppCm4CtiGateW<AppCm4CtiGateSpec> {
        AppCm4CtiGateW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiGateSpec;
impl crate::RegisterSpec for AppCm4CtiGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_gate::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiGateSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_gate::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_GATE to value 0"]
impl crate::Resettable for AppCm4CtiGateSpec {
    const RESET_VALUE: u32 = 0;
}
