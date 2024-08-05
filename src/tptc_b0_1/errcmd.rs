#[doc = "Register `ERRCMD` reader"]
pub type R = crate::R<ErrcmdSpec>;
#[doc = "Register `ERRCMD` writer"]
pub type W = crate::W<ErrcmdSpec>;
#[doc = "Field `EVALUATE_STATE_OF` reader - 0:0\\]
Evaluate state of TPTC error interrupt#br#Write of '1' to EVAL causes TPTC error interrupt to be pulsed if any of the ERRSTAT bits are set to '1'.#br#Writes of '0' have no affect."]
pub type EvaluateStateOfR = crate::BitReader;
#[doc = "Field `EVALUATE_STATE_OF` writer - 0:0\\]
Evaluate state of TPTC error interrupt#br#Write of '1' to EVAL causes TPTC error interrupt to be pulsed if any of the ERRSTAT bits are set to '1'.#br#Writes of '0' have no affect."]
pub type EvaluateStateOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_TPTC_ERROR` reader - 1:1\\]
Set TPTC error interrupt:#br#Write of '1' to SET causes TPTC error interrupt to be pulsed unconditionally.#br#Writes of '0' have no affect."]
pub type SetTptcErrorR = crate::BitReader;
#[doc = "Field `SET_TPTC_ERROR` writer - 1:1\\]
Set TPTC error interrupt:#br#Write of '1' to SET causes TPTC error interrupt to be pulsed unconditionally.#br#Writes of '0' have no affect."]
pub type SetTptcErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Evaluate state of TPTC error interrupt#br#Write of '1' to EVAL causes TPTC error interrupt to be pulsed if any of the ERRSTAT bits are set to '1'.#br#Writes of '0' have no affect."]
    #[inline(always)]
    pub fn evaluate_state_of(&self) -> EvaluateStateOfR {
        EvaluateStateOfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set TPTC error interrupt:#br#Write of '1' to SET causes TPTC error interrupt to be pulsed unconditionally.#br#Writes of '0' have no affect."]
    #[inline(always)]
    pub fn set_tptc_error(&self) -> SetTptcErrorR {
        SetTptcErrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Evaluate state of TPTC error interrupt#br#Write of '1' to EVAL causes TPTC error interrupt to be pulsed if any of the ERRSTAT bits are set to '1'.#br#Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn evaluate_state_of(&mut self) -> EvaluateStateOfW<ErrcmdSpec> {
        EvaluateStateOfW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set TPTC error interrupt:#br#Write of '1' to SET causes TPTC error interrupt to be pulsed unconditionally.#br#Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn set_tptc_error(&mut self) -> SetTptcErrorW<ErrcmdSpec> {
        SetTptcErrorW::new(self, 1)
    }
}
#[doc = "Error Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrcmdSpec;
impl crate::RegisterSpec for ErrcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errcmd::R`](R) reader structure"]
impl crate::Readable for ErrcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`errcmd::W`](W) writer structure"]
impl crate::Writable for ErrcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRCMD to value 0"]
impl crate::Resettable for ErrcmdSpec {
    const RESET_VALUE: u32 = 0;
}
