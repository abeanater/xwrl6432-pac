#[doc = "Register `DEBUG_STATUS_AON_2` reader"]
pub type R = crate::R<DebugStatusAon2Spec>;
#[doc = "Register `DEBUG_STATUS_AON_2` writer"]
pub type W = crate::W<DebugStatusAon2Spec>;
#[doc = "Field `clkm_state` reader - 5:0\\]
status reg for clkm_state"]
pub type ClkmStateR = crate::FieldReader;
#[doc = "Field `clkm_state` writer - 5:0\\]
status reg for clkm_state"]
pub type ClkmStateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `clkm_xtal_det_status_in` reader - 6:6\\]
status reg for clkm_xtal_det_status_in"]
pub type ClkmXtalDetStatusInR = crate::BitReader;
#[doc = "Field `clkm_xtal_det_status_in` writer - 6:6\\]
status reg for clkm_xtal_det_status_in"]
pub type ClkmXtalDetStatusInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_slicer_ldo_en` reader - 7:7\\]
status reg for clkm_slicer_ldo_en"]
pub type ClkmSlicerLdoEnR = crate::BitReader;
#[doc = "Field `clkm_slicer_ldo_en` writer - 7:7\\]
status reg for clkm_slicer_ldo_en"]
pub type ClkmSlicerLdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_slicer_bias_en` reader - 8:8\\]
status reg for clkm_slicer_bias_en"]
pub type ClkmSlicerBiasEnR = crate::BitReader;
#[doc = "Field `clkm_slicer_bias_en` writer - 8:8\\]
status reg for clkm_slicer_bias_en"]
pub type ClkmSlicerBiasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_xtal_en` reader - 9:9\\]
status reg for clkm_xtal_en"]
pub type ClkmXtalEnR = crate::BitReader;
#[doc = "Field `clkm_xtal_en` writer - 9:9\\]
status reg for clkm_xtal_en"]
pub type ClkmXtalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_slicer_en` reader - 10:10\\]
status reg for clkm_slicer_en"]
pub type ClkmSlicerEnR = crate::BitReader;
#[doc = "Field `clkm_slicer_en` writer - 10:10\\]
status reg for clkm_slicer_en"]
pub type ClkmSlicerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_xtal_det_en` reader - 11:11\\]
status reg for clkm_xtal_det_en"]
pub type ClkmXtalDetEnR = crate::BitReader;
#[doc = "Field `clkm_xtal_det_en` writer - 11:11\\]
status reg for clkm_xtal_det_en"]
pub type ClkmXtalDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_xtal_det_status` reader - 12:12\\]
status reg for clkm_xtal_det_status"]
pub type ClkmXtalDetStatusR = crate::BitReader;
#[doc = "Field `clkm_xtal_det_status` writer - 12:12\\]
status reg for clkm_xtal_det_status"]
pub type ClkmXtalDetStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_oscillator_clk_valid` reader - 13:13\\]
status reg for clkm_oscillator_clk_valid"]
pub type ClkmOscillatorClkValidR = crate::BitReader;
#[doc = "Field `clkm_oscillator_clk_valid` writer - 13:13\\]
status reg for clkm_oscillator_clk_valid"]
pub type ClkmOscillatorClkValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_first_wake_up` reader - 14:14\\]
status reg for clkm_first_wake_up"]
pub type ClkmFirstWakeUpR = crate::BitReader;
#[doc = "Field `clkm_first_wake_up` writer - 14:14\\]
status reg for clkm_first_wake_up"]
pub type ClkmFirstWakeUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_host_clk_req` reader - 15:15\\]
status reg for clkm_host_clk_req"]
pub type ClkmHostClkReqR = crate::BitReader;
#[doc = "Field `clkm_host_clk_req` writer - 15:15\\]
status reg for clkm_host_clk_req"]
pub type ClkmHostClkReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_host_clk_req_output_en` reader - 16:16\\]
status reg for clkm_host_clk_req_output_en"]
pub type ClkmHostClkReqOutputEnR = crate::BitReader;
#[doc = "Field `clkm_host_clk_req_output_en` writer - 16:16\\]
status reg for clkm_host_clk_req_output_en"]
pub type ClkmHostClkReqOutputEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_limp_mode` reader - 17:17\\]
status reg for clkm_limp_mode"]
pub type ClkmLimpModeR = crate::BitReader;
#[doc = "Field `clkm_limp_mode` writer - 17:17\\]
status reg for clkm_limp_mode"]
pub type ClkmLimpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkm_xtal_freq` reader - 19:18\\]
status reg for clkm_xtal_freq"]
pub type ClkmXtalFreqR = crate::FieldReader;
#[doc = "Field `clkm_xtal_freq` writer - 19:18\\]
status reg for clkm_xtal_freq"]
pub type ClkmXtalFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkm_xt_drive` reader - 24:20\\]
status reg for clkm_xt_drive"]
pub type ClkmXtDriveR = crate::FieldReader;
#[doc = "Field `clkm_xt_drive` writer - 24:20\\]
status reg for clkm_xt_drive"]
pub type ClkmXtDriveW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
status reg for clkm_state"]
    #[inline(always)]
    pub fn clkm_state(&self) -> ClkmStateR {
        ClkmStateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for clkm_xtal_det_status_in"]
    #[inline(always)]
    pub fn clkm_xtal_det_status_in(&self) -> ClkmXtalDetStatusInR {
        ClkmXtalDetStatusInR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for clkm_slicer_ldo_en"]
    #[inline(always)]
    pub fn clkm_slicer_ldo_en(&self) -> ClkmSlicerLdoEnR {
        ClkmSlicerLdoEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for clkm_slicer_bias_en"]
    #[inline(always)]
    pub fn clkm_slicer_bias_en(&self) -> ClkmSlicerBiasEnR {
        ClkmSlicerBiasEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
status reg for clkm_xtal_en"]
    #[inline(always)]
    pub fn clkm_xtal_en(&self) -> ClkmXtalEnR {
        ClkmXtalEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
status reg for clkm_slicer_en"]
    #[inline(always)]
    pub fn clkm_slicer_en(&self) -> ClkmSlicerEnR {
        ClkmSlicerEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for clkm_xtal_det_en"]
    #[inline(always)]
    pub fn clkm_xtal_det_en(&self) -> ClkmXtalDetEnR {
        ClkmXtalDetEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for clkm_xtal_det_status"]
    #[inline(always)]
    pub fn clkm_xtal_det_status(&self) -> ClkmXtalDetStatusR {
        ClkmXtalDetStatusR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
status reg for clkm_oscillator_clk_valid"]
    #[inline(always)]
    pub fn clkm_oscillator_clk_valid(&self) -> ClkmOscillatorClkValidR {
        ClkmOscillatorClkValidR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
status reg for clkm_first_wake_up"]
    #[inline(always)]
    pub fn clkm_first_wake_up(&self) -> ClkmFirstWakeUpR {
        ClkmFirstWakeUpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
status reg for clkm_host_clk_req"]
    #[inline(always)]
    pub fn clkm_host_clk_req(&self) -> ClkmHostClkReqR {
        ClkmHostClkReqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
status reg for clkm_host_clk_req_output_en"]
    #[inline(always)]
    pub fn clkm_host_clk_req_output_en(&self) -> ClkmHostClkReqOutputEnR {
        ClkmHostClkReqOutputEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
status reg for clkm_limp_mode"]
    #[inline(always)]
    pub fn clkm_limp_mode(&self) -> ClkmLimpModeR {
        ClkmLimpModeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - 19:18\\]
status reg for clkm_xtal_freq"]
    #[inline(always)]
    pub fn clkm_xtal_freq(&self) -> ClkmXtalFreqR {
        ClkmXtalFreqR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:24 - 24:20\\]
status reg for clkm_xt_drive"]
    #[inline(always)]
    pub fn clkm_xt_drive(&self) -> ClkmXtDriveR {
        ClkmXtDriveR::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
status reg for clkm_state"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_state(&mut self) -> ClkmStateW<DebugStatusAon2Spec> {
        ClkmStateW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for clkm_xtal_det_status_in"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_xtal_det_status_in(&mut self) -> ClkmXtalDetStatusInW<DebugStatusAon2Spec> {
        ClkmXtalDetStatusInW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for clkm_slicer_ldo_en"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_slicer_ldo_en(&mut self) -> ClkmSlicerLdoEnW<DebugStatusAon2Spec> {
        ClkmSlicerLdoEnW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for clkm_slicer_bias_en"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_slicer_bias_en(&mut self) -> ClkmSlicerBiasEnW<DebugStatusAon2Spec> {
        ClkmSlicerBiasEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
status reg for clkm_xtal_en"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_xtal_en(&mut self) -> ClkmXtalEnW<DebugStatusAon2Spec> {
        ClkmXtalEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
status reg for clkm_slicer_en"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_slicer_en(&mut self) -> ClkmSlicerEnW<DebugStatusAon2Spec> {
        ClkmSlicerEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for clkm_xtal_det_en"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_xtal_det_en(&mut self) -> ClkmXtalDetEnW<DebugStatusAon2Spec> {
        ClkmXtalDetEnW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for clkm_xtal_det_status"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_xtal_det_status(&mut self) -> ClkmXtalDetStatusW<DebugStatusAon2Spec> {
        ClkmXtalDetStatusW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
status reg for clkm_oscillator_clk_valid"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_oscillator_clk_valid(&mut self) -> ClkmOscillatorClkValidW<DebugStatusAon2Spec> {
        ClkmOscillatorClkValidW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
status reg for clkm_first_wake_up"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_first_wake_up(&mut self) -> ClkmFirstWakeUpW<DebugStatusAon2Spec> {
        ClkmFirstWakeUpW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
status reg for clkm_host_clk_req"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_host_clk_req(&mut self) -> ClkmHostClkReqW<DebugStatusAon2Spec> {
        ClkmHostClkReqW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
status reg for clkm_host_clk_req_output_en"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_host_clk_req_output_en(&mut self) -> ClkmHostClkReqOutputEnW<DebugStatusAon2Spec> {
        ClkmHostClkReqOutputEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
status reg for clkm_limp_mode"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_limp_mode(&mut self) -> ClkmLimpModeW<DebugStatusAon2Spec> {
        ClkmLimpModeW::new(self, 17)
    }
    #[doc = "Bits 18:19 - 19:18\\]
status reg for clkm_xtal_freq"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_xtal_freq(&mut self) -> ClkmXtalFreqW<DebugStatusAon2Spec> {
        ClkmXtalFreqW::new(self, 18)
    }
    #[doc = "Bits 20:24 - 24:20\\]
status reg for clkm_xt_drive"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_xt_drive(&mut self) -> ClkmXtDriveW<DebugStatusAon2Spec> {
        ClkmXtDriveW::new(self, 20)
    }
}
#[doc = "DEBUG_STATUS_AON_2\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon2Spec;
impl crate::RegisterSpec for DebugStatusAon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_2::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon2Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_2::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_2 to value 0"]
impl crate::Resettable for DebugStatusAon2Spec {
    const RESET_VALUE: u32 = 0;
}
