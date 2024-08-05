#[doc = "Register `RTIDWDPRLD` reader"]
pub type R = crate::R<RtidwdprldSpec>;
#[doc = "Register `RTIDWDPRLD` writer"]
pub type W = crate::W<RtidwdprldSpec>;
#[doc = "Field `DWDPRLD` reader - 11:0\\]
DWDPRLD: Digital Watchdog Preload Value. User and priviledge mode (read): A read from this register in any CPU mode returns the current preload value. Priviledge mode (write): If the DWD is always enabled after reset is released: The DWD starts counting down from the reset value of the counter, that is, 0x002DFFFF. The application can configure the DWD preload register any time before this down counter expires. When the application services the DWD, the preload register contents are copied left-justified into the DWD down counter and it starts counting down from that value. If the DWD is implemented such that the down counter is enabled by software: The DWD preload register can be configured only when the DWD is disabled. Therefore, the application can only configure the DWD preload register before it enables the DWD down counter. The expiration time of the DWD Down Counter can be determined with following equation: texp = (RTIDWDPRLD+1) x 2 ^ 13 / RTICLK1 where: RTIDWDPRLD = 0...4095"]
pub type DwdprldR = crate::FieldReader<u16>;
#[doc = "Field `DWDPRLD` writer - 11:0\\]
DWDPRLD: Digital Watchdog Preload Value. User and priviledge mode (read): A read from this register in any CPU mode returns the current preload value. Priviledge mode (write): If the DWD is always enabled after reset is released: The DWD starts counting down from the reset value of the counter, that is, 0x002DFFFF. The application can configure the DWD preload register any time before this down counter expires. When the application services the DWD, the preload register contents are copied left-justified into the DWD down counter and it starts counting down from that value. If the DWD is implemented such that the down counter is enabled by software: The DWD preload register can be configured only when the DWD is disabled. Therefore, the application can only configure the DWD preload register before it enables the DWD down counter. The expiration time of the DWD Down Counter can be determined with following equation: texp = (RTIDWDPRLD+1) x 2 ^ 13 / RTICLK1 where: RTIDWDPRLD = 0...4095"]
pub type DwdprldW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED17` reader - 31:12\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved17R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED17` writer - 31:12\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
DWDPRLD: Digital Watchdog Preload Value. User and priviledge mode (read): A read from this register in any CPU mode returns the current preload value. Priviledge mode (write): If the DWD is always enabled after reset is released: The DWD starts counting down from the reset value of the counter, that is, 0x002DFFFF. The application can configure the DWD preload register any time before this down counter expires. When the application services the DWD, the preload register contents are copied left-justified into the DWD down counter and it starts counting down from that value. If the DWD is implemented such that the down counter is enabled by software: The DWD preload register can be configured only when the DWD is disabled. Therefore, the application can only configure the DWD preload register before it enables the DWD down counter. The expiration time of the DWD Down Counter can be determined with following equation: texp = (RTIDWDPRLD+1) x 2 ^ 13 / RTICLK1 where: RTIDWDPRLD = 0...4095"]
    #[inline(always)]
    pub fn dwdprld(&self) -> DwdprldR {
        DwdprldR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
DWDPRLD: Digital Watchdog Preload Value. User and priviledge mode (read): A read from this register in any CPU mode returns the current preload value. Priviledge mode (write): If the DWD is always enabled after reset is released: The DWD starts counting down from the reset value of the counter, that is, 0x002DFFFF. The application can configure the DWD preload register any time before this down counter expires. When the application services the DWD, the preload register contents are copied left-justified into the DWD down counter and it starts counting down from that value. If the DWD is implemented such that the down counter is enabled by software: The DWD preload register can be configured only when the DWD is disabled. Therefore, the application can only configure the DWD preload register before it enables the DWD down counter. The expiration time of the DWD Down Counter can be determined with following equation: texp = (RTIDWDPRLD+1) x 2 ^ 13 / RTICLK1 where: RTIDWDPRLD = 0...4095"]
    #[inline(always)]
    #[must_use]
    pub fn dwdprld(&mut self) -> DwdprldW<RtidwdprldSpec> {
        DwdprldW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<RtidwdprldSpec> {
        Reserved17W::new(self, 12)
    }
}
#[doc = "Digital Watchdog Preload sets the experation time of the Digital Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rtidwdprld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtidwdprld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtidwdprldSpec;
impl crate::RegisterSpec for RtidwdprldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtidwdprld::R`](R) reader structure"]
impl crate::Readable for RtidwdprldSpec {}
#[doc = "`write(|w| ..)` method takes [`rtidwdprld::W`](W) writer structure"]
impl crate::Writable for RtidwdprldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIDWDPRLD to value 0"]
impl crate::Resettable for RtidwdprldSpec {
    const RESET_VALUE: u32 = 0;
}
