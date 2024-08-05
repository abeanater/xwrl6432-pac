#[doc = "Register `CFG_TIMEOUT_PCR3` reader"]
pub type R = crate::R<CfgTimeoutPcr3Spec>;
#[doc = "Register `CFG_TIMEOUT_PCR3` writer"]
pub type W = crate::W<CfgTimeoutPcr3Spec>;
#[doc = "Field `value` reader - 31:0\\]
PCR3Timeout Value"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `value` writer - 31:0\\]
PCR3Timeout Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
PCR3Timeout Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
PCR3Timeout Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<CfgTimeoutPcr3Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "CFG_TIMEOUT_PCR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_timeout_pcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_timeout_pcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTimeoutPcr3Spec;
impl crate::RegisterSpec for CfgTimeoutPcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_timeout_pcr3::R`](R) reader structure"]
impl crate::Readable for CfgTimeoutPcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_timeout_pcr3::W`](W) writer structure"]
impl crate::Writable for CfgTimeoutPcr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TIMEOUT_PCR3 to value 0"]
impl crate::Resettable for CfgTimeoutPcr3Spec {
    const RESET_VALUE: u32 = 0;
}
