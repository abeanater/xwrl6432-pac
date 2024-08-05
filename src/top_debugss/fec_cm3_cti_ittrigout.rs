#[doc = "Register `FEC_CM3_CTI_ITTRIGOUT` reader"]
pub type R = crate::R<FecCm3CtiIttrigoutSpec>;
#[doc = "Register `FEC_CM3_CTI_ITTRIGOUT` writer"]
pub type W = crate::W<FecCm3CtiIttrigoutSpec>;
#[doc = "Field `FEC_CM3_CTI_ITTRIGOUT` reader - "]
pub type FecCm3CtiIttrigoutR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_ITTRIGOUT` writer - "]
pub type FecCm3CtiIttrigoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_ittrigout(&self) -> FecCm3CtiIttrigoutR {
        FecCm3CtiIttrigoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_ittrigout(&mut self) -> FecCm3CtiIttrigoutW<FecCm3CtiIttrigoutSpec> {
        FecCm3CtiIttrigoutW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_ITTRIGOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_ittrigout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_ittrigout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiIttrigoutSpec;
impl crate::RegisterSpec for FecCm3CtiIttrigoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_ittrigout::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiIttrigoutSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_ittrigout::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiIttrigoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_ITTRIGOUT to value 0"]
impl crate::Resettable for FecCm3CtiIttrigoutSpec {
    const RESET_VALUE: u32 = 0;
}
