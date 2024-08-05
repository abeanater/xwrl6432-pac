#[doc = "Register `TBCTR_TBPRD` reader"]
pub type R = crate::R<TbctrTbprdSpec>;
#[doc = "Register `TBCTR_TBPRD` writer"]
pub type W = crate::W<TbctrTbprdSpec>;
#[doc = "Field `TBCTR` reader - 15:0\\]
Time-Base Counter Register Reading these bits gives the current time-base counter value. Writing to these bits sets the current time-base counter value. The update happens as soon as the write occurs; the write is NOT synchronized to the time-base clock (TBCLK) and the register is not shadowed."]
pub type TbctrR = crate::FieldReader<u16>;
#[doc = "Field `TBCTR` writer - 15:0\\]
Time-Base Counter Register Reading these bits gives the current time-base counter value. Writing to these bits sets the current time-base counter value. The update happens as soon as the write occurs; the write is NOT synchronized to the time-base clock (TBCLK) and the register is not shadowed."]
pub type TbctrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TBPRD` reader - 31:16\\]
Time-Base Period Register These bits determine the period of the time-base counter. This sets the PWM frequency. Shadowing of this register is enabled and disabled by the TBCTL\\[PRDLD\\]
bit. By default this register is shadowed. ΓÇó If TBCTL\\[PRDLD\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the active register will be loaded from the shadow register when the timebase counter equals zero. ΓÇó If TBCTL\\[PRDLD\\]
= 1, then the shadow is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó The active and shadow registers share the same memory map address."]
pub type TbprdR = crate::FieldReader<u16>;
#[doc = "Field `TBPRD` writer - 31:16\\]
Time-Base Period Register These bits determine the period of the time-base counter. This sets the PWM frequency. Shadowing of this register is enabled and disabled by the TBCTL\\[PRDLD\\]
bit. By default this register is shadowed. ΓÇó If TBCTL\\[PRDLD\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the active register will be loaded from the shadow register when the timebase counter equals zero. ΓÇó If TBCTL\\[PRDLD\\]
= 1, then the shadow is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó The active and shadow registers share the same memory map address."]
pub type TbprdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Time-Base Counter Register Reading these bits gives the current time-base counter value. Writing to these bits sets the current time-base counter value. The update happens as soon as the write occurs; the write is NOT synchronized to the time-base clock (TBCLK) and the register is not shadowed."]
    #[inline(always)]
    pub fn tbctr(&self) -> TbctrR {
        TbctrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Time-Base Period Register These bits determine the period of the time-base counter. This sets the PWM frequency. Shadowing of this register is enabled and disabled by the TBCTL\\[PRDLD\\]
bit. By default this register is shadowed. ΓÇó If TBCTL\\[PRDLD\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the active register will be loaded from the shadow register when the timebase counter equals zero. ΓÇó If TBCTL\\[PRDLD\\]
= 1, then the shadow is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó The active and shadow registers share the same memory map address."]
    #[inline(always)]
    pub fn tbprd(&self) -> TbprdR {
        TbprdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Time-Base Counter Register Reading these bits gives the current time-base counter value. Writing to these bits sets the current time-base counter value. The update happens as soon as the write occurs; the write is NOT synchronized to the time-base clock (TBCLK) and the register is not shadowed."]
    #[inline(always)]
    #[must_use]
    pub fn tbctr(&mut self) -> TbctrW<TbctrTbprdSpec> {
        TbctrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Time-Base Period Register These bits determine the period of the time-base counter. This sets the PWM frequency. Shadowing of this register is enabled and disabled by the TBCTL\\[PRDLD\\]
bit. By default this register is shadowed. ΓÇó If TBCTL\\[PRDLD\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register. In this case, the active register will be loaded from the shadow register when the timebase counter equals zero. ΓÇó If TBCTL\\[PRDLD\\]
= 1, then the shadow is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware. ΓÇó The active and shadow registers share the same memory map address."]
    #[inline(always)]
    #[must_use]
    pub fn tbprd(&mut self) -> TbprdW<TbctrTbprdSpec> {
        TbprdW::new(self, 16)
    }
}
#[doc = "Time-Base Counter Register/ Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctr_tbprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctr_tbprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctrTbprdSpec;
impl crate::RegisterSpec for TbctrTbprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctr_tbprd::R`](R) reader structure"]
impl crate::Readable for TbctrTbprdSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctr_tbprd::W`](W) writer structure"]
impl crate::Writable for TbctrTbprdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBCTR_TBPRD to value 0"]
impl crate::Resettable for TbctrTbprdSpec {
    const RESET_VALUE: u32 = 0;
}
