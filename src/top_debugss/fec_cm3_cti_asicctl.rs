#[doc = "Register `FEC_CM3_CTI_ASICCTL` reader"]
pub type R = crate::R<FecCm3CtiAsicctlSpec>;
#[doc = "Register `FEC_CM3_CTI_ASICCTL` writer"]
pub type W = crate::W<FecCm3CtiAsicctlSpec>;
#[doc = "Field `FEC_CM3_CTI_ASICCTL` reader - "]
pub type FecCm3CtiAsicctlR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_ASICCTL` writer - "]
pub type FecCm3CtiAsicctlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_asicctl(&self) -> FecCm3CtiAsicctlR {
        FecCm3CtiAsicctlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_asicctl(&mut self) -> FecCm3CtiAsicctlW<FecCm3CtiAsicctlSpec> {
        FecCm3CtiAsicctlW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_ASICCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_asicctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_asicctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiAsicctlSpec;
impl crate::RegisterSpec for FecCm3CtiAsicctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_asicctl::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiAsicctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_asicctl::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiAsicctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_ASICCTL to value 0"]
impl crate::Resettable for FecCm3CtiAsicctlSpec {
    const RESET_VALUE: u32 = 0;
}
