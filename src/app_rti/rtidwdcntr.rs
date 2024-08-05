#[doc = "Register `RTIDWDCNTR` reader"]
pub type R = crate::R<RtidwdcntrSpec>;
#[doc = "Register `RTIDWDCNTR` writer"]
pub type W = crate::W<RtidwdcntrSpec>;
#[doc = "Field `DWDCNTR` reader - 24:0\\]
DWDCNTR: Digital Watchdog Down Counter. The value of the DWDCNTR after a system reset is 0x002D_FFFF. When the DWD is enabled and the DWD counter starts counting down from this value with an RTICLK1 time base of 3MHz, a watchdog reset will be generated in 1 second. User and privilege mode (read): Reads return the current counter value. Privilege mode (write): Writes donΓÇÖt have an effect."]
pub type DwdcntrR = crate::FieldReader<u32>;
#[doc = "Field `DWDCNTR` writer - 24:0\\]
DWDCNTR: Digital Watchdog Down Counter. The value of the DWDCNTR after a system reset is 0x002D_FFFF. When the DWD is enabled and the DWD counter starts counting down from this value with an RTICLK1 time base of 3MHz, a watchdog reset will be generated in 1 second. User and privilege mode (read): Reads return the current counter value. Privilege mode (write): Writes donΓÇÖt have an effect."]
pub type DwdcntrW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `RESERVED20` reader - 31:25\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved20R = crate::FieldReader;
#[doc = "Field `RESERVED20` writer - 31:25\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:24 - 24:0\\]
DWDCNTR: Digital Watchdog Down Counter. The value of the DWDCNTR after a system reset is 0x002D_FFFF. When the DWD is enabled and the DWD counter starts counting down from this value with an RTICLK1 time base of 3MHz, a watchdog reset will be generated in 1 second. User and privilege mode (read): Reads return the current counter value. Privilege mode (write): Writes donΓÇÖt have an effect."]
    #[inline(always)]
    pub fn dwdcntr(&self) -> DwdcntrR {
        DwdcntrR::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:24 - 24:0\\]
DWDCNTR: Digital Watchdog Down Counter. The value of the DWDCNTR after a system reset is 0x002D_FFFF. When the DWD is enabled and the DWD counter starts counting down from this value with an RTICLK1 time base of 3MHz, a watchdog reset will be generated in 1 second. User and privilege mode (read): Reads return the current counter value. Privilege mode (write): Writes donΓÇÖt have an effect."]
    #[inline(always)]
    #[must_use]
    pub fn dwdcntr(&mut self) -> DwdcntrW<RtidwdcntrSpec> {
        DwdcntrW::new(self, 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<RtidwdcntrSpec> {
        Reserved20W::new(self, 25)
    }
}
#[doc = "Digital Watchdog Down Counter current value of DWD down counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rtidwdcntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtidwdcntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtidwdcntrSpec;
impl crate::RegisterSpec for RtidwdcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtidwdcntr::R`](R) reader structure"]
impl crate::Readable for RtidwdcntrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtidwdcntr::W`](W) writer structure"]
impl crate::Writable for RtidwdcntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIDWDCNTR to value 0"]
impl crate::Resettable for RtidwdcntrSpec {
    const RESET_VALUE: u32 = 0;
}
