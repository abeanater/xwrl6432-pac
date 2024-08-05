#[doc = "Register `ONEMCU_CTI_INTACK` reader"]
pub type R = crate::R<OnemcuCtiIntackSpec>;
#[doc = "Register `ONEMCU_CTI_INTACK` writer"]
pub type W = crate::W<OnemcuCtiIntackSpec>;
#[doc = "Field `ONEMCU_CTI_INTACK` reader - "]
pub type OnemcuCtiIntackR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_INTACK` writer - "]
pub type OnemcuCtiIntackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_intack(&self) -> OnemcuCtiIntackR {
        OnemcuCtiIntackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_intack(&mut self) -> OnemcuCtiIntackW<OnemcuCtiIntackSpec> {
        OnemcuCtiIntackW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_INTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_intack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_intack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiIntackSpec;
impl crate::RegisterSpec for OnemcuCtiIntackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_intack::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiIntackSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_intack::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiIntackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_INTACK to value 0"]
impl crate::Resettable for OnemcuCtiIntackSpec {
    const RESET_VALUE: u32 = 0;
}
