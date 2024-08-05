#[doc = "Register `ONEMCU_CTI_ITTRIGIN` reader"]
pub type R = crate::R<OnemcuCtiIttriginSpec>;
#[doc = "Register `ONEMCU_CTI_ITTRIGIN` writer"]
pub type W = crate::W<OnemcuCtiIttriginSpec>;
#[doc = "Field `ONEMCU_CTI_ITTRIGIN` reader - "]
pub type OnemcuCtiIttriginR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITTRIGIN` writer - "]
pub type OnemcuCtiIttriginW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_ittrigin(&self) -> OnemcuCtiIttriginR {
        OnemcuCtiIttriginR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_ittrigin(&mut self) -> OnemcuCtiIttriginW<OnemcuCtiIttriginSpec> {
        OnemcuCtiIttriginW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITTRIGIN\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_ittrigin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_ittrigin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiIttriginSpec;
impl crate::RegisterSpec for OnemcuCtiIttriginSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_ittrigin::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiIttriginSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_ittrigin::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiIttriginSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITTRIGIN to value 0"]
impl crate::Resettable for OnemcuCtiIttriginSpec {
    const RESET_VALUE: u32 = 0;
}
