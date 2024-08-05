#[doc = "Register `DEBUG_STATUS_AON_14` reader"]
pub type R = crate::R<DebugStatusAon14Spec>;
#[doc = "Register `DEBUG_STATUS_AON_14` writer"]
pub type W = crate::W<DebugStatusAon14Spec>;
#[doc = "Field `app_pd_por_rstn` reader - 0:0\\]
status reg for app_pd_por_rstn"]
pub type AppPdPorRstnR = crate::BitReader;
#[doc = "Field `app_pd_por_rstn` writer - 0:0\\]
status reg for app_pd_por_rstn"]
pub type AppPdPorRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_por_rstn` reader - 1:1\\]
status reg for fec_pd_por_rstn"]
pub type FecPdPorRstnR = crate::BitReader;
#[doc = "Field `fec_pd_por_rstn` writer - 1:1\\]
status reg for fec_pd_por_rstn"]
pub type FecPdPorRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_por_rstn` reader - 2:2\\]
status reg for hwa_pd_por_rstn"]
pub type HwaPdPorRstnR = crate::BitReader;
#[doc = "Field `hwa_pd_por_rstn` writer - 2:2\\]
status reg for hwa_pd_por_rstn"]
pub type HwaPdPorRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_por_rstn` reader - 3:3\\]
status reg for test_dbg_pd_por_rstn"]
pub type TestDbgPdPorRstnR = crate::BitReader;
#[doc = "Field `test_dbg_pd_por_rstn` writer - 3:3\\]
status reg for test_dbg_pd_por_rstn"]
pub type TestDbgPdPorRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_warm_rstn` reader - 4:4\\]
status reg for app_pd_warm_rstn"]
pub type AppPdWarmRstnR = crate::BitReader;
#[doc = "Field `app_pd_warm_rstn` writer - 4:4\\]
status reg for app_pd_warm_rstn"]
pub type AppPdWarmRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_warm_rstn` reader - 5:5\\]
status reg for fec_pd_warm_rstn"]
pub type FecPdWarmRstnR = crate::BitReader;
#[doc = "Field `fec_pd_warm_rstn` writer - 5:5\\]
status reg for fec_pd_warm_rstn"]
pub type FecPdWarmRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_warm_rstn` reader - 6:6\\]
status reg for hwa_pd_warm_rstn"]
pub type HwaPdWarmRstnR = crate::BitReader;
#[doc = "Field `hwa_pd_warm_rstn` writer - 6:6\\]
status reg for hwa_pd_warm_rstn"]
pub type HwaPdWarmRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_core_rstn` reader - 7:7\\]
status reg for app_pd_core_rstn"]
pub type AppPdCoreRstnR = crate::BitReader;
#[doc = "Field `app_pd_core_rstn` writer - 7:7\\]
status reg for app_pd_core_rstn"]
pub type AppPdCoreRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_core_rstn` reader - 8:8\\]
status reg for fec_pd_core_rstn"]
pub type FecPdCoreRstnR = crate::BitReader;
#[doc = "Field `fec_pd_core_rstn` writer - 8:8\\]
status reg for fec_pd_core_rstn"]
pub type FecPdCoreRstnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_por_rstn"]
    #[inline(always)]
    pub fn app_pd_por_rstn(&self) -> AppPdPorRstnR {
        AppPdPorRstnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for fec_pd_por_rstn"]
    #[inline(always)]
    pub fn fec_pd_por_rstn(&self) -> FecPdPorRstnR {
        FecPdPorRstnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for hwa_pd_por_rstn"]
    #[inline(always)]
    pub fn hwa_pd_por_rstn(&self) -> HwaPdPorRstnR {
        HwaPdPorRstnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for test_dbg_pd_por_rstn"]
    #[inline(always)]
    pub fn test_dbg_pd_por_rstn(&self) -> TestDbgPdPorRstnR {
        TestDbgPdPorRstnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for app_pd_warm_rstn"]
    #[inline(always)]
    pub fn app_pd_warm_rstn(&self) -> AppPdWarmRstnR {
        AppPdWarmRstnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for fec_pd_warm_rstn"]
    #[inline(always)]
    pub fn fec_pd_warm_rstn(&self) -> FecPdWarmRstnR {
        FecPdWarmRstnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for hwa_pd_warm_rstn"]
    #[inline(always)]
    pub fn hwa_pd_warm_rstn(&self) -> HwaPdWarmRstnR {
        HwaPdWarmRstnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for app_pd_core_rstn"]
    #[inline(always)]
    pub fn app_pd_core_rstn(&self) -> AppPdCoreRstnR {
        AppPdCoreRstnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for fec_pd_core_rstn"]
    #[inline(always)]
    pub fn fec_pd_core_rstn(&self) -> FecPdCoreRstnR {
        FecPdCoreRstnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_por_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_por_rstn(&mut self) -> AppPdPorRstnW<DebugStatusAon14Spec> {
        AppPdPorRstnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for fec_pd_por_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_por_rstn(&mut self) -> FecPdPorRstnW<DebugStatusAon14Spec> {
        FecPdPorRstnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for hwa_pd_por_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_por_rstn(&mut self) -> HwaPdPorRstnW<DebugStatusAon14Spec> {
        HwaPdPorRstnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for test_dbg_pd_por_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_por_rstn(&mut self) -> TestDbgPdPorRstnW<DebugStatusAon14Spec> {
        TestDbgPdPorRstnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for app_pd_warm_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_warm_rstn(&mut self) -> AppPdWarmRstnW<DebugStatusAon14Spec> {
        AppPdWarmRstnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for fec_pd_warm_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_warm_rstn(&mut self) -> FecPdWarmRstnW<DebugStatusAon14Spec> {
        FecPdWarmRstnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for hwa_pd_warm_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_warm_rstn(&mut self) -> HwaPdWarmRstnW<DebugStatusAon14Spec> {
        HwaPdWarmRstnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for app_pd_core_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_core_rstn(&mut self) -> AppPdCoreRstnW<DebugStatusAon14Spec> {
        AppPdCoreRstnW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for fec_pd_core_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_core_rstn(&mut self) -> FecPdCoreRstnW<DebugStatusAon14Spec> {
        FecPdCoreRstnW::new(self, 8)
    }
}
#[doc = "DEBUG_STATUS_AON_14\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon14Spec;
impl crate::RegisterSpec for DebugStatusAon14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_14::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon14Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_14::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_14 to value 0"]
impl crate::Resettable for DebugStatusAon14Spec {
    const RESET_VALUE: u32 = 0;
}
