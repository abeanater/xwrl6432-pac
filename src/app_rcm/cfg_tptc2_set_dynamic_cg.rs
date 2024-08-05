#[doc = "Register `CFG_TPTC2_SET_DYNAMIC_CG` reader"]
pub type R = crate::R<CfgTptc2SetDynamicCgSpec>;
#[doc = "Register `CFG_TPTC2_SET_DYNAMIC_CG` writer"]
pub type W = crate::W<CfgTptc2SetDynamicCgSpec>;
#[doc = "Field `set` reader - 0:0\\]
Start APPSS tptc2 dynamic clock gating. This is used instead of WFI. 1 - Start the clock gating. In order to start again, write 0 followed by 1. Rise edge is detected internally, to start the clock gating. 0 - Clock is ungated. Fall edge is detected internally to ungate the clock."]
pub type SetR = crate::BitReader;
#[doc = "Field `set` writer - 0:0\\]
Start APPSS tptc2 dynamic clock gating. This is used instead of WFI. 1 - Start the clock gating. In order to start again, write 0 followed by 1. Rise edge is detected internally, to start the clock gating. 0 - Clock is ungated. Fall edge is detected internally to ungate the clock."]
pub type SetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start APPSS tptc2 dynamic clock gating. This is used instead of WFI. 1 - Start the clock gating. In order to start again, write 0 followed by 1. Rise edge is detected internally, to start the clock gating. 0 - Clock is ungated. Fall edge is detected internally to ungate the clock."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start APPSS tptc2 dynamic clock gating. This is used instead of WFI. 1 - Start the clock gating. In order to start again, write 0 followed by 1. Rise edge is detected internally, to start the clock gating. 0 - Clock is ungated. Fall edge is detected internally to ungate the clock."]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<CfgTptc2SetDynamicCgSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "CFG_TPTC2_SET_DYNAMIC_CG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_tptc2_set_dynamic_cg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_tptc2_set_dynamic_cg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTptc2SetDynamicCgSpec;
impl crate::RegisterSpec for CfgTptc2SetDynamicCgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tptc2_set_dynamic_cg::R`](R) reader structure"]
impl crate::Readable for CfgTptc2SetDynamicCgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tptc2_set_dynamic_cg::W`](W) writer structure"]
impl crate::Writable for CfgTptc2SetDynamicCgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TPTC2_SET_DYNAMIC_CG to value 0"]
impl crate::Resettable for CfgTptc2SetDynamicCgSpec {
    const RESET_VALUE: u32 = 0;
}
