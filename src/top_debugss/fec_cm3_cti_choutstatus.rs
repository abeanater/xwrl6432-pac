#[doc = "Register `FEC_CM3_CTI_CHOUTSTATUS` reader"]
pub type R = crate::R<FecCm3CtiChoutstatusSpec>;
#[doc = "Register `FEC_CM3_CTI_CHOUTSTATUS` writer"]
pub type W = crate::W<FecCm3CtiChoutstatusSpec>;
#[doc = "Field `FEC_CM3_CTI_CHOUTSTATUS` reader - "]
pub type FecCm3CtiChoutstatusR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_CHOUTSTATUS` writer - "]
pub type FecCm3CtiChoutstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_choutstatus(&self) -> FecCm3CtiChoutstatusR {
        FecCm3CtiChoutstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_choutstatus(&mut self) -> FecCm3CtiChoutstatusW<FecCm3CtiChoutstatusSpec> {
        FecCm3CtiChoutstatusW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_CHOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_choutstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_choutstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiChoutstatusSpec;
impl crate::RegisterSpec for FecCm3CtiChoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_choutstatus::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiChoutstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_choutstatus::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiChoutstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_CHOUTSTATUS to value 0"]
impl crate::Resettable for FecCm3CtiChoutstatusSpec {
    const RESET_VALUE: u32 = 0;
}
