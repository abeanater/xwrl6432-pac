#[doc = "Register `APPSS_SHARED_MEM_CLK_GATE` reader"]
pub type R = crate::R<AppssSharedMemClkGateSpec>;
#[doc = "Register `APPSS_SHARED_MEM_CLK_GATE` writer"]
pub type W = crate::W<AppssSharedMemClkGateSpec>;
#[doc = "Field `mem0_hwa_enable` reader - 0:0\\]
1'b1 : Enable HWA CLK ICG for first 128 KB of shared mem 1'b0 : Disable HWA CLK ICG for first 128 KB of shared mem"]
pub type Mem0HwaEnableR = crate::BitReader;
#[doc = "Field `mem0_hwa_enable` writer - 0:0\\]
1'b1 : Enable HWA CLK ICG for first 128 KB of shared mem 1'b0 : Disable HWA CLK ICG for first 128 KB of shared mem"]
pub type Mem0HwaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mem0_app_enable` reader - 1:1\\]
1'b1 : Enable APP CLK ICG for first 128KB of shared mem 1'b0 : Disable APP CLK ICG for first 128 KB of shared mem"]
pub type Mem0AppEnableR = crate::BitReader;
#[doc = "Field `mem0_app_enable` writer - 1:1\\]
1'b1 : Enable APP CLK ICG for first 128KB of shared mem 1'b0 : Disable APP CLK ICG for first 128 KB of shared mem"]
pub type Mem0AppEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mem1_hwa_enable` reader - 2:2\\]
1'b1 : Enable HWA CLK ICG for second 128 KB of shared mem 1'b0 : Disable HWA CLK ICG for second 128 KB of shared mem"]
pub type Mem1HwaEnableR = crate::BitReader;
#[doc = "Field `mem1_hwa_enable` writer - 2:2\\]
1'b1 : Enable HWA CLK ICG for second 128 KB of shared mem 1'b0 : Disable HWA CLK ICG for second 128 KB of shared mem"]
pub type Mem1HwaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mem1_app_enable` reader - 3:3\\]
1'b1 : Enable APP CLK ICG for second 128KB of shared mem 1'b0 : Disable APP CLK ICG for second 128 KB of shared mem"]
pub type Mem1AppEnableR = crate::BitReader;
#[doc = "Field `mem1_app_enable` writer - 3:3\\]
1'b1 : Enable APP CLK ICG for second 128KB of shared mem 1'b0 : Disable APP CLK ICG for second 128 KB of shared mem"]
pub type Mem1AppEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b1 : Enable HWA CLK ICG for first 128 KB of shared mem 1'b0 : Disable HWA CLK ICG for first 128 KB of shared mem"]
    #[inline(always)]
    pub fn mem0_hwa_enable(&self) -> Mem0HwaEnableR {
        Mem0HwaEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1'b1 : Enable APP CLK ICG for first 128KB of shared mem 1'b0 : Disable APP CLK ICG for first 128 KB of shared mem"]
    #[inline(always)]
    pub fn mem0_app_enable(&self) -> Mem0AppEnableR {
        Mem0AppEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1'b1 : Enable HWA CLK ICG for second 128 KB of shared mem 1'b0 : Disable HWA CLK ICG for second 128 KB of shared mem"]
    #[inline(always)]
    pub fn mem1_hwa_enable(&self) -> Mem1HwaEnableR {
        Mem1HwaEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1'b1 : Enable APP CLK ICG for second 128KB of shared mem 1'b0 : Disable APP CLK ICG for second 128 KB of shared mem"]
    #[inline(always)]
    pub fn mem1_app_enable(&self) -> Mem1AppEnableR {
        Mem1AppEnableR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b1 : Enable HWA CLK ICG for first 128 KB of shared mem 1'b0 : Disable HWA CLK ICG for first 128 KB of shared mem"]
    #[inline(always)]
    #[must_use]
    pub fn mem0_hwa_enable(&mut self) -> Mem0HwaEnableW<AppssSharedMemClkGateSpec> {
        Mem0HwaEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1'b1 : Enable APP CLK ICG for first 128KB of shared mem 1'b0 : Disable APP CLK ICG for first 128 KB of shared mem"]
    #[inline(always)]
    #[must_use]
    pub fn mem0_app_enable(&mut self) -> Mem0AppEnableW<AppssSharedMemClkGateSpec> {
        Mem0AppEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
1'b1 : Enable HWA CLK ICG for second 128 KB of shared mem 1'b0 : Disable HWA CLK ICG for second 128 KB of shared mem"]
    #[inline(always)]
    #[must_use]
    pub fn mem1_hwa_enable(&mut self) -> Mem1HwaEnableW<AppssSharedMemClkGateSpec> {
        Mem1HwaEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
1'b1 : Enable APP CLK ICG for second 128KB of shared mem 1'b0 : Disable APP CLK ICG for second 128 KB of shared mem"]
    #[inline(always)]
    #[must_use]
    pub fn mem1_app_enable(&mut self) -> Mem1AppEnableW<AppssSharedMemClkGateSpec> {
        Mem1AppEnableW::new(self, 3)
    }
}
#[doc = "APPSS_SHARED_MEM_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_shared_mem_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_shared_mem_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssSharedMemClkGateSpec;
impl crate::RegisterSpec for AppssSharedMemClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_shared_mem_clk_gate::R`](R) reader structure"]
impl crate::Readable for AppssSharedMemClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_shared_mem_clk_gate::W`](W) writer structure"]
impl crate::Writable for AppssSharedMemClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_SHARED_MEM_CLK_GATE to value 0"]
impl crate::Resettable for AppssSharedMemClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
