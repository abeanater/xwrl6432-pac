#[doc = "Register `FEC_CM3_CTI_ITCHINACK` reader"]
pub type R = crate::R<FecCm3CtiItchinackSpec>;
#[doc = "Register `FEC_CM3_CTI_ITCHINACK` writer"]
pub type W = crate::W<FecCm3CtiItchinackSpec>;
#[doc = "Field `FEC_CM3_CTI_ITCHINACK` reader - "]
pub type FecCm3CtiItchinackR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_ITCHINACK` writer - "]
pub type FecCm3CtiItchinackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_itchinack(&self) -> FecCm3CtiItchinackR {
        FecCm3CtiItchinackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_itchinack(&mut self) -> FecCm3CtiItchinackW<FecCm3CtiItchinackSpec> {
        FecCm3CtiItchinackW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_ITCHINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itchinack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itchinack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiItchinackSpec;
impl crate::RegisterSpec for FecCm3CtiItchinackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_itchinack::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiItchinackSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_itchinack::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiItchinackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_ITCHINACK to value 0"]
impl crate::Resettable for FecCm3CtiItchinackSpec {
    const RESET_VALUE: u32 = 0;
}
