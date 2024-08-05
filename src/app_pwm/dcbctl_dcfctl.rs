#[doc = "Register `DCBCTL_DCFCTL` reader"]
pub type R = crate::R<DcbctlDcfctlSpec>;
#[doc = "Register `DCBCTL_DCFCTL` writer"]
pub type W = crate::W<DcbctlDcfctlSpec>;
#[doc = "Field `DCBCTL_EVT1SRCSEL` reader - 0:0\\]
DCBEVT1 Source Signal Select 0 Source Is DCBEVT1 Signal 1 Source Is DCEVTFILT Signal"]
pub type DcbctlEvt1srcselR = crate::BitReader;
#[doc = "Field `DCBCTL_EVT1SRCSEL` writer - 0:0\\]
DCBEVT1 Source Signal Select 0 Source Is DCBEVT1 Signal 1 Source Is DCEVTFILT Signal"]
pub type DcbctlEvt1srcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCBCTL_EVT1FRC_SYNCSEL` reader - 1:1\\]
DCBEVT1 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
pub type DcbctlEvt1frcSyncselR = crate::BitReader;
#[doc = "Field `DCBCTL_EVT1FRC_SYNCSEL` writer - 1:1\\]
DCBEVT1 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
pub type DcbctlEvt1frcSyncselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCBCTL_EVT1SOCE` reader - 2:2\\]
DCBEVT1 SOC, Enable/Disable 0 SOC Generation Disabled 1 SOC Generation Enabled"]
pub type DcbctlEvt1soceR = crate::BitReader;
#[doc = "Field `DCBCTL_EVT1SOCE` writer - 2:2\\]
DCBEVT1 SOC, Enable/Disable 0 SOC Generation Disabled 1 SOC Generation Enabled"]
pub type DcbctlEvt1soceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCBCTL_EVT1SYNCE` reader - 3:3\\]
DCBEVT1 SYNC, Enable/Disable 0 SYNC Generation Disabled 1 SYNC Generation Enabled"]
pub type DcbctlEvt1synceR = crate::BitReader;
#[doc = "Field `DCBCTL_EVT1SYNCE` writer - 3:3\\]
DCBEVT1 SYNC, Enable/Disable 0 SYNC Generation Disabled 1 SYNC Generation Enabled"]
pub type DcbctlEvt1synceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 7:4\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 7:4\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DCBCTL_EVT2SRCSEL` reader - 8:8\\]
DCBEVT2 Source Signal Select 0 Source Is DCBEVT2 Signal 1 Source Is DCEVTFILT Signal"]
pub type DcbctlEvt2srcselR = crate::BitReader;
#[doc = "Field `DCBCTL_EVT2SRCSEL` writer - 8:8\\]
DCBEVT2 Source Signal Select 0 Source Is DCBEVT2 Signal 1 Source Is DCEVTFILT Signal"]
pub type DcbctlEvt2srcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCBCTL_EVT2FRC_SYNCSEL` reader - 9:9\\]
DCBEVT2 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
pub type DcbctlEvt2frcSyncselR = crate::BitReader;
#[doc = "Field `DCBCTL_EVT2FRC_SYNCSEL` writer - 9:9\\]
DCBEVT2 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
pub type DcbctlEvt2frcSyncselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 15:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 15:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCFCTL_SRCSEL` reader - 17:16\\]
Filter Block Signal Source Select 0 Source Is DCAEVT1 Signal 1h Source Is DCAEVT2 Signal 2h Source Is DCBEVT1 Signal 3h Source Is DCBEVT2 Signal"]
pub type DcfctlSrcselR = crate::FieldReader;
#[doc = "Field `DCFCTL_SRCSEL` writer - 17:16\\]
Filter Block Signal Source Select 0 Source Is DCAEVT1 Signal 1h Source Is DCAEVT2 Signal 2h Source Is DCBEVT1 Signal 3h Source Is DCBEVT2 Signal"]
pub type DcfctlSrcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCFCTL_BLANKE` reader - 18:18\\]
Blanking Window Enable/Disable 0 Blanking window is disabled 1 Blanking window is enabled"]
pub type DcfctlBlankeR = crate::BitReader;
#[doc = "Field `DCFCTL_BLANKE` writer - 18:18\\]
Blanking Window Enable/Disable 0 Blanking window is disabled 1 Blanking window is enabled"]
pub type DcfctlBlankeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCFCTL_BLANKINV` reader - 19:19\\]
Blanking Window Inversion 0 Blanking window not inverted 1 Blanking window inverted"]
pub type DcfctlBlankinvR = crate::BitReader;
#[doc = "Field `DCFCTL_BLANKINV` writer - 19:19\\]
Blanking Window Inversion 0 Blanking window not inverted 1 Blanking window inverted"]
pub type DcfctlBlankinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCFCTL_PULSESEL` reader - 21:20\\]
Pulse Select For Blanking &amp; Capture Alignment 0 Time-base counter equal to period (TBCTR = TBPRD) 1h Time-base counter equal to zero (TBCTR = 0x0000) 2h-3h Reserved"]
pub type DcfctlPulseselR = crate::FieldReader;
#[doc = "Field `DCFCTL_PULSESEL` writer - 21:20\\]
Pulse Select For Blanking &amp; Capture Alignment 0 Time-base counter equal to period (TBCTR = TBPRD) 1h Time-base counter equal to zero (TBCTR = 0x0000) 2h-3h Reserved"]
pub type DcfctlPulseselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - 31:22\\]
Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `Reserved3` writer - 31:22\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DCBEVT1 Source Signal Select 0 Source Is DCBEVT1 Signal 1 Source Is DCEVTFILT Signal"]
    #[inline(always)]
    pub fn dcbctl_evt1srcsel(&self) -> DcbctlEvt1srcselR {
        DcbctlEvt1srcselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DCBEVT1 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
    #[inline(always)]
    pub fn dcbctl_evt1frc_syncsel(&self) -> DcbctlEvt1frcSyncselR {
        DcbctlEvt1frcSyncselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
DCBEVT1 SOC, Enable/Disable 0 SOC Generation Disabled 1 SOC Generation Enabled"]
    #[inline(always)]
    pub fn dcbctl_evt1soce(&self) -> DcbctlEvt1soceR {
        DcbctlEvt1soceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
DCBEVT1 SYNC, Enable/Disable 0 SYNC Generation Disabled 1 SYNC Generation Enabled"]
    #[inline(always)]
    pub fn dcbctl_evt1synce(&self) -> DcbctlEvt1synceR {
        DcbctlEvt1synceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
DCBEVT2 Source Signal Select 0 Source Is DCBEVT2 Signal 1 Source Is DCEVTFILT Signal"]
    #[inline(always)]
    pub fn dcbctl_evt2srcsel(&self) -> DcbctlEvt2srcselR {
        DcbctlEvt2srcselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
DCBEVT2 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
    #[inline(always)]
    pub fn dcbctl_evt2frc_syncsel(&self) -> DcbctlEvt2frcSyncselR {
        DcbctlEvt2frcSyncselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Filter Block Signal Source Select 0 Source Is DCAEVT1 Signal 1h Source Is DCAEVT2 Signal 2h Source Is DCBEVT1 Signal 3h Source Is DCBEVT2 Signal"]
    #[inline(always)]
    pub fn dcfctl_srcsel(&self) -> DcfctlSrcselR {
        DcfctlSrcselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Blanking Window Enable/Disable 0 Blanking window is disabled 1 Blanking window is enabled"]
    #[inline(always)]
    pub fn dcfctl_blanke(&self) -> DcfctlBlankeR {
        DcfctlBlankeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Blanking Window Inversion 0 Blanking window not inverted 1 Blanking window inverted"]
    #[inline(always)]
    pub fn dcfctl_blankinv(&self) -> DcfctlBlankinvR {
        DcfctlBlankinvR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Pulse Select For Blanking &amp; Capture Alignment 0 Time-base counter equal to period (TBCTR = TBPRD) 1h Time-base counter equal to zero (TBCTR = 0x0000) 2h-3h Reserved"]
    #[inline(always)]
    pub fn dcfctl_pulsesel(&self) -> DcfctlPulseselR {
        DcfctlPulseselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DCBEVT1 Source Signal Select 0 Source Is DCBEVT1 Signal 1 Source Is DCEVTFILT Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcbctl_evt1srcsel(&mut self) -> DcbctlEvt1srcselW<DcbctlDcfctlSpec> {
        DcbctlEvt1srcselW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DCBEVT1 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcbctl_evt1frc_syncsel(&mut self) -> DcbctlEvt1frcSyncselW<DcbctlDcfctlSpec> {
        DcbctlEvt1frcSyncselW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
DCBEVT1 SOC, Enable/Disable 0 SOC Generation Disabled 1 SOC Generation Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dcbctl_evt1soce(&mut self) -> DcbctlEvt1soceW<DcbctlDcfctlSpec> {
        DcbctlEvt1soceW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
DCBEVT1 SYNC, Enable/Disable 0 SYNC Generation Disabled 1 SYNC Generation Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dcbctl_evt1synce(&mut self) -> DcbctlEvt1synceW<DcbctlDcfctlSpec> {
        DcbctlEvt1synceW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DcbctlDcfctlSpec> {
        Reserved1W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DCBEVT2 Source Signal Select 0 Source Is DCBEVT2 Signal 1 Source Is DCEVTFILT Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcbctl_evt2srcsel(&mut self) -> DcbctlEvt2srcselW<DcbctlDcfctlSpec> {
        DcbctlEvt2srcselW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
DCBEVT2 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcbctl_evt2frc_syncsel(&mut self) -> DcbctlEvt2frcSyncselW<DcbctlDcfctlSpec> {
        DcbctlEvt2frcSyncselW::new(self, 9)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<DcbctlDcfctlSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Filter Block Signal Source Select 0 Source Is DCAEVT1 Signal 1h Source Is DCAEVT2 Signal 2h Source Is DCBEVT1 Signal 3h Source Is DCBEVT2 Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcfctl_srcsel(&mut self) -> DcfctlSrcselW<DcbctlDcfctlSpec> {
        DcfctlSrcselW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
Blanking Window Enable/Disable 0 Blanking window is disabled 1 Blanking window is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dcfctl_blanke(&mut self) -> DcfctlBlankeW<DcbctlDcfctlSpec> {
        DcfctlBlankeW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Blanking Window Inversion 0 Blanking window not inverted 1 Blanking window inverted"]
    #[inline(always)]
    #[must_use]
    pub fn dcfctl_blankinv(&mut self) -> DcfctlBlankinvW<DcbctlDcfctlSpec> {
        DcfctlBlankinvW::new(self, 19)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Pulse Select For Blanking &amp; Capture Alignment 0 Time-base counter equal to period (TBCTR = TBPRD) 1h Time-base counter equal to zero (TBCTR = 0x0000) 2h-3h Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dcfctl_pulsesel(&mut self) -> DcfctlPulseselW<DcbctlDcfctlSpec> {
        DcfctlPulseselW::new(self, 20)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<DcbctlDcfctlSpec> {
        Reserved3W::new(self, 22)
    }
}
#[doc = "Digital Compare B Control Register/ Digital Compare Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcbctl_dcfctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcbctl_dcfctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcbctlDcfctlSpec;
impl crate::RegisterSpec for DcbctlDcfctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcbctl_dcfctl::R`](R) reader structure"]
impl crate::Readable for DcbctlDcfctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcbctl_dcfctl::W`](W) writer structure"]
impl crate::Writable for DcbctlDcfctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCBCTL_DCFCTL to value 0"]
impl crate::Resettable for DcbctlDcfctlSpec {
    const RESET_VALUE: u32 = 0;
}
