#[doc = "Register `FEC_CM3_CTI_APPPULSE` reader"]
pub type R = crate::R<FecCm3CtiApppulseSpec>;
#[doc = "Register `FEC_CM3_CTI_APPPULSE` writer"]
pub type W = crate::W<FecCm3CtiApppulseSpec>;
#[doc = "Field `FEC_CM3_CTI_APPPULSE` reader - "]
pub type FecCm3CtiApppulseR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_APPPULSE` writer - "]
pub type FecCm3CtiApppulseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_apppulse(&self) -> FecCm3CtiApppulseR {
        FecCm3CtiApppulseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_apppulse(&mut self) -> FecCm3CtiApppulseW<FecCm3CtiApppulseSpec> {
        FecCm3CtiApppulseW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_APPPULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_apppulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_apppulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiApppulseSpec;
impl crate::RegisterSpec for FecCm3CtiApppulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_apppulse::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiApppulseSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_apppulse::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiApppulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_APPPULSE to value 0"]
impl crate::Resettable for FecCm3CtiApppulseSpec {
    const RESET_VALUE: u32 = 0;
}
