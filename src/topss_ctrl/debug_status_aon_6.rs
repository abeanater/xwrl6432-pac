#[doc = "Register `DEBUG_STATUS_AON_6` reader"]
pub type R = crate::R<DebugStatusAon6Spec>;
#[doc = "Register `DEBUG_STATUS_AON_6` writer"]
pub type W = crate::W<DebugStatusAon6Spec>;
#[doc = "Field `app_pd_is_sleep` reader - 0:0\\]
status reg for app_pd_is_sleep"]
pub type AppPdIsSleepR = crate::BitReader;
#[doc = "Field `app_pd_is_sleep` writer - 0:0\\]
status reg for app_pd_is_sleep"]
pub type AppPdIsSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_is_sleep` reader - 1:1\\]
status reg for fec_pd_is_sleep"]
pub type FecPdIsSleepR = crate::BitReader;
#[doc = "Field `fec_pd_is_sleep` writer - 1:1\\]
status reg for fec_pd_is_sleep"]
pub type FecPdIsSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_is_sleep` reader - 2:2\\]
status reg for hwa_pd_is_sleep"]
pub type HwaPdIsSleepR = crate::BitReader;
#[doc = "Field `hwa_pd_is_sleep` writer - 2:2\\]
status reg for hwa_pd_is_sleep"]
pub type HwaPdIsSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_is_sleep` reader - 3:3\\]
status reg for test_dbg_pd_is_sleep"]
pub type TestDbgPdIsSleepR = crate::BitReader;
#[doc = "Field `test_dbg_pd_is_sleep` writer - 3:3\\]
status reg for test_dbg_pd_is_sleep"]
pub type TestDbgPdIsSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_pwr_req` reader - 4:4\\]
status reg for app_pd_pwr_req"]
pub type AppPdPwrReqR = crate::BitReader;
#[doc = "Field `app_pd_pwr_req` writer - 4:4\\]
status reg for app_pd_pwr_req"]
pub type AppPdPwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_pwr_req` reader - 5:5\\]
status reg for fec_pd_pwr_req"]
pub type FecPdPwrReqR = crate::BitReader;
#[doc = "Field `fec_pd_pwr_req` writer - 5:5\\]
status reg for fec_pd_pwr_req"]
pub type FecPdPwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_pwr_req` reader - 6:6\\]
status reg for hwa_pd_pwr_req"]
pub type HwaPdPwrReqR = crate::BitReader;
#[doc = "Field `hwa_pd_pwr_req` writer - 6:6\\]
status reg for hwa_pd_pwr_req"]
pub type HwaPdPwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_pwr_req` reader - 7:7\\]
status reg for test_dbg_pd_pwr_req"]
pub type TestDbgPdPwrReqR = crate::BitReader;
#[doc = "Field `test_dbg_pd_pwr_req` writer - 7:7\\]
status reg for test_dbg_pd_pwr_req"]
pub type TestDbgPdPwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_is_sleep"]
    #[inline(always)]
    pub fn app_pd_is_sleep(&self) -> AppPdIsSleepR {
        AppPdIsSleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for fec_pd_is_sleep"]
    #[inline(always)]
    pub fn fec_pd_is_sleep(&self) -> FecPdIsSleepR {
        FecPdIsSleepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for hwa_pd_is_sleep"]
    #[inline(always)]
    pub fn hwa_pd_is_sleep(&self) -> HwaPdIsSleepR {
        HwaPdIsSleepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for test_dbg_pd_is_sleep"]
    #[inline(always)]
    pub fn test_dbg_pd_is_sleep(&self) -> TestDbgPdIsSleepR {
        TestDbgPdIsSleepR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for app_pd_pwr_req"]
    #[inline(always)]
    pub fn app_pd_pwr_req(&self) -> AppPdPwrReqR {
        AppPdPwrReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for fec_pd_pwr_req"]
    #[inline(always)]
    pub fn fec_pd_pwr_req(&self) -> FecPdPwrReqR {
        FecPdPwrReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for hwa_pd_pwr_req"]
    #[inline(always)]
    pub fn hwa_pd_pwr_req(&self) -> HwaPdPwrReqR {
        HwaPdPwrReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for test_dbg_pd_pwr_req"]
    #[inline(always)]
    pub fn test_dbg_pd_pwr_req(&self) -> TestDbgPdPwrReqR {
        TestDbgPdPwrReqR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_is_sleep"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_is_sleep(&mut self) -> AppPdIsSleepW<DebugStatusAon6Spec> {
        AppPdIsSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for fec_pd_is_sleep"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_is_sleep(&mut self) -> FecPdIsSleepW<DebugStatusAon6Spec> {
        FecPdIsSleepW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for hwa_pd_is_sleep"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_is_sleep(&mut self) -> HwaPdIsSleepW<DebugStatusAon6Spec> {
        HwaPdIsSleepW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for test_dbg_pd_is_sleep"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_is_sleep(&mut self) -> TestDbgPdIsSleepW<DebugStatusAon6Spec> {
        TestDbgPdIsSleepW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for app_pd_pwr_req"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_pwr_req(&mut self) -> AppPdPwrReqW<DebugStatusAon6Spec> {
        AppPdPwrReqW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for fec_pd_pwr_req"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_pwr_req(&mut self) -> FecPdPwrReqW<DebugStatusAon6Spec> {
        FecPdPwrReqW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for hwa_pd_pwr_req"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_pwr_req(&mut self) -> HwaPdPwrReqW<DebugStatusAon6Spec> {
        HwaPdPwrReqW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for test_dbg_pd_pwr_req"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_pwr_req(&mut self) -> TestDbgPdPwrReqW<DebugStatusAon6Spec> {
        TestDbgPdPwrReqW::new(self, 7)
    }
}
#[doc = "DEBUG_STATUS_AON_6\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon6Spec;
impl crate::RegisterSpec for DebugStatusAon6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_6::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon6Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_6::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_6 to value 0"]
impl crate::Resettable for DebugStatusAon6Spec {
    const RESET_VALUE: u32 = 0;
}
