#[doc = "Register `CFG_XBARA_DYNAMIC_CG` reader"]
pub type R = crate::R<CfgXbaraDynamicCgSpec>;
#[doc = "Register `CFG_XBARA_DYNAMIC_CG` writer"]
pub type W = crate::W<CfgXbaraDynamicCgSpec>;
#[doc = "Field `enable` reader - 2:0\\]
Enable APPSS crossbar dynamic clock gating. 1 - Dynamic clock gating feature is enabled. When the feature is enabed, the CM4 should monitor for any possible pending transactions from various masters like DMA/NWA and if no transaction is expected to be iniated by the masters, the CM4 executes WFI. On ssertion of WFI signal, the clock to crossbar is gated. The clock is automatically ungated under two conditions (i) when the WFI signal is deasserted by any interrupted (ii) when any of the APPSS TPCC triggers are asserted. Instead of WFI, cfg_xbara_set_dynamic_cg also can be used to start the clock gating. 0 - Dynamic clock gating feature is disabled. The clock to APPSS crossbar is not gated dynamically. The clock to APPSS crossbar is gated/ungated as per device ice level power states."]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
Enable APPSS crossbar dynamic clock gating. 1 - Dynamic clock gating feature is enabled. When the feature is enabed, the CM4 should monitor for any possible pending transactions from various masters like DMA/NWA and if no transaction is expected to be iniated by the masters, the CM4 executes WFI. On ssertion of WFI signal, the clock to crossbar is gated. The clock is automatically ungated under two conditions (i) when the WFI signal is deasserted by any interrupted (ii) when any of the APPSS TPCC triggers are asserted. Instead of WFI, cfg_xbara_set_dynamic_cg also can be used to start the clock gating. 0 - Dynamic clock gating feature is disabled. The clock to APPSS crossbar is not gated dynamically. The clock to APPSS crossbar is gated/ungated as per device ice level power states."]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable APPSS crossbar dynamic clock gating. 1 - Dynamic clock gating feature is enabled. When the feature is enabed, the CM4 should monitor for any possible pending transactions from various masters like DMA/NWA and if no transaction is expected to be iniated by the masters, the CM4 executes WFI. On ssertion of WFI signal, the clock to crossbar is gated. The clock is automatically ungated under two conditions (i) when the WFI signal is deasserted by any interrupted (ii) when any of the APPSS TPCC triggers are asserted. Instead of WFI, cfg_xbara_set_dynamic_cg also can be used to start the clock gating. 0 - Dynamic clock gating feature is disabled. The clock to APPSS crossbar is not gated dynamically. The clock to APPSS crossbar is gated/ungated as per device ice level power states."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable APPSS crossbar dynamic clock gating. 1 - Dynamic clock gating feature is enabled. When the feature is enabed, the CM4 should monitor for any possible pending transactions from various masters like DMA/NWA and if no transaction is expected to be iniated by the masters, the CM4 executes WFI. On ssertion of WFI signal, the clock to crossbar is gated. The clock is automatically ungated under two conditions (i) when the WFI signal is deasserted by any interrupted (ii) when any of the APPSS TPCC triggers are asserted. Instead of WFI, cfg_xbara_set_dynamic_cg also can be used to start the clock gating. 0 - Dynamic clock gating feature is disabled. The clock to APPSS crossbar is not gated dynamically. The clock to APPSS crossbar is gated/ungated as per device ice level power states."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CfgXbaraDynamicCgSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "CFG_XBARA_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_xbara_dynamic_cg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_xbara_dynamic_cg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgXbaraDynamicCgSpec;
impl crate::RegisterSpec for CfgXbaraDynamicCgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_xbara_dynamic_cg::R`](R) reader structure"]
impl crate::Readable for CfgXbaraDynamicCgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_xbara_dynamic_cg::W`](W) writer structure"]
impl crate::Writable for CfgXbaraDynamicCgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_XBARA_DYNAMIC_CG to value 0"]
impl crate::Resettable for CfgXbaraDynamicCgSpec {
    const RESET_VALUE: u32 = 0;
}
