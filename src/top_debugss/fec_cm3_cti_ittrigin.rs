#[doc = "Register `FEC_CM3_CTI_ITTRIGIN` reader"]
pub type R = crate::R<FecCm3CtiIttriginSpec>;
#[doc = "Register `FEC_CM3_CTI_ITTRIGIN` writer"]
pub type W = crate::W<FecCm3CtiIttriginSpec>;
#[doc = "Field `FEC_CM3_CTI_ITTRIGIN` reader - "]
pub type FecCm3CtiIttriginR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_ITTRIGIN` writer - "]
pub type FecCm3CtiIttriginW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_ittrigin(&self) -> FecCm3CtiIttriginR {
        FecCm3CtiIttriginR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_ittrigin(&mut self) -> FecCm3CtiIttriginW<FecCm3CtiIttriginSpec> {
        FecCm3CtiIttriginW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_ITTRIGIN\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_ittrigin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_ittrigin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiIttriginSpec;
impl crate::RegisterSpec for FecCm3CtiIttriginSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_ittrigin::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiIttriginSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_ittrigin::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiIttriginSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_ITTRIGIN to value 0"]
impl crate::Resettable for FecCm3CtiIttriginSpec {
    const RESET_VALUE: u32 = 0;
}
