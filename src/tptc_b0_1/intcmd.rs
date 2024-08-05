#[doc = "Register `INTCMD` reader"]
pub type R = crate::R<IntcmdSpec>;
#[doc = "Register `INTCMD` writer"]
pub type W = crate::W<IntcmdSpec>;
#[doc = "Field `EVALUATE_STATE_OF` reader - 0:0\\]
Evaluate state of TPTC interrupt#br#Write of '1' to EVAL causes TPTC interrupt to be pulsed if any of the INTSTAT bits are set to '1'.#br#Writes of '0' have no affect."]
pub type EvaluateStateOfR = crate::BitReader;
#[doc = "Field `EVALUATE_STATE_OF` writer - 0:0\\]
Evaluate state of TPTC interrupt#br#Write of '1' to EVAL causes TPTC interrupt to be pulsed if any of the INTSTAT bits are set to '1'.#br#Writes of '0' have no affect."]
pub type EvaluateStateOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_TPTC_INTERRUPT` reader - 1:1\\]
Set TPTC interrupt:#br#Write of '1' to SET causes TPTC interrupt to be pulsed unconditionally.#br#Writes of '0' have no affect."]
pub type SetTptcInterruptR = crate::BitReader;
#[doc = "Field `SET_TPTC_INTERRUPT` writer - 1:1\\]
Set TPTC interrupt:#br#Write of '1' to SET causes TPTC interrupt to be pulsed unconditionally.#br#Writes of '0' have no affect."]
pub type SetTptcInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Evaluate state of TPTC interrupt#br#Write of '1' to EVAL causes TPTC interrupt to be pulsed if any of the INTSTAT bits are set to '1'.#br#Writes of '0' have no affect."]
    #[inline(always)]
    pub fn evaluate_state_of(&self) -> EvaluateStateOfR {
        EvaluateStateOfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set TPTC interrupt:#br#Write of '1' to SET causes TPTC interrupt to be pulsed unconditionally.#br#Writes of '0' have no affect."]
    #[inline(always)]
    pub fn set_tptc_interrupt(&self) -> SetTptcInterruptR {
        SetTptcInterruptR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Evaluate state of TPTC interrupt#br#Write of '1' to EVAL causes TPTC interrupt to be pulsed if any of the INTSTAT bits are set to '1'.#br#Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn evaluate_state_of(&mut self) -> EvaluateStateOfW<IntcmdSpec> {
        EvaluateStateOfW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set TPTC interrupt:#br#Write of '1' to SET causes TPTC interrupt to be pulsed unconditionally.#br#Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn set_tptc_interrupt(&mut self) -> SetTptcInterruptW<IntcmdSpec> {
        SetTptcInterruptW::new(self, 1)
    }
}
#[doc = "Interrupt Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcmdSpec;
impl crate::RegisterSpec for IntcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intcmd::R`](R) reader structure"]
impl crate::Readable for IntcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`intcmd::W`](W) writer structure"]
impl crate::Writable for IntcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCMD to value 0"]
impl crate::Resettable for IntcmdSpec {
    const RESET_VALUE: u32 = 0;
}
