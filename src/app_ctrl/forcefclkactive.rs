#[doc = "Register `FORCEFCLKACTIVE` reader"]
pub type R = crate::R<ForcefclkactiveSpec>;
#[doc = "Register `FORCEFCLKACTIVE` writer"]
pub type W = crate::W<ForcefclkactiveSpec>;
#[doc = "Field `forcefclkactive` reader - 0:0\\]
1 :> Forces FCLK to be active and inhibits CM4 Entering CPU DeepSleep mode 0 :> Allows gating of FCLK based on CPU DEEPSLEEP entry mechanism"]
pub type ForcefclkactiveR = crate::BitReader;
#[doc = "Field `forcefclkactive` writer - 0:0\\]
1 :> Forces FCLK to be active and inhibits CM4 Entering CPU DeepSleep mode 0 :> Allows gating of FCLK based on CPU DEEPSLEEP entry mechanism"]
pub type ForcefclkactiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1 :> Forces FCLK to be active and inhibits CM4 Entering CPU DeepSleep mode 0 :> Allows gating of FCLK based on CPU DEEPSLEEP entry mechanism"]
    #[inline(always)]
    pub fn forcefclkactive(&self) -> ForcefclkactiveR {
        ForcefclkactiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1 :> Forces FCLK to be active and inhibits CM4 Entering CPU DeepSleep mode 0 :> Allows gating of FCLK based on CPU DEEPSLEEP entry mechanism"]
    #[inline(always)]
    #[must_use]
    pub fn forcefclkactive(&mut self) -> ForcefclkactiveW<ForcefclkactiveSpec> {
        ForcefclkactiveW::new(self, 0)
    }
}
#[doc = "FORCEFCLKACTIVE\n\nYou can [`read`](crate::Reg::read) this register and get [`forcefclkactive::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`forcefclkactive::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForcefclkactiveSpec;
impl crate::RegisterSpec for ForcefclkactiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`forcefclkactive::R`](R) reader structure"]
impl crate::Readable for ForcefclkactiveSpec {}
#[doc = "`write(|w| ..)` method takes [`forcefclkactive::W`](W) writer structure"]
impl crate::Writable for ForcefclkactiveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FORCEFCLKACTIVE to value 0"]
impl crate::Resettable for ForcefclkactiveSpec {
    const RESET_VALUE: u32 = 0;
}
