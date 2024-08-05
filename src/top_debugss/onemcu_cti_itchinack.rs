#[doc = "Register `ONEMCU_CTI_ITCHINACK` reader"]
pub type R = crate::R<OnemcuCtiItchinackSpec>;
#[doc = "Register `ONEMCU_CTI_ITCHINACK` writer"]
pub type W = crate::W<OnemcuCtiItchinackSpec>;
#[doc = "Field `ONEMCU_CTI_ITCHINACK` reader - "]
pub type OnemcuCtiItchinackR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITCHINACK` writer - "]
pub type OnemcuCtiItchinackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_itchinack(&self) -> OnemcuCtiItchinackR {
        OnemcuCtiItchinackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_itchinack(&mut self) -> OnemcuCtiItchinackW<OnemcuCtiItchinackSpec> {
        OnemcuCtiItchinackW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITCHINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itchinack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itchinack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiItchinackSpec;
impl crate::RegisterSpec for OnemcuCtiItchinackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_itchinack::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiItchinackSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_itchinack::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiItchinackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITCHINACK to value 0"]
impl crate::Resettable for OnemcuCtiItchinackSpec {
    const RESET_VALUE: u32 = 0;
}
