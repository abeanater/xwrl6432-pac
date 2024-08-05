#[doc = "Register `INTR_STATUS_RAW_SET` reader"]
pub type R = crate::R<IntrStatusRawSetSpec>;
#[doc = "Register `INTR_STATUS_RAW_SET` writer"]
pub type W = crate::W<IntrStatusRawSetSpec>;
#[doc = "Field `FIRQ_RAW` reader - 0:0\\]
Frame Interrupt Status Read indicates raw status 0 = inactive 1 = active Writing 1 will set status Writing 0 has no effect"]
pub type FirqRawR = crate::BitReader;
#[doc = "Field `FIRQ_RAW` writer - 0:0\\]
Frame Interrupt Status Read indicates raw status 0 = inactive 1 = active Writing 1 will set status Writing 0 has no effect"]
pub type FirqRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIRQ_RAW` reader - 1:1\\]
Word Interrupt Status Read indicates raw status 0 = inactive 1 = active Writing 1 will set status Writing 0 has no effect"]
pub type WirqRawR = crate::BitReader;
#[doc = "Field `WIRQ_RAW` writer - 1:1\\]
Word Interrupt Status Read indicates raw status 0 = inactive 1 = active Writing 1 will set status Writing 0 has no effect"]
pub type WirqRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frame Interrupt Status Read indicates raw status 0 = inactive 1 = active Writing 1 will set status Writing 0 has no effect"]
    #[inline(always)]
    pub fn firq_raw(&self) -> FirqRawR {
        FirqRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Interrupt Status Read indicates raw status 0 = inactive 1 = active Writing 1 will set status Writing 0 has no effect"]
    #[inline(always)]
    pub fn wirq_raw(&self) -> WirqRawR {
        WirqRawR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frame Interrupt Status Read indicates raw status 0 = inactive 1 = active Writing 1 will set status Writing 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn firq_raw(&mut self) -> FirqRawW<IntrStatusRawSetSpec> {
        FirqRawW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Interrupt Status Read indicates raw status 0 = inactive 1 = active Writing 1 will set status Writing 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn wirq_raw(&mut self) -> WirqRawW<IntrStatusRawSetSpec> {
        WirqRawW::new(self, 1)
    }
}
#[doc = "INTR Interrupt Status Raw/Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_status_raw_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_status_raw_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrStatusRawSetSpec;
impl crate::RegisterSpec for IntrStatusRawSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status_raw_set::R`](R) reader structure"]
impl crate::Readable for IntrStatusRawSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_status_raw_set::W`](W) writer structure"]
impl crate::Writable for IntrStatusRawSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_STATUS_RAW_SET to value 0"]
impl crate::Resettable for IntrStatusRawSetSpec {
    const RESET_VALUE: u32 = 0;
}
