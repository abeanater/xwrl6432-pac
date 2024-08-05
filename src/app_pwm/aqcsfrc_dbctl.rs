#[doc = "Register `AQCSFRC_DBCTL` reader"]
pub type R = crate::R<AqcsfrcDbctlSpec>;
#[doc = "Register `AQCSFRC_DBCTL` writer"]
pub type W = crate::W<AqcsfrcDbctlSpec>;
#[doc = "Field `AQCSFRC_CSFA` reader - 1:0\\]
Continuous Software Force on Output A In immediate mode, a continuous force takes effect on the next TBCLK edge. In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register. 0 Forcing disabled, that is, has no effect 1h Forces a continuous low on output A 2h Forces a continuous high on output A 3h Software forcing is disabled and has no effect"]
pub type AqcsfrcCsfaR = crate::FieldReader;
#[doc = "Field `AQCSFRC_CSFA` writer - 1:0\\]
Continuous Software Force on Output A In immediate mode, a continuous force takes effect on the next TBCLK edge. In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register. 0 Forcing disabled, that is, has no effect 1h Forces a continuous low on output A 2h Forces a continuous high on output A 3h Software forcing is disabled and has no effect"]
pub type AqcsfrcCsfaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AQCSFRC_CSFB` reader - 3:2\\]
Continuous Software Force on Output B In immediate mode, a continuous force takes effect on the next TBCLK edge. In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register. To configure shadow mode, use AQSFRC\\[RLDCSF\\]. 0 Forcing disabled, that is, has no effect 1h Forces a continuous low on output B 2h Forces a continuous high on output B 3h Software forcing is disabled and has no effect"]
pub type AqcsfrcCsfbR = crate::FieldReader;
#[doc = "Field `AQCSFRC_CSFB` writer - 3:2\\]
Continuous Software Force on Output B In immediate mode, a continuous force takes effect on the next TBCLK edge. In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register. To configure shadow mode, use AQSFRC\\[RLDCSF\\]. 0 Forcing disabled, that is, has no effect 1h Forces a continuous low on output B 2h Forces a continuous high on output B 3h Software forcing is disabled and has no effect"]
pub type AqcsfrcCsfbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - 15:4\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - 15:4\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DBCTL_OUT_MODE` reader - 17:16\\]
Dead-band Output Mode Control Bit 1 controls the S1 switch and bit 0 controls the S0 switch shown in Figure 35-28. This allows you to selectively enable or bypass the dead-band generation for the falling-edge and rising-edge delay. 0 Dead-band generation is bypassed for both output signals. In this mode, both the EPWMxA and EPWMxB output signals from the action-qualifier are passed directly to the PWM-chopper submodule. In this mode, the POLSEL and IN_MODE bits have no effect. 1h Disable rising-edge delay. The EPWMxA signal from the action-qualifier is passed straight through to the EPWMxA input of the PWM-chopper submodule. The falling-edge delayed signal is seen on output EPWMxB. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]. 2h The rising-edge delayed signal is seen on output EPWMxA. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]. Disable falling-edge delay. The EPWMxB signal from the action-qualifier is passed straight through to the EPWMxB input of the PWM-chopper submodule. 3h Dead-band is fully enabled for both rising-edge delay on output EPWMxA and falling-edge delay on output EPWMxB. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]."]
pub type DbctlOutModeR = crate::FieldReader;
#[doc = "Field `DBCTL_OUT_MODE` writer - 17:16\\]
Dead-band Output Mode Control Bit 1 controls the S1 switch and bit 0 controls the S0 switch shown in Figure 35-28. This allows you to selectively enable or bypass the dead-band generation for the falling-edge and rising-edge delay. 0 Dead-band generation is bypassed for both output signals. In this mode, both the EPWMxA and EPWMxB output signals from the action-qualifier are passed directly to the PWM-chopper submodule. In this mode, the POLSEL and IN_MODE bits have no effect. 1h Disable rising-edge delay. The EPWMxA signal from the action-qualifier is passed straight through to the EPWMxA input of the PWM-chopper submodule. The falling-edge delayed signal is seen on output EPWMxB. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]. 2h The rising-edge delayed signal is seen on output EPWMxA. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]. Disable falling-edge delay. The EPWMxB signal from the action-qualifier is passed straight through to the EPWMxB input of the PWM-chopper submodule. 3h Dead-band is fully enabled for both rising-edge delay on output EPWMxA and falling-edge delay on output EPWMxB. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]."]
pub type DbctlOutModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBCTL_POLSEL` reader - 19:18\\]
Polarity Select Control Bit 3 controls the S3 switch and bit 2 controls the S2 switch shown in Figure 35-28. This allows you to selectively invert one of the delayed signals before it is sent out of the dead-band submodule. The following descriptions correspond to classical upper/lower switch control as found in one leg of a digital motor control inverter. These assume that DBCTL\\[OUT_MODE\\]
= 1,1 and DBCTL\\[IN_MODE\\]
= 0,0. Other enhanced modes are also possible, but not regarded as typical usage modes. 0 Active high (AH) mode. Neither EPWMxA nor EPWMxB is inverted (default). 1h Active low complementary (ALC) mode. EPWMxA is inverted. 2h Active high complementary (AHC). EPWMxB is inverted. 3h Active low (AL) mode. Both EPWMxA and EPWMxB are inverted"]
pub type DbctlPolselR = crate::FieldReader;
#[doc = "Field `DBCTL_POLSEL` writer - 19:18\\]
Polarity Select Control Bit 3 controls the S3 switch and bit 2 controls the S2 switch shown in Figure 35-28. This allows you to selectively invert one of the delayed signals before it is sent out of the dead-band submodule. The following descriptions correspond to classical upper/lower switch control as found in one leg of a digital motor control inverter. These assume that DBCTL\\[OUT_MODE\\]
= 1,1 and DBCTL\\[IN_MODE\\]
= 0,0. Other enhanced modes are also possible, but not regarded as typical usage modes. 0 Active high (AH) mode. Neither EPWMxA nor EPWMxB is inverted (default). 1h Active low complementary (ALC) mode. EPWMxA is inverted. 2h Active high complementary (AHC). EPWMxB is inverted. 3h Active low (AL) mode. Both EPWMxA and EPWMxB are inverted"]
pub type DbctlPolselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBCTL_IN_MODE` reader - 21:20\\]
Dead Band Input Mode Control Bit 5 controls the S5 switch and bit 4 controls the S4 switch shown in Figure 35-28. This allows you to select the input source to the falling-edge and rising-edge delay. To produce classical dead-band waveforms the default is EPWMxA In is the source for both falling and rising-edge delays. 0 EPWMxA In (from the action-qualifier) is the source for both falling-edge and rising-edge delay. 1h EPWMxB In (from the action-qualifier) is the source for rising-edge delayed signal. EPWMxA In (from the action-qualifier) is the source for falling-edge delayed signal. 2h EPWMxA In (from the action-qualifier) is the source for rising-edge delayed signal. EPWMxB In (from the action-qualifier) is the source for falling-edge delayed signal. 3h EPWMxB In (from the action-qualifier) is the source for both rising-edge delay and falling-edge delayed signal."]
pub type DbctlInModeR = crate::FieldReader;
#[doc = "Field `DBCTL_IN_MODE` writer - 21:20\\]
Dead Band Input Mode Control Bit 5 controls the S5 switch and bit 4 controls the S4 switch shown in Figure 35-28. This allows you to select the input source to the falling-edge and rising-edge delay. To produce classical dead-band waveforms the default is EPWMxA In is the source for both falling and rising-edge delays. 0 EPWMxA In (from the action-qualifier) is the source for both falling-edge and rising-edge delay. 1h EPWMxB In (from the action-qualifier) is the source for rising-edge delayed signal. EPWMxA In (from the action-qualifier) is the source for falling-edge delayed signal. 2h EPWMxA In (from the action-qualifier) is the source for rising-edge delayed signal. EPWMxB In (from the action-qualifier) is the source for falling-edge delayed signal. 3h EPWMxB In (from the action-qualifier) is the source for both rising-edge delay and falling-edge delayed signal."]
pub type DbctlInModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - 30:22\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 30:22\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DBCTL_HALFCYCLE` reader - 31:31\\]
Half Cycle Clocking Enable Bit: 0 Full cycle clocking enabled. The dead-band counters are clocked at the TBCLK rate. 1 Half cycle clocking enabled. The dead-band counters are clocked at TBCLK ∩┐╜ 2."]
pub type DbctlHalfcycleR = crate::BitReader;
#[doc = "Field `DBCTL_HALFCYCLE` writer - 31:31\\]
Half Cycle Clocking Enable Bit: 0 Full cycle clocking enabled. The dead-band counters are clocked at the TBCLK rate. 1 Half cycle clocking enabled. The dead-band counters are clocked at TBCLK ∩┐╜ 2."]
pub type DbctlHalfcycleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Continuous Software Force on Output A In immediate mode, a continuous force takes effect on the next TBCLK edge. In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register. 0 Forcing disabled, that is, has no effect 1h Forces a continuous low on output A 2h Forces a continuous high on output A 3h Software forcing is disabled and has no effect"]
    #[inline(always)]
    pub fn aqcsfrc_csfa(&self) -> AqcsfrcCsfaR {
        AqcsfrcCsfaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Continuous Software Force on Output B In immediate mode, a continuous force takes effect on the next TBCLK edge. In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register. To configure shadow mode, use AQSFRC\\[RLDCSF\\]. 0 Forcing disabled, that is, has no effect 1h Forces a continuous low on output B 2h Forces a continuous high on output B 3h Software forcing is disabled and has no effect"]
    #[inline(always)]
    pub fn aqcsfrc_csfb(&self) -> AqcsfrcCsfbR {
        AqcsfrcCsfbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Dead-band Output Mode Control Bit 1 controls the S1 switch and bit 0 controls the S0 switch shown in Figure 35-28. This allows you to selectively enable or bypass the dead-band generation for the falling-edge and rising-edge delay. 0 Dead-band generation is bypassed for both output signals. In this mode, both the EPWMxA and EPWMxB output signals from the action-qualifier are passed directly to the PWM-chopper submodule. In this mode, the POLSEL and IN_MODE bits have no effect. 1h Disable rising-edge delay. The EPWMxA signal from the action-qualifier is passed straight through to the EPWMxA input of the PWM-chopper submodule. The falling-edge delayed signal is seen on output EPWMxB. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]. 2h The rising-edge delayed signal is seen on output EPWMxA. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]. Disable falling-edge delay. The EPWMxB signal from the action-qualifier is passed straight through to the EPWMxB input of the PWM-chopper submodule. 3h Dead-band is fully enabled for both rising-edge delay on output EPWMxA and falling-edge delay on output EPWMxB. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]."]
    #[inline(always)]
    pub fn dbctl_out_mode(&self) -> DbctlOutModeR {
        DbctlOutModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Polarity Select Control Bit 3 controls the S3 switch and bit 2 controls the S2 switch shown in Figure 35-28. This allows you to selectively invert one of the delayed signals before it is sent out of the dead-band submodule. The following descriptions correspond to classical upper/lower switch control as found in one leg of a digital motor control inverter. These assume that DBCTL\\[OUT_MODE\\]
= 1,1 and DBCTL\\[IN_MODE\\]
= 0,0. Other enhanced modes are also possible, but not regarded as typical usage modes. 0 Active high (AH) mode. Neither EPWMxA nor EPWMxB is inverted (default). 1h Active low complementary (ALC) mode. EPWMxA is inverted. 2h Active high complementary (AHC). EPWMxB is inverted. 3h Active low (AL) mode. Both EPWMxA and EPWMxB are inverted"]
    #[inline(always)]
    pub fn dbctl_polsel(&self) -> DbctlPolselR {
        DbctlPolselR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Dead Band Input Mode Control Bit 5 controls the S5 switch and bit 4 controls the S4 switch shown in Figure 35-28. This allows you to select the input source to the falling-edge and rising-edge delay. To produce classical dead-band waveforms the default is EPWMxA In is the source for both falling and rising-edge delays. 0 EPWMxA In (from the action-qualifier) is the source for both falling-edge and rising-edge delay. 1h EPWMxB In (from the action-qualifier) is the source for rising-edge delayed signal. EPWMxA In (from the action-qualifier) is the source for falling-edge delayed signal. 2h EPWMxA In (from the action-qualifier) is the source for rising-edge delayed signal. EPWMxB In (from the action-qualifier) is the source for falling-edge delayed signal. 3h EPWMxB In (from the action-qualifier) is the source for both rising-edge delay and falling-edge delayed signal."]
    #[inline(always)]
    pub fn dbctl_in_mode(&self) -> DbctlInModeR {
        DbctlInModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:30 - 30:22\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Half Cycle Clocking Enable Bit: 0 Full cycle clocking enabled. The dead-band counters are clocked at the TBCLK rate. 1 Half cycle clocking enabled. The dead-band counters are clocked at TBCLK ∩┐╜ 2."]
    #[inline(always)]
    pub fn dbctl_halfcycle(&self) -> DbctlHalfcycleR {
        DbctlHalfcycleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Continuous Software Force on Output A In immediate mode, a continuous force takes effect on the next TBCLK edge. In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register. 0 Forcing disabled, that is, has no effect 1h Forces a continuous low on output A 2h Forces a continuous high on output A 3h Software forcing is disabled and has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn aqcsfrc_csfa(&mut self) -> AqcsfrcCsfaW<AqcsfrcDbctlSpec> {
        AqcsfrcCsfaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Continuous Software Force on Output B In immediate mode, a continuous force takes effect on the next TBCLK edge. In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register. To configure shadow mode, use AQSFRC\\[RLDCSF\\]. 0 Forcing disabled, that is, has no effect 1h Forces a continuous low on output B 2h Forces a continuous high on output B 3h Software forcing is disabled and has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn aqcsfrc_csfb(&mut self) -> AqcsfrcCsfbW<AqcsfrcDbctlSpec> {
        AqcsfrcCsfbW::new(self, 2)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AqcsfrcDbctlSpec> {
        Reserved2W::new(self, 4)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Dead-band Output Mode Control Bit 1 controls the S1 switch and bit 0 controls the S0 switch shown in Figure 35-28. This allows you to selectively enable or bypass the dead-band generation for the falling-edge and rising-edge delay. 0 Dead-band generation is bypassed for both output signals. In this mode, both the EPWMxA and EPWMxB output signals from the action-qualifier are passed directly to the PWM-chopper submodule. In this mode, the POLSEL and IN_MODE bits have no effect. 1h Disable rising-edge delay. The EPWMxA signal from the action-qualifier is passed straight through to the EPWMxA input of the PWM-chopper submodule. The falling-edge delayed signal is seen on output EPWMxB. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]. 2h The rising-edge delayed signal is seen on output EPWMxA. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]. Disable falling-edge delay. The EPWMxB signal from the action-qualifier is passed straight through to the EPWMxB input of the PWM-chopper submodule. 3h Dead-band is fully enabled for both rising-edge delay on output EPWMxA and falling-edge delay on output EPWMxB. The input signal for the delay is determined by DBCTL\\[IN_MODE\\]."]
    #[inline(always)]
    #[must_use]
    pub fn dbctl_out_mode(&mut self) -> DbctlOutModeW<AqcsfrcDbctlSpec> {
        DbctlOutModeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Polarity Select Control Bit 3 controls the S3 switch and bit 2 controls the S2 switch shown in Figure 35-28. This allows you to selectively invert one of the delayed signals before it is sent out of the dead-band submodule. The following descriptions correspond to classical upper/lower switch control as found in one leg of a digital motor control inverter. These assume that DBCTL\\[OUT_MODE\\]
= 1,1 and DBCTL\\[IN_MODE\\]
= 0,0. Other enhanced modes are also possible, but not regarded as typical usage modes. 0 Active high (AH) mode. Neither EPWMxA nor EPWMxB is inverted (default). 1h Active low complementary (ALC) mode. EPWMxA is inverted. 2h Active high complementary (AHC). EPWMxB is inverted. 3h Active low (AL) mode. Both EPWMxA and EPWMxB are inverted"]
    #[inline(always)]
    #[must_use]
    pub fn dbctl_polsel(&mut self) -> DbctlPolselW<AqcsfrcDbctlSpec> {
        DbctlPolselW::new(self, 18)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Dead Band Input Mode Control Bit 5 controls the S5 switch and bit 4 controls the S4 switch shown in Figure 35-28. This allows you to select the input source to the falling-edge and rising-edge delay. To produce classical dead-band waveforms the default is EPWMxA In is the source for both falling and rising-edge delays. 0 EPWMxA In (from the action-qualifier) is the source for both falling-edge and rising-edge delay. 1h EPWMxB In (from the action-qualifier) is the source for rising-edge delayed signal. EPWMxA In (from the action-qualifier) is the source for falling-edge delayed signal. 2h EPWMxA In (from the action-qualifier) is the source for rising-edge delayed signal. EPWMxB In (from the action-qualifier) is the source for falling-edge delayed signal. 3h EPWMxB In (from the action-qualifier) is the source for both rising-edge delay and falling-edge delayed signal."]
    #[inline(always)]
    #[must_use]
    pub fn dbctl_in_mode(&mut self) -> DbctlInModeW<AqcsfrcDbctlSpec> {
        DbctlInModeW::new(self, 20)
    }
    #[doc = "Bits 22:30 - 30:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AqcsfrcDbctlSpec> {
        Reserved1W::new(self, 22)
    }
    #[doc = "Bit 31 - 31:31\\]
Half Cycle Clocking Enable Bit: 0 Full cycle clocking enabled. The dead-band counters are clocked at the TBCLK rate. 1 Half cycle clocking enabled. The dead-band counters are clocked at TBCLK ∩┐╜ 2."]
    #[inline(always)]
    #[must_use]
    pub fn dbctl_halfcycle(&mut self) -> DbctlHalfcycleW<AqcsfrcDbctlSpec> {
        DbctlHalfcycleW::new(self, 31)
    }
}
#[doc = "Dead-Band Generator Control Register/ Action-Qualifier Continuous S/W Force Register Set\n\nYou can [`read`](crate::Reg::read) this register and get [`aqcsfrc_dbctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aqcsfrc_dbctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AqcsfrcDbctlSpec;
impl crate::RegisterSpec for AqcsfrcDbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aqcsfrc_dbctl::R`](R) reader structure"]
impl crate::Readable for AqcsfrcDbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`aqcsfrc_dbctl::W`](W) writer structure"]
impl crate::Writable for AqcsfrcDbctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AQCSFRC_DBCTL to value 0"]
impl crate::Resettable for AqcsfrcDbctlSpec {
    const RESET_VALUE: u32 = 0;
}
