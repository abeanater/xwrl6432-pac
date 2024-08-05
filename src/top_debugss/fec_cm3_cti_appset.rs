#[doc = "Register `FEC_CM3_CTI_APPSET` reader"]
pub type R = crate::R<FecCm3CtiAppsetSpec>;
#[doc = "Register `FEC_CM3_CTI_APPSET` writer"]
pub type W = crate::W<FecCm3CtiAppsetSpec>;
#[doc = "Field `FEC_CM3_CTI_APPSET` reader - "]
pub type FecCm3CtiAppsetR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_APPSET` writer - "]
pub type FecCm3CtiAppsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_appset(&self) -> FecCm3CtiAppsetR {
        FecCm3CtiAppsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_appset(&mut self) -> FecCm3CtiAppsetW<FecCm3CtiAppsetSpec> {
        FecCm3CtiAppsetW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_APPSET\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_appset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_appset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiAppsetSpec;
impl crate::RegisterSpec for FecCm3CtiAppsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_appset::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiAppsetSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_appset::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiAppsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_APPSET to value 0"]
impl crate::Resettable for FecCm3CtiAppsetSpec {
    const RESET_VALUE: u32 = 0;
}
