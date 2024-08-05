#[doc = "Register `APPSS_ERRAGG_STATUS0` reader"]
pub type R = crate::R<AppssErraggStatus0Spec>;
#[doc = "Register `APPSS_ERRAGG_STATUS0` writer"]
pub type W = crate::W<AppssErraggStatus0Spec>;
#[doc = "Field `app_rcm_rd` reader - 0:0\\]
Status of Interrupt from APP_RCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppRcmRdR = crate::BitReader;
#[doc = "Field `app_rcm_rd` writer - 0:0\\]
Status of Interrupt from APP_RCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppRcmRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_rcm_wr` reader - 1:1\\]
Status of Interrupt from APP_RCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppRcmWrR = crate::BitReader;
#[doc = "Field `app_rcm_wr` writer - 1:1\\]
Status of Interrupt from APP_RCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppRcmWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_ctrl_rd` reader - 2:2\\]
Status of Interrupt from APP_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppCtrlRdR = crate::BitReader;
#[doc = "Field `app_ctrl_rd` writer - 2:2\\]
Status of Interrupt from APP_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_ctrl_wr` reader - 3:3\\]
Status of Interrupt from APP_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppCtrlWrR = crate::BitReader;
#[doc = "Field `app_ctrl_wr` writer - 3:3\\]
Status of Interrupt from APP_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_idalloc_rd` reader - 4:4\\]
Status of Interrupt from APP_IDALLOC Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppIdallocRdR = crate::BitReader;
#[doc = "Field `app_idalloc_rd` writer - 4:4\\]
Status of Interrupt from APP_IDALLOC Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppIdallocRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_idalloc_wr` reader - 5:5\\]
Status of Interrupt from APP_IDALLOC Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppIdallocWrR = crate::BitReader;
#[doc = "Field `app_idalloc_wr` writer - 5:5\\]
Status of Interrupt from APP_IDALLOC Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppIdallocWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adcbuff_ctrl_rd` reader - 6:6\\]
Status of Interrupt from ADCBUFF_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AdcbuffCtrlRdR = crate::BitReader;
#[doc = "Field `adcbuff_ctrl_rd` writer - 6:6\\]
Status of Interrupt from ADCBUFF_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AdcbuffCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adcbuff_ctrl_wr` reader - 7:7\\]
Status of Interrupt from ADCBUFF_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AdcbuffCtrlWrR = crate::BitReader;
#[doc = "Field `adcbuff_ctrl_wr` writer - 7:7\\]
Status of Interrupt from ADCBUFF_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AdcbuffCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `plldig_ctrl_rd` reader - 8:8\\]
Status of Interrupt from PLLDIG_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type PlldigCtrlRdR = crate::BitReader;
#[doc = "Field `plldig_ctrl_rd` writer - 8:8\\]
Status of Interrupt from PLLDIG_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type PlldigCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `plldig_ctrl_wr` reader - 9:9\\]
Status of Interrupt from PLLDIG_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type PlldigCtrlWrR = crate::BitReader;
#[doc = "Field `plldig_ctrl_wr` writer - 9:9\\]
Status of Interrupt from PLLDIG_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type PlldigCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `topss_ctrl_rd` reader - 10:10\\]
Status of Interrupt from TOPSS_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopssCtrlRdR = crate::BitReader;
#[doc = "Field `topss_ctrl_rd` writer - 10:10\\]
Status of Interrupt from TOPSS_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopssCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `topss_ctrl_wr` reader - 11:11\\]
Status of Interrupt from TOPSS_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopssCtrlWrR = crate::BitReader;
#[doc = "Field `topss_ctrl_wr` writer - 11:11\\]
Status of Interrupt from TOPSS_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopssCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `apll_ctrl_rd` reader - 12:12\\]
Status of Interrupt from APLL_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type ApllCtrlRdR = crate::BitReader;
#[doc = "Field `apll_ctrl_rd` writer - 12:12\\]
Status of Interrupt from APLL_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type ApllCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `apll_ctrl_wr` reader - 13:13\\]
Status of Interrupt from APLL_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type ApllCtrlWrR = crate::BitReader;
#[doc = "Field `apll_ctrl_wr` writer - 13:13\\]
Status of Interrupt from APLL_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type ApllCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frame_timer_rd` reader - 14:14\\]
Status of Interrupt from FRAME_TIMER Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type FrameTimerRdR = crate::BitReader;
#[doc = "Field `frame_timer_rd` writer - 14:14\\]
Status of Interrupt from FRAME_TIMER Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type FrameTimerRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frame_timer_wr` reader - 15:15\\]
Status of Interrupt from FRAME_TIMER Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type FrameTimerWrR = crate::BitReader;
#[doc = "Field `frame_timer_wr` writer - 15:15\\]
Status of Interrupt from FRAME_TIMER Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type FrameTimerWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_prcm_rd` reader - 16:16\\]
Status of Interrupt from TOP_PRCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopPrcmRdR = crate::BitReader;
#[doc = "Field `top_prcm_rd` writer - 16:16\\]
Status of Interrupt from TOP_PRCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopPrcmRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_prcm_wr` reader - 17:17\\]
Status of Interrupt from TOP_PRCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopPrcmWrR = crate::BitReader;
#[doc = "Field `top_prcm_wr` writer - 17:17\\]
Status of Interrupt from TOP_PRCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopPrcmWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_efuse_ctrl_rd` reader - 18:18\\]
Status of Interrupt from TOP_EFUSE_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopEfuseCtrlRdR = crate::BitReader;
#[doc = "Field `top_efuse_ctrl_rd` writer - 18:18\\]
Status of Interrupt from TOP_EFUSE_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopEfuseCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_efuse_ctrl_wr` reader - 19:19\\]
Status of Interrupt from TOP_EFUSE_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopEfuseCtrlWrR = crate::BitReader;
#[doc = "Field `top_efuse_ctrl_wr` writer - 19:19\\]
Status of Interrupt from TOP_EFUSE_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type TopEfuseCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_mpu_rd` reader - 20:20\\]
Status of Interrupt from APP_MPU Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppMpuRdR = crate::BitReader;
#[doc = "Field `app_mpu_rd` writer - 20:20\\]
Status of Interrupt from APP_MPU Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppMpuRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_ahb_wr` reader - 21:21\\]
Status of Interrupt from APP_AHB Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppAhbWrR = crate::BitReader;
#[doc = "Field `app_ahb_wr` writer - 21:21\\]
Status of Interrupt from APP_AHB Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppAhbWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_shared_mem_err` reader - 22:22\\]
Status of Interrupt from APP_SHARED_MEM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppSharedMemErrR = crate::BitReader;
#[doc = "Field `app_shared_mem_err` writer - 22:22\\]
Status of Interrupt from APP_SHARED_MEM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppSharedMemErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_erroragg` reader - 23:23\\]
Status of Interrupt from FEC_ERRORAGG Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type FecErroraggR = crate::BitReader;
#[doc = "Field `fec_erroragg` writer - 23:23\\]
Status of Interrupt from FEC_ERRORAGG Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type FecErroraggW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_ahb_slv_wr` reader - 24:24\\]
Status of Interrupt from AHB_SLAVE Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppAhbSlvWrR = crate::BitReader;
#[doc = "Field `app_ahb_slv_wr` writer - 24:24\\]
Status of Interrupt from AHB_SLAVE Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppAhbSlvWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_ahb_slv_rd` reader - 25:25\\]
Status of Interrupt from AHB_SLAVE Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppAhbSlvRdR = crate::BitReader;
#[doc = "Field `app_ahb_slv_rd` writer - 25:25\\]
Status of Interrupt from AHB_SLAVE Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
pub type AppAhbSlvRdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of Interrupt from APP_RCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_rcm_rd(&self) -> AppRcmRdR {
        AppRcmRdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of Interrupt from APP_RCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_rcm_wr(&self) -> AppRcmWrR {
        AppRcmWrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of Interrupt from APP_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_ctrl_rd(&self) -> AppCtrlRdR {
        AppCtrlRdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of Interrupt from APP_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_ctrl_wr(&self) -> AppCtrlWrR {
        AppCtrlWrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Status of Interrupt from APP_IDALLOC Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_idalloc_rd(&self) -> AppIdallocRdR {
        AppIdallocRdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Status of Interrupt from APP_IDALLOC Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_idalloc_wr(&self) -> AppIdallocWrR {
        AppIdallocWrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Status of Interrupt from ADCBUFF_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn adcbuff_ctrl_rd(&self) -> AdcbuffCtrlRdR {
        AdcbuffCtrlRdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Status of Interrupt from ADCBUFF_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn adcbuff_ctrl_wr(&self) -> AdcbuffCtrlWrR {
        AdcbuffCtrlWrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of Interrupt from PLLDIG_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn plldig_ctrl_rd(&self) -> PlldigCtrlRdR {
        PlldigCtrlRdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Status of Interrupt from PLLDIG_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn plldig_ctrl_wr(&self) -> PlldigCtrlWrR {
        PlldigCtrlWrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Status of Interrupt from TOPSS_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn topss_ctrl_rd(&self) -> TopssCtrlRdR {
        TopssCtrlRdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Status of Interrupt from TOPSS_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn topss_ctrl_wr(&self) -> TopssCtrlWrR {
        TopssCtrlWrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Status of Interrupt from APLL_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn apll_ctrl_rd(&self) -> ApllCtrlRdR {
        ApllCtrlRdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Status of Interrupt from APLL_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn apll_ctrl_wr(&self) -> ApllCtrlWrR {
        ApllCtrlWrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Status of Interrupt from FRAME_TIMER Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn frame_timer_rd(&self) -> FrameTimerRdR {
        FrameTimerRdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Status of Interrupt from FRAME_TIMER Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn frame_timer_wr(&self) -> FrameTimerWrR {
        FrameTimerWrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Status of Interrupt from TOP_PRCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn top_prcm_rd(&self) -> TopPrcmRdR {
        TopPrcmRdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Status of Interrupt from TOP_PRCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn top_prcm_wr(&self) -> TopPrcmWrR {
        TopPrcmWrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Status of Interrupt from TOP_EFUSE_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn top_efuse_ctrl_rd(&self) -> TopEfuseCtrlRdR {
        TopEfuseCtrlRdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Status of Interrupt from TOP_EFUSE_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn top_efuse_ctrl_wr(&self) -> TopEfuseCtrlWrR {
        TopEfuseCtrlWrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Status of Interrupt from APP_MPU Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_mpu_rd(&self) -> AppMpuRdR {
        AppMpuRdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Status of Interrupt from APP_AHB Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_ahb_wr(&self) -> AppAhbWrR {
        AppAhbWrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Status of Interrupt from APP_SHARED_MEM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_shared_mem_err(&self) -> AppSharedMemErrR {
        AppSharedMemErrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Status of Interrupt from FEC_ERRORAGG Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn fec_erroragg(&self) -> FecErroraggR {
        FecErroraggR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Status of Interrupt from AHB_SLAVE Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_ahb_slv_wr(&self) -> AppAhbSlvWrR {
        AppAhbSlvWrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Status of Interrupt from AHB_SLAVE Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn app_ahb_slv_rd(&self) -> AppAhbSlvRdR {
        AppAhbSlvRdR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of Interrupt from APP_RCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_rcm_rd(&mut self) -> AppRcmRdW<AppssErraggStatus0Spec> {
        AppRcmRdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of Interrupt from APP_RCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_rcm_wr(&mut self) -> AppRcmWrW<AppssErraggStatus0Spec> {
        AppRcmWrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of Interrupt from APP_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl_rd(&mut self) -> AppCtrlRdW<AppssErraggStatus0Spec> {
        AppCtrlRdW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of Interrupt from APP_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl_wr(&mut self) -> AppCtrlWrW<AppssErraggStatus0Spec> {
        AppCtrlWrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Status of Interrupt from APP_IDALLOC Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_idalloc_rd(&mut self) -> AppIdallocRdW<AppssErraggStatus0Spec> {
        AppIdallocRdW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Status of Interrupt from APP_IDALLOC Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_idalloc_wr(&mut self) -> AppIdallocWrW<AppssErraggStatus0Spec> {
        AppIdallocWrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Status of Interrupt from ADCBUFF_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adcbuff_ctrl_rd(&mut self) -> AdcbuffCtrlRdW<AppssErraggStatus0Spec> {
        AdcbuffCtrlRdW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Status of Interrupt from ADCBUFF_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adcbuff_ctrl_wr(&mut self) -> AdcbuffCtrlWrW<AppssErraggStatus0Spec> {
        AdcbuffCtrlWrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of Interrupt from PLLDIG_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn plldig_ctrl_rd(&mut self) -> PlldigCtrlRdW<AppssErraggStatus0Spec> {
        PlldigCtrlRdW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Status of Interrupt from PLLDIG_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn plldig_ctrl_wr(&mut self) -> PlldigCtrlWrW<AppssErraggStatus0Spec> {
        PlldigCtrlWrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Status of Interrupt from TOPSS_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn topss_ctrl_rd(&mut self) -> TopssCtrlRdW<AppssErraggStatus0Spec> {
        TopssCtrlRdW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Status of Interrupt from TOPSS_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn topss_ctrl_wr(&mut self) -> TopssCtrlWrW<AppssErraggStatus0Spec> {
        TopssCtrlWrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Status of Interrupt from APLL_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn apll_ctrl_rd(&mut self) -> ApllCtrlRdW<AppssErraggStatus0Spec> {
        ApllCtrlRdW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Status of Interrupt from APLL_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn apll_ctrl_wr(&mut self) -> ApllCtrlWrW<AppssErraggStatus0Spec> {
        ApllCtrlWrW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Status of Interrupt from FRAME_TIMER Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn frame_timer_rd(&mut self) -> FrameTimerRdW<AppssErraggStatus0Spec> {
        FrameTimerRdW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Status of Interrupt from FRAME_TIMER Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn frame_timer_wr(&mut self) -> FrameTimerWrW<AppssErraggStatus0Spec> {
        FrameTimerWrW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Status of Interrupt from TOP_PRCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn top_prcm_rd(&mut self) -> TopPrcmRdW<AppssErraggStatus0Spec> {
        TopPrcmRdW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Status of Interrupt from TOP_PRCM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn top_prcm_wr(&mut self) -> TopPrcmWrW<AppssErraggStatus0Spec> {
        TopPrcmWrW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Status of Interrupt from TOP_EFUSE_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn top_efuse_ctrl_rd(&mut self) -> TopEfuseCtrlRdW<AppssErraggStatus0Spec> {
        TopEfuseCtrlRdW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Status of Interrupt from TOP_EFUSE_CTRL Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn top_efuse_ctrl_wr(&mut self) -> TopEfuseCtrlWrW<AppssErraggStatus0Spec> {
        TopEfuseCtrlWrW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Status of Interrupt from APP_MPU Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_mpu_rd(&mut self) -> AppMpuRdW<AppssErraggStatus0Spec> {
        AppMpuRdW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Status of Interrupt from APP_AHB Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_ahb_wr(&mut self) -> AppAhbWrW<AppssErraggStatus0Spec> {
        AppAhbWrW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Status of Interrupt from APP_SHARED_MEM Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_shared_mem_err(&mut self) -> AppSharedMemErrW<AppssErraggStatus0Spec> {
        AppSharedMemErrW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Status of Interrupt from FEC_ERRORAGG Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fec_erroragg(&mut self) -> FecErroraggW<AppssErraggStatus0Spec> {
        FecErroraggW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Status of Interrupt from AHB_SLAVE Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_ahb_slv_wr(&mut self) -> AppAhbSlvWrW<AppssErraggStatus0Spec> {
        AppAhbSlvWrW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Status of Interrupt from AHB_SLAVE Set only if Interupt is unmasked in APPSS_ERRAGG_MASK0 Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_ahb_slv_rd(&mut self) -> AppAhbSlvRdW<AppssErraggStatus0Spec> {
        AppAhbSlvRdW::new(self, 25)
    }
}
#[doc = "APPSS_ERRAGG_STATUS0\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_erragg_status0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_erragg_status0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssErraggStatus0Spec;
impl crate::RegisterSpec for AppssErraggStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_erragg_status0::R`](R) reader structure"]
impl crate::Readable for AppssErraggStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`appss_erragg_status0::W`](W) writer structure"]
impl crate::Writable for AppssErraggStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_ERRAGG_STATUS0 to value 0"]
impl crate::Resettable for AppssErraggStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
