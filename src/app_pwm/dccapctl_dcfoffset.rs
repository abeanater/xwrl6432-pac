#[doc = "Register `DCCAPCTL_DCFOFFSET` reader"]
pub type R = crate::R<DccapctlDcfoffsetSpec>;
#[doc = "Register `DCCAPCTL_DCFOFFSET` writer"]
pub type W = crate::W<DccapctlDcfoffsetSpec>;
#[doc = "Field `DCCAPCTL_CAPE` reader - 0:0\\]
TBCTR Counter Capture Enable/Disable 0 Disable the time-base counter capture. 1 Enable the time-base counter capture."]
pub type DccapctlCapeR = crate::BitReader;
#[doc = "Field `DCCAPCTL_CAPE` writer - 0:0\\]
TBCTR Counter Capture Enable/Disable 0 Disable the time-base counter capture. 1 Enable the time-base counter capture."]
pub type DccapctlCapeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCCAPCTL_SHDWMODE` reader - 1:1\\]
TBCTR Counter Capture Shadow Select Mode 0 Enable shadow mode. The DCCAP active register is copied to shadow register on a TBCTR = TBPRD or TBCTR = zero event as defined by the DCFCTL\\[PULSESEL\\]
bit. CPU reads of the DCCAP register will return the shadow register contents. 1 Active Mode. In this mode the shadow register is disabled. CPU reads from the DCCAP register will always return the active register contents"]
pub type DccapctlShdwmodeR = crate::BitReader;
#[doc = "Field `DCCAPCTL_SHDWMODE` writer - 1:1\\]
TBCTR Counter Capture Shadow Select Mode 0 Enable shadow mode. The DCCAP active register is copied to shadow register on a TBCTR = TBPRD or TBCTR = zero event as defined by the DCFCTL\\[PULSESEL\\]
bit. CPU reads of the DCCAP register will return the shadow register contents. 1 Active Mode. In this mode the shadow register is disabled. CPU reads from the DCCAP register will always return the active register contents"]
pub type DccapctlShdwmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCFOFFSET_OFFSET` reader - 31:16\\]
Blanking Window Offset These 16-bits specify the number of TBCLK cycles from the blanking window reference to the point when the blanking window is applied. The blanking window reference is either period or zero as defined by the DCFCTL\\[PULSESEL\\]
bit. This offset register is shadowed and the active register is loaded at the reference point defined by DCFCTL\\[PULSESEL\\]. The offset counter is also initialized and begins to count down when the active register is loaded. When the counter expires, the blanking window is applied. If the blanking window is currently active, then the blanking window counter is restarted."]
pub type DcfoffsetOffsetR = crate::FieldReader<u16>;
#[doc = "Field `DCFOFFSET_OFFSET` writer - 31:16\\]
Blanking Window Offset These 16-bits specify the number of TBCLK cycles from the blanking window reference to the point when the blanking window is applied. The blanking window reference is either period or zero as defined by the DCFCTL\\[PULSESEL\\]
bit. This offset register is shadowed and the active register is loaded at the reference point defined by DCFCTL\\[PULSESEL\\]. The offset counter is also initialized and begins to count down when the active register is loaded. When the counter expires, the blanking window is applied. If the blanking window is currently active, then the blanking window counter is restarted."]
pub type DcfoffsetOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TBCTR Counter Capture Enable/Disable 0 Disable the time-base counter capture. 1 Enable the time-base counter capture."]
    #[inline(always)]
    pub fn dccapctl_cape(&self) -> DccapctlCapeR {
        DccapctlCapeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TBCTR Counter Capture Shadow Select Mode 0 Enable shadow mode. The DCCAP active register is copied to shadow register on a TBCTR = TBPRD or TBCTR = zero event as defined by the DCFCTL\\[PULSESEL\\]
bit. CPU reads of the DCCAP register will return the shadow register contents. 1 Active Mode. In this mode the shadow register is disabled. CPU reads from the DCCAP register will always return the active register contents"]
    #[inline(always)]
    pub fn dccapctl_shdwmode(&self) -> DccapctlShdwmodeR {
        DccapctlShdwmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Blanking Window Offset These 16-bits specify the number of TBCLK cycles from the blanking window reference to the point when the blanking window is applied. The blanking window reference is either period or zero as defined by the DCFCTL\\[PULSESEL\\]
bit. This offset register is shadowed and the active register is loaded at the reference point defined by DCFCTL\\[PULSESEL\\]. The offset counter is also initialized and begins to count down when the active register is loaded. When the counter expires, the blanking window is applied. If the blanking window is currently active, then the blanking window counter is restarted."]
    #[inline(always)]
    pub fn dcfoffset_offset(&self) -> DcfoffsetOffsetR {
        DcfoffsetOffsetR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TBCTR Counter Capture Enable/Disable 0 Disable the time-base counter capture. 1 Enable the time-base counter capture."]
    #[inline(always)]
    #[must_use]
    pub fn dccapctl_cape(&mut self) -> DccapctlCapeW<DccapctlDcfoffsetSpec> {
        DccapctlCapeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TBCTR Counter Capture Shadow Select Mode 0 Enable shadow mode. The DCCAP active register is copied to shadow register on a TBCTR = TBPRD or TBCTR = zero event as defined by the DCFCTL\\[PULSESEL\\]
bit. CPU reads of the DCCAP register will return the shadow register contents. 1 Active Mode. In this mode the shadow register is disabled. CPU reads from the DCCAP register will always return the active register contents"]
    #[inline(always)]
    #[must_use]
    pub fn dccapctl_shdwmode(&mut self) -> DccapctlShdwmodeW<DccapctlDcfoffsetSpec> {
        DccapctlShdwmodeW::new(self, 1)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Blanking Window Offset These 16-bits specify the number of TBCLK cycles from the blanking window reference to the point when the blanking window is applied. The blanking window reference is either period or zero as defined by the DCFCTL\\[PULSESEL\\]
bit. This offset register is shadowed and the active register is loaded at the reference point defined by DCFCTL\\[PULSESEL\\]. The offset counter is also initialized and begins to count down when the active register is loaded. When the counter expires, the blanking window is applied. If the blanking window is currently active, then the blanking window counter is restarted."]
    #[inline(always)]
    #[must_use]
    pub fn dcfoffset_offset(&mut self) -> DcfoffsetOffsetW<DccapctlDcfoffsetSpec> {
        DcfoffsetOffsetW::new(self, 16)
    }
}
#[doc = "Digital Compare Capture Control Register/ Digital Compare Filter Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dccapctl_dcfoffset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccapctl_dcfoffset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DccapctlDcfoffsetSpec;
impl crate::RegisterSpec for DccapctlDcfoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccapctl_dcfoffset::R`](R) reader structure"]
impl crate::Readable for DccapctlDcfoffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dccapctl_dcfoffset::W`](W) writer structure"]
impl crate::Writable for DccapctlDcfoffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCAPCTL_DCFOFFSET to value 0"]
impl crate::Resettable for DccapctlDcfoffsetSpec {
    const RESET_VALUE: u32 = 0;
}
