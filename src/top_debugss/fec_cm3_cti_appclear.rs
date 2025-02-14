#[doc = "Register `FEC_CM3_CTI_APPCLEAR` reader"]
pub type R = crate::R<FecCm3CtiAppclearSpec>;
#[doc = "Register `FEC_CM3_CTI_APPCLEAR` writer"]
pub type W = crate::W<FecCm3CtiAppclearSpec>;
#[doc = "Field `FEC_CM3_CTI_APPCLEAR` reader - "]
pub type FecCm3CtiAppclearR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_APPCLEAR` writer - "]
pub type FecCm3CtiAppclearW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_appclear(&self) -> FecCm3CtiAppclearR {
        FecCm3CtiAppclearR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_appclear(&mut self) -> FecCm3CtiAppclearW<FecCm3CtiAppclearSpec> {
        FecCm3CtiAppclearW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_APPCLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_appclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_appclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiAppclearSpec;
impl crate::RegisterSpec for FecCm3CtiAppclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_appclear::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiAppclearSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_appclear::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiAppclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_APPCLEAR to value 0"]
impl crate::Resettable for FecCm3CtiAppclearSpec {
    const RESET_VALUE: u32 = 0;
}
