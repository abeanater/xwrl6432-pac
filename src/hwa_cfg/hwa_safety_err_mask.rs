#[doc = "Register `HWA_SAFETY_ERR_MASK` reader"]
pub type R = crate::R<HwaSafetyErrMaskSpec>;
#[doc = "Register `HWA_SAFETY_ERR_MASK` writer"]
pub type W = crate::W<HwaSafetyErrMaskSpec>;
#[doc = "Field `HWA_SAFETY_ERR_MASK_FSM_LOCKSTEP` reader - 0:0\\]
When 1'b1 : FSM lockstep error is masked.1'b0 : FSM lockstep error is not masked"]
pub type HwaSafetyErrMaskFsmLockstepR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ERR_MASK_FSM_LOCKSTEP` writer - 0:0\\]
When 1'b1 : FSM lockstep error is masked.1'b0 : FSM lockstep error is not masked"]
pub type HwaSafetyErrMaskFsmLockstepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_WINDOW_RAM` reader - 1:1\\]
When 1'b1 : Window RAM parity error is masked.1'b0 : Window RAM parity error is not masked"]
pub type HwaSafetyParityErrMaskWindowRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_WINDOW_RAM` writer - 1:1\\]
When 1'b1 : Window RAM parity error is masked.1'b0 : Window RAM parity error is not masked"]
pub type HwaSafetyParityErrMaskWindowRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_IPING_RAM` reader - 2:2\\]
When 1'b1 : ACCEL_MEM0 parity error is masked.1'b0 : ACCEL_MEM0 parity error is not masked"]
pub type HwaSafetyParityErrMaskIpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_IPING_RAM` writer - 2:2\\]
When 1'b1 : ACCEL_MEM0 parity error is masked.1'b0 : ACCEL_MEM0 parity error is not masked"]
pub type HwaSafetyParityErrMaskIpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_IPONG_RAM` reader - 3:3\\]
When 1'b1 : ACCEL_MEM1 parity error is masked.1'b0 : ACCEL_MEM1 parity error is not masked"]
pub type HwaSafetyParityErrMaskIpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_IPONG_RAM` writer - 3:3\\]
When 1'b1 : ACCEL_MEM1 parity error is masked.1'b0 : ACCEL_MEM1 parity error is not masked"]
pub type HwaSafetyParityErrMaskIpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_OPING_RAM` reader - 4:4\\]
When 1'b1 : ACCEL_MEM2 parity error is masked.1'b0 : ACCEL_MEM2 parity error is not masked"]
pub type HwaSafetyParityErrMaskOpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_OPING_RAM` writer - 4:4\\]
When 1'b1 : ACCEL_MEM2 parity error is masked.1'b0 : ACCEL_MEM2 parity error is not masked"]
pub type HwaSafetyParityErrMaskOpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_OPONG_RAM` reader - 5:5\\]
When 1'b1 : ACCEL_MEM3 parity error is masked.1'b0 : ACCEL_MEM03 parity error is not masked"]
pub type HwaSafetyParityErrMaskOpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_PARITY_ERR_MASK_OPONG_RAM` writer - 5:5\\]
When 1'b1 : ACCEL_MEM3 parity error is masked.1'b0 : ACCEL_MEM03 parity error is not masked"]
pub type HwaSafetyParityErrMaskOpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_MASK_IPING_RAM` reader - 6:6\\]
When 1'b1 : ACCEL_MEM0 access error is masked.1'b0 : ACCEL_MEM0 access error is not masked"]
pub type HwaSafetyAccessErrMaskIpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_MASK_IPING_RAM` writer - 6:6\\]
When 1'b1 : ACCEL_MEM0 access error is masked.1'b0 : ACCEL_MEM0 access error is not masked"]
pub type HwaSafetyAccessErrMaskIpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_MASK_IPONG_RAM` reader - 7:7\\]
When 1'b1 : ACCEL_MEM1 access error is masked.1'b0 : ACCEL_MEM1 access error is not masked"]
pub type HwaSafetyAccessErrMaskIpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_MASK_IPONG_RAM` writer - 7:7\\]
When 1'b1 : ACCEL_MEM1 access error is masked.1'b0 : ACCEL_MEM1 access error is not masked"]
pub type HwaSafetyAccessErrMaskIpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_MASK_OPING_RAM` reader - 8:8\\]
When 1'b1 : ACCEL_MEM2 access error is masked.1'b0 : ACCEL_MEM2 access error is not masked"]
pub type HwaSafetyAccessErrMaskOpingRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_MASK_OPING_RAM` writer - 8:8\\]
When 1'b1 : ACCEL_MEM2 access error is masked.1'b0 : ACCEL_MEM2 access error is not masked"]
pub type HwaSafetyAccessErrMaskOpingRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_MASK_OPONG_RAM` reader - 9:9\\]
When 1'b1 : ACCEL_MEM3 access error is masked.1'b0 : ACCEL_MEM3 access error is not masked"]
pub type HwaSafetyAccessErrMaskOpongRamR = crate::BitReader;
#[doc = "Field `HWA_SAFETY_ACCESS_ERR_MASK_OPONG_RAM` writer - 9:9\\]
When 1'b1 : ACCEL_MEM3 access error is masked.1'b0 : ACCEL_MEM3 access error is not masked"]
pub type HwaSafetyAccessErrMaskOpongRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When 1'b1 : FSM lockstep error is masked.1'b0 : FSM lockstep error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_err_mask_fsm_lockstep(&self) -> HwaSafetyErrMaskFsmLockstepR {
        HwaSafetyErrMaskFsmLockstepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When 1'b1 : Window RAM parity error is masked.1'b0 : Window RAM parity error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_mask_window_ram(&self) -> HwaSafetyParityErrMaskWindowRamR {
        HwaSafetyParityErrMaskWindowRamR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When 1'b1 : ACCEL_MEM0 parity error is masked.1'b0 : ACCEL_MEM0 parity error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_mask_iping_ram(&self) -> HwaSafetyParityErrMaskIpingRamR {
        HwaSafetyParityErrMaskIpingRamR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When 1'b1 : ACCEL_MEM1 parity error is masked.1'b0 : ACCEL_MEM1 parity error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_mask_ipong_ram(&self) -> HwaSafetyParityErrMaskIpongRamR {
        HwaSafetyParityErrMaskIpongRamR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When 1'b1 : ACCEL_MEM2 parity error is masked.1'b0 : ACCEL_MEM2 parity error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_mask_oping_ram(&self) -> HwaSafetyParityErrMaskOpingRamR {
        HwaSafetyParityErrMaskOpingRamR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
When 1'b1 : ACCEL_MEM3 parity error is masked.1'b0 : ACCEL_MEM03 parity error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_parity_err_mask_opong_ram(&self) -> HwaSafetyParityErrMaskOpongRamR {
        HwaSafetyParityErrMaskOpongRamR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
When 1'b1 : ACCEL_MEM0 access error is masked.1'b0 : ACCEL_MEM0 access error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_access_err_mask_iping_ram(&self) -> HwaSafetyAccessErrMaskIpingRamR {
        HwaSafetyAccessErrMaskIpingRamR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
When 1'b1 : ACCEL_MEM1 access error is masked.1'b0 : ACCEL_MEM1 access error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_access_err_mask_ipong_ram(&self) -> HwaSafetyAccessErrMaskIpongRamR {
        HwaSafetyAccessErrMaskIpongRamR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
When 1'b1 : ACCEL_MEM2 access error is masked.1'b0 : ACCEL_MEM2 access error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_access_err_mask_oping_ram(&self) -> HwaSafetyAccessErrMaskOpingRamR {
        HwaSafetyAccessErrMaskOpingRamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
When 1'b1 : ACCEL_MEM3 access error is masked.1'b0 : ACCEL_MEM3 access error is not masked"]
    #[inline(always)]
    pub fn hwa_safety_access_err_mask_opong_ram(&self) -> HwaSafetyAccessErrMaskOpongRamR {
        HwaSafetyAccessErrMaskOpongRamR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When 1'b1 : FSM lockstep error is masked.1'b0 : FSM lockstep error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_err_mask_fsm_lockstep(
        &mut self,
    ) -> HwaSafetyErrMaskFsmLockstepW<HwaSafetyErrMaskSpec> {
        HwaSafetyErrMaskFsmLockstepW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When 1'b1 : Window RAM parity error is masked.1'b0 : Window RAM parity error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_mask_window_ram(
        &mut self,
    ) -> HwaSafetyParityErrMaskWindowRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyParityErrMaskWindowRamW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
When 1'b1 : ACCEL_MEM0 parity error is masked.1'b0 : ACCEL_MEM0 parity error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_mask_iping_ram(
        &mut self,
    ) -> HwaSafetyParityErrMaskIpingRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyParityErrMaskIpingRamW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
When 1'b1 : ACCEL_MEM1 parity error is masked.1'b0 : ACCEL_MEM1 parity error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_mask_ipong_ram(
        &mut self,
    ) -> HwaSafetyParityErrMaskIpongRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyParityErrMaskIpongRamW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
When 1'b1 : ACCEL_MEM2 parity error is masked.1'b0 : ACCEL_MEM2 parity error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_mask_oping_ram(
        &mut self,
    ) -> HwaSafetyParityErrMaskOpingRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyParityErrMaskOpingRamW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
When 1'b1 : ACCEL_MEM3 parity error is masked.1'b0 : ACCEL_MEM03 parity error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_parity_err_mask_opong_ram(
        &mut self,
    ) -> HwaSafetyParityErrMaskOpongRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyParityErrMaskOpongRamW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
When 1'b1 : ACCEL_MEM0 access error is masked.1'b0 : ACCEL_MEM0 access error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_mask_iping_ram(
        &mut self,
    ) -> HwaSafetyAccessErrMaskIpingRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyAccessErrMaskIpingRamW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
When 1'b1 : ACCEL_MEM1 access error is masked.1'b0 : ACCEL_MEM1 access error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_mask_ipong_ram(
        &mut self,
    ) -> HwaSafetyAccessErrMaskIpongRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyAccessErrMaskIpongRamW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
When 1'b1 : ACCEL_MEM2 access error is masked.1'b0 : ACCEL_MEM2 access error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_mask_oping_ram(
        &mut self,
    ) -> HwaSafetyAccessErrMaskOpingRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyAccessErrMaskOpingRamW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
When 1'b1 : ACCEL_MEM3 access error is masked.1'b0 : ACCEL_MEM3 access error is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_access_err_mask_opong_ram(
        &mut self,
    ) -> HwaSafetyAccessErrMaskOpongRamW<HwaSafetyErrMaskSpec> {
        HwaSafetyAccessErrMaskOpongRamW::new(self, 9)
    }
    #[doc = "Bits 10:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<HwaSafetyErrMaskSpec> {
        Nu1W::new(self, 10)
    }
}
#[doc = "HWA_SAFETY_ERR_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyErrMaskSpec;
impl crate::RegisterSpec for HwaSafetyErrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_err_mask::R`](R) reader structure"]
impl crate::Readable for HwaSafetyErrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_err_mask::W`](W) writer structure"]
impl crate::Writable for HwaSafetyErrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_ERR_MASK to value 0"]
impl crate::Resettable for HwaSafetyErrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
