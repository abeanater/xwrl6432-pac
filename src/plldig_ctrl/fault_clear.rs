#[doc = "Register `fault_clear` reader"]
pub type R = crate::R<FaultClearSpec>;
#[doc = "Register `fault_clear` writer"]
pub type W = crate::W<FaultClearSpec>;
#[doc = "Field `FAULT_CLEAR__WRITING` reader - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
pub type FaultClear_WritingR = crate::BitReader;
#[doc = "Field `FAULT_CLEAR__WRITING` writer - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
pub type FaultClear_WritingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn fault_clear__writing(&self) -> FaultClear_WritingR {
        FaultClear_WritingR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn fault_clear__writing(&mut self) -> FaultClear_WritingW<FaultClearSpec> {
        FaultClear_WritingW::new(self, 0)
    }
}
#[doc = "Fault Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultClearSpec;
impl crate::RegisterSpec for FaultClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_clear::R`](R) reader structure"]
impl crate::Readable for FaultClearSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_clear::W`](W) writer structure"]
impl crate::Writable for FaultClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fault_clear to value 0"]
impl crate::Resettable for FaultClearSpec {
    const RESET_VALUE: u32 = 0;
}
