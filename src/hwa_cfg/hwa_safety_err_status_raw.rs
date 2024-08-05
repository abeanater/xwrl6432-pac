#[doc = "Register `HWA_SAFETY_ERR_STATUS_RAW` reader"]
pub type R = crate::R<HwaSafetyErrStatusRawSpec>;
#[doc = "Register `HWA_SAFETY_ERR_STATUS_RAW` writer"]
pub type W = crate::W<HwaSafetyErrStatusRawSpec>;
#[doc = "Field `HWA_SAFETY_ERR_STATUS_RAW_FSM_LOCKSTEP` reader - 0:0\\]
Indicates the FSM lockstep error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 0"]
pub type HwaSafetyErrStatusRawFsmLockstepR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ERR_STATUS_RAW_FSM_LOCKSTEP` writer - 0:0\\]
Indicates the FSM lockstep error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 0"]
pub type HwaSafetyErrStatusRawFsmLockstepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_WINDOW_RAM` reader - 1:1\\]
Indicates the Window RAM parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 1"]
pub type HwaSafetyParityErrStatusRawWindowRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_WINDOW_RAM` writer - 1:1\\]
Indicates the Window RAM parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 1"]
pub type HwaSafetyParityErrStatusRawWindowRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_IPING_RAM` reader - 2:2\\]
Indicates the ACCEL_MEM0 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 2"]
pub type HwaSafetyParityErrStatusRawIpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_IPING_RAM` writer - 2:2\\]
Indicates the ACCEL_MEM0 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 2"]
pub type HwaSafetyParityErrStatusRawIpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_IPONG_RAM` reader - 3:3\\]
Indicates the ACCEL_MEM1 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 3"]
pub type HwaSafetyParityErrStatusRawIpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_IPONG_RAM` writer - 3:3\\]
Indicates the ACCEL_MEM1 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 3"]
pub type HwaSafetyParityErrStatusRawIpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_OPING_RAM` reader - 4:4\\]
Indicates the ACCEL_MEM2 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 4"]
pub type HwaSafetyParityErrStatusRawOpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_OPING_RAM` writer - 4:4\\]
Indicates the ACCEL_MEM2 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 4"]
pub type HwaSafetyParityErrStatusRawOpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_OPONG_RAM` reader - 5:5\\]
Indicates the ACCEL_MEM3 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 5"]
pub type HwaSafetyParityErrStatusRawOpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_RAW_OPONG_RAM` writer - 5:5\\]
Indicates the ACCEL_MEM3 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 5"]
pub type HwaSafetyParityErrStatusRawOpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_RAW_IPING_RAM` reader - 6:6\\]
Indicates the ACCEL_MEM0 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 6"]
pub type HwaSafetyAccessErrStatusRawIpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_RAW_IPING_RAM` writer - 6:6\\]
Indicates the ACCEL_MEM0 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 6"]
pub type HwaSafetyAccessErrStatusRawIpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_RAW_IPONG_RAM` reader - 7:7\\]
Indicates the ACCEL_MEM1 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 7"]
pub type HwaSafetyAccessErrStatusRawIpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_RAW_IPONG_RAM` writer - 7:7\\]
Indicates the ACCEL_MEM1 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 7"]
pub type HwaSafetyAccessErrStatusRawIpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_RAW_OPING_RAM` reader - 8:8\\]
Indicates the ACCEL_MEM2 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 8"]
pub type HwaSafetyAccessErrStatusRawOpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_RAW_OPING_RAM` writer - 8:8\\]
Indicates the ACCEL_MEM2 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 8"]
pub type HwaSafetyAccessErrStatusRawOpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_RAW_OPONG_RAM` reader - 9:9\\]
Indicates the ACCEL_MEM3 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 9"]
pub type HwaSafetyAccessErrStatusRawOpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_RAW_OPONG_RAM` writer - 9:9\\]
Indicates the ACCEL_MEM3 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 9"]
pub type HwaSafetyAccessErrStatusRawOpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the FSM lockstep error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 0"]
    #[inline(always)]
    pub fn hwa_safety_err_status_raw_fsm_lockstep(&self) -> HwaSafetyErrStatusRawFsmLockstepR {
        HwaSafetyErrStatusRawFsmLockstepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the Window RAM parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 1"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_raw_window_ram(
        &self,
    ) -> HwaSafetyParityErrStatusRawWindowRamR {
        HwaSafetyParityErrStatusRawWindowRamR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates the ACCEL_MEM0 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 2"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_raw_iping_ram(
        &self,
    ) -> HwaSafetyParityErrStatusRawIpingRamR {
        HwaSafetyParityErrStatusRawIpingRamR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the ACCEL_MEM1 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 3"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_raw_ipong_ram(
        &self,
    ) -> HwaSafetyParityErrStatusRawIpongRamR {
        HwaSafetyParityErrStatusRawIpongRamR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates the ACCEL_MEM2 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 4"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_raw_oping_ram(
        &self,
    ) -> HwaSafetyParityErrStatusRawOpingRamR {
        HwaSafetyParityErrStatusRawOpingRamR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates the ACCEL_MEM3 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 5"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_raw_opong_ram(
        &self,
    ) -> HwaSafetyParityErrStatusRawOpongRamR {
        HwaSafetyParityErrStatusRawOpongRamR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates the ACCEL_MEM0 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 6"]
    #[inline(always)]
    pub fn hwa_safety_access_err_status_raw_iping_ram(
        &self,
    ) -> HwaSafetyAccessErrStatusRawIpingRamR {
        HwaSafetyAccessErrStatusRawIpingRamR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates the ACCEL_MEM1 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 7"]
    #[inline(always)]
    pub fn hwa_safety_access_err_status_raw_ipong_ram(
        &self,
    ) -> HwaSafetyAccessErrStatusRawIpongRamR {
        HwaSafetyAccessErrStatusRawIpongRamR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the ACCEL_MEM2 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 8"]
    #[inline(always)]
    pub fn hwa_safety_access_err_status_raw_oping_ram(
        &self,
    ) -> HwaSafetyAccessErrStatusRawOpingRamR {
        HwaSafetyAccessErrStatusRawOpingRamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the ACCEL_MEM3 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 9"]
    #[inline(always)]
    pub fn hwa_safety_access_err_status_raw_opong_ram(
        &self,
    ) -> HwaSafetyAccessErrStatusRawOpongRamR {
        HwaSafetyAccessErrStatusRawOpongRamR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the FSM lockstep error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_err_status_raw_fsm_lockstep(
        &mut self,
    ) -> HwaSafetyErrStatusRawFsmLockstepW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyErrStatusRawFsmLockstepW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the Window RAM parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_raw_window_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusRawWindowRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyParityErrStatusRawWindowRamW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates the ACCEL_MEM0 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_raw_iping_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusRawIpingRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyParityErrStatusRawIpingRamW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the ACCEL_MEM1 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_raw_ipong_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusRawIpongRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyParityErrStatusRawIpongRamW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates the ACCEL_MEM2 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_raw_oping_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusRawOpingRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyParityErrStatusRawOpingRamW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates the ACCEL_MEM3 parity error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_raw_opong_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusRawOpongRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyParityErrStatusRawOpongRamW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates the ACCEL_MEM0 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_status_raw_iping_ram(
        &mut self,
    ) -> HwaSafetyAccessErrStatusRawIpingRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyAccessErrStatusRawIpingRamW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates the ACCEL_MEM1 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_status_raw_ipong_ram(
        &mut self,
    ) -> HwaSafetyAccessErrStatusRawIpongRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyAccessErrStatusRawIpongRamW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the ACCEL_MEM2 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_status_raw_oping_ram(
        &mut self,
    ) -> HwaSafetyAccessErrStatusRawOpingRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyAccessErrStatusRawOpingRamW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the ACCEL_MEM3 access error (raw status).Set irrespective of HWA_SAFETY_ERR_MASK bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_status_raw_opong_ram(
        &mut self,
    ) -> HwaSafetyAccessErrStatusRawOpongRamW<HwaSafetyErrStatusRawSpec> {
        HwaSafetyAccessErrStatusRawOpongRamW::new(self, 9)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<HwaSafetyErrStatusRawSpec> {
        Nu1W::new(self, 10)
    }
}
#[doc = "HWA_SAFETY_ERR_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_status_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_status_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyErrStatusRawSpec;
impl crate::RegisterSpec for HwaSafetyErrStatusRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_err_status_raw::R`](R) reader structure"]
impl crate::Readable for HwaSafetyErrStatusRawSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_err_status_raw::W`](W) writer structure"]
impl crate::Writable for HwaSafetyErrStatusRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_ERR_STATUS_RAW to value 0"]
impl crate::Resettable for HwaSafetyErrStatusRawSpec {
    const RESET_VALUE: u32 = 0;
}
