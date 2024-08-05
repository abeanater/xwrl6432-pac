#[doc = "Register `ONEMCU_CTI_ITTRIGOUTACK` reader"]
pub type R = crate::R<OnemcuCtiIttrigoutackSpec>;
#[doc = "Register `ONEMCU_CTI_ITTRIGOUTACK` writer"]
pub type W = crate::W<OnemcuCtiIttrigoutackSpec>;
#[doc = "Field `ONEMCU_CTI_ITTRIGOUTACK` reader - "]
pub type OnemcuCtiIttrigoutackR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITTRIGOUTACK` writer - "]
pub type OnemcuCtiIttrigoutackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_ittrigoutack(&self) -> OnemcuCtiIttrigoutackR {
        OnemcuCtiIttrigoutackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_ittrigoutack(&mut self) -> OnemcuCtiIttrigoutackW<OnemcuCtiIttrigoutackSpec> {
        OnemcuCtiIttrigoutackW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITTRIGOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_ittrigoutack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_ittrigoutack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiIttrigoutackSpec;
impl crate::RegisterSpec for OnemcuCtiIttrigoutackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_ittrigoutack::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiIttrigoutackSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_ittrigoutack::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiIttrigoutackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITTRIGOUTACK to value 0"]
impl crate::Resettable for OnemcuCtiIttrigoutackSpec {
    const RESET_VALUE: u32 = 0;
}
