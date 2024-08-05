#[doc = "Register `HWA_SAFETY_ENABLE` reader"]
pub type R = crate::R<HwaSafetyEnableSpec>;
#[doc = "Register `HWA_SAFETY_ENABLE` writer"]
pub type W = crate::W<HwaSafetyEnableSpec>;
#[doc = "Field `WIN_RAM_PARITY_EN` reader - 0:0\\]
1: Enable PARITY for Window RAM"]
pub type WinRamParityEnR = crate::BitReader;
#[doc = "Field `WIN_RAM_PARITY_EN` writer - 0:0\\]
1: Enable PARITY for Window RAM"]
pub type WinRamParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARAM_ECC_EN` reader - 1:1\\]
Not used."]
pub type ParamEccEnR = crate::BitReader;
#[doc = "Field `PARAM_ECC_EN` writer - 1:1\\]
Not used."]
pub type ParamEccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u16>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `IPING_PARITY_EN` reader - 12:12\\]
1: Enable PARITY for ACCEL_MEM0"]
pub type IpingParityEnR = crate::BitReader;
#[doc = "Field `IPING_PARITY_EN` writer - 12:12\\]
1: Enable PARITY for ACCEL_MEM0"]
pub type IpingParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPONG_PARITY_EN` reader - 13:13\\]
1: Enable PARITY for ACCEL_MEM1"]
pub type IpongParityEnR = crate::BitReader;
#[doc = "Field `IPONG_PARITY_EN` writer - 13:13\\]
1: Enable PARITY for ACCEL_MEM1"]
pub type IpongParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPING_PARITY_EN` reader - 14:14\\]
1: Enable PARITY for ACCEL_MEM2"]
pub type OpingParityEnR = crate::BitReader;
#[doc = "Field `OPING_PARITY_EN` writer - 14:14\\]
1: Enable PARITY for ACCEL_MEM2"]
pub type OpingParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPONG_PARITY_EN` reader - 15:15\\]
1: Enable PARITY for ACCEL_MEM3"]
pub type OpongParityEnR = crate::BitReader;
#[doc = "Field `OPONG_PARITY_EN` writer - 15:15\\]
1: Enable PARITY for ACCEL_MEM3"]
pub type OpongParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSM_LOCKSTEP_EN` reader - 16:16\\]
1: Enable Lockstep for Accelerator FSM"]
pub type FsmLockstepEnR = crate::BitReader;
#[doc = "Field `FSM_LOCKSTEP_EN` writer - 16:16\\]
1: Enable Lockstep for Accelerator FSM"]
pub type FsmLockstepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSM_LOCKSTEP_SELFTEST_EN` reader - 17:17\\]
1: Enable Selftest for Accelerator FSM"]
pub type FsmLockstepSelftestEnR = crate::BitReader;
#[doc = "Field `FSM_LOCKSTEP_SELFTEST_EN` writer - 17:17\\]
1: Enable Selftest for Accelerator FSM"]
pub type FsmLockstepSelftestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader<u16>;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Enable PARITY for Window RAM"]
    #[inline(always)]
    pub fn win_ram_parity_en(&self) -> WinRamParityEnR {
        WinRamParityEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Not used."]
    #[inline(always)]
    pub fn param_ecc_en(&self) -> ParamEccEnR {
        ParamEccEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - 12:12\\]
1: Enable PARITY for ACCEL_MEM0"]
    #[inline(always)]
    pub fn iping_parity_en(&self) -> IpingParityEnR {
        IpingParityEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
1: Enable PARITY for ACCEL_MEM1"]
    #[inline(always)]
    pub fn ipong_parity_en(&self) -> IpongParityEnR {
        IpongParityEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
1: Enable PARITY for ACCEL_MEM2"]
    #[inline(always)]
    pub fn oping_parity_en(&self) -> OpingParityEnR {
        OpingParityEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
1: Enable PARITY for ACCEL_MEM3"]
    #[inline(always)]
    pub fn opong_parity_en(&self) -> OpongParityEnR {
        OpongParityEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
1: Enable Lockstep for Accelerator FSM"]
    #[inline(always)]
    pub fn fsm_lockstep_en(&self) -> FsmLockstepEnR {
        FsmLockstepEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
1: Enable Selftest for Accelerator FSM"]
    #[inline(always)]
    pub fn fsm_lockstep_selftest_en(&self) -> FsmLockstepSelftestEnR {
        FsmLockstepSelftestEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Enable PARITY for Window RAM"]
    #[inline(always)]
    #[must_use]
    pub fn win_ram_parity_en(&mut self) -> WinRamParityEnW<HwaSafetyEnableSpec> {
        WinRamParityEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Not used."]
    #[inline(always)]
    #[must_use]
    pub fn param_ecc_en(&mut self) -> ParamEccEnW<HwaSafetyEnableSpec> {
        ParamEccEnW::new(self, 1)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<HwaSafetyEnableSpec> {
        Nu1W::new(self, 2)
    }
    #[doc = "Bit 12 - 12:12\\]
1: Enable PARITY for ACCEL_MEM0"]
    #[inline(always)]
    #[must_use]
    pub fn iping_parity_en(&mut self) -> IpingParityEnW<HwaSafetyEnableSpec> {
        IpingParityEnW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
1: Enable PARITY for ACCEL_MEM1"]
    #[inline(always)]
    #[must_use]
    pub fn ipong_parity_en(&mut self) -> IpongParityEnW<HwaSafetyEnableSpec> {
        IpongParityEnW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
1: Enable PARITY for ACCEL_MEM2"]
    #[inline(always)]
    #[must_use]
    pub fn oping_parity_en(&mut self) -> OpingParityEnW<HwaSafetyEnableSpec> {
        OpingParityEnW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
1: Enable PARITY for ACCEL_MEM3"]
    #[inline(always)]
    #[must_use]
    pub fn opong_parity_en(&mut self) -> OpongParityEnW<HwaSafetyEnableSpec> {
        OpongParityEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
1: Enable Lockstep for Accelerator FSM"]
    #[inline(always)]
    #[must_use]
    pub fn fsm_lockstep_en(&mut self) -> FsmLockstepEnW<HwaSafetyEnableSpec> {
        FsmLockstepEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
1: Enable Selftest for Accelerator FSM"]
    #[inline(always)]
    #[must_use]
    pub fn fsm_lockstep_selftest_en(&mut self) -> FsmLockstepSelftestEnW<HwaSafetyEnableSpec> {
        FsmLockstepSelftestEnW::new(self, 17)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<HwaSafetyEnableSpec> {
        Nu2W::new(self, 18)
    }
}
#[doc = "HWA_SAFETY_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyEnableSpec;
impl crate::RegisterSpec for HwaSafetyEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_enable::R`](R) reader structure"]
impl crate::Readable for HwaSafetyEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_enable::W`](W) writer structure"]
impl crate::Writable for HwaSafetyEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_ENABLE to value 0"]
impl crate::Resettable for HwaSafetyEnableSpec {
    const RESET_VALUE: u32 = 0;
}
