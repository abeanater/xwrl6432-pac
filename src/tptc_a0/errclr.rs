#[doc = "Register `ERRCLR` reader"]
pub type R = crate::R<ErrclrSpec>;
#[doc = "Register `ERRCLR` writer"]
pub type W = crate::W<ErrclrSpec>;
#[doc = "Field `INTERRUPT_CLEAR_FOR_2` reader - 0:0\\]
Interrupt clear for ERRSTAT.BUSERR:#br#ERRCLR.BUSERR = 0 : Writes of '0' have no effect.#br#ERRCLR.BUSERR = 1 : Write of '1' clears ERRSTAT.BUSERR bit. Write of '1' to ERRCLR.BUSERR clears the ERRDET register."]
pub type InterruptClearFor2R = crate::BitReader;
#[doc = "Field `INTERRUPT_CLEAR_FOR_2` writer - 0:0\\]
Interrupt clear for ERRSTAT.BUSERR:#br#ERRCLR.BUSERR = 0 : Writes of '0' have no effect.#br#ERRCLR.BUSERR = 1 : Write of '1' clears ERRSTAT.BUSERR bit. Write of '1' to ERRCLR.BUSERR clears the ERRDET register."]
pub type InterruptClearFor2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_CLEAR_FOR_1` reader - 2:2\\]
Interrupt clear for ERRSTAT.TRERR:#br#ERRCLR.TRERR = 0 : Writes of '0' have no effect.#br#ERRCLR.TRERR = 1 : Write of '1' clears ERRSTAT.TRERR bit. Write of '1' to ERRCLR.TRERR does not clear the ERRDET register."]
pub type InterruptClearFor1R = crate::BitReader;
#[doc = "Field `INTERRUPT_CLEAR_FOR_1` writer - 2:2\\]
Interrupt clear for ERRSTAT.TRERR:#br#ERRCLR.TRERR = 0 : Writes of '0' have no effect.#br#ERRCLR.TRERR = 1 : Write of '1' clears ERRSTAT.TRERR bit. Write of '1' to ERRCLR.TRERR does not clear the ERRDET register."]
pub type InterruptClearFor1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_CLEAR_FOR` reader - 3:3\\]
Interrupt clear for ERRSTAT.MMRAERR:#br#ERRCLR.MMRAERR = 0 : Writes of '0' have no effect.#br#ERRCLR.MMRAERR = 1 : Write of '1' clears ERRSTAT.MMRAERR bit. Write of '1' to ERRCLR.MMRAERR does not clear the ERRDET register."]
pub type InterruptClearForR = crate::BitReader;
#[doc = "Field `INTERRUPT_CLEAR_FOR` writer - 3:3\\]
Interrupt clear for ERRSTAT.MMRAERR:#br#ERRCLR.MMRAERR = 0 : Writes of '0' have no effect.#br#ERRCLR.MMRAERR = 1 : Write of '1' clears ERRSTAT.MMRAERR bit. Write of '1' to ERRCLR.MMRAERR does not clear the ERRDET register."]
pub type InterruptClearForW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt clear for ERRSTAT.BUSERR:#br#ERRCLR.BUSERR = 0 : Writes of '0' have no effect.#br#ERRCLR.BUSERR = 1 : Write of '1' clears ERRSTAT.BUSERR bit. Write of '1' to ERRCLR.BUSERR clears the ERRDET register."]
    #[inline(always)]
    pub fn interrupt_clear_for_2(&self) -> InterruptClearFor2R {
        InterruptClearFor2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt clear for ERRSTAT.TRERR:#br#ERRCLR.TRERR = 0 : Writes of '0' have no effect.#br#ERRCLR.TRERR = 1 : Write of '1' clears ERRSTAT.TRERR bit. Write of '1' to ERRCLR.TRERR does not clear the ERRDET register."]
    #[inline(always)]
    pub fn interrupt_clear_for_1(&self) -> InterruptClearFor1R {
        InterruptClearFor1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt clear for ERRSTAT.MMRAERR:#br#ERRCLR.MMRAERR = 0 : Writes of '0' have no effect.#br#ERRCLR.MMRAERR = 1 : Write of '1' clears ERRSTAT.MMRAERR bit. Write of '1' to ERRCLR.MMRAERR does not clear the ERRDET register."]
    #[inline(always)]
    pub fn interrupt_clear_for(&self) -> InterruptClearForR {
        InterruptClearForR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt clear for ERRSTAT.BUSERR:#br#ERRCLR.BUSERR = 0 : Writes of '0' have no effect.#br#ERRCLR.BUSERR = 1 : Write of '1' clears ERRSTAT.BUSERR bit. Write of '1' to ERRCLR.BUSERR clears the ERRDET register."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_clear_for_2(&mut self) -> InterruptClearFor2W<ErrclrSpec> {
        InterruptClearFor2W::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt clear for ERRSTAT.TRERR:#br#ERRCLR.TRERR = 0 : Writes of '0' have no effect.#br#ERRCLR.TRERR = 1 : Write of '1' clears ERRSTAT.TRERR bit. Write of '1' to ERRCLR.TRERR does not clear the ERRDET register."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_clear_for_1(&mut self) -> InterruptClearFor1W<ErrclrSpec> {
        InterruptClearFor1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt clear for ERRSTAT.MMRAERR:#br#ERRCLR.MMRAERR = 0 : Writes of '0' have no effect.#br#ERRCLR.MMRAERR = 1 : Write of '1' clears ERRSTAT.MMRAERR bit. Write of '1' to ERRCLR.MMRAERR does not clear the ERRDET register."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_clear_for(&mut self) -> InterruptClearForW<ErrclrSpec> {
        InterruptClearForW::new(self, 3)
    }
}
#[doc = "Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrclrSpec;
impl crate::RegisterSpec for ErrclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errclr::R`](R) reader structure"]
impl crate::Readable for ErrclrSpec {}
#[doc = "`write(|w| ..)` method takes [`errclr::W`](W) writer structure"]
impl crate::Writable for ErrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRCLR to value 0"]
impl crate::Resettable for ErrclrSpec {
    const RESET_VALUE: u32 = 0;
}
