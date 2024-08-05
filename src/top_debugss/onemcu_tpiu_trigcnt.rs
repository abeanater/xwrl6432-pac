#[doc = "Register `ONEMCU_TPIU_TRIGCNT` reader"]
pub type R = crate::R<OnemcuTpiuTrigcntSpec>;
#[doc = "Register `ONEMCU_TPIU_TRIGCNT` writer"]
pub type W = crate::W<OnemcuTpiuTrigcntSpec>;
#[doc = "Field `ONEMCU_TPIU_TRIGCNT` reader - 31:0\\]
Trigger counter value"]
pub type OnemcuTpiuTrigcntR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_TRIGCNT` writer - 31:0\\]
Trigger counter value"]
pub type OnemcuTpiuTrigcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Trigger counter value"]
    #[inline(always)]
    pub fn onemcu_tpiu_trigcnt(&self) -> OnemcuTpiuTrigcntR {
        OnemcuTpiuTrigcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Trigger counter value"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_trigcnt(&mut self) -> OnemcuTpiuTrigcntW<OnemcuTpiuTrigcntSpec> {
        OnemcuTpiuTrigcntW::new(self, 0)
    }
}
#[doc = "Trigger counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_trigcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_trigcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuTrigcntSpec;
impl crate::RegisterSpec for OnemcuTpiuTrigcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_trigcnt::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuTrigcntSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_trigcnt::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuTrigcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_TRIGCNT to value 0"]
impl crate::Resettable for OnemcuTpiuTrigcntSpec {
    const RESET_VALUE: u32 = 0;
}
