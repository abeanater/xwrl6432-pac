#[doc = "Register `DED_ENABLE_CLR_REG0` reader"]
pub type R = crate::R<DedEnableClrReg0Spec>;
#[doc = "Register `DED_ENABLE_CLR_REG0` writer"]
pub type W = crate::W<DedEnableClrReg0Spec>;
#[doc = "Field `APP_SS_ROM_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for app_ss_rom_pend - (RW )"]
pub type AppSsRomEnableClrR = crate::BitReader;
#[doc = "Field `APP_SS_ROM_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for app_ss_rom_pend - (RW )"]
pub type AppSsRomEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM1_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for app_ss_ram1_pend - (RW )"]
pub type AppSsRam1EnableClrR = crate::BitReader;
#[doc = "Field `APP_SS_RAM1_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for app_ss_ram1_pend - (RW )"]
pub type AppSsRam1EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM2_ENABLE_CLR` reader - 2:2\\]
Interrupt Enable Clear Register for app_ss_ram2_pend - (RW )"]
pub type AppSsRam2EnableClrR = crate::BitReader;
#[doc = "Field `APP_SS_RAM2_ENABLE_CLR` writer - 2:2\\]
Interrupt Enable Clear Register for app_ss_ram2_pend - (RW )"]
pub type AppSsRam2EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM3_ENABLE_CLR` reader - 3:3\\]
Interrupt Enable Clear Register for app_ss_ram3_pend - (RW )"]
pub type AppSsRam3EnableClrR = crate::BitReader;
#[doc = "Field `APP_SS_RAM3_ENABLE_CLR` writer - 3:3\\]
Interrupt Enable Clear Register for app_ss_ram3_pend - (RW )"]
pub type AppSsRam3EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_TPTC1_ENABLE_CLR` reader - 4:4\\]
Interrupt Enable Clear Register for app_ss_tptc1_pend - (RW )"]
pub type AppSsTptc1EnableClrR = crate::BitReader;
#[doc = "Field `APP_SS_TPTC1_ENABLE_CLR` writer - 4:4\\]
Interrupt Enable Clear Register for app_ss_tptc1_pend - (RW )"]
pub type AppSsTptc1EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_TPTC2_ENABLE_CLR` reader - 5:5\\]
Interrupt Enable Clear Register for app_ss_tptc2_pend - (RW )"]
pub type AppSsTptc2EnableClrR = crate::BitReader;
#[doc = "Field `APP_SS_TPTC2_ENABLE_CLR` writer - 5:5\\]
Interrupt Enable Clear Register for app_ss_tptc2_pend - (RW )"]
pub type AppSsTptc2EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_TPTC1_ENABLE_CLR` reader - 6:6\\]
Interrupt Enable Clear Register for hwa_tptc1_pend - (RW )"]
pub type HwaTptc1EnableClrR = crate::BitReader;
#[doc = "Field `HWA_TPTC1_ENABLE_CLR` writer - 6:6\\]
Interrupt Enable Clear Register for hwa_tptc1_pend - (RW )"]
pub type HwaTptc1EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_TPTC2_ENABLE_CLR` reader - 7:7\\]
Interrupt Enable Clear Register for hwa_tptc2_pend - (RW )"]
pub type HwaTptc2EnableClrR = crate::BitReader;
#[doc = "Field `HWA_TPTC2_ENABLE_CLR` writer - 7:7\\]
Interrupt Enable Clear Register for hwa_tptc2_pend - (RW )"]
pub type HwaTptc2EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM0_RAMECC_ENABLE_CLR` reader - 8:8\\]
Interrupt Enable Clear Register for shared_mem0_ramecc_pend - (RW )"]
pub type SharedMem0RameccEnableClrR = crate::BitReader;
#[doc = "Field `SHARED_MEM0_RAMECC_ENABLE_CLR` writer - 8:8\\]
Interrupt Enable Clear Register for shared_mem0_ramecc_pend - (RW )"]
pub type SharedMem0RameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM1_RAMECC_ENABLE_CLR` reader - 9:9\\]
Interrupt Enable Clear Register for shared_mem1_ramecc_pend - (RW )"]
pub type SharedMem1RameccEnableClrR = crate::BitReader;
#[doc = "Field `SHARED_MEM1_RAMECC_ENABLE_CLR` writer - 9:9\\]
Interrupt Enable Clear Register for shared_mem1_ramecc_pend - (RW )"]
pub type SharedMem1RameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_PARAM_MEM_ENABLE_CLR` reader - 10:10\\]
Interrupt Enable Clear Register for hwa_param_mem_pend - (RW )"]
pub type HwaParamMemEnableClrR = crate::BitReader;
#[doc = "Field `HWA_PARAM_MEM_ENABLE_CLR` writer - 10:10\\]
Interrupt Enable Clear Register for hwa_param_mem_pend - (RW )"]
pub type HwaParamMemEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM2_RAMECC_ENABLE_CLR` reader - 11:11\\]
Interrupt Enable Clear Register for shared_mem2_ramecc_pend - (RW )"]
pub type SharedMem2RameccEnableClrR = crate::BitReader;
#[doc = "Field `SHARED_MEM2_RAMECC_ENABLE_CLR` writer - 11:11\\]
Interrupt Enable Clear Register for shared_mem2_ramecc_pend - (RW )"]
pub type SharedMem2RameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPING_RAMECC_ENABLE_CLR` reader - 12:12\\]
Interrupt Enable Clear Register for adcping_ramecc_pend - (RW )"]
pub type AdcpingRameccEnableClrR = crate::BitReader;
#[doc = "Field `ADCPING_RAMECC_ENABLE_CLR` writer - 12:12\\]
Interrupt Enable Clear Register for adcping_ramecc_pend - (RW )"]
pub type AdcpingRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPONG_RAMECC_ENABLE_CLR` reader - 13:13\\]
Interrupt Enable Clear Register for adcpong_ramecc_pend - (RW )"]
pub type AdcpongRameccEnableClrR = crate::BitReader;
#[doc = "Field `ADCPONG_RAMECC_ENABLE_CLR` writer - 13:13\\]
Interrupt Enable Clear Register for adcpong_ramecc_pend - (RW )"]
pub type AdcpongRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES14` reader - 31:14\\]
RESERVE FIELD"]
pub type Res14R = crate::FieldReader<u32>;
#[doc = "Field `RES14` writer - 31:14\\]
RESERVE FIELD"]
pub type Res14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for app_ss_rom_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_rom_enable_clr(&self) -> AppSsRomEnableClrR {
        AppSsRomEnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for app_ss_ram1_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram1_enable_clr(&self) -> AppSsRam1EnableClrR {
        AppSsRam1EnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for app_ss_ram2_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram2_enable_clr(&self) -> AppSsRam2EnableClrR {
        AppSsRam2EnableClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for app_ss_ram3_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram3_enable_clr(&self) -> AppSsRam3EnableClrR {
        AppSsRam3EnableClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Clear Register for app_ss_tptc1_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_tptc1_enable_clr(&self) -> AppSsTptc1EnableClrR {
        AppSsTptc1EnableClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Clear Register for app_ss_tptc2_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_tptc2_enable_clr(&self) -> AppSsTptc2EnableClrR {
        AppSsTptc2EnableClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Clear Register for hwa_tptc1_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_tptc1_enable_clr(&self) -> HwaTptc1EnableClrR {
        HwaTptc1EnableClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Clear Register for hwa_tptc2_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_tptc2_enable_clr(&self) -> HwaTptc2EnableClrR {
        HwaTptc2EnableClrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Clear Register for shared_mem0_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem0_ramecc_enable_clr(&self) -> SharedMem0RameccEnableClrR {
        SharedMem0RameccEnableClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Clear Register for shared_mem1_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem1_ramecc_enable_clr(&self) -> SharedMem1RameccEnableClrR {
        SharedMem1RameccEnableClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Clear Register for hwa_param_mem_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_param_mem_enable_clr(&self) -> HwaParamMemEnableClrR {
        HwaParamMemEnableClrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Enable Clear Register for shared_mem2_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem2_ramecc_enable_clr(&self) -> SharedMem2RameccEnableClrR {
        SharedMem2RameccEnableClrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Enable Clear Register for adcping_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn adcping_ramecc_enable_clr(&self) -> AdcpingRameccEnableClrR {
        AdcpingRameccEnableClrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Enable Clear Register for adcpong_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn adcpong_ramecc_enable_clr(&self) -> AdcpongRameccEnableClrR {
        AdcpongRameccEnableClrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res14(&self) -> Res14R {
        Res14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for app_ss_rom_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_rom_enable_clr(&mut self) -> AppSsRomEnableClrW<DedEnableClrReg0Spec> {
        AppSsRomEnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for app_ss_ram1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram1_enable_clr(&mut self) -> AppSsRam1EnableClrW<DedEnableClrReg0Spec> {
        AppSsRam1EnableClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for app_ss_ram2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram2_enable_clr(&mut self) -> AppSsRam2EnableClrW<DedEnableClrReg0Spec> {
        AppSsRam2EnableClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for app_ss_ram3_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram3_enable_clr(&mut self) -> AppSsRam3EnableClrW<DedEnableClrReg0Spec> {
        AppSsRam3EnableClrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Clear Register for app_ss_tptc1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_tptc1_enable_clr(&mut self) -> AppSsTptc1EnableClrW<DedEnableClrReg0Spec> {
        AppSsTptc1EnableClrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Clear Register for app_ss_tptc2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_tptc2_enable_clr(&mut self) -> AppSsTptc2EnableClrW<DedEnableClrReg0Spec> {
        AppSsTptc2EnableClrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Clear Register for hwa_tptc1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_tptc1_enable_clr(&mut self) -> HwaTptc1EnableClrW<DedEnableClrReg0Spec> {
        HwaTptc1EnableClrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Clear Register for hwa_tptc2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_tptc2_enable_clr(&mut self) -> HwaTptc2EnableClrW<DedEnableClrReg0Spec> {
        HwaTptc2EnableClrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Clear Register for shared_mem0_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem0_ramecc_enable_clr(
        &mut self,
    ) -> SharedMem0RameccEnableClrW<DedEnableClrReg0Spec> {
        SharedMem0RameccEnableClrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Clear Register for shared_mem1_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem1_ramecc_enable_clr(
        &mut self,
    ) -> SharedMem1RameccEnableClrW<DedEnableClrReg0Spec> {
        SharedMem1RameccEnableClrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Clear Register for hwa_param_mem_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_param_mem_enable_clr(&mut self) -> HwaParamMemEnableClrW<DedEnableClrReg0Spec> {
        HwaParamMemEnableClrW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Enable Clear Register for shared_mem2_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem2_ramecc_enable_clr(
        &mut self,
    ) -> SharedMem2RameccEnableClrW<DedEnableClrReg0Spec> {
        SharedMem2RameccEnableClrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Enable Clear Register for adcping_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn adcping_ramecc_enable_clr(&mut self) -> AdcpingRameccEnableClrW<DedEnableClrReg0Spec> {
        AdcpingRameccEnableClrW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Enable Clear Register for adcpong_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn adcpong_ramecc_enable_clr(&mut self) -> AdcpongRameccEnableClrW<DedEnableClrReg0Spec> {
        AdcpongRameccEnableClrW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res14(&mut self) -> Res14W<DedEnableClrReg0Spec> {
        Res14W::new(self, 14)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_enable_clr_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_enable_clr_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedEnableClrReg0Spec;
impl crate::RegisterSpec for DedEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for DedEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ded_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for DedEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DED_ENABLE_CLR_REG0 to value 0"]
impl crate::Resettable for DedEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
