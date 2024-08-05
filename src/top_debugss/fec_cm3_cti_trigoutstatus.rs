#[doc = "Register `FEC_CM3_CTI_TRIGOUTSTATUS` reader"]
pub type R = crate::R<FecCm3CtiTrigoutstatusSpec>;
#[doc = "Register `FEC_CM3_CTI_TRIGOUTSTATUS` writer"]
pub type W = crate::W<FecCm3CtiTrigoutstatusSpec>;
#[doc = "Field `FEC_CM3_CTI_TRIGOUTSTATUS` reader - "]
pub type FecCm3CtiTrigoutstatusR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_TRIGOUTSTATUS` writer - "]
pub type FecCm3CtiTrigoutstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_trigoutstatus(&self) -> FecCm3CtiTrigoutstatusR {
        FecCm3CtiTrigoutstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_trigoutstatus(
        &mut self,
    ) -> FecCm3CtiTrigoutstatusW<FecCm3CtiTrigoutstatusSpec> {
        FecCm3CtiTrigoutstatusW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_TRIGOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_trigoutstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_trigoutstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiTrigoutstatusSpec;
impl crate::RegisterSpec for FecCm3CtiTrigoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_trigoutstatus::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiTrigoutstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_trigoutstatus::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiTrigoutstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_TRIGOUTSTATUS to value 0"]
impl crate::Resettable for FecCm3CtiTrigoutstatusSpec {
    const RESET_VALUE: u32 = 0;
}
