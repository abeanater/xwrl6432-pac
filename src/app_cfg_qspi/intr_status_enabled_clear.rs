#[doc = "Register `INTR_STATUS_ENABLED_CLEAR` reader"]
pub type R = crate::R<IntrStatusEnabledClearSpec>;
#[doc = "Register `INTR_STATUS_ENABLED_CLEAR` writer"]
pub type W = crate::W<IntrStatusEnabledClearSpec>;
#[doc = "Field `FIRQ_ENA` reader - 0:0\\]
Frame Interrupt Enabled Status Read indicates enabled status 0 = inactive 1 = active Writing 1 will clear interrupt Writing 0 has no effect"]
pub type FirqEnaR = crate::BitReader;
#[doc = "Field `FIRQ_ENA` writer - 0:0\\]
Frame Interrupt Enabled Status Read indicates enabled status 0 = inactive 1 = active Writing 1 will clear interrupt Writing 0 has no effect"]
pub type FirqEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIRQ_ENA` reader - 1:1\\]
Word Interrupt Enabled Status Read indicates enabled status 0 = inactive 1 = active Writing 1 will clear interrupt Writing 0 has no effect"]
pub type WirqEnaR = crate::BitReader;
#[doc = "Field `WIRQ_ENA` writer - 1:1\\]
Word Interrupt Enabled Status Read indicates enabled status 0 = inactive 1 = active Writing 1 will clear interrupt Writing 0 has no effect"]
pub type WirqEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frame Interrupt Enabled Status Read indicates enabled status 0 = inactive 1 = active Writing 1 will clear interrupt Writing 0 has no effect"]
    #[inline(always)]
    pub fn firq_ena(&self) -> FirqEnaR {
        FirqEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Interrupt Enabled Status Read indicates enabled status 0 = inactive 1 = active Writing 1 will clear interrupt Writing 0 has no effect"]
    #[inline(always)]
    pub fn wirq_ena(&self) -> WirqEnaR {
        WirqEnaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frame Interrupt Enabled Status Read indicates enabled status 0 = inactive 1 = active Writing 1 will clear interrupt Writing 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn firq_ena(&mut self) -> FirqEnaW<IntrStatusEnabledClearSpec> {
        FirqEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Interrupt Enabled Status Read indicates enabled status 0 = inactive 1 = active Writing 1 will clear interrupt Writing 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn wirq_ena(&mut self) -> WirqEnaW<IntrStatusEnabledClearSpec> {
        WirqEnaW::new(self, 1)
    }
}
#[doc = "INTR Interrupt Status Enabled/Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_status_enabled_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_status_enabled_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrStatusEnabledClearSpec;
impl crate::RegisterSpec for IntrStatusEnabledClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status_enabled_clear::R`](R) reader structure"]
impl crate::Readable for IntrStatusEnabledClearSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_status_enabled_clear::W`](W) writer structure"]
impl crate::Writable for IntrStatusEnabledClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_STATUS_ENABLED_CLEAR to value 0"]
impl crate::Resettable for IntrStatusEnabledClearSpec {
    const RESET_VALUE: u32 = 0;
}
