#[doc = "Register `SYSCONFIG` reader"]
pub type R = crate::R<SysconfigSpec>;
#[doc = "Register `SYSCONFIG` writer"]
pub type W = crate::W<SysconfigSpec>;
#[doc = "Field `Reserved1` reader - 1:0\\]
Always read as 0"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 1:0\\]
Always read as 0"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDLEMODE` reader - 3:2\\]
Configuration of the local target state management mode. By definition, target can handle read/write transaction as long as it is out of IDLE state 0x0 : Force-idle mode: local target's idle state follows (acknowledges) the system's idle requests unconditionally, i.e. regardless of the IP module's internal requirements. Backup mode, for debug only 0x1 : No-idle mode: local target never enters idle state. Backup mode, for debug only 0x2 : Smart-idle mode: local target's idle state eventually follows (acknowledges) the system's idle requests, depending on the IP module's internal requirements. IP module shall not generate (IRQ- or DMA-request-related) wakeup events 0x3 : Smart-idle wakeup-capable mode: local target's idle state eventually follows (acknowledges) the system's idle requests, depending on the IP module's internal requirements. IP module may generate (IRQ- or DMA-request-related) wakeup events when in idle state. Mode is only relevant if the appropriate IP module \"swakeup\" output(s) is (are) implemented"]
pub type IdlemodeR = crate::FieldReader;
#[doc = "Field `IDLEMODE` writer - 3:2\\]
Configuration of the local target state management mode. By definition, target can handle read/write transaction as long as it is out of IDLE state 0x0 : Force-idle mode: local target's idle state follows (acknowledges) the system's idle requests unconditionally, i.e. regardless of the IP module's internal requirements. Backup mode, for debug only 0x1 : No-idle mode: local target never enters idle state. Backup mode, for debug only 0x2 : Smart-idle mode: local target's idle state eventually follows (acknowledges) the system's idle requests, depending on the IP module's internal requirements. IP module shall not generate (IRQ- or DMA-request-related) wakeup events 0x3 : Smart-idle wakeup-capable mode: local target's idle state eventually follows (acknowledges) the system's idle requests, depending on the IP module's internal requirements. IP module may generate (IRQ- or DMA-request-related) wakeup events when in idle state. Mode is only relevant if the appropriate IP module \"swakeup\" output(s) is (are) implemented"]
pub type IdlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - 5:4\\]
Always read as 0"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 5:4\\]
Always read as 0"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - 31:6\\]
Always read as 0"]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `Reserved3` writer - 31:6\\]
Always read as 0"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Configuration of the local target state management mode. By definition, target can handle read/write transaction as long as it is out of IDLE state 0x0 : Force-idle mode: local target's idle state follows (acknowledges) the system's idle requests unconditionally, i.e. regardless of the IP module's internal requirements. Backup mode, for debug only 0x1 : No-idle mode: local target never enters idle state. Backup mode, for debug only 0x2 : Smart-idle mode: local target's idle state eventually follows (acknowledges) the system's idle requests, depending on the IP module's internal requirements. IP module shall not generate (IRQ- or DMA-request-related) wakeup events 0x3 : Smart-idle wakeup-capable mode: local target's idle state eventually follows (acknowledges) the system's idle requests, depending on the IP module's internal requirements. IP module may generate (IRQ- or DMA-request-related) wakeup events when in idle state. Mode is only relevant if the appropriate IP module \"swakeup\" output(s) is (are) implemented"]
    #[inline(always)]
    pub fn idlemode(&self) -> IdlemodeR {
        IdlemodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SysconfigSpec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Configuration of the local target state management mode. By definition, target can handle read/write transaction as long as it is out of IDLE state 0x0 : Force-idle mode: local target's idle state follows (acknowledges) the system's idle requests unconditionally, i.e. regardless of the IP module's internal requirements. Backup mode, for debug only 0x1 : No-idle mode: local target never enters idle state. Backup mode, for debug only 0x2 : Smart-idle mode: local target's idle state eventually follows (acknowledges) the system's idle requests, depending on the IP module's internal requirements. IP module shall not generate (IRQ- or DMA-request-related) wakeup events 0x3 : Smart-idle wakeup-capable mode: local target's idle state eventually follows (acknowledges) the system's idle requests, depending on the IP module's internal requirements. IP module may generate (IRQ- or DMA-request-related) wakeup events when in idle state. Mode is only relevant if the appropriate IP module \"swakeup\" output(s) is (are) implemented"]
    #[inline(always)]
    #[must_use]
    pub fn idlemode(&mut self) -> IdlemodeW<SysconfigSpec> {
        IdlemodeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SysconfigSpec> {
        Reserved2W::new(self, 4)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SysconfigSpec> {
        Reserved3W::new(self, 6)
    }
}
#[doc = "SYSCONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`sysconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysconfigSpec;
impl crate::RegisterSpec for SysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysconfig::R`](R) reader structure"]
impl crate::Readable for SysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`sysconfig::W`](W) writer structure"]
impl crate::Writable for SysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCONFIG to value 0"]
impl crate::Resettable for SysconfigSpec {
    const RESET_VALUE: u32 = 0;
}
