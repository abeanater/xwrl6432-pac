#[doc = "Register `ERREN` reader"]
pub type R = crate::R<ErrenSpec>;
#[doc = "Register `ERREN` writer"]
pub type W = crate::W<ErrenSpec>;
#[doc = "Field `INTERRUPT_ENABLE_FOR_2` reader - 0:0\\]
Interrupt enable for ERRSTAT.BUSERR:#br#ERREN.BUSERR = 0 : BUSERR is disabled.#br#ERREN.BUSERR = 1 : BUSERR is enabled and contributes to the TPTC error interrupt generation."]
pub type InterruptEnableFor2R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_FOR_2` writer - 0:0\\]
Interrupt enable for ERRSTAT.BUSERR:#br#ERREN.BUSERR = 0 : BUSERR is disabled.#br#ERREN.BUSERR = 1 : BUSERR is enabled and contributes to the TPTC error interrupt generation."]
pub type InterruptEnableFor2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_FOR_1` reader - 2:2\\]
Interrupt enable for ERRSTAT.TRERR:#br#ERREN.TRERR = 0 : BUSERR is disabled.#br#ERREN.TRERR = 1 : TRERR is enabled and contributes to the TPTC error interrupt generation."]
pub type InterruptEnableFor1R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_FOR_1` writer - 2:2\\]
Interrupt enable for ERRSTAT.TRERR:#br#ERREN.TRERR = 0 : BUSERR is disabled.#br#ERREN.TRERR = 1 : TRERR is enabled and contributes to the TPTC error interrupt generation."]
pub type InterruptEnableFor1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_FOR` reader - 3:3\\]
Interrupt enable for ERRSTAT.MMRAERR:#br#ERREN.MMRAERR = 0 : BUSERR is disabled.#br#ERREN.MMRAERR = 1 : MMRAERR is enabled and contributes to the TPTC error interrupt generation."]
pub type InterruptEnableForR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_FOR` writer - 3:3\\]
Interrupt enable for ERRSTAT.MMRAERR:#br#ERREN.MMRAERR = 0 : BUSERR is disabled.#br#ERREN.MMRAERR = 1 : MMRAERR is enabled and contributes to the TPTC error interrupt generation."]
pub type InterruptEnableForW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt enable for ERRSTAT.BUSERR:#br#ERREN.BUSERR = 0 : BUSERR is disabled.#br#ERREN.BUSERR = 1 : BUSERR is enabled and contributes to the TPTC error interrupt generation."]
    #[inline(always)]
    pub fn interrupt_enable_for_2(&self) -> InterruptEnableFor2R {
        InterruptEnableFor2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for ERRSTAT.TRERR:#br#ERREN.TRERR = 0 : BUSERR is disabled.#br#ERREN.TRERR = 1 : TRERR is enabled and contributes to the TPTC error interrupt generation."]
    #[inline(always)]
    pub fn interrupt_enable_for_1(&self) -> InterruptEnableFor1R {
        InterruptEnableFor1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for ERRSTAT.MMRAERR:#br#ERREN.MMRAERR = 0 : BUSERR is disabled.#br#ERREN.MMRAERR = 1 : MMRAERR is enabled and contributes to the TPTC error interrupt generation."]
    #[inline(always)]
    pub fn interrupt_enable_for(&self) -> InterruptEnableForR {
        InterruptEnableForR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt enable for ERRSTAT.BUSERR:#br#ERREN.BUSERR = 0 : BUSERR is disabled.#br#ERREN.BUSERR = 1 : BUSERR is enabled and contributes to the TPTC error interrupt generation."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_for_2(&mut self) -> InterruptEnableFor2W<ErrenSpec> {
        InterruptEnableFor2W::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for ERRSTAT.TRERR:#br#ERREN.TRERR = 0 : BUSERR is disabled.#br#ERREN.TRERR = 1 : TRERR is enabled and contributes to the TPTC error interrupt generation."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_for_1(&mut self) -> InterruptEnableFor1W<ErrenSpec> {
        InterruptEnableFor1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for ERRSTAT.MMRAERR:#br#ERREN.MMRAERR = 0 : BUSERR is disabled.#br#ERREN.MMRAERR = 1 : MMRAERR is enabled and contributes to the TPTC error interrupt generation."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_for(&mut self) -> InterruptEnableForW<ErrenSpec> {
        InterruptEnableForW::new(self, 3)
    }
}
#[doc = "Error Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrenSpec;
impl crate::RegisterSpec for ErrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erren::R`](R) reader structure"]
impl crate::Readable for ErrenSpec {}
#[doc = "`write(|w| ..)` method takes [`erren::W`](W) writer structure"]
impl crate::Writable for ErrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERREN to value 0"]
impl crate::Resettable for ErrenSpec {
    const RESET_VALUE: u32 = 0;
}
