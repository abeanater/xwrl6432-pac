#[doc = "Register `ONEMCU_CTI_ASICCTL` reader"]
pub type R = crate::R<OnemcuCtiAsicctlSpec>;
#[doc = "Register `ONEMCU_CTI_ASICCTL` writer"]
pub type W = crate::W<OnemcuCtiAsicctlSpec>;
#[doc = "Field `ONEMCU_CTI_ASICCTL` reader - "]
pub type OnemcuCtiAsicctlR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ASICCTL` writer - "]
pub type OnemcuCtiAsicctlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_asicctl(&self) -> OnemcuCtiAsicctlR {
        OnemcuCtiAsicctlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_asicctl(&mut self) -> OnemcuCtiAsicctlW<OnemcuCtiAsicctlSpec> {
        OnemcuCtiAsicctlW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ASICCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_asicctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_asicctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiAsicctlSpec;
impl crate::RegisterSpec for OnemcuCtiAsicctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_asicctl::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiAsicctlSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_asicctl::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiAsicctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ASICCTL to value 0"]
impl crate::Resettable for OnemcuCtiAsicctlSpec {
    const RESET_VALUE: u32 = 0;
}
