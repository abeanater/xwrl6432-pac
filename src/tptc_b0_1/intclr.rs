#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `PROGRAM_SET_EMPTY` reader - 0:0\\]
Program Set Empty Event Clear:#br#INTCLR.PROGEMPTY = 0 : Writes of '0' have no effect.#br#INTCLR.PROGEMPTY = 1 : Write of '1' clears INTSTAT.PROGEMPTY bit"]
pub type ProgramSetEmptyR = crate::BitReader;
#[doc = "Field `PROGRAM_SET_EMPTY` writer - 0:0\\]
Program Set Empty Event Clear:#br#INTCLR.PROGEMPTY = 0 : Writes of '0' have no effect.#br#INTCLR.PROGEMPTY = 1 : Write of '1' clears INTSTAT.PROGEMPTY bit"]
pub type ProgramSetEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_DONE_EVENT` reader - 1:1\\]
TR Done Event Clear:#br#INTCLR.TRDONE = 0 : Writes of '0' have no effect.#br#INTCLR.TRDONE = 1 : Write of '1' clears INTSTAT.TRDONE bit"]
pub type TrDoneEventR = crate::BitReader;
#[doc = "Field `TR_DONE_EVENT` writer - 1:1\\]
TR Done Event Clear:#br#INTCLR.TRDONE = 0 : Writes of '0' have no effect.#br#INTCLR.TRDONE = 1 : Write of '1' clears INTSTAT.TRDONE bit"]
pub type TrDoneEventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Program Set Empty Event Clear:#br#INTCLR.PROGEMPTY = 0 : Writes of '0' have no effect.#br#INTCLR.PROGEMPTY = 1 : Write of '1' clears INTSTAT.PROGEMPTY bit"]
    #[inline(always)]
    pub fn program_set_empty(&self) -> ProgramSetEmptyR {
        ProgramSetEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TR Done Event Clear:#br#INTCLR.TRDONE = 0 : Writes of '0' have no effect.#br#INTCLR.TRDONE = 1 : Write of '1' clears INTSTAT.TRDONE bit"]
    #[inline(always)]
    pub fn tr_done_event(&self) -> TrDoneEventR {
        TrDoneEventR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Program Set Empty Event Clear:#br#INTCLR.PROGEMPTY = 0 : Writes of '0' have no effect.#br#INTCLR.PROGEMPTY = 1 : Write of '1' clears INTSTAT.PROGEMPTY bit"]
    #[inline(always)]
    #[must_use]
    pub fn program_set_empty(&mut self) -> ProgramSetEmptyW<IntclrSpec> {
        ProgramSetEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TR Done Event Clear:#br#INTCLR.TRDONE = 0 : Writes of '0' have no effect.#br#INTCLR.TRDONE = 1 : Write of '1' clears INTSTAT.TRDONE bit"]
    #[inline(always)]
    #[must_use]
    pub fn tr_done_event(&mut self) -> TrDoneEventW<IntclrSpec> {
        TrDoneEventW::new(self, 1)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
