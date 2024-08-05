#[doc = "Register `CMPB_AQCTLA` reader"]
pub type R = crate::R<CmpbAqctlaSpec>;
#[doc = "Register `CMPB_AQCTLA` writer"]
pub type W = crate::W<CmpbAqctlaSpec>;
#[doc = "Field `CMPB` reader - 15:0\\]
The value in the active CMPB register is continuously compared to the time-base counter (TBCTR). When the values are equal, the counter-compare module generates a \"time-base counter equal to counter compare B\" event. This event is sent to the action-qualifier where it is qualified and converted it into one or more actions. These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers. The actions that can be defined in the AQCTLA and AQCTLB registers include: ΓÇó Do nothing. event is ignored. ΓÇó Clear: Pull the EPWMxA and/or EPWMxB signal low ΓÇó Set: Pull the EPWMxA and/or EPWMxB signal high ΓÇó Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWBMODE\\]
bit. By default this register is shadowed. ΓÇó If CMPCTL\\[SHDWBMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the CMPCTL\\[LOADBMODE\\]
bit field determines which event will load the active register from the shadow register: ΓÇó Before a write, the CMPCTL\\[SHDWBFULL\\]
bit can be read to determine if the shadow register is currently full. ΓÇó If CMPCTL\\[SHDWBMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó In either mode, the active and shadow registers share the same memory map address"]
pub type CmpbR = crate::FieldReader<u16>;
#[doc = "Field `CMPB` writer - 15:0\\]
The value in the active CMPB register is continuously compared to the time-base counter (TBCTR). When the values are equal, the counter-compare module generates a \"time-base counter equal to counter compare B\" event. This event is sent to the action-qualifier where it is qualified and converted it into one or more actions. These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers. The actions that can be defined in the AQCTLA and AQCTLB registers include: ΓÇó Do nothing. event is ignored. ΓÇó Clear: Pull the EPWMxA and/or EPWMxB signal low ΓÇó Set: Pull the EPWMxA and/or EPWMxB signal high ΓÇó Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWBMODE\\]
bit. By default this register is shadowed. ΓÇó If CMPCTL\\[SHDWBMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the CMPCTL\\[LOADBMODE\\]
bit field determines which event will load the active register from the shadow register: ΓÇó Before a write, the CMPCTL\\[SHDWBFULL\\]
bit can be read to determine if the shadow register is currently full. ΓÇó If CMPCTL\\[SHDWBMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó In either mode, the active and shadow registers share the same memory map address"]
pub type CmpbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AQCTLA_ZRO` reader - 17:16\\]
Action when counter equals zero. Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaZroR = crate::FieldReader;
#[doc = "Field `AQCTLA_ZRO` writer - 17:16\\]
Action when counter equals zero. Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaZroW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLA_PRD` reader - 19:18\\]
Action when the counter equals the period. Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaPrdR = crate::FieldReader;
#[doc = "Field `AQCTLA_PRD` writer - 19:18\\]
Action when the counter equals the period. Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaPrdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLA_CAU` reader - 21:20\\]
Action when the counter equals the active CMPA register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaCauR = crate::FieldReader;
#[doc = "Field `AQCTLA_CAU` writer - 21:20\\]
Action when the counter equals the active CMPA register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaCauW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLA_CAD` reader - 23:22\\]
Action when the counter equals the active CMPA register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaCadR = crate::FieldReader;
#[doc = "Field `AQCTLA_CAD` writer - 23:22\\]
Action when the counter equals the active CMPA register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaCadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLA_CBU` reader - 25:24\\]
Action when the counter equals the active CMPB register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaCbuR = crate::FieldReader;
#[doc = "Field `AQCTLA_CBU` writer - 25:24\\]
Action when the counter equals the active CMPB register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaCbuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLA_CBD` reader - 27:26\\]
Action when the time-base counter equals the active CMPB register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaCbdR = crate::FieldReader;
#[doc = "Field `AQCTLA_CBD` writer - 27:26\\]
Action when the time-base counter equals the active CMPB register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlaCbdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The value in the active CMPB register is continuously compared to the time-base counter (TBCTR). When the values are equal, the counter-compare module generates a \"time-base counter equal to counter compare B\" event. This event is sent to the action-qualifier where it is qualified and converted it into one or more actions. These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers. The actions that can be defined in the AQCTLA and AQCTLB registers include: ΓÇó Do nothing. event is ignored. ΓÇó Clear: Pull the EPWMxA and/or EPWMxB signal low ΓÇó Set: Pull the EPWMxA and/or EPWMxB signal high ΓÇó Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWBMODE\\]
bit. By default this register is shadowed. ΓÇó If CMPCTL\\[SHDWBMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the CMPCTL\\[LOADBMODE\\]
bit field determines which event will load the active register from the shadow register: ΓÇó Before a write, the CMPCTL\\[SHDWBFULL\\]
bit can be read to determine if the shadow register is currently full. ΓÇó If CMPCTL\\[SHDWBMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó In either mode, the active and shadow registers share the same memory map address"]
    #[inline(always)]
    pub fn cmpb(&self) -> CmpbR {
        CmpbR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Action when counter equals zero. Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctla_zro(&self) -> AqctlaZroR {
        AqctlaZroR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Action when the counter equals the period. Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctla_prd(&self) -> AqctlaPrdR {
        AqctlaPrdR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Action when the counter equals the active CMPA register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctla_cau(&self) -> AqctlaCauR {
        AqctlaCauR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Action when the counter equals the active CMPA register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctla_cad(&self) -> AqctlaCadR {
        AqctlaCadR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Action when the counter equals the active CMPB register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctla_cbu(&self) -> AqctlaCbuR {
        AqctlaCbuR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Action when the time-base counter equals the active CMPB register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctla_cbd(&self) -> AqctlaCbdR {
        AqctlaCbdR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The value in the active CMPB register is continuously compared to the time-base counter (TBCTR). When the values are equal, the counter-compare module generates a \"time-base counter equal to counter compare B\" event. This event is sent to the action-qualifier where it is qualified and converted it into one or more actions. These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers. The actions that can be defined in the AQCTLA and AQCTLB registers include: ΓÇó Do nothing. event is ignored. ΓÇó Clear: Pull the EPWMxA and/or EPWMxB signal low ΓÇó Set: Pull the EPWMxA and/or EPWMxB signal high ΓÇó Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWBMODE\\]
bit. By default this register is shadowed. ΓÇó If CMPCTL\\[SHDWBMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the CMPCTL\\[LOADBMODE\\]
bit field determines which event will load the active register from the shadow register: ΓÇó Before a write, the CMPCTL\\[SHDWBFULL\\]
bit can be read to determine if the shadow register is currently full. ΓÇó If CMPCTL\\[SHDWBMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó In either mode, the active and shadow registers share the same memory map address"]
    #[inline(always)]
    #[must_use]
    pub fn cmpb(&mut self) -> CmpbW<CmpbAqctlaSpec> {
        CmpbW::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Action when counter equals zero. Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctla_zro(&mut self) -> AqctlaZroW<CmpbAqctlaSpec> {
        AqctlaZroW::new(self, 16)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Action when the counter equals the period. Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctla_prd(&mut self) -> AqctlaPrdW<CmpbAqctlaSpec> {
        AqctlaPrdW::new(self, 18)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Action when the counter equals the active CMPA register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctla_cau(&mut self) -> AqctlaCauW<CmpbAqctlaSpec> {
        AqctlaCauW::new(self, 20)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Action when the counter equals the active CMPA register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctla_cad(&mut self) -> AqctlaCadW<CmpbAqctlaSpec> {
        AqctlaCadW::new(self, 22)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Action when the counter equals the active CMPB register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctla_cbu(&mut self) -> AqctlaCbuW<CmpbAqctlaSpec> {
        AqctlaCbuW::new(self, 24)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Action when the time-base counter equals the active CMPB register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxA output low. 2h Set: force EPWMxA output high. 3h Toggle EPWMxA output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctla_cbd(&mut self) -> AqctlaCbdW<CmpbAqctlaSpec> {
        AqctlaCbdW::new(self, 26)
    }
}
#[doc = "Counter-Compare B Register/ Action-Qualifier Control Register for Output A (EPWMxA)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpb_aqctla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpb_aqctla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpbAqctlaSpec;
impl crate::RegisterSpec for CmpbAqctlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpb_aqctla::R`](R) reader structure"]
impl crate::Readable for CmpbAqctlaSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpb_aqctla::W`](W) writer structure"]
impl crate::Writable for CmpbAqctlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPB_AQCTLA to value 0"]
impl crate::Resettable for CmpbAqctlaSpec {
    const RESET_VALUE: u32 = 0;
}
