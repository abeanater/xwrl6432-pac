#[doc = "Register `POWERMODE` reader"]
pub type R = crate::R<PowermodeSpec>;
#[doc = "Register `POWERMODE` writer"]
pub type W = crate::W<PowermodeSpec>;
#[doc = "Field `SLEEP` reader - 0:0\\]
0x0 : CM4 CORE SLEEP 0x1 : DEVICE SLEEP"]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - 0:0\\]
0x0 : CM4 CORE SLEEP 0x1 : DEVICE SLEEP"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPSLEEP` reader - 1:1\\]
0x0 : CM4 CORE DEEP SLEEP 0x1 : DEVICE DEEP SLEEP"]
pub type DeepsleepR = crate::BitReader;
#[doc = "Field `DEEPSLEEP` writer - 1:1\\]
0x0 : CM4 CORE DEEP SLEEP 0x1 : DEVICE DEEP SLEEP"]
pub type DeepsleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM3_SLEEP_STATUS` reader - 2:2\\]
CM3 Core Sleep Status"]
pub type Cm3SleepStatusR = crate::BitReader;
#[doc = "Field `CM3_SLEEP_STATUS` writer - 2:2\\]
CM3 Core Sleep Status"]
pub type Cm3SleepStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM3_DEEPSLEEP_STATUS` reader - 3:3\\]
CM3 Core Deep Sleep Status"]
pub type Cm3DeepsleepStatusR = crate::BitReader;
#[doc = "Field `CM3_DEEPSLEEP_STATUS` writer - 3:3\\]
CM3 Core Deep Sleep Status"]
pub type Cm3DeepsleepStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0x0 : CM4 CORE SLEEP 0x1 : DEVICE SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0x0 : CM4 CORE DEEP SLEEP 0x1 : DEVICE DEEP SLEEP"]
    #[inline(always)]
    pub fn deepsleep(&self) -> DeepsleepR {
        DeepsleepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CM3 Core Sleep Status"]
    #[inline(always)]
    pub fn cm3_sleep_status(&self) -> Cm3SleepStatusR {
        Cm3SleepStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
CM3 Core Deep Sleep Status"]
    #[inline(always)]
    pub fn cm3_deepsleep_status(&self) -> Cm3DeepsleepStatusR {
        Cm3DeepsleepStatusR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0x0 : CM4 CORE SLEEP 0x1 : DEVICE SLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<PowermodeSpec> {
        SleepW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0x0 : CM4 CORE DEEP SLEEP 0x1 : DEVICE DEEP SLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep(&mut self) -> DeepsleepW<PowermodeSpec> {
        DeepsleepW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CM3 Core Sleep Status"]
    #[inline(always)]
    #[must_use]
    pub fn cm3_sleep_status(&mut self) -> Cm3SleepStatusW<PowermodeSpec> {
        Cm3SleepStatusW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
CM3 Core Deep Sleep Status"]
    #[inline(always)]
    #[must_use]
    pub fn cm3_deepsleep_status(&mut self) -> Cm3DeepsleepStatusW<PowermodeSpec> {
        Cm3DeepsleepStatusW::new(self, 3)
    }
}
#[doc = "POWERMODE\n\nYou can [`read`](crate::Reg::read) this register and get [`powermode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powermode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowermodeSpec;
impl crate::RegisterSpec for PowermodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powermode::R`](R) reader structure"]
impl crate::Readable for PowermodeSpec {}
#[doc = "`write(|w| ..)` method takes [`powermode::W`](W) writer structure"]
impl crate::Writable for PowermodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWERMODE to value 0"]
impl crate::Resettable for PowermodeSpec {
    const RESET_VALUE: u32 = 0;
}
