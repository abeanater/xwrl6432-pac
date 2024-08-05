#[doc = "Register `DFT_APPSS_LSTC_CLK_GATE` reader"]
pub type R = crate::R<DftAppssLstcClkGateSpec>;
#[doc = "Register `DFT_APPSS_LSTC_CLK_GATE` writer"]
pub type W = crate::W<DftAppssLstcClkGateSpec>;
#[doc = "Field `DFT_APPSS_LSTC_CLK_GATE` reader - 2:0\\]
Writing 3'b111 will gate the clock to LSTC.Writing 3'b000 will ungate the clock."]
pub type DftAppssLstcClkGateR = crate::FieldReader;
#[doc = "Field `DFT_APPSS_LSTC_CLK_GATE` writer - 2:0\\]
Writing 3'b111 will gate the clock to LSTC.Writing 3'b000 will ungate the clock."]
pub type DftAppssLstcClkGateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Writing 3'b111 will gate the clock to LSTC.Writing 3'b000 will ungate the clock."]
    #[inline(always)]
    pub fn dft_appss_lstc_clk_gate(&self) -> DftAppssLstcClkGateR {
        DftAppssLstcClkGateR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Writing 3'b111 will gate the clock to LSTC.Writing 3'b000 will ungate the clock."]
    #[inline(always)]
    #[must_use]
    pub fn dft_appss_lstc_clk_gate(&mut self) -> DftAppssLstcClkGateW<DftAppssLstcClkGateSpec> {
        DftAppssLstcClkGateW::new(self, 0)
    }
}
#[doc = "DFT_APPSS_LSTC_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_appss_lstc_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_appss_lstc_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftAppssLstcClkGateSpec;
impl crate::RegisterSpec for DftAppssLstcClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_appss_lstc_clk_gate::R`](R) reader structure"]
impl crate::Readable for DftAppssLstcClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`dft_appss_lstc_clk_gate::W`](W) writer structure"]
impl crate::Writable for DftAppssLstcClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFT_APPSS_LSTC_CLK_GATE to value 0"]
impl crate::Resettable for DftAppssLstcClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
