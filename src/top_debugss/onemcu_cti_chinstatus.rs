#[doc = "Register `ONEMCU_CTI_CHINSTATUS` reader"]
pub type R = crate::R<OnemcuCtiChinstatusSpec>;
#[doc = "Register `ONEMCU_CTI_CHINSTATUS` writer"]
pub type W = crate::W<OnemcuCtiChinstatusSpec>;
#[doc = "Field `ONEMCU_CTI_CHINSTATUS` reader - "]
pub type OnemcuCtiChinstatusR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_CHINSTATUS` writer - "]
pub type OnemcuCtiChinstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_chinstatus(&self) -> OnemcuCtiChinstatusR {
        OnemcuCtiChinstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_chinstatus(&mut self) -> OnemcuCtiChinstatusW<OnemcuCtiChinstatusSpec> {
        OnemcuCtiChinstatusW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_CHINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_chinstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_chinstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiChinstatusSpec;
impl crate::RegisterSpec for OnemcuCtiChinstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_chinstatus::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiChinstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_chinstatus::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiChinstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_CHINSTATUS to value 0"]
impl crate::Resettable for OnemcuCtiChinstatusSpec {
    const RESET_VALUE: u32 = 0;
}
