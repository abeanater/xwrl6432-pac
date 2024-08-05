#[doc = "Register `LIN_SMART_IDLE` reader"]
pub type R = crate::R<LinSmartIdleSpec>;
#[doc = "Register `LIN_SMART_IDLE` writer"]
pub type W = crate::W<LinSmartIdleSpec>;
#[doc = "Field `enable` reader - 0:0\\]
1 => Smart IDLE mode enabled. When set, Request the clock gating of LIN module.0 => Disable Smart IDLE mode for LIN"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - 0:0\\]
1 => Smart IDLE mode enabled. When set, Request the clock gating of LIN module.0 => Disable Smart IDLE mode for LIN"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ack` reader - 1:1\\]
1 => LIN in smart idle mode0 => LIN not in smart idle mode"]
pub type AckR = crate::BitReader;
#[doc = "Field `ack` writer - 1:1\\]
1 => LIN in smart idle mode0 => LIN not in smart idle mode"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1 => Smart IDLE mode enabled. When set, Request the clock gating of LIN module.0 => Disable Smart IDLE mode for LIN"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 => LIN in smart idle mode0 => LIN not in smart idle mode"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1 => Smart IDLE mode enabled. When set, Request the clock gating of LIN module.0 => Disable Smart IDLE mode for LIN"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<LinSmartIdleSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 => LIN in smart idle mode0 => LIN not in smart idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<LinSmartIdleSpec> {
        AckW::new(self, 1)
    }
}
#[doc = "LIN_SMART_IDLE\n\nYou can [`read`](crate::Reg::read) this register and get [`lin_smart_idle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lin_smart_idle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinSmartIdleSpec;
impl crate::RegisterSpec for LinSmartIdleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lin_smart_idle::R`](R) reader structure"]
impl crate::Readable for LinSmartIdleSpec {}
#[doc = "`write(|w| ..)` method takes [`lin_smart_idle::W`](W) writer structure"]
impl crate::Writable for LinSmartIdleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIN_SMART_IDLE to value 0"]
impl crate::Resettable for LinSmartIdleSpec {
    const RESET_VALUE: u32 = 0;
}
