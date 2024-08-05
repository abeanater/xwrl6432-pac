#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `PROGRAM_SET_EMPTY` reader - 0:0\\]
Program Set Empty Event Enable:#br#INTEN.PROGEMPTY = 0 : PROGEMPTY Event is disabled.#br#INTEN.PROGEMPTY = 1 : PROGEMPTY Event is enabled and contributes to interrupt generation"]
pub type ProgramSetEmptyR = crate::BitReader;
#[doc = "Field `PROGRAM_SET_EMPTY` writer - 0:0\\]
Program Set Empty Event Enable:#br#INTEN.PROGEMPTY = 0 : PROGEMPTY Event is disabled.#br#INTEN.PROGEMPTY = 1 : PROGEMPTY Event is enabled and contributes to interrupt generation"]
pub type ProgramSetEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_DONE_EVENT` reader - 1:1\\]
TR Done Event Enable:#br#INTEN.TRDONE = 0 : TRDONE Event is disabled.#br#INTEN.TRDONE = 1 : TRDONE Event is enabled and contributes to interrupt generation"]
pub type TrDoneEventR = crate::BitReader;
#[doc = "Field `TR_DONE_EVENT` writer - 1:1\\]
TR Done Event Enable:#br#INTEN.TRDONE = 0 : TRDONE Event is disabled.#br#INTEN.TRDONE = 1 : TRDONE Event is enabled and contributes to interrupt generation"]
pub type TrDoneEventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Program Set Empty Event Enable:#br#INTEN.PROGEMPTY = 0 : PROGEMPTY Event is disabled.#br#INTEN.PROGEMPTY = 1 : PROGEMPTY Event is enabled and contributes to interrupt generation"]
    #[inline(always)]
    pub fn program_set_empty(&self) -> ProgramSetEmptyR {
        ProgramSetEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TR Done Event Enable:#br#INTEN.TRDONE = 0 : TRDONE Event is disabled.#br#INTEN.TRDONE = 1 : TRDONE Event is enabled and contributes to interrupt generation"]
    #[inline(always)]
    pub fn tr_done_event(&self) -> TrDoneEventR {
        TrDoneEventR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Program Set Empty Event Enable:#br#INTEN.PROGEMPTY = 0 : PROGEMPTY Event is disabled.#br#INTEN.PROGEMPTY = 1 : PROGEMPTY Event is enabled and contributes to interrupt generation"]
    #[inline(always)]
    #[must_use]
    pub fn program_set_empty(&mut self) -> ProgramSetEmptyW<IntenSpec> {
        ProgramSetEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TR Done Event Enable:#br#INTEN.TRDONE = 0 : TRDONE Event is disabled.#br#INTEN.TRDONE = 1 : TRDONE Event is enabled and contributes to interrupt generation"]
    #[inline(always)]
    #[must_use]
    pub fn tr_done_event(&mut self) -> TrDoneEventW<IntenSpec> {
        TrDoneEventW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
