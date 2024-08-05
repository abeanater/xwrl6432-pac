#[doc = "Register `RTIDWDCTRL` reader"]
pub type R = crate::R<RtidwdctrlSpec>;
#[doc = "Register `RTIDWDCTRL` writer"]
pub type W = crate::W<RtidwdctrlSpec>;
#[doc = "Field `DWDCTRL` reader - 31:0\\]
DWDCTRL: Digital Watchdog Control. User and priviledge mode (read): 0x5312ACED = DWD counter is disabled. This is the default value. 0xA98559DA = DWD counter is enabled Any other value = DWD counter state is unchanged (enabled or disabled) Priviledge mode (write): 0xA98559DA = DWD counter is enabled Any other value = State of DWD counter is unchanged (stays enabled or disabled) Note: One-Write Functionality of DWDCTRL Register The RTIDWDCTRL register implements a one-write functionality, such that the application cannot write to this registermore than once. Writing the default value will not enable the watchdog as described above. Writing the enable value will start the watchdog counters. A write to RTIDWDCTRL will only be enabled after a system reset again."]
pub type DwdctrlR = crate::FieldReader<u32>;
#[doc = "Field `DWDCTRL` writer - 31:0\\]
DWDCTRL: Digital Watchdog Control. User and priviledge mode (read): 0x5312ACED = DWD counter is disabled. This is the default value. 0xA98559DA = DWD counter is enabled Any other value = DWD counter state is unchanged (enabled or disabled) Priviledge mode (write): 0xA98559DA = DWD counter is enabled Any other value = State of DWD counter is unchanged (stays enabled or disabled) Note: One-Write Functionality of DWDCTRL Register The RTIDWDCTRL register implements a one-write functionality, such that the application cannot write to this registermore than once. Writing the default value will not enable the watchdog as described above. Writing the enable value will start the watchdog counters. A write to RTIDWDCTRL will only be enabled after a system reset again."]
pub type DwdctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
DWDCTRL: Digital Watchdog Control. User and priviledge mode (read): 0x5312ACED = DWD counter is disabled. This is the default value. 0xA98559DA = DWD counter is enabled Any other value = DWD counter state is unchanged (enabled or disabled) Priviledge mode (write): 0xA98559DA = DWD counter is enabled Any other value = State of DWD counter is unchanged (stays enabled or disabled) Note: One-Write Functionality of DWDCTRL Register The RTIDWDCTRL register implements a one-write functionality, such that the application cannot write to this registermore than once. Writing the default value will not enable the watchdog as described above. Writing the enable value will start the watchdog counters. A write to RTIDWDCTRL will only be enabled after a system reset again."]
    #[inline(always)]
    pub fn dwdctrl(&self) -> DwdctrlR {
        DwdctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
DWDCTRL: Digital Watchdog Control. User and priviledge mode (read): 0x5312ACED = DWD counter is disabled. This is the default value. 0xA98559DA = DWD counter is enabled Any other value = DWD counter state is unchanged (enabled or disabled) Priviledge mode (write): 0xA98559DA = DWD counter is enabled Any other value = State of DWD counter is unchanged (stays enabled or disabled) Note: One-Write Functionality of DWDCTRL Register The RTIDWDCTRL register implements a one-write functionality, such that the application cannot write to this registermore than once. Writing the default value will not enable the watchdog as described above. Writing the enable value will start the watchdog counters. A write to RTIDWDCTRL will only be enabled after a system reset again."]
    #[inline(always)]
    #[must_use]
    pub fn dwdctrl(&mut self) -> DwdctrlW<RtidwdctrlSpec> {
        DwdctrlW::new(self, 0)
    }
}
#[doc = "Digital Watchdog Control Enables the Digital Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rtidwdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtidwdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtidwdctrlSpec;
impl crate::RegisterSpec for RtidwdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtidwdctrl::R`](R) reader structure"]
impl crate::Readable for RtidwdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtidwdctrl::W`](W) writer structure"]
impl crate::Writable for RtidwdctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIDWDCTRL to value 0"]
impl crate::Resettable for RtidwdctrlSpec {
    const RESET_VALUE: u32 = 0;
}
