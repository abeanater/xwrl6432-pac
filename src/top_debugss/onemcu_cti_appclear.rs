#[doc = "Register `ONEMCU_CTI_APPCLEAR` reader"]
pub type R = crate::R<OnemcuCtiAppclearSpec>;
#[doc = "Register `ONEMCU_CTI_APPCLEAR` writer"]
pub type W = crate::W<OnemcuCtiAppclearSpec>;
#[doc = "Field `ONEMCU_CTI_APPCLEAR` reader - "]
pub type OnemcuCtiAppclearR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_APPCLEAR` writer - "]
pub type OnemcuCtiAppclearW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_appclear(&self) -> OnemcuCtiAppclearR {
        OnemcuCtiAppclearR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_appclear(&mut self) -> OnemcuCtiAppclearW<OnemcuCtiAppclearSpec> {
        OnemcuCtiAppclearW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_APPCLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_appclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_appclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiAppclearSpec;
impl crate::RegisterSpec for OnemcuCtiAppclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_appclear::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiAppclearSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_appclear::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiAppclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_APPCLEAR to value 0"]
impl crate::Resettable for OnemcuCtiAppclearSpec {
    const RESET_VALUE: u32 = 0;
}
