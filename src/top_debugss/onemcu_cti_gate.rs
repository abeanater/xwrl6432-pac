#[doc = "Register `ONEMCU_CTI_GATE` reader"]
pub type R = crate::R<OnemcuCtiGateSpec>;
#[doc = "Register `ONEMCU_CTI_GATE` writer"]
pub type W = crate::W<OnemcuCtiGateSpec>;
#[doc = "Field `ONEMCU_CTI_GATE` reader - "]
pub type OnemcuCtiGateR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_GATE` writer - "]
pub type OnemcuCtiGateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_gate(&self) -> OnemcuCtiGateR {
        OnemcuCtiGateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_gate(&mut self) -> OnemcuCtiGateW<OnemcuCtiGateSpec> {
        OnemcuCtiGateW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiGateSpec;
impl crate::RegisterSpec for OnemcuCtiGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_gate::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiGateSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_gate::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_GATE to value 0"]
impl crate::Resettable for OnemcuCtiGateSpec {
    const RESET_VALUE: u32 = 0;
}
