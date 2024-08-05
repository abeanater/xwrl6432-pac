#[doc = "Register `DEBUG_STATUS_AON_12` reader"]
pub type R = crate::R<DebugStatusAon12Spec>;
#[doc = "Register `DEBUG_STATUS_AON_12` writer"]
pub type W = crate::W<DebugStatusAon12Spec>;
#[doc = "Field `app_pd_mem_grp1_highres_switch_en` reader - 0:0\\]
status reg for app_pd_mem_grp1_highres_switch_en"]
pub type AppPdMemGrp1HighresSwitchEnR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp1_highres_switch_en` writer - 0:0\\]
status reg for app_pd_mem_grp1_highres_switch_en"]
pub type AppPdMemGrp1HighresSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp2_highres_switch_en` reader - 1:1\\]
status reg for app_pd_mem_grp2_highres_switch_en"]
pub type AppPdMemGrp2HighresSwitchEnR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_highres_switch_en` writer - 1:1\\]
status reg for app_pd_mem_grp2_highres_switch_en"]
pub type AppPdMemGrp2HighresSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_mem_grp3_highres_switch_en` reader - 2:2\\]
status reg for hwa_pd_mem_grp3_highres_switch_en"]
pub type HwaPdMemGrp3HighresSwitchEnR = crate::BitReader;
#[doc = "Field `hwa_pd_mem_grp3_highres_switch_en` writer - 2:2\\]
status reg for hwa_pd_mem_grp3_highres_switch_en"]
pub type HwaPdMemGrp3HighresSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_grp4_highres_switch_en` reader - 3:3\\]
status reg for fec_pd_mem_grp4_highres_switch_en"]
pub type FecPdMemGrp4HighresSwitchEnR = crate::BitReader;
#[doc = "Field `fec_pd_mem_grp4_highres_switch_en` writer - 3:3\\]
status reg for fec_pd_mem_grp4_highres_switch_en"]
pub type FecPdMemGrp4HighresSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp1_lowres_switch_en` reader - 4:4\\]
status reg for app_pd_mem_grp1_lowres_switch_en"]
pub type AppPdMemGrp1LowresSwitchEnR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp1_lowres_switch_en` writer - 4:4\\]
status reg for app_pd_mem_grp1_lowres_switch_en"]
pub type AppPdMemGrp1LowresSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp2_lowres_switch_en` reader - 5:5\\]
status reg for app_pd_mem_grp2_lowres_switch_en"]
pub type AppPdMemGrp2LowresSwitchEnR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_lowres_switch_en` writer - 5:5\\]
status reg for app_pd_mem_grp2_lowres_switch_en"]
pub type AppPdMemGrp2LowresSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_mem_gpr3_lowres_switch_en` reader - 6:6\\]
status reg for hwa_pd_mem_gpr3_lowres_switch_en"]
pub type HwaPdMemGpr3LowresSwitchEnR = crate::BitReader;
#[doc = "Field `hwa_pd_mem_gpr3_lowres_switch_en` writer - 6:6\\]
status reg for hwa_pd_mem_gpr3_lowres_switch_en"]
pub type HwaPdMemGpr3LowresSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_gpr4_lowres_switch_en` reader - 7:7\\]
status reg for fec_pd_mem_gpr4_lowres_switch_en"]
pub type FecPdMemGpr4LowresSwitchEnR = crate::BitReader;
#[doc = "Field `fec_pd_mem_gpr4_lowres_switch_en` writer - 7:7\\]
status reg for fec_pd_mem_gpr4_lowres_switch_en"]
pub type FecPdMemGpr4LowresSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp1_vnwa_switch_en` reader - 8:8\\]
status reg for app_pd_mem_grp1_vnwa_switch_en"]
pub type AppPdMemGrp1VnwaSwitchEnR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp1_vnwa_switch_en` writer - 8:8\\]
status reg for app_pd_mem_grp1_vnwa_switch_en"]
pub type AppPdMemGrp1VnwaSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp2_vnwa_switch_en` reader - 9:9\\]
status reg for app_pd_mem_grp2_vnwa_switch_en"]
pub type AppPdMemGrp2VnwaSwitchEnR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_vnwa_switch_en` writer - 9:9\\]
status reg for app_pd_mem_grp2_vnwa_switch_en"]
pub type AppPdMemGrp2VnwaSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_mem_grp3_vnwa_switch_en` reader - 10:10\\]
status reg for hwa_pd_mem_grp3_vnwa_switch_en"]
pub type HwaPdMemGrp3VnwaSwitchEnR = crate::BitReader;
#[doc = "Field `hwa_pd_mem_grp3_vnwa_switch_en` writer - 10:10\\]
status reg for hwa_pd_mem_grp3_vnwa_switch_en"]
pub type HwaPdMemGrp3VnwaSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_grp4_vnwa_switch_en` reader - 11:11\\]
status reg for fec_pd_mem_grp4_vnwa_switch_en"]
pub type FecPdMemGrp4VnwaSwitchEnR = crate::BitReader;
#[doc = "Field `fec_pd_mem_grp4_vnwa_switch_en` writer - 11:11\\]
status reg for fec_pd_mem_grp4_vnwa_switch_en"]
pub type FecPdMemGrp4VnwaSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vnwa_switch_weak_process` reader - 12:12\\]
status reg for vnwa_switch_weak_process"]
pub type VnwaSwitchWeakProcessR = crate::BitReader;
#[doc = "Field `vnwa_switch_weak_process` writer - 12:12\\]
status reg for vnwa_switch_weak_process"]
pub type VnwaSwitchWeakProcessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vnwa_switch_screen_en` reader - 13:13\\]
status reg for vnwa_switch_screen_en"]
pub type VnwaSwitchScreenEnR = crate::BitReader;
#[doc = "Field `vnwa_switch_screen_en` writer - 13:13\\]
status reg for vnwa_switch_screen_en"]
pub type VnwaSwitchScreenEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_mem_grp1_highres_switch_en"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_highres_switch_en(&self) -> AppPdMemGrp1HighresSwitchEnR {
        AppPdMemGrp1HighresSwitchEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for app_pd_mem_grp2_highres_switch_en"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_highres_switch_en(&self) -> AppPdMemGrp2HighresSwitchEnR {
        AppPdMemGrp2HighresSwitchEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for hwa_pd_mem_grp3_highres_switch_en"]
    #[inline(always)]
    pub fn hwa_pd_mem_grp3_highres_switch_en(&self) -> HwaPdMemGrp3HighresSwitchEnR {
        HwaPdMemGrp3HighresSwitchEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for fec_pd_mem_grp4_highres_switch_en"]
    #[inline(always)]
    pub fn fec_pd_mem_grp4_highres_switch_en(&self) -> FecPdMemGrp4HighresSwitchEnR {
        FecPdMemGrp4HighresSwitchEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for app_pd_mem_grp1_lowres_switch_en"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_lowres_switch_en(&self) -> AppPdMemGrp1LowresSwitchEnR {
        AppPdMemGrp1LowresSwitchEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for app_pd_mem_grp2_lowres_switch_en"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_lowres_switch_en(&self) -> AppPdMemGrp2LowresSwitchEnR {
        AppPdMemGrp2LowresSwitchEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for hwa_pd_mem_gpr3_lowres_switch_en"]
    #[inline(always)]
    pub fn hwa_pd_mem_gpr3_lowres_switch_en(&self) -> HwaPdMemGpr3LowresSwitchEnR {
        HwaPdMemGpr3LowresSwitchEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for fec_pd_mem_gpr4_lowres_switch_en"]
    #[inline(always)]
    pub fn fec_pd_mem_gpr4_lowres_switch_en(&self) -> FecPdMemGpr4LowresSwitchEnR {
        FecPdMemGpr4LowresSwitchEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for app_pd_mem_grp1_vnwa_switch_en"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_vnwa_switch_en(&self) -> AppPdMemGrp1VnwaSwitchEnR {
        AppPdMemGrp1VnwaSwitchEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
status reg for app_pd_mem_grp2_vnwa_switch_en"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_vnwa_switch_en(&self) -> AppPdMemGrp2VnwaSwitchEnR {
        AppPdMemGrp2VnwaSwitchEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
status reg for hwa_pd_mem_grp3_vnwa_switch_en"]
    #[inline(always)]
    pub fn hwa_pd_mem_grp3_vnwa_switch_en(&self) -> HwaPdMemGrp3VnwaSwitchEnR {
        HwaPdMemGrp3VnwaSwitchEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for fec_pd_mem_grp4_vnwa_switch_en"]
    #[inline(always)]
    pub fn fec_pd_mem_grp4_vnwa_switch_en(&self) -> FecPdMemGrp4VnwaSwitchEnR {
        FecPdMemGrp4VnwaSwitchEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for vnwa_switch_weak_process"]
    #[inline(always)]
    pub fn vnwa_switch_weak_process(&self) -> VnwaSwitchWeakProcessR {
        VnwaSwitchWeakProcessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
status reg for vnwa_switch_screen_en"]
    #[inline(always)]
    pub fn vnwa_switch_screen_en(&self) -> VnwaSwitchScreenEnR {
        VnwaSwitchScreenEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_mem_grp1_highres_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_highres_switch_en(
        &mut self,
    ) -> AppPdMemGrp1HighresSwitchEnW<DebugStatusAon12Spec> {
        AppPdMemGrp1HighresSwitchEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for app_pd_mem_grp2_highres_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_highres_switch_en(
        &mut self,
    ) -> AppPdMemGrp2HighresSwitchEnW<DebugStatusAon12Spec> {
        AppPdMemGrp2HighresSwitchEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for hwa_pd_mem_grp3_highres_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_grp3_highres_switch_en(
        &mut self,
    ) -> HwaPdMemGrp3HighresSwitchEnW<DebugStatusAon12Spec> {
        HwaPdMemGrp3HighresSwitchEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for fec_pd_mem_grp4_highres_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_grp4_highres_switch_en(
        &mut self,
    ) -> FecPdMemGrp4HighresSwitchEnW<DebugStatusAon12Spec> {
        FecPdMemGrp4HighresSwitchEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for app_pd_mem_grp1_lowres_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_lowres_switch_en(
        &mut self,
    ) -> AppPdMemGrp1LowresSwitchEnW<DebugStatusAon12Spec> {
        AppPdMemGrp1LowresSwitchEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for app_pd_mem_grp2_lowres_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_lowres_switch_en(
        &mut self,
    ) -> AppPdMemGrp2LowresSwitchEnW<DebugStatusAon12Spec> {
        AppPdMemGrp2LowresSwitchEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for hwa_pd_mem_gpr3_lowres_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_gpr3_lowres_switch_en(
        &mut self,
    ) -> HwaPdMemGpr3LowresSwitchEnW<DebugStatusAon12Spec> {
        HwaPdMemGpr3LowresSwitchEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for fec_pd_mem_gpr4_lowres_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_gpr4_lowres_switch_en(
        &mut self,
    ) -> FecPdMemGpr4LowresSwitchEnW<DebugStatusAon12Spec> {
        FecPdMemGpr4LowresSwitchEnW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for app_pd_mem_grp1_vnwa_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_vnwa_switch_en(
        &mut self,
    ) -> AppPdMemGrp1VnwaSwitchEnW<DebugStatusAon12Spec> {
        AppPdMemGrp1VnwaSwitchEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
status reg for app_pd_mem_grp2_vnwa_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_vnwa_switch_en(
        &mut self,
    ) -> AppPdMemGrp2VnwaSwitchEnW<DebugStatusAon12Spec> {
        AppPdMemGrp2VnwaSwitchEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
status reg for hwa_pd_mem_grp3_vnwa_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_grp3_vnwa_switch_en(
        &mut self,
    ) -> HwaPdMemGrp3VnwaSwitchEnW<DebugStatusAon12Spec> {
        HwaPdMemGrp3VnwaSwitchEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for fec_pd_mem_grp4_vnwa_switch_en"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_grp4_vnwa_switch_en(
        &mut self,
    ) -> FecPdMemGrp4VnwaSwitchEnW<DebugStatusAon12Spec> {
        FecPdMemGrp4VnwaSwitchEnW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for vnwa_switch_weak_process"]
    #[inline(always)]
    #[must_use]
    pub fn vnwa_switch_weak_process(&mut self) -> VnwaSwitchWeakProcessW<DebugStatusAon12Spec> {
        VnwaSwitchWeakProcessW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
status reg for vnwa_switch_screen_en"]
    #[inline(always)]
    #[must_use]
    pub fn vnwa_switch_screen_en(&mut self) -> VnwaSwitchScreenEnW<DebugStatusAon12Spec> {
        VnwaSwitchScreenEnW::new(self, 13)
    }
}
#[doc = "DEBUG_STATUS_AON_12\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon12Spec;
impl crate::RegisterSpec for DebugStatusAon12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_12::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon12Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_12::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_12 to value 0"]
impl crate::Resettable for DebugStatusAon12Spec {
    const RESET_VALUE: u32 = 0;
}
