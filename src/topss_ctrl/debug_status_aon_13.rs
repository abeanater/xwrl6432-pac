#[doc = "Register `DEBUG_STATUS_AON_13` reader"]
pub type R = crate::R<DebugStatusAon13Spec>;
#[doc = "Register `DEBUG_STATUS_AON_13` writer"]
pub type W = crate::W<DebugStatusAon13Spec>;
#[doc = "Field `app_pd_clkgate_en` reader - 0:0\\]
status reg for app_pd_clkgate_en"]
pub type AppPdClkgateEnR = crate::BitReader;
#[doc = "Field `app_pd_clkgate_en` writer - 0:0\\]
status reg for app_pd_clkgate_en"]
pub type AppPdClkgateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_clkgate_en` reader - 1:1\\]
status reg for fec_pd_clkgate_en"]
pub type FecPdClkgateEnR = crate::BitReader;
#[doc = "Field `fec_pd_clkgate_en` writer - 1:1\\]
status reg for fec_pd_clkgate_en"]
pub type FecPdClkgateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_clkgate_en` reader - 2:2\\]
status reg for hwa_pd_clkgate_en"]
pub type HwaPdClkgateEnR = crate::BitReader;
#[doc = "Field `hwa_pd_clkgate_en` writer - 2:2\\]
status reg for hwa_pd_clkgate_en"]
pub type HwaPdClkgateEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_clkgate_en"]
    #[inline(always)]
    pub fn app_pd_clkgate_en(&self) -> AppPdClkgateEnR {
        AppPdClkgateEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for fec_pd_clkgate_en"]
    #[inline(always)]
    pub fn fec_pd_clkgate_en(&self) -> FecPdClkgateEnR {
        FecPdClkgateEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for hwa_pd_clkgate_en"]
    #[inline(always)]
    pub fn hwa_pd_clkgate_en(&self) -> HwaPdClkgateEnR {
        HwaPdClkgateEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_clkgate_en"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_clkgate_en(&mut self) -> AppPdClkgateEnW<DebugStatusAon13Spec> {
        AppPdClkgateEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for fec_pd_clkgate_en"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_clkgate_en(&mut self) -> FecPdClkgateEnW<DebugStatusAon13Spec> {
        FecPdClkgateEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for hwa_pd_clkgate_en"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_clkgate_en(&mut self) -> HwaPdClkgateEnW<DebugStatusAon13Spec> {
        HwaPdClkgateEnW::new(self, 2)
    }
}
#[doc = "DEBUG_STATUS_AON_13\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon13Spec;
impl crate::RegisterSpec for DebugStatusAon13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_13::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon13Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_13::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_13 to value 0"]
impl crate::Resettable for DebugStatusAon13Spec {
    const RESET_VALUE: u32 = 0;
}
