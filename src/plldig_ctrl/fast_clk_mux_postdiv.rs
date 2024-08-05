#[doc = "Register `FAST_CLK_MUX_POSTDIV` reader"]
pub type R = crate::R<FastClkMuxPostdivSpec>;
#[doc = "Register `FAST_CLK_MUX_POSTDIV` writer"]
pub type W = crate::W<FastClkMuxPostdivSpec>;
#[doc = "Field `currdivr` reader - 3:0\\]
Status shows the current divider value choosen for FAST_CLK."]
pub type CurrdivrR = crate::FieldReader;
#[doc = "Field `currdivr` writer - 3:0\\]
Status shows the current divider value choosen for FAST_CLK."]
pub type CurrdivrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `divr` reader - 15:4\\]
Divider value for FAST selected clock. Data should be loaded as multibit. For example: if divider value of 8 (1000) needs to be selected then '100010001000' should be configured to the register."]
pub type DivrR = crate::FieldReader<u16>;
#[doc = "Field `divr` writer - 15:4\\]
Divider value for FAST selected clock. Data should be loaded as multibit. For example: if divider value of 8 (1000) needs to be selected then '100010001000' should be configured to the register."]
pub type DivrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Status shows the current divider value choosen for FAST_CLK."]
    #[inline(always)]
    pub fn currdivr(&self) -> CurrdivrR {
        CurrdivrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Divider value for FAST selected clock. Data should be loaded as multibit. For example: if divider value of 8 (1000) needs to be selected then '100010001000' should be configured to the register."]
    #[inline(always)]
    pub fn divr(&self) -> DivrR {
        DivrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Status shows the current divider value choosen for FAST_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn currdivr(&mut self) -> CurrdivrW<FastClkMuxPostdivSpec> {
        CurrdivrW::new(self, 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Divider value for FAST selected clock. Data should be loaded as multibit. For example: if divider value of 8 (1000) needs to be selected then '100010001000' should be configured to the register."]
    #[inline(always)]
    #[must_use]
    pub fn divr(&mut self) -> DivrW<FastClkMuxPostdivSpec> {
        DivrW::new(self, 4)
    }
}
#[doc = "FAST_CLK_MUX_POSTDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_clk_mux_postdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_clk_mux_postdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FastClkMuxPostdivSpec;
impl crate::RegisterSpec for FastClkMuxPostdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fast_clk_mux_postdiv::R`](R) reader structure"]
impl crate::Readable for FastClkMuxPostdivSpec {}
#[doc = "`write(|w| ..)` method takes [`fast_clk_mux_postdiv::W`](W) writer structure"]
impl crate::Writable for FastClkMuxPostdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAST_CLK_MUX_POSTDIV to value 0"]
impl crate::Resettable for FastClkMuxPostdivSpec {
    const RESET_VALUE: u32 = 0;
}
