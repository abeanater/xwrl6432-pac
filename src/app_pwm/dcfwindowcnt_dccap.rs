#[doc = "Register `DCFWINDOWCNT_DCCAP` reader"]
pub type R = crate::R<DcfwindowcntDccapSpec>;
#[doc = "Register `DCFWINDOWCNT_DCCAP` writer"]
pub type W = crate::W<DcfwindowcntDccapSpec>;
#[doc = "Field `DCFWINDOWCNT` reader - 7:0\\]
0-FFh Blanking Window Counter These 8 bits are read only and indicate the current value of the window counter. The counter counts down to zero and then stops until it is re-loaded when the offset counter reaches zero again."]
pub type DcfwindowcntR = crate::FieldReader;
#[doc = "Field `DCFWINDOWCNT` writer - 7:0\\]
0-FFh Blanking Window Counter These 8 bits are read only and indicate the current value of the window counter. The counter counts down to zero and then stops until it is re-loaded when the offset counter reaches zero again."]
pub type DcfwindowcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - 15:8\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 15:8\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DCCAP` reader - 31:16\\]
Digital Compare Time-Base Counter Capture To enable time-base counter capture, set the DCCAPCLT\\[CAPE\\]
bit to 1. If enabled, reflects the value of the time-base counter (TBCTR) on the low to high edge transition of a filtered (DCEVTFLT) event. Further capture events are ignored until the next period or zero as selected by the DCFCTL\\[PULSESEL\\]
bit. Shadowing of DCCAP is enabled and disabled by the DCCAPCTL\\[SHDWMODE\\]
bit. By default this register is shadowed. ΓÇó If DCCAPCTL\\[SHDWMODE\\]
= 0, then the shadow is enabled. In this mode, the active register is copied to the shadow register on the TBCTR = TBPRD or TBCTR = zero as defined by the DCFCTL\\[PULSESEL\\]
bit. CPU reads of this register will return the shadow register value. ΓÇó If DCCAPCTL\\[SHDWMODE\\]
= 1, then the shadow register is disabled. In this mode, CPU reads will return the active register value. The active and shadow registers share the same memory map address."]
pub type DccapR = crate::FieldReader<u16>;
#[doc = "Field `DCCAP` writer - 31:16\\]
Digital Compare Time-Base Counter Capture To enable time-base counter capture, set the DCCAPCLT\\[CAPE\\]
bit to 1. If enabled, reflects the value of the time-base counter (TBCTR) on the low to high edge transition of a filtered (DCEVTFLT) event. Further capture events are ignored until the next period or zero as selected by the DCFCTL\\[PULSESEL\\]
bit. Shadowing of DCCAP is enabled and disabled by the DCCAPCTL\\[SHDWMODE\\]
bit. By default this register is shadowed. ΓÇó If DCCAPCTL\\[SHDWMODE\\]
= 0, then the shadow is enabled. In this mode, the active register is copied to the shadow register on the TBCTR = TBPRD or TBCTR = zero as defined by the DCFCTL\\[PULSESEL\\]
bit. CPU reads of this register will return the shadow register value. ΓÇó If DCCAPCTL\\[SHDWMODE\\]
= 1, then the shadow register is disabled. In this mode, CPU reads will return the active register value. The active and shadow registers share the same memory map address."]
pub type DccapW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
0-FFh Blanking Window Counter These 8 bits are read only and indicate the current value of the window counter. The counter counts down to zero and then stops until it is re-loaded when the offset counter reaches zero again."]
    #[inline(always)]
    pub fn dcfwindowcnt(&self) -> DcfwindowcntR {
        DcfwindowcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Digital Compare Time-Base Counter Capture To enable time-base counter capture, set the DCCAPCLT\\[CAPE\\]
bit to 1. If enabled, reflects the value of the time-base counter (TBCTR) on the low to high edge transition of a filtered (DCEVTFLT) event. Further capture events are ignored until the next period or zero as selected by the DCFCTL\\[PULSESEL\\]
bit. Shadowing of DCCAP is enabled and disabled by the DCCAPCTL\\[SHDWMODE\\]
bit. By default this register is shadowed. ΓÇó If DCCAPCTL\\[SHDWMODE\\]
= 0, then the shadow is enabled. In this mode, the active register is copied to the shadow register on the TBCTR = TBPRD or TBCTR = zero as defined by the DCFCTL\\[PULSESEL\\]
bit. CPU reads of this register will return the shadow register value. ΓÇó If DCCAPCTL\\[SHDWMODE\\]
= 1, then the shadow register is disabled. In this mode, CPU reads will return the active register value. The active and shadow registers share the same memory map address."]
    #[inline(always)]
    pub fn dccap(&self) -> DccapR {
        DccapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
0-FFh Blanking Window Counter These 8 bits are read only and indicate the current value of the window counter. The counter counts down to zero and then stops until it is re-loaded when the offset counter reaches zero again."]
    #[inline(always)]
    #[must_use]
    pub fn dcfwindowcnt(&mut self) -> DcfwindowcntW<DcfwindowcntDccapSpec> {
        DcfwindowcntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DcfwindowcntDccapSpec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Digital Compare Time-Base Counter Capture To enable time-base counter capture, set the DCCAPCLT\\[CAPE\\]
bit to 1. If enabled, reflects the value of the time-base counter (TBCTR) on the low to high edge transition of a filtered (DCEVTFLT) event. Further capture events are ignored until the next period or zero as selected by the DCFCTL\\[PULSESEL\\]
bit. Shadowing of DCCAP is enabled and disabled by the DCCAPCTL\\[SHDWMODE\\]
bit. By default this register is shadowed. ΓÇó If DCCAPCTL\\[SHDWMODE\\]
= 0, then the shadow is enabled. In this mode, the active register is copied to the shadow register on the TBCTR = TBPRD or TBCTR = zero as defined by the DCFCTL\\[PULSESEL\\]
bit. CPU reads of this register will return the shadow register value. ΓÇó If DCCAPCTL\\[SHDWMODE\\]
= 1, then the shadow register is disabled. In this mode, CPU reads will return the active register value. The active and shadow registers share the same memory map address."]
    #[inline(always)]
    #[must_use]
    pub fn dccap(&mut self) -> DccapW<DcfwindowcntDccapSpec> {
        DccapW::new(self, 16)
    }
}
#[doc = "Digital Compare Filter Window Counter Register/ Digital Compare Counter Capture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfwindowcnt_dccap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfwindowcnt_dccap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfwindowcntDccapSpec;
impl crate::RegisterSpec for DcfwindowcntDccapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfwindowcnt_dccap::R`](R) reader structure"]
impl crate::Readable for DcfwindowcntDccapSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfwindowcnt_dccap::W`](W) writer structure"]
impl crate::Writable for DcfwindowcntDccapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFWINDOWCNT_DCCAP to value 0"]
impl crate::Resettable for DcfwindowcntDccapSpec {
    const RESET_VALUE: u32 = 0;
}
