#[doc = "Register `DEBUG_STATUS_AON_3` reader"]
pub type R = crate::R<DebugStatusAon3Spec>;
#[doc = "Register `DEBUG_STATUS_AON_3` writer"]
pub type W = crate::W<DebugStatusAon3Spec>;
#[doc = "Field `radar_state` reader - 3:0\\]
status reg for radar_state"]
pub type RadarStateR = crate::FieldReader;
#[doc = "Field `radar_state` writer - 3:0\\]
status reg for radar_state"]
pub type RadarStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wakeup_source_sleep_counter` reader - 4:4\\]
status reg for wakeup_source_sleep_counter"]
pub type WakeupSourceSleepCounterR = crate::BitReader;
#[doc = "Field `wakeup_source_sleep_counter` writer - 4:4\\]
status reg for wakeup_source_sleep_counter"]
pub type WakeupSourceSleepCounterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup_source_uart` reader - 5:5\\]
status reg for wakeup_source_uart"]
pub type WakeupSourceUartR = crate::BitReader;
#[doc = "Field `wakeup_source_uart` writer - 5:5\\]
status reg for wakeup_source_uart"]
pub type WakeupSourceUartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup_source_spi` reader - 6:6\\]
status reg for wakeup_source_spi"]
pub type WakeupSourceSpiR = crate::BitReader;
#[doc = "Field `wakeup_source_spi` writer - 6:6\\]
status reg for wakeup_source_spi"]
pub type WakeupSourceSpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup_source_gpio` reader - 7:7\\]
status reg for wakeup_source_gpio"]
pub type WakeupSourceGpioR = crate::BitReader;
#[doc = "Field `wakeup_source_gpio` writer - 7:7\\]
status reg for wakeup_source_gpio"]
pub type WakeupSourceGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup_source_rtc` reader - 8:8\\]
status reg for wakeup_source_rtc"]
pub type WakeupSourceRtcR = crate::BitReader;
#[doc = "Field `wakeup_source_rtc` writer - 8:8\\]
status reg for wakeup_source_rtc"]
pub type WakeupSourceRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup_source_frc` reader - 9:9\\]
status reg for wakeup_source_frc"]
pub type WakeupSourceFrcR = crate::BitReader;
#[doc = "Field `wakeup_source_frc` writer - 9:9\\]
status reg for wakeup_source_frc"]
pub type WakeupSourceFrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
status reg for radar_state"]
    #[inline(always)]
    pub fn radar_state(&self) -> RadarStateR {
        RadarStateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for wakeup_source_sleep_counter"]
    #[inline(always)]
    pub fn wakeup_source_sleep_counter(&self) -> WakeupSourceSleepCounterR {
        WakeupSourceSleepCounterR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for wakeup_source_uart"]
    #[inline(always)]
    pub fn wakeup_source_uart(&self) -> WakeupSourceUartR {
        WakeupSourceUartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for wakeup_source_spi"]
    #[inline(always)]
    pub fn wakeup_source_spi(&self) -> WakeupSourceSpiR {
        WakeupSourceSpiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for wakeup_source_gpio"]
    #[inline(always)]
    pub fn wakeup_source_gpio(&self) -> WakeupSourceGpioR {
        WakeupSourceGpioR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for wakeup_source_rtc"]
    #[inline(always)]
    pub fn wakeup_source_rtc(&self) -> WakeupSourceRtcR {
        WakeupSourceRtcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
status reg for wakeup_source_frc"]
    #[inline(always)]
    pub fn wakeup_source_frc(&self) -> WakeupSourceFrcR {
        WakeupSourceFrcR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
status reg for radar_state"]
    #[inline(always)]
    #[must_use]
    pub fn radar_state(&mut self) -> RadarStateW<DebugStatusAon3Spec> {
        RadarStateW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for wakeup_source_sleep_counter"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_source_sleep_counter(
        &mut self,
    ) -> WakeupSourceSleepCounterW<DebugStatusAon3Spec> {
        WakeupSourceSleepCounterW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for wakeup_source_uart"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_source_uart(&mut self) -> WakeupSourceUartW<DebugStatusAon3Spec> {
        WakeupSourceUartW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for wakeup_source_spi"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_source_spi(&mut self) -> WakeupSourceSpiW<DebugStatusAon3Spec> {
        WakeupSourceSpiW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for wakeup_source_gpio"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_source_gpio(&mut self) -> WakeupSourceGpioW<DebugStatusAon3Spec> {
        WakeupSourceGpioW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for wakeup_source_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_source_rtc(&mut self) -> WakeupSourceRtcW<DebugStatusAon3Spec> {
        WakeupSourceRtcW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
status reg for wakeup_source_frc"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_source_frc(&mut self) -> WakeupSourceFrcW<DebugStatusAon3Spec> {
        WakeupSourceFrcW::new(self, 9)
    }
}
#[doc = "DEBUG_STATUS_AON_3\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon3Spec;
impl crate::RegisterSpec for DebugStatusAon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_3::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon3Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_3::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_3 to value 0"]
impl crate::Resettable for DebugStatusAon3Spec {
    const RESET_VALUE: u32 = 0;
}
