#[doc = "Register `RST_CAUSE` reader"]
pub type R = crate::R<RstCauseSpec>;
#[doc = "Register `RST_CAUSE` writer"]
pub type W = crate::W<RstCauseSpec>;
#[doc = "Field `common` reader - 7:0\\]
Reset cause register for APP CPU 0x00 - All cleared 0x01 - Power On Reset (PoR) 0x02 - Subsystem Reset (Combination of Warm Reset initiated from PRCM using LPRADAR:TOP_PRCM:RST_APP_PD_SOFT_RESET and PoR reset) 0x04 - STC RESET 0x08 - Reserved 0x10 - CPU Only Reset triggered by writing to LPRADAR:APP_RCM:RST_FSM_TRIG&lt;RST_FSM_TRIG_CPU> (self triggered CPU reset during MEMSWAP/ECLIPSE mode to wait for WFI to assert the reset to CPU) 0x20 - Core Reset initiated from PRCM using LPRADAR:TOP_PRCM:RST_SOFT_APP_CORE_SYSRESET_REQ (reset CPU unconditionally - by debugger) or LPRADAR:TOP_PRCM:APP_CORE_SYSRESET_PARAM_WAKEUP_OUT_STATE 0x40 - Reserved"]
pub type CommonR = crate::FieldReader;
#[doc = "Field `common` writer - 7:0\\]
Reset cause register for APP CPU 0x00 - All cleared 0x01 - Power On Reset (PoR) 0x02 - Subsystem Reset (Combination of Warm Reset initiated from PRCM using LPRADAR:TOP_PRCM:RST_APP_PD_SOFT_RESET and PoR reset) 0x04 - STC RESET 0x08 - Reserved 0x10 - CPU Only Reset triggered by writing to LPRADAR:APP_RCM:RST_FSM_TRIG&lt;RST_FSM_TRIG_CPU> (self triggered CPU reset during MEMSWAP/ECLIPSE mode to wait for WFI to assert the reset to CPU) 0x20 - Core Reset initiated from PRCM using LPRADAR:TOP_PRCM:RST_SOFT_APP_CORE_SYSRESET_REQ (reset CPU unconditionally - by debugger) or LPRADAR:TOP_PRCM:APP_CORE_SYSRESET_PARAM_WAKEUP_OUT_STATE 0x40 - Reserved"]
pub type CommonW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reset cause register for APP CPU 0x00 - All cleared 0x01 - Power On Reset (PoR) 0x02 - Subsystem Reset (Combination of Warm Reset initiated from PRCM using LPRADAR:TOP_PRCM:RST_APP_PD_SOFT_RESET and PoR reset) 0x04 - STC RESET 0x08 - Reserved 0x10 - CPU Only Reset triggered by writing to LPRADAR:APP_RCM:RST_FSM_TRIG&lt;RST_FSM_TRIG_CPU> (self triggered CPU reset during MEMSWAP/ECLIPSE mode to wait for WFI to assert the reset to CPU) 0x20 - Core Reset initiated from PRCM using LPRADAR:TOP_PRCM:RST_SOFT_APP_CORE_SYSRESET_REQ (reset CPU unconditionally - by debugger) or LPRADAR:TOP_PRCM:APP_CORE_SYSRESET_PARAM_WAKEUP_OUT_STATE 0x40 - Reserved"]
    #[inline(always)]
    pub fn common(&self) -> CommonR {
        CommonR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reset cause register for APP CPU 0x00 - All cleared 0x01 - Power On Reset (PoR) 0x02 - Subsystem Reset (Combination of Warm Reset initiated from PRCM using LPRADAR:TOP_PRCM:RST_APP_PD_SOFT_RESET and PoR reset) 0x04 - STC RESET 0x08 - Reserved 0x10 - CPU Only Reset triggered by writing to LPRADAR:APP_RCM:RST_FSM_TRIG&lt;RST_FSM_TRIG_CPU> (self triggered CPU reset during MEMSWAP/ECLIPSE mode to wait for WFI to assert the reset to CPU) 0x20 - Core Reset initiated from PRCM using LPRADAR:TOP_PRCM:RST_SOFT_APP_CORE_SYSRESET_REQ (reset CPU unconditionally - by debugger) or LPRADAR:TOP_PRCM:APP_CORE_SYSRESET_PARAM_WAKEUP_OUT_STATE 0x40 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn common(&mut self) -> CommonW<RstCauseSpec> {
        CommonW::new(self, 0)
    }
}
#[doc = "RST_CAUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstCauseSpec;
impl crate::RegisterSpec for RstCauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_cause::R`](R) reader structure"]
impl crate::Readable for RstCauseSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_cause::W`](W) writer structure"]
impl crate::Writable for RstCauseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_CAUSE to value 0"]
impl crate::Resettable for RstCauseSpec {
    const RESET_VALUE: u32 = 0;
}
