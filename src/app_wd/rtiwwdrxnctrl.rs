#[doc = "Register `RTIWWDRXNCTRL` reader"]
pub type R = crate::R<RtiwwdrxnctrlSpec>;
#[doc = "Register `RTIWWDRXNCTRL` writer"]
pub type W = crate::W<RtiwwdrxnctrlSpec>;
#[doc = "Field `WWDRXN` reader - 3:0\\]
WWDRXN: Digital Windowed Watchdog Reaction. User and privilege mode (read), privileged mode (write): 0x5 = This is the default value. The windowed watchdog will cause a reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. 0xA = The windowed watchdog will generate a non-maskable interrupt to the CPU if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Writing any other value will cause a system reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Note: Configuration of DWWD Reaction The DWWD reaction can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDRXN is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDRXN is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
pub type WwdrxnR = crate::FieldReader;
#[doc = "Field `WWDRXN` writer - 3:0\\]
WWDRXN: Digital Windowed Watchdog Reaction. User and privilege mode (read), privileged mode (write): 0x5 = This is the default value. The windowed watchdog will cause a reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. 0xA = The windowed watchdog will generate a non-maskable interrupt to the CPU if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Writing any other value will cause a system reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Note: Configuration of DWWD Reaction The DWWD reaction can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDRXN is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDRXN is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
pub type WwdrxnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED21` reader - 31:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved21R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED21` writer - 31:4\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
WWDRXN: Digital Windowed Watchdog Reaction. User and privilege mode (read), privileged mode (write): 0x5 = This is the default value. The windowed watchdog will cause a reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. 0xA = The windowed watchdog will generate a non-maskable interrupt to the CPU if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Writing any other value will cause a system reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Note: Configuration of DWWD Reaction The DWWD reaction can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDRXN is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDRXN is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
    #[inline(always)]
    pub fn wwdrxn(&self) -> WwdrxnR {
        WwdrxnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
WWDRXN: Digital Windowed Watchdog Reaction. User and privilege mode (read), privileged mode (write): 0x5 = This is the default value. The windowed watchdog will cause a reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. 0xA = The windowed watchdog will generate a non-maskable interrupt to the CPU if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Writing any other value will cause a system reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Note: Configuration of DWWD Reaction The DWWD reaction can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDRXN is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDRXN is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
    #[inline(always)]
    #[must_use]
    pub fn wwdrxn(&mut self) -> WwdrxnW<RtiwwdrxnctrlSpec> {
        WwdrxnW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<RtiwwdrxnctrlSpec> {
        Reserved21W::new(self, 4)
    }
}
#[doc = "Windowed Watchdog Reaction Control configures the windowed watchdog to either generate a non-maskable interrupt to the CPU or to generate a system reset\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiwwdrxnctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiwwdrxnctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtiwwdrxnctrlSpec;
impl crate::RegisterSpec for RtiwwdrxnctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiwwdrxnctrl::R`](R) reader structure"]
impl crate::Readable for RtiwwdrxnctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtiwwdrxnctrl::W`](W) writer structure"]
impl crate::Writable for RtiwwdrxnctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIWWDRXNCTRL to value 0"]
impl crate::Resettable for RtiwwdrxnctrlSpec {
    const RESET_VALUE: u32 = 0;
}
