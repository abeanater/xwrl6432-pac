#[doc = "Register `DED_STATUS_REG0` reader"]
pub type R = crate::R<DedStatusReg0Spec>;
#[doc = "Register `DED_STATUS_REG0` writer"]
pub type W = crate::W<DedStatusReg0Spec>;
#[doc = "Field `APP_SS_ROM_PEND` reader - 0:0\\]
Interrupt Pending Status for app_ss_rom_pend - (RW )"]
pub type AppSsRomPendR = crate::BitReader;
#[doc = "Field `APP_SS_ROM_PEND` writer - 0:0\\]
Interrupt Pending Status for app_ss_rom_pend - (RW )"]
pub type AppSsRomPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM1_PEND` reader - 1:1\\]
Interrupt Pending Status for app_ss_ram1_pend - (RW )"]
pub type AppSsRam1PendR = crate::BitReader;
#[doc = "Field `APP_SS_RAM1_PEND` writer - 1:1\\]
Interrupt Pending Status for app_ss_ram1_pend - (RW )"]
pub type AppSsRam1PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM2_PEND` reader - 2:2\\]
Interrupt Pending Status for app_ss_ram2_pend - (RW )"]
pub type AppSsRam2PendR = crate::BitReader;
#[doc = "Field `APP_SS_RAM2_PEND` writer - 2:2\\]
Interrupt Pending Status for app_ss_ram2_pend - (RW )"]
pub type AppSsRam2PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_RAM3_PEND` reader - 3:3\\]
Interrupt Pending Status for app_ss_ram3_pend - (RW )"]
pub type AppSsRam3PendR = crate::BitReader;
#[doc = "Field `APP_SS_RAM3_PEND` writer - 3:3\\]
Interrupt Pending Status for app_ss_ram3_pend - (RW )"]
pub type AppSsRam3PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_TPTC1_PEND` reader - 4:4\\]
Interrupt Pending Status for app_ss_tptc1_pend - (RW )"]
pub type AppSsTptc1PendR = crate::BitReader;
#[doc = "Field `APP_SS_TPTC1_PEND` writer - 4:4\\]
Interrupt Pending Status for app_ss_tptc1_pend - (RW )"]
pub type AppSsTptc1PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_SS_TPTC2_PEND` reader - 5:5\\]
Interrupt Pending Status for app_ss_tptc2_pend - (RW )"]
pub type AppSsTptc2PendR = crate::BitReader;
#[doc = "Field `APP_SS_TPTC2_PEND` writer - 5:5\\]
Interrupt Pending Status for app_ss_tptc2_pend - (RW )"]
pub type AppSsTptc2PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_TPTC1_PEND` reader - 6:6\\]
Interrupt Pending Status for hwa_tptc1_pend - (RW )"]
pub type HwaTptc1PendR = crate::BitReader;
#[doc = "Field `HWA_TPTC1_PEND` writer - 6:6\\]
Interrupt Pending Status for hwa_tptc1_pend - (RW )"]
pub type HwaTptc1PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_TPTC2_PEND` reader - 7:7\\]
Interrupt Pending Status for hwa_tptc2_pend - (RW )"]
pub type HwaTptc2PendR = crate::BitReader;
#[doc = "Field `HWA_TPTC2_PEND` writer - 7:7\\]
Interrupt Pending Status for hwa_tptc2_pend - (RW )"]
pub type HwaTptc2PendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM0_RAMECC_PEND` reader - 8:8\\]
Interrupt Pending Status for shared_mem0_ramecc_pend - (RW )"]
pub type SharedMem0RameccPendR = crate::BitReader;
#[doc = "Field `SHARED_MEM0_RAMECC_PEND` writer - 8:8\\]
Interrupt Pending Status for shared_mem0_ramecc_pend - (RW )"]
pub type SharedMem0RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM1_RAMECC_PEND` reader - 9:9\\]
Interrupt Pending Status for shared_mem1_ramecc_pend - (RW )"]
pub type SharedMem1RameccPendR = crate::BitReader;
#[doc = "Field `SHARED_MEM1_RAMECC_PEND` writer - 9:9\\]
Interrupt Pending Status for shared_mem1_ramecc_pend - (RW )"]
pub type SharedMem1RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWA_PARAM_MEM_PEND` reader - 10:10\\]
Interrupt Pending Status for hwa_param_mem_pend - (RW )"]
pub type HwaParamMemPendR = crate::BitReader;
#[doc = "Field `HWA_PARAM_MEM_PEND` writer - 10:10\\]
Interrupt Pending Status for hwa_param_mem_pend - (RW )"]
pub type HwaParamMemPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARED_MEM2_RAMECC_PEND` reader - 11:11\\]
Interrupt Pending Status for shared_mem2_ramecc_pend - (RW )"]
pub type SharedMem2RameccPendR = crate::BitReader;
#[doc = "Field `SHARED_MEM2_RAMECC_PEND` writer - 11:11\\]
Interrupt Pending Status for shared_mem2_ramecc_pend - (RW )"]
pub type SharedMem2RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPING_RAMECC_PEND` reader - 12:12\\]
Interrupt Pending Status for adcping_ramecc_pend - (RW )"]
pub type AdcpingRameccPendR = crate::BitReader;
#[doc = "Field `ADCPING_RAMECC_PEND` writer - 12:12\\]
Interrupt Pending Status for adcping_ramecc_pend - (RW )"]
pub type AdcpingRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPONG_RAMECC_PEND` reader - 13:13\\]
Interrupt Pending Status for adcpong_ramecc_pend - (RW )"]
pub type AdcpongRameccPendR = crate::BitReader;
#[doc = "Field `ADCPONG_RAMECC_PEND` writer - 13:13\\]
Interrupt Pending Status for adcpong_ramecc_pend - (RW )"]
pub type AdcpongRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES12` reader - 31:14\\]
RESERVE FIELD"]
pub type Res12R = crate::FieldReader<u32>;
#[doc = "Field `RES12` writer - 31:14\\]
RESERVE FIELD"]
pub type Res12W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for app_ss_rom_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_rom_pend(&self) -> AppSsRomPendR {
        AppSsRomPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for app_ss_ram1_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram1_pend(&self) -> AppSsRam1PendR {
        AppSsRam1PendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for app_ss_ram2_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram2_pend(&self) -> AppSsRam2PendR {
        AppSsRam2PendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for app_ss_ram3_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_ram3_pend(&self) -> AppSsRam3PendR {
        AppSsRam3PendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for app_ss_tptc1_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_tptc1_pend(&self) -> AppSsTptc1PendR {
        AppSsTptc1PendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for app_ss_tptc2_pend - (RW )"]
    #[inline(always)]
    pub fn app_ss_tptc2_pend(&self) -> AppSsTptc2PendR {
        AppSsTptc2PendR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for hwa_tptc1_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_tptc1_pend(&self) -> HwaTptc1PendR {
        HwaTptc1PendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for hwa_tptc2_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_tptc2_pend(&self) -> HwaTptc2PendR {
        HwaTptc2PendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for shared_mem0_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem0_ramecc_pend(&self) -> SharedMem0RameccPendR {
        SharedMem0RameccPendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Pending Status for shared_mem1_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem1_ramecc_pend(&self) -> SharedMem1RameccPendR {
        SharedMem1RameccPendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Pending Status for hwa_param_mem_pend - (RW )"]
    #[inline(always)]
    pub fn hwa_param_mem_pend(&self) -> HwaParamMemPendR {
        HwaParamMemPendR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Pending Status for shared_mem2_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn shared_mem2_ramecc_pend(&self) -> SharedMem2RameccPendR {
        SharedMem2RameccPendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Pending Status for adcping_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn adcping_ramecc_pend(&self) -> AdcpingRameccPendR {
        AdcpingRameccPendR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Pending Status for adcpong_ramecc_pend - (RW )"]
    #[inline(always)]
    pub fn adcpong_ramecc_pend(&self) -> AdcpongRameccPendR {
        AdcpongRameccPendR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res12(&self) -> Res12R {
        Res12R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for app_ss_rom_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_rom_pend(&mut self) -> AppSsRomPendW<DedStatusReg0Spec> {
        AppSsRomPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for app_ss_ram1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram1_pend(&mut self) -> AppSsRam1PendW<DedStatusReg0Spec> {
        AppSsRam1PendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for app_ss_ram2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram2_pend(&mut self) -> AppSsRam2PendW<DedStatusReg0Spec> {
        AppSsRam2PendW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for app_ss_ram3_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_ram3_pend(&mut self) -> AppSsRam3PendW<DedStatusReg0Spec> {
        AppSsRam3PendW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for app_ss_tptc1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_tptc1_pend(&mut self) -> AppSsTptc1PendW<DedStatusReg0Spec> {
        AppSsTptc1PendW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for app_ss_tptc2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn app_ss_tptc2_pend(&mut self) -> AppSsTptc2PendW<DedStatusReg0Spec> {
        AppSsTptc2PendW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for hwa_tptc1_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_tptc1_pend(&mut self) -> HwaTptc1PendW<DedStatusReg0Spec> {
        HwaTptc1PendW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for hwa_tptc2_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_tptc2_pend(&mut self) -> HwaTptc2PendW<DedStatusReg0Spec> {
        HwaTptc2PendW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for shared_mem0_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem0_ramecc_pend(&mut self) -> SharedMem0RameccPendW<DedStatusReg0Spec> {
        SharedMem0RameccPendW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Pending Status for shared_mem1_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem1_ramecc_pend(&mut self) -> SharedMem1RameccPendW<DedStatusReg0Spec> {
        SharedMem1RameccPendW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Pending Status for hwa_param_mem_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_param_mem_pend(&mut self) -> HwaParamMemPendW<DedStatusReg0Spec> {
        HwaParamMemPendW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Pending Status for shared_mem2_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn shared_mem2_ramecc_pend(&mut self) -> SharedMem2RameccPendW<DedStatusReg0Spec> {
        SharedMem2RameccPendW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Pending Status for adcping_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn adcping_ramecc_pend(&mut self) -> AdcpingRameccPendW<DedStatusReg0Spec> {
        AdcpingRameccPendW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Pending Status for adcpong_ramecc_pend - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn adcpong_ramecc_pend(&mut self) -> AdcpongRameccPendW<DedStatusReg0Spec> {
        AdcpongRameccPendW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res12(&mut self) -> Res12W<DedStatusReg0Spec> {
        Res12W::new(self, 14)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_status_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_status_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedStatusReg0Spec;
impl crate::RegisterSpec for DedStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_status_reg0::R`](R) reader structure"]
impl crate::Readable for DedStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ded_status_reg0::W`](W) writer structure"]
impl crate::Writable for DedStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DED_STATUS_REG0 to value 0"]
impl crate::Resettable for DedStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
