#[doc = "Register `SEC_ENABLE_SET_REG0` reader"]
pub type R = crate::R<SecEnableSetReg0Spec>;
#[doc = "Register `SEC_ENABLE_SET_REG0` writer"]
pub type W = crate::W<SecEnableSetReg0Spec>;
#[doc = "Field `APP_SS_ROM_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for app_ss_rom_pend - (RW )"]
pub type AppSsRomEnableSetR = crate::BitReader;
#[doc = "Field `APP_SS_ROM_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for app_ss_rom_pend - (RW )"]
pub type AppSsRomEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM1_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for app_ss_ram1_pend - (RW )"]
pub type AppSsRam1EnableSetR = crate::BitReader;
#[doc = "Field `APP_SS_RAM1_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for app_ss_ram1_pend - (RW )"]
pub type AppSsRam1EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM2_ENABLE_SET` reader - 2:2\\]
Interrupt Enable Set Register for app_ss_ram2_pend - (RW )"]
pub type AppSsRam2EnableSetR = crate::BitReader;
#[doc = "Field `APP_SS_RAM2_ENABLE_SET` writer - 2:2\\]
Interrupt Enable Set Register for app_ss_ram2_pend - (RW )"]
pub type AppSsRam2EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM3_ENABLE_SET` reader - 3:3\\]
Interrupt Enable Set Register for app_ss_ram3_pend - (RW )"]
pub type AppSsRam3EnableSetR = crate::BitReader;
#[doc = "Field `APP_SS_RAM3_ENABLE_SET` writer - 3:3\\]
Interrupt Enable Set Register for app_ss_ram3_pend - (RW )"]
pub type AppSsRam3EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_TPTC1_ENABLE_SET` reader - 4:4\\]
Interrupt Enable Set Register for app_ss_tptc1_pend - (RW )"]
pub type AppSsTptc1EnableSetR = crate::BitReader;
#[doc = "Field `APP_SS_TPTC1_ENABLE_SET` writer - 4:4\\]
Interrupt Enable Set Register for app_ss_tptc1_pend - (RW )"]
pub type AppSsTptc1EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_TPTC2_ENABLE_SET` reader - 5:5\\]
Interrupt Enable Set Register for app_ss_tptc2_pend - (RW )"]
pub type AppSsTptc2EnableSetR = crate::BitReader;
#[doc = "Field `APP_SS_TPTC2_ENABLE_SET` writer - 5:5\\]
Interrupt Enable Set Register for app_ss_tptc2_pend - (RW )"]
pub type AppSsTptc2EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_TPTC1_ENABLE_SET` reader - 6:6\\]
Interrupt Enable Set Register for hwa_tptc1_pend - (RW )"]
pub type HwaTptc1EnableSetR = crate::BitReader;
#[doc = "Field `HWA_TPTC1_ENABLE_SET` writer - 6:6\\]
Interrupt Enable Set Register for hwa_tptc1_pend - (RW )"]
pub type HwaTptc1EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_TPTC2_ENABLE_SET` reader - 7:7\\]
Interrupt Enable Set Register for hwa_tptc2_pend - (RW )"]
pub type HwaTptc2EnableSetR = crate::BitReader;
#[doc = "Field `HWA_TPTC2_ENABLE_SET` writer - 7:7\\]
Interrupt Enable Set Register for hwa_tptc2_pend - (RW )"]
pub type HwaTptc2EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM0_RAMECC_ENABLE_SET` reader - 8:8\\]
Interrupt Enable Set Register for shared_mem0_ramecc_pend - (RW )"]
pub type SharedMem0RameccEnableSetR = crate::BitReader;
#[doc = "Field `SHARED_MEM0_RAMECC_ENABLE_SET` writer - 8:8\\]
Interrupt Enable Set Register for shared_mem0_ramecc_pend - (RW )"]
pub type SharedMem0RameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM1_RAMECC_ENABLE_SET` reader - 9:9\\]
Interrupt Enable Set Register for shared_mem1_ramecc_pend - (RW )"]
pub type SharedMem1RameccEnableSetR = crate::BitReader;
#[doc = "Field `SHARED_MEM1_RAMECC_ENABLE_SET` writer - 9:9\\]
Interrupt Enable Set Register for shared_mem1_ramecc_pend - (RW )"]
pub type SharedMem1RameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_PARAM_MEM_ENABLE_SET` reader - 10:10\\]
Interrupt Enable Set Register for hwa_param_mem_pend - (RW )"]
pub type HwaParamMemEnableSetR = crate::BitReader;
#[doc = "Field `HWA_PARAM_MEM_ENABLE_SET` writer - 10:10\\]
Interrupt Enable Set Register for hwa_param_mem_pend - (RW )"]
pub type HwaParamMemEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM2_RAMECC_ENABLE_SET` reader - 11:11\\]
Interrupt Enable Set Register for shared_mem2_ramecc_pend - (RW )"]
pub type SharedMem2RameccEnableSetR = crate::BitReader;
#[doc = "Field `SHARED_MEM2_RAMECC_ENABLE_SET` writer - 11:11\\]
Interrupt Enable Set Register for shared_mem2_ramecc_pend - (RW )"]
pub type SharedMem2RameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPING_RAMECC_ENABLE_SET` reader - 12:12\\]
Interrupt Enable Set Register for adcping_ramecc_pend - (RW )"]
pub type AdcpingRameccEnableSetR = crate::BitReader;
#[doc = "Field `ADCPING_RAMECC_ENABLE_SET` writer - 12:12\\]
Interrupt Enable Set Register for adcping_ramecc_pend - (RW )"]
pub type AdcpingRameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPONG_RAMECC_ENABLE_SET` reader - 13:13\\]
Interrupt Enable Set Register for adcpong_ramecc_pend - (RW )"]
pub type AdcpongRameccEnableSetR = crate::BitReader;
#[doc = "Field `ADCPONG_RAMECC_ENABLE_SET` writer - 13:13\\]
Interrupt Enable Set Register for adcpong_ramecc_pend - (RW )"]
pub type AdcpongRameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES9` reader - 31:14\\]
RESERVE FIELD"]
pub type Res9R = crate::FieldReader<u32>;
#[doc = "Field `RES9` writer - 31:14\\]
RESERVE FIELD"]
pub type Res9W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for app_ss_rom_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_rom_enable_set(&self) -> AppSsRomEnableSetR {
        AppSsRomEnableSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for app_ss_ram1_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram1_enable_set(&self) -> AppSsRam1EnableSetR {
        AppSsRam1EnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for app_ss_ram2_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram2_enable_set(&self) -> AppSsRam2EnableSetR {
        AppSsRam2EnableSetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for app_ss_ram3_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram3_enable_set(&self) -> AppSsRam3EnableSetR {
        AppSsRam3EnableSetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Set Register for app_ss_tptc1_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_tptc1_enable_set(&self) -> AppSsTptc1EnableSetR {
        AppSsTptc1EnableSetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Set Register for app_ss_tptc2_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_tptc2_enable_set(&self) -> AppSsTptc2EnableSetR {
        AppSsTptc2EnableSetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Set Register for hwa_tptc1_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_tptc1_enable_set(&self) -> HwaTptc1EnableSetR {
        HwaTptc1EnableSetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Set Register for hwa_tptc2_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_tptc2_enable_set(&self) -> HwaTptc2EnableSetR {
        HwaTptc2EnableSetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Set Register for shared_mem0_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem0_ramecc_enable_set(&self) -> SharedMem0RameccEnableSetR {
        SharedMem0RameccEnableSetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Set Register for shared_mem1_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem1_ramecc_enable_set(&self) -> SharedMem1RameccEnableSetR {
        SharedMem1RameccEnableSetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Set Register for hwa_param_mem_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_param_mem_enable_set(&self) -> HwaParamMemEnableSetR {
        HwaParamMemEnableSetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Enable Set Register for shared_mem2_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem2_ramecc_enable_set(&self) -> SharedMem2RameccEnableSetR {
        SharedMem2RameccEnableSetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Enable Set Register for adcping_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn adcping_ramecc_enable_set(&self) -> AdcpingRameccEnableSetR {
        AdcpingRameccEnableSetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Enable Set Register for adcpong_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn adcpong_ramecc_enable_set(&self) -> AdcpongRameccEnableSetR {
        AdcpongRameccEnableSetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res9(&self) -> Res9R {
        Res9R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for app_ss_rom_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_rom_enable_set(&mut self) -> AppSsRomEnableSetW<SecEnableSetReg0Spec> {
        AppSsRomEnableSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for app_ss_ram1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram1_enable_set(&mut self) -> AppSsRam1EnableSetW<SecEnableSetReg0Spec> {
        AppSsRam1EnableSetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for app_ss_ram2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram2_enable_set(&mut self) -> AppSsRam2EnableSetW<SecEnableSetReg0Spec> {
        AppSsRam2EnableSetW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for app_ss_ram3_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram3_enable_set(&mut self) -> AppSsRam3EnableSetW<SecEnableSetReg0Spec> {
        AppSsRam3EnableSetW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Set Register for app_ss_tptc1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_tptc1_enable_set(&mut self) -> AppSsTptc1EnableSetW<SecEnableSetReg0Spec> {
        AppSsTptc1EnableSetW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Set Register for app_ss_tptc2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_tptc2_enable_set(&mut self) -> AppSsTptc2EnableSetW<SecEnableSetReg0Spec> {
        AppSsTptc2EnableSetW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Set Register for hwa_tptc1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_tptc1_enable_set(&mut self) -> HwaTptc1EnableSetW<SecEnableSetReg0Spec> {
        HwaTptc1EnableSetW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Set Register for hwa_tptc2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_tptc2_enable_set(&mut self) -> HwaTptc2EnableSetW<SecEnableSetReg0Spec> {
        HwaTptc2EnableSetW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Set Register for shared_mem0_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem0_ramecc_enable_set(
        &mut self,
    ) -> SharedMem0RameccEnableSetW<SecEnableSetReg0Spec> {
        SharedMem0RameccEnableSetW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Set Register for shared_mem1_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem1_ramecc_enable_set(
        &mut self,
    ) -> SharedMem1RameccEnableSetW<SecEnableSetReg0Spec> {
        SharedMem1RameccEnableSetW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Set Register for hwa_param_mem_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_param_mem_enable_set(&mut self) -> HwaParamMemEnableSetW<SecEnableSetReg0Spec> {
        HwaParamMemEnableSetW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Enable Set Register for shared_mem2_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem2_ramecc_enable_set(
        &mut self,
    ) -> SharedMem2RameccEnableSetW<SecEnableSetReg0Spec> {
        SharedMem2RameccEnableSetW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Enable Set Register for adcping_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn adcping_ramecc_enable_set(&mut self) -> AdcpingRameccEnableSetW<SecEnableSetReg0Spec> {
        AdcpingRameccEnableSetW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Enable Set Register for adcpong_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn adcpong_ramecc_enable_set(&mut self) -> AdcpongRameccEnableSetW<SecEnableSetReg0Spec> {
        AdcpongRameccEnableSetW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res9(&mut self) -> Res9W<SecEnableSetReg0Spec> {
        Res9W::new(self, 14)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableSetReg0Spec;
impl crate::RegisterSpec for SecEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for SecEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for SecEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_ENABLE_SET_REG0 to value 0"]
impl crate::Resettable for SecEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
