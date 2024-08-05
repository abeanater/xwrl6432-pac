#[doc = "Register `INTR_ENABLE_SET` reader"]
pub type R = crate::R<IntrEnableSetSpec>;
#[doc = "Register `INTR_ENABLE_SET` writer"]
pub type W = crate::W<IntrEnableSetSpec>;
#[doc = "Field `FIRQ_ENA_SET` reader - 0:0\\]
Frame Interrupt Enable/Set Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will set interrupt enabled Writing 0 has no effect"]
pub type FirqEnaSetR = crate::BitReader;
#[doc = "Field `FIRQ_ENA_SET` writer - 0:0\\]
Frame Interrupt Enable/Set Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will set interrupt enabled Writing 0 has no effect"]
pub type FirqEnaSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIRQ_ENA_SET` reader - 1:1\\]
Word Interrupt Enable/Set Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will set interrupt enabled Writing 0 has no effect"]
pub type WirqEnaSetR = crate::BitReader;
#[doc = "Field `WIRQ_ENA_SET` writer - 1:1\\]
Word Interrupt Enable/Set Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will set interrupt enabled Writing 0 has no effect"]
pub type WirqEnaSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frame Interrupt Enable/Set Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will set interrupt enabled Writing 0 has no effect"]
    #[inline(always)]
    pub fn firq_ena_set(&self) -> FirqEnaSetR {
        FirqEnaSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Interrupt Enable/Set Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will set interrupt enabled Writing 0 has no effect"]
    #[inline(always)]
    pub fn wirq_ena_set(&self) -> WirqEnaSetR {
        WirqEnaSetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frame Interrupt Enable/Set Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will set interrupt enabled Writing 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn firq_ena_set(&mut self) -> FirqEnaSetW<IntrEnableSetSpec> {
        FirqEnaSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Interrupt Enable/Set Read indicates interrupt enable 0 = disabled 1 = enabled Writing 1 will set interrupt enabled Writing 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn wirq_ena_set(&mut self) -> WirqEnaSetW<IntrEnableSetSpec> {
        WirqEnaSetW::new(self, 1)
    }
}
#[doc = "INTR Interrupt Enable/Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEnableSetSpec;
impl crate::RegisterSpec for IntrEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_enable_set::R`](R) reader structure"]
impl crate::Readable for IntrEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_enable_set::W`](W) writer structure"]
impl crate::Writable for IntrEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_ENABLE_SET to value 0"]
impl crate::Resettable for IntrEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
