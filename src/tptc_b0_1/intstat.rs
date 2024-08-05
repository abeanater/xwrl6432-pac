#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `PROGRAM_SET_EMPTY` reader - 0:0\\]
Program Set Empty Event Status:#br#PROGEMPTY = 0 : Condition not detected.#br#PROGEMPTY = 1 : Set when Program Register set transitions to empty state. Cleared when user writes '1' to INTCLR.PROGEMPTY register bit."]
pub type ProgramSetEmptyR = crate::BitReader;
#[doc = "Field `PROGRAM_SET_EMPTY` writer - 0:0\\]
Program Set Empty Event Status:#br#PROGEMPTY = 0 : Condition not detected.#br#PROGEMPTY = 1 : Set when Program Register set transitions to empty state. Cleared when user writes '1' to INTCLR.PROGEMPTY register bit."]
pub type ProgramSetEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_DONE_EVENT` reader - 1:1\\]
TR Done Event Status:#br#TRDONE = 0 : Condition not detected.#br#TRDONE = 1 : Set when TC has completed a Transfer Request. TRDONE should be set when the write status is returned for the final write of a TR. Cleared when user writes '1' to INTCLR.TRDONE register bit."]
pub type TrDoneEventR = crate::BitReader;
#[doc = "Field `TR_DONE_EVENT` writer - 1:1\\]
TR Done Event Status:#br#TRDONE = 0 : Condition not detected.#br#TRDONE = 1 : Set when TC has completed a Transfer Request. TRDONE should be set when the write status is returned for the final write of a TR. Cleared when user writes '1' to INTCLR.TRDONE register bit."]
pub type TrDoneEventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Program Set Empty Event Status:#br#PROGEMPTY = 0 : Condition not detected.#br#PROGEMPTY = 1 : Set when Program Register set transitions to empty state. Cleared when user writes '1' to INTCLR.PROGEMPTY register bit."]
    #[inline(always)]
    pub fn program_set_empty(&self) -> ProgramSetEmptyR {
        ProgramSetEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TR Done Event Status:#br#TRDONE = 0 : Condition not detected.#br#TRDONE = 1 : Set when TC has completed a Transfer Request. TRDONE should be set when the write status is returned for the final write of a TR. Cleared when user writes '1' to INTCLR.TRDONE register bit."]
    #[inline(always)]
    pub fn tr_done_event(&self) -> TrDoneEventR {
        TrDoneEventR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Program Set Empty Event Status:#br#PROGEMPTY = 0 : Condition not detected.#br#PROGEMPTY = 1 : Set when Program Register set transitions to empty state. Cleared when user writes '1' to INTCLR.PROGEMPTY register bit."]
    #[inline(always)]
    #[must_use]
    pub fn program_set_empty(&mut self) -> ProgramSetEmptyW<IntstatSpec> {
        ProgramSetEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TR Done Event Status:#br#TRDONE = 0 : Condition not detected.#br#TRDONE = 1 : Set when TC has completed a Transfer Request. TRDONE should be set when the write status is returned for the final write of a TR. Cleared when user writes '1' to INTCLR.TRDONE register bit."]
    #[inline(always)]
    #[must_use]
    pub fn tr_done_event(&mut self) -> TrDoneEventW<IntstatSpec> {
        TrDoneEventW::new(self, 1)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
