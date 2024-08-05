#[doc = "Register `APPSS_DYNAMIC_CLK_GATE_STATUS` reader"]
pub type R = crate::R<AppssDynamicClkGateStatusSpec>;
#[doc = "Register `APPSS_DYNAMIC_CLK_GATE_STATUS` writer"]
pub type W = crate::W<AppssDynamicClkGateStatusSpec>;
#[doc = "Field `xbara` reader - 0:0\\]
Dynamic Clock gate Status of XBARA 1 - Clock is Enabled 0 - Clock is Gated."]
pub type XbaraR = crate::BitReader;
#[doc = "Field `xbara` writer - 0:0\\]
Dynamic Clock gate Status of XBARA 1 - Clock is Enabled 0 - Clock is Gated."]
pub type XbaraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc1` reader - 1:1\\]
Dynamic Clock gate Status of TPTC1 1 - Clock is Enabled 0 - Clock is Gated."]
pub type Tptc1R = crate::BitReader;
#[doc = "Field `tptc1` writer - 1:1\\]
Dynamic Clock gate Status of TPTC1 1 - Clock is Enabled 0 - Clock is Gated."]
pub type Tptc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc2` reader - 2:2\\]
Dynamic Clock gate Status of TPTC2 1 - Clock is Enabled 0 - Clock is Gated."]
pub type Tptc2R = crate::BitReader;
#[doc = "Field `tptc2` writer - 2:2\\]
Dynamic Clock gate Status of TPTC2 1 - Clock is Enabled 0 - Clock is Gated."]
pub type Tptc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Dynamic Clock gate Status of XBARA 1 - Clock is Enabled 0 - Clock is Gated."]
    #[inline(always)]
    pub fn xbara(&self) -> XbaraR {
        XbaraR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Dynamic Clock gate Status of TPTC1 1 - Clock is Enabled 0 - Clock is Gated."]
    #[inline(always)]
    pub fn tptc1(&self) -> Tptc1R {
        Tptc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Dynamic Clock gate Status of TPTC2 1 - Clock is Enabled 0 - Clock is Gated."]
    #[inline(always)]
    pub fn tptc2(&self) -> Tptc2R {
        Tptc2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Dynamic Clock gate Status of XBARA 1 - Clock is Enabled 0 - Clock is Gated."]
    #[inline(always)]
    #[must_use]
    pub fn xbara(&mut self) -> XbaraW<AppssDynamicClkGateStatusSpec> {
        XbaraW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Dynamic Clock gate Status of TPTC1 1 - Clock is Enabled 0 - Clock is Gated."]
    #[inline(always)]
    #[must_use]
    pub fn tptc1(&mut self) -> Tptc1W<AppssDynamicClkGateStatusSpec> {
        Tptc1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Dynamic Clock gate Status of TPTC2 1 - Clock is Enabled 0 - Clock is Gated."]
    #[inline(always)]
    #[must_use]
    pub fn tptc2(&mut self) -> Tptc2W<AppssDynamicClkGateStatusSpec> {
        Tptc2W::new(self, 2)
    }
}
#[doc = "APPSS_DYNAMIC_CLK_GATE_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_dynamic_clk_gate_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_dynamic_clk_gate_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssDynamicClkGateStatusSpec;
impl crate::RegisterSpec for AppssDynamicClkGateStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_dynamic_clk_gate_status::R`](R) reader structure"]
impl crate::Readable for AppssDynamicClkGateStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_dynamic_clk_gate_status::W`](W) writer structure"]
impl crate::Writable for AppssDynamicClkGateStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_DYNAMIC_CLK_GATE_STATUS to value 0"]
impl crate::Resettable for AppssDynamicClkGateStatusSpec {
    const RESET_VALUE: u32 = 0;
}
