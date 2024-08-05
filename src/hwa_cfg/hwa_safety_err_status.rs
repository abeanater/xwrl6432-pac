#[doc = "Register `HWA_SAFETY_ERR_STATUS` reader"]
pub type R = crate::R<HwaSafetyErrStatusSpec>;
#[doc = "Register `HWA_SAFETY_ERR_STATUS` writer"]
pub type W = crate::W<HwaSafetyErrStatusSpec>;
#[doc = "Field `HWA_SAFETY_ERR_STATUS_FSM_LOCKSTEP` reader - 0:0\\]
Indicates the FSM lockstep error (Masked status)"]
pub type HwaSafetyErrStatusFsmLockstepR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ERR_STATUS_FSM_LOCKSTEP` writer - 0:0\\]
Indicates the FSM lockstep error (Masked status)"]
pub type HwaSafetyErrStatusFsmLockstepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_WINDOW_RAM` reader - 1:1\\]
Indicates the Window RAM parity error (Masked status)"]
pub type HwaSafetyParityErrStatusWindowRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_WINDOW_RAM` writer - 1:1\\]
Indicates the Window RAM parity error (Masked status)"]
pub type HwaSafetyParityErrStatusWindowRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_IPING_RAM` reader - 2:2\\]
Indicates the ACCEL_MEM0 parity error (Masked status)"]
pub type HwaSafetyParityErrStatusIpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_IPING_RAM` writer - 2:2\\]
Indicates the ACCEL_MEM0 parity error (Masked status)"]
pub type HwaSafetyParityErrStatusIpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_IPONG_RAM` reader - 3:3\\]
Indicates the ACCEL_MEM1 parity error (Masked status)"]
pub type HwaSafetyParityErrStatusIpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_IPONG_RAM` writer - 3:3\\]
Indicates the ACCEL_MEM1 parity error (Masked status)"]
pub type HwaSafetyParityErrStatusIpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_OPING_RAM` reader - 4:4\\]
Indicates the ACCEL_MEM2 parity error (Masked status)"]
pub type HwaSafetyParityErrStatusOpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_OPING_RAM` writer - 4:4\\]
Indicates the ACCEL_MEM2 parity error (Masked status)"]
pub type HwaSafetyParityErrStatusOpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_OPONG_RAM` reader - 5:5\\]
Indicates the ACCEL_MEM3 parity error (Masked status)"]
pub type HwaSafetyParityErrStatusOpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_STATUS_OPONG_RAM` writer - 5:5\\]
Indicates the ACCEL_MEM3 parity error (Masked status)"]
pub type HwaSafetyParityErrStatusOpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_IPING_RAM` reader - 6:6\\]
Indicates the ACCEL_MEM0 access error (Masked status)"]
pub type HwaSafetyAccessErrStatusIpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_IPING_RAM` writer - 6:6\\]
Indicates the ACCEL_MEM0 access error (Masked status)"]
pub type HwaSafetyAccessErrStatusIpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_IPONG_RAM` reader - 7:7\\]
Indicates the ACCEL_MEM1 access error (Masked status)"]
pub type HwaSafetyAccessErrStatusIpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_IPONG_RAM` writer - 7:7\\]
Indicates the ACCEL_MEM1 access error (Masked status)"]
pub type HwaSafetyAccessErrStatusIpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_OPING_RAM` reader - 8:8\\]
Indicates the ACCEL_MEM2 access error (Masked status)"]
pub type HwaSafetyAccessErrStatusOpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_OPING_RAM` writer - 8:8\\]
Indicates the ACCEL_MEM2 access error (Masked status)"]
pub type HwaSafetyAccessErrStatusOpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_OPONG_RAM` reader - 9:9\\]
Indicates the ACCEL_MEM3 access error (Masked status)"]
pub type HwaSafetyAccessErrStatusOpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_STATUS_OPONG_RAM` writer - 9:9\\]
Indicates the ACCEL_MEM3 access error (Masked status)"]
pub type HwaSafetyAccessErrStatusOpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the FSM lockstep error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_err_status_fsm_lockstep(&self) -> HwaSafetyErrStatusFsmLockstepR {
        HwaSafetyErrStatusFsmLockstepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the Window RAM parity error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_window_ram(&self) -> HwaSafetyParityErrStatusWindowRamR {
        HwaSafetyParityErrStatusWindowRamR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates the ACCEL_MEM0 parity error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_iping_ram(&self) -> HwaSafetyParityErrStatusIpingRamR {
        HwaSafetyParityErrStatusIpingRamR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the ACCEL_MEM1 parity error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_ipong_ram(&self) -> HwaSafetyParityErrStatusIpongRamR {
        HwaSafetyParityErrStatusIpongRamR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates the ACCEL_MEM2 parity error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_oping_ram(&self) -> HwaSafetyParityErrStatusOpingRamR {
        HwaSafetyParityErrStatusOpingRamR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates the ACCEL_MEM3 parity error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_status_opong_ram(&self) -> HwaSafetyParityErrStatusOpongRamR {
        HwaSafetyParityErrStatusOpongRamR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates the ACCEL_MEM0 access error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_access_err_status_iping_ram(&self) -> HwaSafetyAccessErrStatusIpingRamR {
        HwaSafetyAccessErrStatusIpingRamR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates the ACCEL_MEM1 access error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_access_err_status_ipong_ram(&self) -> HwaSafetyAccessErrStatusIpongRamR {
        HwaSafetyAccessErrStatusIpongRamR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the ACCEL_MEM2 access error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_access_err_status_oping_ram(&self) -> HwaSafetyAccessErrStatusOpingRamR {
        HwaSafetyAccessErrStatusOpingRamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the ACCEL_MEM3 access error (Masked status)"]
    #[inline(always)]
    pub fn hwa_safety_access_err_status_opong_ram(&self) -> HwaSafetyAccessErrStatusOpongRamR {
        HwaSafetyAccessErrStatusOpongRamR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the FSM lockstep error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_err_status_fsm_lockstep(
        &mut self,
    ) -> HwaSafetyErrStatusFsmLockstepW<HwaSafetyErrStatusSpec> {
        HwaSafetyErrStatusFsmLockstepW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the Window RAM parity error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_window_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusWindowRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyParityErrStatusWindowRamW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates the ACCEL_MEM0 parity error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_iping_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusIpingRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyParityErrStatusIpingRamW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the ACCEL_MEM1 parity error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_ipong_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusIpongRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyParityErrStatusIpongRamW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates the ACCEL_MEM2 parity error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_oping_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusOpingRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyParityErrStatusOpingRamW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates the ACCEL_MEM3 parity error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_status_opong_ram(
        &mut self,
    ) -> HwaSafetyParityErrStatusOpongRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyParityErrStatusOpongRamW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates the ACCEL_MEM0 access error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_status_iping_ram(
        &mut self,
    ) -> HwaSafetyAccessErrStatusIpingRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyAccessErrStatusIpingRamW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates the ACCEL_MEM1 access error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_status_ipong_ram(
        &mut self,
    ) -> HwaSafetyAccessErrStatusIpongRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyAccessErrStatusIpongRamW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the ACCEL_MEM2 access error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_status_oping_ram(
        &mut self,
    ) -> HwaSafetyAccessErrStatusOpingRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyAccessErrStatusOpingRamW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the ACCEL_MEM3 access error (Masked status)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_status_opong_ram(
        &mut self,
    ) -> HwaSafetyAccessErrStatusOpongRamW<HwaSafetyErrStatusSpec> {
        HwaSafetyAccessErrStatusOpongRamW::new(self, 9)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<HwaSafetyErrStatusSpec> {
        Nu1W::new(self, 10)
    }
}
#[doc = "HWA_SAFETY_ERR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyErrStatusSpec;
impl crate::RegisterSpec for HwaSafetyErrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_err_status::R`](R) reader structure"]
impl crate::Readable for HwaSafetyErrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_err_status::W`](W) writer structure"]
impl crate::Writable for HwaSafetyErrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_ERR_STATUS to value 0"]
impl crate::Resettable for HwaSafetyErrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
