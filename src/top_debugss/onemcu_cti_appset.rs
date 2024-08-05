#[doc = "Register `ONEMCU_CTI_APPSET` reader"]
pub type R = crate::R<OnemcuCtiAppsetSpec>;
#[doc = "Register `ONEMCU_CTI_APPSET` writer"]
pub type W = crate::W<OnemcuCtiAppsetSpec>;
#[doc = "Field `ONEMCU_CTI_APPSET` reader - "]
pub type OnemcuCtiAppsetR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_APPSET` writer - "]
pub type OnemcuCtiAppsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_appset(&self) -> OnemcuCtiAppsetR {
        OnemcuCtiAppsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_appset(&mut self) -> OnemcuCtiAppsetW<OnemcuCtiAppsetSpec> {
        OnemcuCtiAppsetW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_APPSET\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_appset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_appset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiAppsetSpec;
impl crate::RegisterSpec for OnemcuCtiAppsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_appset::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiAppsetSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_appset::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiAppsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_APPSET to value 0"]
impl crate::Resettable for OnemcuCtiAppsetSpec {
    const RESET_VALUE: u32 = 0;
}
