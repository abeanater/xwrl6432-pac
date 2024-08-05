#[doc = "Register `INTR_ENABLE_CLEAR` reader"]
pub type R = crate::R<IntrEnableClearSpec>;
#[doc = "Register `INTR_ENABLE_CLEAR` writer"]
pub type W = crate::W<IntrEnableClearSpec>;
#[doc = "Field `FIRQ_ENA_CLR` reader - 0:0\\]
Frame Interrupt Enable/Clear Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will clear interrupt enabled Writing 0 has no effect"]
pub type FirqEnaClrR = crate::BitReader;
#[doc = "Field `FIRQ_ENA_CLR` writer - 0:0\\]
Frame Interrupt Enable/Clear Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will clear interrupt enabled Writing 0 has no effect"]
pub type FirqEnaClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIRQ_ENA_CLR` reader - 1:1\\]
Word Interrupt Enable/Clear Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will clear interrupt enabled Writing 0 has no effect"]
pub type WirqEnaClrR = crate::BitReader;
#[doc = "Field `WIRQ_ENA_CLR` writer - 1:1\\]
Word Interrupt Enable/Clear Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will clear interrupt enabled Writing 0 has no effect"]
pub type WirqEnaClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frame Interrupt Enable/Clear Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will clear interrupt enabled Writing 0 has no effect"]
    #[inline(always)]
    pub fn firq_ena_clr(&self) -> FirqEnaClrR {
        FirqEnaClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Interrupt Enable/Clear Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will clear interrupt enabled Writing 0 has no effect"]
    #[inline(always)]
    pub fn wirq_ena_clr(&self) -> WirqEnaClrR {
        WirqEnaClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frame Interrupt Enable/Clear Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will clear interrupt enabled Writing 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn firq_ena_clr(&mut self) -> FirqEnaClrW<IntrEnableClearSpec> {
        FirqEnaClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Interrupt Enable/Clear Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will clear interrupt enabled Writing 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn wirq_ena_clr(&mut self) -> WirqEnaClrW<IntrEnableClearSpec> {
        WirqEnaClrW::new(self, 1)
    }
}
#[doc = "INTR Interrupt Enable/Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEnableClearSpec;
impl crate::RegisterSpec for IntrEnableClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_enable_clear::R`](R) reader structure"]
impl crate::Readable for IntrEnableClearSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_enable_clear::W`](W) writer structure"]
impl crate::Writable for IntrEnableClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_ENABLE_CLEAR to value 0"]
impl crate::Resettable for IntrEnableClearSpec {
    const RESET_VALUE: u32 = 0;
}
