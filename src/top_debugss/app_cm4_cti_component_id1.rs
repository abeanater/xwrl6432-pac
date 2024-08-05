#[doc = "Register `APP_CM4_CTI_Component_ID1` reader"]
pub type R = crate::R<AppCm4CtiComponentId1Spec>;
#[doc = "Register `APP_CM4_CTI_Component_ID1` writer"]
pub type W = crate::W<AppCm4CtiComponentId1Spec>;
#[doc = "Field `APP_CM4_CTI_Component_ID1` reader - "]
pub type AppCm4CtiComponentId1R = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_Component_ID1` writer - "]
pub type AppCm4CtiComponentId1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_component_id1(&self) -> AppCm4CtiComponentId1R {
        AppCm4CtiComponentId1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_component_id1(
        &mut self,
    ) -> AppCm4CtiComponentId1W<AppCm4CtiComponentId1Spec> {
        AppCm4CtiComponentId1W::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_Component_ID1\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_component_id1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_component_id1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiComponentId1Spec;
impl crate::RegisterSpec for AppCm4CtiComponentId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_component_id1::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiComponentId1Spec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_component_id1::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiComponentId1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_Component_ID1 to value 0"]
impl crate::Resettable for AppCm4CtiComponentId1Spec {
    const RESET_VALUE: u32 = 0;
}
