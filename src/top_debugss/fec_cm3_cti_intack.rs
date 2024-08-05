#[doc = "Register `FEC_CM3_CTI_INTACK` reader"]
pub type R = crate::R<FecCm3CtiIntackSpec>;
#[doc = "Register `FEC_CM3_CTI_INTACK` writer"]
pub type W = crate::W<FecCm3CtiIntackSpec>;
#[doc = "Field `FEC_CM3_CTI_INTACK` reader - "]
pub type FecCm3CtiIntackR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_INTACK` writer - "]
pub type FecCm3CtiIntackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_intack(&self) -> FecCm3CtiIntackR {
        FecCm3CtiIntackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_intack(&mut self) -> FecCm3CtiIntackW<FecCm3CtiIntackSpec> {
        FecCm3CtiIntackW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_INTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_intack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_intack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiIntackSpec;
impl crate::RegisterSpec for FecCm3CtiIntackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_intack::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiIntackSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_intack::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiIntackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_INTACK to value 0"]
impl crate::Resettable for FecCm3CtiIntackSpec {
    const RESET_VALUE: u32 = 0;
}
