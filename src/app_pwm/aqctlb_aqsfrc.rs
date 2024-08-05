#[doc = "Register `AQCTLB_AQSFRC` reader"]
pub type R = crate::R<AqctlbAqsfrcSpec>;
#[doc = "Register `AQCTLB_AQSFRC` writer"]
pub type W = crate::W<AqctlbAqsfrcSpec>;
#[doc = "Field `AQCTLB_ZRO` reader - 1:0\\]
Action when counter equals zero. Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low."]
pub type AqctlbZroR = crate::FieldReader;
#[doc = "Field `AQCTLB_ZRO` writer - 1:0\\]
Action when counter equals zero. Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low."]
pub type AqctlbZroW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLB_PRD` reader - 3:2\\]
Action when the counter equals the period. Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbPrdR = crate::FieldReader;
#[doc = "Field `AQCTLB_PRD` writer - 3:2\\]
Action when the counter equals the period. Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbPrdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLB_CAU` reader - 5:4\\]
Action when the counter equals the active CMPA register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbCauR = crate::FieldReader;
#[doc = "Field `AQCTLB_CAU` writer - 5:4\\]
Action when the counter equals the active CMPA register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbCauW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLB_CAD` reader - 7:6\\]
Action when the counter equals the active CMPA register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbCadR = crate::FieldReader;
#[doc = "Field `AQCTLB_CAD` writer - 7:6\\]
Action when the counter equals the active CMPA register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbCadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLB_CBU` reader - 9:8\\]
Action when the counter equals the active CMPB register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbCbuR = crate::FieldReader;
#[doc = "Field `AQCTLB_CBU` writer - 9:8\\]
Action when the counter equals the active CMPB register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbCbuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCTLB_CBD` reader - 11:10\\]
Action when the counter equals the active CMPB register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbCbdR = crate::FieldReader;
#[doc = "Field `AQCTLB_CBD` writer - 11:10\\]
Action when the counter equals the active CMPB register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
pub type AqctlbCbdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - 15:12\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 15:12\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AQSFRC_ACTSFA` reader - 17:16\\]
Action When One-Time Software Force A Is Invoked 0 Does nothing (action disabled) 1h Clear (low) 2h Set (high) 3h Toggle (Low ΓåÆ High, High ΓåÆ Low) Note: This action is not qualified by counter direction (CNT_dir)"]
pub type AqsfrcActsfaR = crate::FieldReader;
#[doc = "Field `AQSFRC_ACTSFA` writer - 17:16\\]
Action When One-Time Software Force A Is Invoked 0 Does nothing (action disabled) 1h Clear (low) 2h Set (high) 3h Toggle (Low ΓåÆ High, High ΓåÆ Low) Note: This action is not qualified by counter direction (CNT_dir)"]
pub type AqsfrcActsfaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQSFRC_OTSFA` reader - 18:18\\]
One-Time Software Forced Event on Output A 0 Writing a 0 has no effect. Always reads back a 0. This bit is auto cleared once a write to this register is complete (that is, a forced event is initiated). 1 Initiates a single software forced event"]
pub type AqsfrcOtsfaR = crate::BitReader;
#[doc = "Field `AQSFRC_OTSFA` writer - 18:18\\]
One-Time Software Forced Event on Output A 0 Writing a 0 has no effect. Always reads back a 0. This bit is auto cleared once a write to this register is complete (that is, a forced event is initiated). 1 Initiates a single software forced event"]
pub type AqsfrcOtsfaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AQSFRC_ACTSFB` reader - 20:19\\]
Action when One-Time Software Force B Is invoked 0 Does nothing (action disabled) 1h Clear (low) 2h Set (high) 3h Toggle (Low -> High, High -> Low) Note: This action is not qualified by counter direction (CNT_dir)"]
pub type AqsfrcActsfbR = crate::FieldReader;
#[doc = "Field `AQSFRC_ACTSFB` writer - 20:19\\]
Action when One-Time Software Force B Is invoked 0 Does nothing (action disabled) 1h Clear (low) 2h Set (high) 3h Toggle (Low -> High, High -> Low) Note: This action is not qualified by counter direction (CNT_dir)"]
pub type AqsfrcActsfbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQSFRC_OTSFB` reader - 21:21\\]
One-Time Software Forced Event on Output B 0 Writing a 0 has no effect. Always reads back a 0 This bit is auto cleared once a write to this register is complete (that is, a forced event is initiated.) This is a one-shot forced event. It can be overridden by another subsequent event on output B. 1 Initiates a single s/w forced event"]
pub type AqsfrcOtsfbR = crate::BitReader;
#[doc = "Field `AQSFRC_OTSFB` writer - 21:21\\]
One-Time Software Forced Event on Output B 0 Writing a 0 has no effect. Always reads back a 0 This bit is auto cleared once a write to this register is complete (that is, a forced event is initiated.) This is a one-shot forced event. It can be overridden by another subsequent event on output B. 1 Initiates a single s/w forced event"]
pub type AqsfrcOtsfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AQSFRC_RLDCSF` reader - 23:22\\]
AQCSFRC Active Register Reload From Shadow Options 0 Load on event counter equals zero 1h Load on event counter equals period 2h Load on event counter equals zero or counter equals period 3h Load immediately (the active register is directly accessed by the CPU and is not loaded from the shadow register)."]
pub type AqsfrcRldcsfR = crate::FieldReader;
#[doc = "Field `AQSFRC_RLDCSF` writer - 23:22\\]
AQCSFRC Active Register Reload From Shadow Options 0 Load on event counter equals zero 1h Load on event counter equals period 2h Load on event counter equals zero or counter equals period 3h Load immediately (the active register is directly accessed by the CPU and is not loaded from the shadow register)."]
pub type AqsfrcRldcsfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - 31:24\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 31:24\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Action when counter equals zero. Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low."]
    #[inline(always)]
    pub fn aqctlb_zro(&self) -> AqctlbZroR {
        AqctlbZroR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Action when the counter equals the period. Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctlb_prd(&self) -> AqctlbPrdR {
        AqctlbPrdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Action when the counter equals the active CMPA register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctlb_cau(&self) -> AqctlbCauR {
        AqctlbCauR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Action when the counter equals the active CMPA register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctlb_cad(&self) -> AqctlbCadR {
        AqctlbCadR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Action when the counter equals the active CMPB register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctlb_cbu(&self) -> AqctlbCbuR {
        AqctlbCbuR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Action when the counter equals the active CMPB register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    pub fn aqctlb_cbd(&self) -> AqctlbCbdR {
        AqctlbCbdR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Action When One-Time Software Force A Is Invoked 0 Does nothing (action disabled) 1h Clear (low) 2h Set (high) 3h Toggle (Low ΓåÆ High, High ΓåÆ Low) Note: This action is not qualified by counter direction (CNT_dir)"]
    #[inline(always)]
    pub fn aqsfrc_actsfa(&self) -> AqsfrcActsfaR {
        AqsfrcActsfaR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
One-Time Software Forced Event on Output A 0 Writing a 0 has no effect. Always reads back a 0. This bit is auto cleared once a write to this register is complete (that is, a forced event is initiated). 1 Initiates a single software forced event"]
    #[inline(always)]
    pub fn aqsfrc_otsfa(&self) -> AqsfrcOtsfaR {
        AqsfrcOtsfaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Action when One-Time Software Force B Is invoked 0 Does nothing (action disabled) 1h Clear (low) 2h Set (high) 3h Toggle (Low -> High, High -> Low) Note: This action is not qualified by counter direction (CNT_dir)"]
    #[inline(always)]
    pub fn aqsfrc_actsfb(&self) -> AqsfrcActsfbR {
        AqsfrcActsfbR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
One-Time Software Forced Event on Output B 0 Writing a 0 has no effect. Always reads back a 0 This bit is auto cleared once a write to this register is complete (that is, a forced event is initiated.) This is a one-shot forced event. It can be overridden by another subsequent event on output B. 1 Initiates a single s/w forced event"]
    #[inline(always)]
    pub fn aqsfrc_otsfb(&self) -> AqsfrcOtsfbR {
        AqsfrcOtsfbR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
AQCSFRC Active Register Reload From Shadow Options 0 Load on event counter equals zero 1h Load on event counter equals period 2h Load on event counter equals zero or counter equals period 3h Load immediately (the active register is directly accessed by the CPU and is not loaded from the shadow register)."]
    #[inline(always)]
    pub fn aqsfrc_rldcsf(&self) -> AqsfrcRldcsfR {
        AqsfrcRldcsfR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Action when counter equals zero. Note: By definition, in count up-down mode when the counter equals 0 the direction is defined as 1 or counting up. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low."]
    #[inline(always)]
    #[must_use]
    pub fn aqctlb_zro(&mut self) -> AqctlbZroW<AqctlbAqsfrcSpec> {
        AqctlbZroW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Action when the counter equals the period. Note: By definition, in count up-down mode when the counter equals period the direction is defined as 0 or counting down. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctlb_prd(&mut self) -> AqctlbPrdW<AqctlbAqsfrcSpec> {
        AqctlbPrdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Action when the counter equals the active CMPA register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctlb_cau(&mut self) -> AqctlbCauW<AqctlbAqsfrcSpec> {
        AqctlbCauW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Action when the counter equals the active CMPA register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctlb_cad(&mut self) -> AqctlbCadW<AqctlbAqsfrcSpec> {
        AqctlbCadW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Action when the counter equals the active CMPB register and the counter is incrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctlb_cbu(&mut self) -> AqctlbCbuW<AqctlbAqsfrcSpec> {
        AqctlbCbuW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Action when the counter equals the active CMPB register and the counter is decrementing. 0 Do nothing (action disabled) 1h Clear: force EPWMxB output low. 2h Set: force EPWMxB output high. 3h Toggle EPWMxB output: low output signal will be forced high, and a high signal will be forced low"]
    #[inline(always)]
    #[must_use]
    pub fn aqctlb_cbd(&mut self) -> AqctlbCbdW<AqctlbAqsfrcSpec> {
        AqctlbCbdW::new(self, 10)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AqctlbAqsfrcSpec> {
        Reserved2W::new(self, 12)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Action When One-Time Software Force A Is Invoked 0 Does nothing (action disabled) 1h Clear (low) 2h Set (high) 3h Toggle (Low ΓåÆ High, High ΓåÆ Low) Note: This action is not qualified by counter direction (CNT_dir)"]
    #[inline(always)]
    #[must_use]
    pub fn aqsfrc_actsfa(&mut self) -> AqsfrcActsfaW<AqctlbAqsfrcSpec> {
        AqsfrcActsfaW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
One-Time Software Forced Event on Output A 0 Writing a 0 has no effect. Always reads back a 0. This bit is auto cleared once a write to this register is complete (that is, a forced event is initiated). 1 Initiates a single software forced event"]
    #[inline(always)]
    #[must_use]
    pub fn aqsfrc_otsfa(&mut self) -> AqsfrcOtsfaW<AqctlbAqsfrcSpec> {
        AqsfrcOtsfaW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Action when One-Time Software Force B Is invoked 0 Does nothing (action disabled) 1h Clear (low) 2h Set (high) 3h Toggle (Low -> High, High -> Low) Note: This action is not qualified by counter direction (CNT_dir)"]
    #[inline(always)]
    #[must_use]
    pub fn aqsfrc_actsfb(&mut self) -> AqsfrcActsfbW<AqctlbAqsfrcSpec> {
        AqsfrcActsfbW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
One-Time Software Forced Event on Output B 0 Writing a 0 has no effect. Always reads back a 0 This bit is auto cleared once a write to this register is complete (that is, a forced event is initiated.) This is a one-shot forced event. It can be overridden by another subsequent event on output B. 1 Initiates a single s/w forced event"]
    #[inline(always)]
    #[must_use]
    pub fn aqsfrc_otsfb(&mut self) -> AqsfrcOtsfbW<AqctlbAqsfrcSpec> {
        AqsfrcOtsfbW::new(self, 21)
    }
    #[doc = "Bits 22:23 - 23:22\\]
AQCSFRC Active Register Reload From Shadow Options 0 Load on event counter equals zero 1h Load on event counter equals period 2h Load on event counter equals zero or counter equals period 3h Load immediately (the active register is directly accessed by the CPU and is not loaded from the shadow register)."]
    #[inline(always)]
    #[must_use]
    pub fn aqsfrc_rldcsf(&mut self) -> AqsfrcRldcsfW<AqctlbAqsfrcSpec> {
        AqsfrcRldcsfW::new(self, 22)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AqctlbAqsfrcSpec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Action-Qualifier Control Register for Output B (EPWMxB)/ Action-Qualifier Software Force Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aqctlb_aqsfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aqctlb_aqsfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AqctlbAqsfrcSpec;
impl crate::RegisterSpec for AqctlbAqsfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aqctlb_aqsfrc::R`](R) reader structure"]
impl crate::Readable for AqctlbAqsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`aqctlb_aqsfrc::W`](W) writer structure"]
impl crate::Writable for AqctlbAqsfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AQCTLB_AQSFRC to value 0"]
impl crate::Resettable for AqctlbAqsfrcSpec {
    const RESET_VALUE: u32 = 0;
}
