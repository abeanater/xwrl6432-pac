#[doc = "Register `ONEMCU_CTI_ITTRIGOUT` reader"]
pub type R = crate::R<OnemcuCtiIttrigoutSpec>;
#[doc = "Register `ONEMCU_CTI_ITTRIGOUT` writer"]
pub type W = crate::W<OnemcuCtiIttrigoutSpec>;
#[doc = "Field `ONEMCU_CTI_ITTRIGOUT` reader - "]
pub type OnemcuCtiIttrigoutR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITTRIGOUT` writer - "]
pub type OnemcuCtiIttrigoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_ittrigout(&self) -> OnemcuCtiIttrigoutR {
        OnemcuCtiIttrigoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_ittrigout(&mut self) -> OnemcuCtiIttrigoutW<OnemcuCtiIttrigoutSpec> {
        OnemcuCtiIttrigoutW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITTRIGOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_ittrigout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_ittrigout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiIttrigoutSpec;
impl crate::RegisterSpec for OnemcuCtiIttrigoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_ittrigout::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiIttrigoutSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_ittrigout::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiIttrigoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITTRIGOUT to value 0"]
impl crate::Resettable for OnemcuCtiIttrigoutSpec {
    const RESET_VALUE: u32 = 0;
}
