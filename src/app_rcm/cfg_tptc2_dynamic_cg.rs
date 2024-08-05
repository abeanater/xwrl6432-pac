#[doc = "Register `CFG_TPTC2_DYNAMIC_CG` reader"]
pub type R = crate::R<CfgTptc2DynamicCgSpec>;
#[doc = "Register `CFG_TPTC2_DYNAMIC_CG` writer"]
pub type W = crate::W<CfgTptc2DynamicCgSpec>;
#[doc = "Field `enable` reader - 2:0\\]
Enable APPSS TPTC2 crossbar dynamic clock gating. 1 - Dynamic clock gating feature is enabled. Same behaviour as cfg_xbara_dynamic_cg_en - for both entry to clock gating and exit from clock gating. WFI or cfg_tptc2_set_dynamic_cg 0 - Dynamic clock gating feature is disabled. The clock to APPSS TPTC2 crossbar is not gated dynamically. The clock to APPSS TPTC2 crossbar is gated/ungated as per device ice level power states"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
Enable APPSS TPTC2 crossbar dynamic clock gating. 1 - Dynamic clock gating feature is enabled. Same behaviour as cfg_xbara_dynamic_cg_en - for both entry to clock gating and exit from clock gating. WFI or cfg_tptc2_set_dynamic_cg 0 - Dynamic clock gating feature is disabled. The clock to APPSS TPTC2 crossbar is not gated dynamically. The clock to APPSS TPTC2 crossbar is gated/ungated as per device ice level power states"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable APPSS TPTC2 crossbar dynamic clock gating. 1 - Dynamic clock gating feature is enabled. Same behaviour as cfg_xbara_dynamic_cg_en - for both entry to clock gating and exit from clock gating. WFI or cfg_tptc2_set_dynamic_cg 0 - Dynamic clock gating feature is disabled. The clock to APPSS TPTC2 crossbar is not gated dynamically. The clock to APPSS TPTC2 crossbar is gated/ungated as per device ice level power states"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable APPSS TPTC2 crossbar dynamic clock gating. 1 - Dynamic clock gating feature is enabled. Same behaviour as cfg_xbara_dynamic_cg_en - for both entry to clock gating and exit from clock gating. WFI or cfg_tptc2_set_dynamic_cg 0 - Dynamic clock gating feature is disabled. The clock to APPSS TPTC2 crossbar is not gated dynamically. The clock to APPSS TPTC2 crossbar is gated/ungated as per device ice level power states"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CfgTptc2DynamicCgSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "CFG_TPTC2_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_tptc2_dynamic_cg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_tptc2_dynamic_cg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTptc2DynamicCgSpec;
impl crate::RegisterSpec for CfgTptc2DynamicCgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tptc2_dynamic_cg::R`](R) reader structure"]
impl crate::Readable for CfgTptc2DynamicCgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tptc2_dynamic_cg::W`](W) writer structure"]
impl crate::Writable for CfgTptc2DynamicCgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TPTC2_DYNAMIC_CG to value 0"]
impl crate::Resettable for CfgTptc2DynamicCgSpec {
    const RESET_VALUE: u32 = 0;
}
