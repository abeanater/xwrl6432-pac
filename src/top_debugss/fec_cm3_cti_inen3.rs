#[doc = "Register `FEC_CM3_CTI_INEN3` reader"]
pub type R = crate::R<FecCm3CtiInen3Spec>;
#[doc = "Register `FEC_CM3_CTI_INEN3` writer"]
pub type W = crate::W<FecCm3CtiInen3Spec>;
#[doc = "Field `FEC_CM3_CTI_INEN3` reader - "]
pub type FecCm3CtiInen3R = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_INEN3` writer - "]
pub type FecCm3CtiInen3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_inen3(&self) -> FecCm3CtiInen3R {
        FecCm3CtiInen3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_inen3(&mut self) -> FecCm3CtiInen3W<FecCm3CtiInen3Spec> {
        FecCm3CtiInen3W::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_INEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiInen3Spec;
impl crate::RegisterSpec for FecCm3CtiInen3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_inen3::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiInen3Spec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_inen3::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiInen3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_INEN3 to value 0"]
impl crate::Resettable for FecCm3CtiInen3Spec {
    const RESET_VALUE: u32 = 0;
}
