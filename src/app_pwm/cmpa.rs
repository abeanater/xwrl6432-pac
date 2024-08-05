#[doc = "Register `CMPA` reader"]
pub type R = crate::R<CmpaSpec>;
#[doc = "Register `CMPA` writer"]
pub type W = crate::W<CmpaSpec>;
#[doc = "Field `CMPA` reader - 31:16\\]
Counter-Compare A Register The value in the active CMPA register is continuously compared to the time-base counter (TBCTR). When the values are equal, the counter-compare module generates a \"time-base counter equal to counter compare A\" event. This event is sent to the action-qualifier where it is qualified and converted it into one or more actions. These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers. The actions that can be defined in the AQCTLA and AQCTLB registers include: ΓÇó Do nothing; the event is ignored. ΓÇó Clear: Pull the EPWMxA and/or EPWMxB signal low ΓÇó Set: Pull the EPWMxA and/or EPWMxB signal high ΓÇó Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWAMODE\\]
bit. By default this register is shadowed. ΓÇó If CMPCTL\\[SHDWAMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the CMPCTL\\[LOADAMODE\\]
bit field determines which event will load the active register from the shadow register. ΓÇó Before a write, the CMPCTL\\[SHDWAFULL\\]
bit can be read to determine if the shadow register is currently full. ΓÇó If CMPCTL\\[SHDWAMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó In either mode, the active and shadow registers share the same memory map address."]
pub type CmpaR = crate::FieldReader<u16>;
#[doc = "Field `CMPA` writer - 31:16\\]
Counter-Compare A Register The value in the active CMPA register is continuously compared to the time-base counter (TBCTR). When the values are equal, the counter-compare module generates a \"time-base counter equal to counter compare A\" event. This event is sent to the action-qualifier where it is qualified and converted it into one or more actions. These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers. The actions that can be defined in the AQCTLA and AQCTLB registers include: ΓÇó Do nothing; the event is ignored. ΓÇó Clear: Pull the EPWMxA and/or EPWMxB signal low ΓÇó Set: Pull the EPWMxA and/or EPWMxB signal high ΓÇó Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWAMODE\\]
bit. By default this register is shadowed. ΓÇó If CMPCTL\\[SHDWAMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the CMPCTL\\[LOADAMODE\\]
bit field determines which event will load the active register from the shadow register. ΓÇó Before a write, the CMPCTL\\[SHDWAFULL\\]
bit can be read to determine if the shadow register is currently full. ΓÇó If CMPCTL\\[SHDWAMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó In either mode, the active and shadow registers share the same memory map address."]
pub type CmpaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Counter-Compare A Register The value in the active CMPA register is continuously compared to the time-base counter (TBCTR). When the values are equal, the counter-compare module generates a \"time-base counter equal to counter compare A\" event. This event is sent to the action-qualifier where it is qualified and converted it into one or more actions. These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers. The actions that can be defined in the AQCTLA and AQCTLB registers include: ΓÇó Do nothing; the event is ignored. ΓÇó Clear: Pull the EPWMxA and/or EPWMxB signal low ΓÇó Set: Pull the EPWMxA and/or EPWMxB signal high ΓÇó Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWAMODE\\]
bit. By default this register is shadowed. ΓÇó If CMPCTL\\[SHDWAMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the CMPCTL\\[LOADAMODE\\]
bit field determines which event will load the active register from the shadow register. ΓÇó Before a write, the CMPCTL\\[SHDWAFULL\\]
bit can be read to determine if the shadow register is currently full. ΓÇó If CMPCTL\\[SHDWAMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó In either mode, the active and shadow registers share the same memory map address."]
    #[inline(always)]
    pub fn cmpa(&self) -> CmpaR {
        CmpaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Counter-Compare A Register The value in the active CMPA register is continuously compared to the time-base counter (TBCTR). When the values are equal, the counter-compare module generates a \"time-base counter equal to counter compare A\" event. This event is sent to the action-qualifier where it is qualified and converted it into one or more actions. These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers. The actions that can be defined in the AQCTLA and AQCTLB registers include: ΓÇó Do nothing; the event is ignored. ΓÇó Clear: Pull the EPWMxA and/or EPWMxB signal low ΓÇó Set: Pull the EPWMxA and/or EPWMxB signal high ΓÇó Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWAMODE\\]
bit. By default this register is shadowed. ΓÇó If CMPCTL\\[SHDWAMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the CMPCTL\\[LOADAMODE\\]
bit field determines which event will load the active register from the shadow register. ΓÇó Before a write, the CMPCTL\\[SHDWAFULL\\]
bit can be read to determine if the shadow register is currently full. ΓÇó If CMPCTL\\[SHDWAMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó In either mode, the active and shadow registers share the same memory map address."]
    #[inline(always)]
    #[must_use]
    pub fn cmpa(&mut self) -> CmpaW<CmpaSpec> {
        CmpaW::new(self, 16)
    }
}
#[doc = "Counter-Compare A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpaSpec;
impl crate::RegisterSpec for CmpaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpa::R`](R) reader structure"]
impl crate::Readable for CmpaSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpa::W`](W) writer structure"]
impl crate::Writable for CmpaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPA to value 0"]
impl crate::Resettable for CmpaSpec {
    const RESET_VALUE: u32 = 0;
}
