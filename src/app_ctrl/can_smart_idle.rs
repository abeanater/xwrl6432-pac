#[doc = "Register `CAN_SMART_IDLE` reader"]
pub type R = crate::R<CanSmartIdleSpec>;
#[doc = "Register `CAN_SMART_IDLE` writer"]
pub type W = crate::W<CanSmartIdleSpec>;
#[doc = "Field `enable` reader - 0:0\\]
1 => Smart IDLE mode enabled. When set, Request the clock gating of CAN module.0 => Disable Smart IDLE mode for CAN"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - 0:0\\]
1 => Smart IDLE mode enabled. When set, Request the clock gating of CAN module.0 => Disable Smart IDLE mode for CAN"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ack` reader - 1:1\\]
1 => CAN in smart idle mode0 => CAN not in smart idle mode The bit is sticky bit and the user is should clear once the status is read by write-1-to-clear."]
pub type AckR = crate::BitReader;
#[doc = "Field `ack` writer - 1:1\\]
1 => CAN in smart idle mode0 => CAN not in smart idle mode The bit is sticky bit and the user is should clear once the status is read by write-1-to-clear."]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `auto_en` reader - 2:2\\]
It is used to select smart idle mode. 1 => Automatic mode - In this mode, entry to smart idle mode is manual by setting SMART_IDLE_ENABLE = 1. When the wakeup Signal is asserted (based on the activity), The clkstop_req is pulled low automatically. 0 => Manual mode - The entry and exit to Smart Idle is user controlled based on polling SMART_IDLE_ACK and SMART_IDLE_WAKEUP"]
pub type AutoEnR = crate::BitReader;
#[doc = "Field `auto_en` writer - 2:2\\]
It is used to select smart idle mode. 1 => Automatic mode - In this mode, entry to smart idle mode is manual by setting SMART_IDLE_ENABLE = 1. When the wakeup Signal is asserted (based on the activity), The clkstop_req is pulled low automatically. 0 => Manual mode - The entry and exit to Smart Idle is user controlled based on polling SMART_IDLE_ACK and SMART_IDLE_WAKEUP"]
pub type AutoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup` reader - 3:3\\]
This register reflects the Wakeup Status of the IP. The bit is sticky bit and the user is should clear once the status is read by write-1-to-clear."]
pub type WakeupR = crate::BitReader;
#[doc = "Field `wakeup` writer - 3:3\\]
This register reflects the Wakeup Status of the IP. The bit is sticky bit and the user is should clear once the status is read by write-1-to-clear."]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ack_raw` reader - 4:4\\]
Description: RAW status of CLKSTOP_ACK from CANFD module. This should be interpreted along with CAN_SMART_IDLE_ACK CAN_SMART_IDLE_ACK_RAW, CAN_SMART_IDLE_ACK 0 , 0 => ACK is LOW from IP, and No pending ACK status 0 , 1 => ACK is LOW from IP, and pending ACK status 1 , 0 => ACK is HIGH from IP, and No pending ACK status 1 , 1 => ACK is HIGH from IP, and pending ACK status"]
pub type AckRawR = crate::BitReader;
#[doc = "Field `ack_raw` writer - 4:4\\]
Description: RAW status of CLKSTOP_ACK from CANFD module. This should be interpreted along with CAN_SMART_IDLE_ACK CAN_SMART_IDLE_ACK_RAW, CAN_SMART_IDLE_ACK 0 , 0 => ACK is LOW from IP, and No pending ACK status 0 , 1 => ACK is LOW from IP, and pending ACK status 1 , 0 => ACK is HIGH from IP, and No pending ACK status 1 , 1 => ACK is HIGH from IP, and pending ACK status"]
pub type AckRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wakeup_raw` reader - 5:5\\]
Description: RAW status of CLKSTOP_WAKEUP from CANFD module. This should be interpreted along with CAN_SMART_IDLE_WAKEUP CAN_SMART_IDLE_WAKEUP_RAW, CAN_SMART_IDLE_WAKEUP 0 , 0 => WAKEUP is LOW from IP, and No pending WAKEUP status 0 , 1 => WAKEUP is LOW from IP, and pending WAKEUP status 1 , 0 => WAKEUP is HIGH from IP, and No pending WAKEUP status 1 , 1 => WAKEUP is HIGH from IP, and pending WAKEUP status"]
pub type WakeupRawR = crate::BitReader;
#[doc = "Field `wakeup_raw` writer - 5:5\\]
Description: RAW status of CLKSTOP_WAKEUP from CANFD module. This should be interpreted along with CAN_SMART_IDLE_WAKEUP CAN_SMART_IDLE_WAKEUP_RAW, CAN_SMART_IDLE_WAKEUP 0 , 0 => WAKEUP is LOW from IP, and No pending WAKEUP status 0 , 1 => WAKEUP is LOW from IP, and pending WAKEUP status 1 , 0 => WAKEUP is HIGH from IP, and No pending WAKEUP status 1 , 1 => WAKEUP is HIGH from IP, and pending WAKEUP status"]
pub type WakeupRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1 => Smart IDLE mode enabled. When set, Request the clock gating of CAN module.0 => Disable Smart IDLE mode for CAN"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 => CAN in smart idle mode0 => CAN not in smart idle mode The bit is sticky bit and the user is should clear once the status is read by write-1-to-clear."]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
It is used to select smart idle mode. 1 => Automatic mode - In this mode, entry to smart idle mode is manual by setting SMART_IDLE_ENABLE = 1. When the wakeup Signal is asserted (based on the activity), The clkstop_req is pulled low automatically. 0 => Manual mode - The entry and exit to Smart Idle is user controlled based on polling SMART_IDLE_ACK and SMART_IDLE_WAKEUP"]
    #[inline(always)]
    pub fn auto_en(&self) -> AutoEnR {
        AutoEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This register reflects the Wakeup Status of the IP. The bit is sticky bit and the user is should clear once the status is read by write-1-to-clear."]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Description: RAW status of CLKSTOP_ACK from CANFD module. This should be interpreted along with CAN_SMART_IDLE_ACK CAN_SMART_IDLE_ACK_RAW, CAN_SMART_IDLE_ACK 0 , 0 => ACK is LOW from IP, and No pending ACK status 0 , 1 => ACK is LOW from IP, and pending ACK status 1 , 0 => ACK is HIGH from IP, and No pending ACK status 1 , 1 => ACK is HIGH from IP, and pending ACK status"]
    #[inline(always)]
    pub fn ack_raw(&self) -> AckRawR {
        AckRawR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Description: RAW status of CLKSTOP_WAKEUP from CANFD module. This should be interpreted along with CAN_SMART_IDLE_WAKEUP CAN_SMART_IDLE_WAKEUP_RAW, CAN_SMART_IDLE_WAKEUP 0 , 0 => WAKEUP is LOW from IP, and No pending WAKEUP status 0 , 1 => WAKEUP is LOW from IP, and pending WAKEUP status 1 , 0 => WAKEUP is HIGH from IP, and No pending WAKEUP status 1 , 1 => WAKEUP is HIGH from IP, and pending WAKEUP status"]
    #[inline(always)]
    pub fn wakeup_raw(&self) -> WakeupRawR {
        WakeupRawR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1 => Smart IDLE mode enabled. When set, Request the clock gating of CAN module.0 => Disable Smart IDLE mode for CAN"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CanSmartIdleSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 => CAN in smart idle mode0 => CAN not in smart idle mode The bit is sticky bit and the user is should clear once the status is read by write-1-to-clear."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<CanSmartIdleSpec> {
        AckW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
It is used to select smart idle mode. 1 => Automatic mode - In this mode, entry to smart idle mode is manual by setting SMART_IDLE_ENABLE = 1. When the wakeup Signal is asserted (based on the activity), The clkstop_req is pulled low automatically. 0 => Manual mode - The entry and exit to Smart Idle is user controlled based on polling SMART_IDLE_ACK and SMART_IDLE_WAKEUP"]
    #[inline(always)]
    #[must_use]
    pub fn auto_en(&mut self) -> AutoEnW<CanSmartIdleSpec> {
        AutoEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This register reflects the Wakeup Status of the IP. The bit is sticky bit and the user is should clear once the status is read by write-1-to-clear."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<CanSmartIdleSpec> {
        WakeupW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Description: RAW status of CLKSTOP_ACK from CANFD module. This should be interpreted along with CAN_SMART_IDLE_ACK CAN_SMART_IDLE_ACK_RAW, CAN_SMART_IDLE_ACK 0 , 0 => ACK is LOW from IP, and No pending ACK status 0 , 1 => ACK is LOW from IP, and pending ACK status 1 , 0 => ACK is HIGH from IP, and No pending ACK status 1 , 1 => ACK is HIGH from IP, and pending ACK status"]
    #[inline(always)]
    #[must_use]
    pub fn ack_raw(&mut self) -> AckRawW<CanSmartIdleSpec> {
        AckRawW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Description: RAW status of CLKSTOP_WAKEUP from CANFD module. This should be interpreted along with CAN_SMART_IDLE_WAKEUP CAN_SMART_IDLE_WAKEUP_RAW, CAN_SMART_IDLE_WAKEUP 0 , 0 => WAKEUP is LOW from IP, and No pending WAKEUP status 0 , 1 => WAKEUP is LOW from IP, and pending WAKEUP status 1 , 0 => WAKEUP is HIGH from IP, and No pending WAKEUP status 1 , 1 => WAKEUP is HIGH from IP, and pending WAKEUP status"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_raw(&mut self) -> WakeupRawW<CanSmartIdleSpec> {
        WakeupRawW::new(self, 5)
    }
}
#[doc = "CAN_SMART_IDLE\n\nYou can [`read`](crate::Reg::read) this register and get [`can_smart_idle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_smart_idle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanSmartIdleSpec;
impl crate::RegisterSpec for CanSmartIdleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_smart_idle::R`](R) reader structure"]
impl crate::Readable for CanSmartIdleSpec {}
#[doc = "`write(|w| ..)` method takes [`can_smart_idle::W`](W) writer structure"]
impl crate::Writable for CanSmartIdleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAN_SMART_IDLE to value 0"]
impl crate::Resettable for CanSmartIdleSpec {
    const RESET_VALUE: u32 = 0;
}
