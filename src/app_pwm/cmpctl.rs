#[doc = "Register `CMPCTL` reader"]
pub type R = crate::R<CmpctlSpec>;
#[doc = "Register `CMPCTL` writer"]
pub type W = crate::W<CmpctlSpec>;
#[doc = "Field `Reserved1` reader - 15:0\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 15:0\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LOADAMODE` reader - 17:16\\]
Active Counter-Compare A (CMPA) Load From Shadow Select Mode. This bit has no effect in immediate mode (CMPCTL\\[SHDWAMODE\\]
= 1). 0 Load on CTR = Zero: Time-base counter equal to zero (TBCTR = 0x0000) 1h Load on CTR = PRD: Time-base counter equal to period (TBCTR = TBPRD) 2h Load on either CTR = Zero or CTR = PRD 3h Freeze (no loads possible)"]
pub type LoadamodeR = crate::FieldReader;
#[doc = "Field `LOADAMODE` writer - 17:16\\]
Active Counter-Compare A (CMPA) Load From Shadow Select Mode. This bit has no effect in immediate mode (CMPCTL\\[SHDWAMODE\\]
= 1). 0 Load on CTR = Zero: Time-base counter equal to zero (TBCTR = 0x0000) 1h Load on CTR = PRD: Time-base counter equal to period (TBCTR = TBPRD) 2h Load on either CTR = Zero or CTR = PRD 3h Freeze (no loads possible)"]
pub type LoadamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOADBMODE` reader - 19:18\\]
Active Counter-Compare B (CMPB) Load From Shadow Select Mode This bit has no effect in immediate mode (CMPCTL\\[SHDWBMODE\\]
= 1). 0 Load on CTR = Zero: Time-base counter equal to zero (TBCTR = 0x0000) 1h Load on CTR = PRD: Time-base counter equal to period (TBCTR = TBPRD) 2h Load on either CTR = Zero or CTR = PRD 3h Freeze (no loads possible)"]
pub type LoadbmodeR = crate::FieldReader;
#[doc = "Field `LOADBMODE` writer - 19:18\\]
Active Counter-Compare B (CMPB) Load From Shadow Select Mode This bit has no effect in immediate mode (CMPCTL\\[SHDWBMODE\\]
= 1). 0 Load on CTR = Zero: Time-base counter equal to zero (TBCTR = 0x0000) 1h Load on CTR = PRD: Time-base counter equal to period (TBCTR = TBPRD) 2h Load on either CTR = Zero or CTR = PRD 3h Freeze (no loads possible)"]
pub type LoadbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SHDWAMODE` reader - 20:20\\]
Counter-compare A (CMPA) Register Operating Mode 0 Shadow mode. Operates as a double buffer. All writes via the CPU access the shadow register. 1 Immediate mode. Only the active compare register is used. All writes and reads directly access the active register for immediate compare action"]
pub type ShdwamodeR = crate::BitReader;
#[doc = "Field `SHDWAMODE` writer - 20:20\\]
Counter-compare A (CMPA) Register Operating Mode 0 Shadow mode. Operates as a double buffer. All writes via the CPU access the shadow register. 1 Immediate mode. Only the active compare register is used. All writes and reads directly access the active register for immediate compare action"]
pub type ShdwamodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 21:21\\]
Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - 21:21\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHDWBMODE` reader - 22:22\\]
Counter-compare B (CMPB) Register Operating Mode 0 Shadow mode. Operates as a double buffer. All writes via the CPU access the shadow register. 1 Immediate mode. Only the active compare B register is used. All writes and reads directly access the active register for immediate compare action."]
pub type ShdwbmodeR = crate::BitReader;
#[doc = "Field `SHDWBMODE` writer - 22:22\\]
Counter-compare B (CMPB) Register Operating Mode 0 Shadow mode. Operates as a double buffer. All writes via the CPU access the shadow register. 1 Immediate mode. Only the active compare B register is used. All writes and reads directly access the active register for immediate compare action."]
pub type ShdwbmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 23:23\\]
Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - 23:23\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHDWAFULL` reader - 24:24\\]
Counter-compare A (CMPA) Shadow Register Full Status Flag The flag bit is set when a 32-bit write to CMPA:CMPAHR register or a 16-bit write to CMPA register is made. A 16-bit write to CMPAHR register will not affect the flag. This bit self clears once a load-strobe occurs. 0 CMPA shadow FIFO not full yet 1 Indicates the CMPA shadow FIFO is full, a CPU write will overwrite the current shadow value"]
pub type ShdwafullR = crate::BitReader;
#[doc = "Field `SHDWAFULL` writer - 24:24\\]
Counter-compare A (CMPA) Shadow Register Full Status Flag The flag bit is set when a 32-bit write to CMPA:CMPAHR register or a 16-bit write to CMPA register is made. A 16-bit write to CMPAHR register will not affect the flag. This bit self clears once a load-strobe occurs. 0 CMPA shadow FIFO not full yet 1 Indicates the CMPA shadow FIFO is full, a CPU write will overwrite the current shadow value"]
pub type ShdwafullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHDWBFULL` reader - 25:25\\]
Counter-compare B (CMPB) Shadow Register Full Status Flag This bit self clears once a load-strobe occurs. 0 CMPB shadow FIFO not full yet 1 Indicates the CMPB shadow FIFO is full; a CPU write will overwrite current shadow value."]
pub type ShdwbfullR = crate::BitReader;
#[doc = "Field `SHDWBFULL` writer - 25:25\\]
Counter-compare B (CMPB) Shadow Register Full Status Flag This bit self clears once a load-strobe occurs. 0 CMPB shadow FIFO not full yet 1 Indicates the CMPB shadow FIFO is full; a CPU write will overwrite current shadow value."]
pub type ShdwbfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - 31:26\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 31:26\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Active Counter-Compare A (CMPA) Load From Shadow Select Mode. This bit has no effect in immediate mode (CMPCTL\\[SHDWAMODE\\]
= 1). 0 Load on CTR = Zero: Time-base counter equal to zero (TBCTR = 0x0000) 1h Load on CTR = PRD: Time-base counter equal to period (TBCTR = TBPRD) 2h Load on either CTR = Zero or CTR = PRD 3h Freeze (no loads possible)"]
    #[inline(always)]
    pub fn loadamode(&self) -> LoadamodeR {
        LoadamodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Active Counter-Compare B (CMPB) Load From Shadow Select Mode This bit has no effect in immediate mode (CMPCTL\\[SHDWBMODE\\]
= 1). 0 Load on CTR = Zero: Time-base counter equal to zero (TBCTR = 0x0000) 1h Load on CTR = PRD: Time-base counter equal to period (TBCTR = TBPRD) 2h Load on either CTR = Zero or CTR = PRD 3h Freeze (no loads possible)"]
    #[inline(always)]
    pub fn loadbmode(&self) -> LoadbmodeR {
        LoadbmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Counter-compare A (CMPA) Register Operating Mode 0 Shadow mode. Operates as a double buffer. All writes via the CPU access the shadow register. 1 Immediate mode. Only the active compare register is used. All writes and reads directly access the active register for immediate compare action"]
    #[inline(always)]
    pub fn shdwamode(&self) -> ShdwamodeR {
        ShdwamodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Counter-compare B (CMPB) Register Operating Mode 0 Shadow mode. Operates as a double buffer. All writes via the CPU access the shadow register. 1 Immediate mode. Only the active compare B register is used. All writes and reads directly access the active register for immediate compare action."]
    #[inline(always)]
    pub fn shdwbmode(&self) -> ShdwbmodeR {
        ShdwbmodeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Counter-compare A (CMPA) Shadow Register Full Status Flag The flag bit is set when a 32-bit write to CMPA:CMPAHR register or a 16-bit write to CMPA register is made. A 16-bit write to CMPAHR register will not affect the flag. This bit self clears once a load-strobe occurs. 0 CMPA shadow FIFO not full yet 1 Indicates the CMPA shadow FIFO is full, a CPU write will overwrite the current shadow value"]
    #[inline(always)]
    pub fn shdwafull(&self) -> ShdwafullR {
        ShdwafullR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Counter-compare B (CMPB) Shadow Register Full Status Flag This bit self clears once a load-strobe occurs. 0 CMPB shadow FIFO not full yet 1 Indicates the CMPB shadow FIFO is full; a CPU write will overwrite current shadow value."]
    #[inline(always)]
    pub fn shdwbfull(&self) -> ShdwbfullR {
        ShdwbfullR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CmpctlSpec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Active Counter-Compare A (CMPA) Load From Shadow Select Mode. This bit has no effect in immediate mode (CMPCTL\\[SHDWAMODE\\]
= 1). 0 Load on CTR = Zero: Time-base counter equal to zero (TBCTR = 0x0000) 1h Load on CTR = PRD: Time-base counter equal to period (TBCTR = TBPRD) 2h Load on either CTR = Zero or CTR = PRD 3h Freeze (no loads possible)"]
    #[inline(always)]
    #[must_use]
    pub fn loadamode(&mut self) -> LoadamodeW<CmpctlSpec> {
        LoadamodeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Active Counter-Compare B (CMPB) Load From Shadow Select Mode This bit has no effect in immediate mode (CMPCTL\\[SHDWBMODE\\]
= 1). 0 Load on CTR = Zero: Time-base counter equal to zero (TBCTR = 0x0000) 1h Load on CTR = PRD: Time-base counter equal to period (TBCTR = TBPRD) 2h Load on either CTR = Zero or CTR = PRD 3h Freeze (no loads possible)"]
    #[inline(always)]
    #[must_use]
    pub fn loadbmode(&mut self) -> LoadbmodeW<CmpctlSpec> {
        LoadbmodeW::new(self, 18)
    }
    #[doc = "Bit 20 - 20:20\\]
Counter-compare A (CMPA) Register Operating Mode 0 Shadow mode. Operates as a double buffer. All writes via the CPU access the shadow register. 1 Immediate mode. Only the active compare register is used. All writes and reads directly access the active register for immediate compare action"]
    #[inline(always)]
    #[must_use]
    pub fn shdwamode(&mut self) -> ShdwamodeW<CmpctlSpec> {
        ShdwamodeW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CmpctlSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Counter-compare B (CMPB) Register Operating Mode 0 Shadow mode. Operates as a double buffer. All writes via the CPU access the shadow register. 1 Immediate mode. Only the active compare B register is used. All writes and reads directly access the active register for immediate compare action."]
    #[inline(always)]
    #[must_use]
    pub fn shdwbmode(&mut self) -> ShdwbmodeW<CmpctlSpec> {
        ShdwbmodeW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CmpctlSpec> {
        Reserved3W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Counter-compare A (CMPA) Shadow Register Full Status Flag The flag bit is set when a 32-bit write to CMPA:CMPAHR register or a 16-bit write to CMPA register is made. A 16-bit write to CMPAHR register will not affect the flag. This bit self clears once a load-strobe occurs. 0 CMPA shadow FIFO not full yet 1 Indicates the CMPA shadow FIFO is full, a CPU write will overwrite the current shadow value"]
    #[inline(always)]
    #[must_use]
    pub fn shdwafull(&mut self) -> ShdwafullW<CmpctlSpec> {
        ShdwafullW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Counter-compare B (CMPB) Shadow Register Full Status Flag This bit self clears once a load-strobe occurs. 0 CMPB shadow FIFO not full yet 1 Indicates the CMPB shadow FIFO is full; a CPU write will overwrite current shadow value."]
    #[inline(always)]
    #[must_use]
    pub fn shdwbfull(&mut self) -> ShdwbfullW<CmpctlSpec> {
        ShdwbfullW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CmpctlSpec> {
        Reserved4W::new(self, 26)
    }
}
#[doc = "Counter-Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpctlSpec;
impl crate::RegisterSpec for CmpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpctl::R`](R) reader structure"]
impl crate::Readable for CmpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpctl::W`](W) writer structure"]
impl crate::Writable for CmpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPCTL to value 0"]
impl crate::Resettable for CmpctlSpec {
    const RESET_VALUE: u32 = 0;
}
