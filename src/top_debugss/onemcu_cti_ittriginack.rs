#[doc = "Register `ONEMCU_CTI_ITTRIGINACK` reader"]
pub type R = crate::R<OnemcuCtiIttriginackSpec>;
#[doc = "Register `ONEMCU_CTI_ITTRIGINACK` writer"]
pub type W = crate::W<OnemcuCtiIttriginackSpec>;
#[doc = "Field `ONEMCU_CTI_ITTRIGINACK` reader - "]
pub type OnemcuCtiIttriginackR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITTRIGINACK` writer - "]
pub type OnemcuCtiIttriginackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_ittriginack(&self) -> OnemcuCtiIttriginackR {
        OnemcuCtiIttriginackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_ittriginack(&mut self) -> OnemcuCtiIttriginackW<OnemcuCtiIttriginackSpec> {
        OnemcuCtiIttriginackW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITTRIGINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_ittriginack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_ittriginack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiIttriginackSpec;
impl crate::RegisterSpec for OnemcuCtiIttriginackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_ittriginack::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiIttriginackSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_ittriginack::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiIttriginackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITTRIGINACK to value 0"]
impl crate::Resettable for OnemcuCtiIttriginackSpec {
    const RESET_VALUE: u32 = 0;
}
