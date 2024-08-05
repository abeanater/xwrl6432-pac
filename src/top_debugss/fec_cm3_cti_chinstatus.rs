#[doc = "Register `FEC_CM3_CTI_CHINSTATUS` reader"]
pub type R = crate::R<FecCm3CtiChinstatusSpec>;
#[doc = "Register `FEC_CM3_CTI_CHINSTATUS` writer"]
pub type W = crate::W<FecCm3CtiChinstatusSpec>;
#[doc = "Field `FEC_CM3_CTI_CHINSTATUS` reader - "]
pub type FecCm3CtiChinstatusR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_CHINSTATUS` writer - "]
pub type FecCm3CtiChinstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_chinstatus(&self) -> FecCm3CtiChinstatusR {
        FecCm3CtiChinstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_chinstatus(&mut self) -> FecCm3CtiChinstatusW<FecCm3CtiChinstatusSpec> {
        FecCm3CtiChinstatusW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_CHINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_chinstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_chinstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiChinstatusSpec;
impl crate::RegisterSpec for FecCm3CtiChinstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_chinstatus::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiChinstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_chinstatus::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiChinstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_CHINSTATUS to value 0"]
impl crate::Resettable for FecCm3CtiChinstatusSpec {
    const RESET_VALUE: u32 = 0;
}
