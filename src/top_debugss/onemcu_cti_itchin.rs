#[doc = "Register `ONEMCU_CTI_ITCHIN` reader"]
pub type R = crate::R<OnemcuCtiItchinSpec>;
#[doc = "Register `ONEMCU_CTI_ITCHIN` writer"]
pub type W = crate::W<OnemcuCtiItchinSpec>;
#[doc = "Field `ONEMCU_CTI_ITCHIN` reader - "]
pub type OnemcuCtiItchinR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITCHIN` writer - "]
pub type OnemcuCtiItchinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_itchin(&self) -> OnemcuCtiItchinR {
        OnemcuCtiItchinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_itchin(&mut self) -> OnemcuCtiItchinW<OnemcuCtiItchinSpec> {
        OnemcuCtiItchinW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITCHIN\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itchin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itchin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiItchinSpec;
impl crate::RegisterSpec for OnemcuCtiItchinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_itchin::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiItchinSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_itchin::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiItchinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITCHIN to value 0"]
impl crate::Resettable for OnemcuCtiItchinSpec {
    const RESET_VALUE: u32 = 0;
}
