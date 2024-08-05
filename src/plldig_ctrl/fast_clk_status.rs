#[doc = "Register `FAST_CLK_STATUS` reader"]
pub type R = crate::R<FastClkStatusSpec>;
#[doc = "Register `FAST_CLK_STATUS` writer"]
pub type W = crate::W<FastClkStatusSpec>;
#[doc = "Field `currclk` reader - 1:0\\]
Current Clock selected by GCM Clock Mux 0x1 : PLLDIG 0x2 : APLL"]
pub type CurrclkR = crate::FieldReader;
#[doc = "Field `currclk` writer - 1:0\\]
Current Clock selected by GCM Clock Mux 0x1 : PLLDIG 0x2 : APLL"]
pub type CurrclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Current Clock selected by GCM Clock Mux 0x1 : PLLDIG 0x2 : APLL"]
    #[inline(always)]
    pub fn currclk(&self) -> CurrclkR {
        CurrclkR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Current Clock selected by GCM Clock Mux 0x1 : PLLDIG 0x2 : APLL"]
    #[inline(always)]
    #[must_use]
    pub fn currclk(&mut self) -> CurrclkW<FastClkStatusSpec> {
        CurrclkW::new(self, 0)
    }
}
#[doc = "FAST_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_clk_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_clk_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FastClkStatusSpec;
impl crate::RegisterSpec for FastClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fast_clk_status::R`](R) reader structure"]
impl crate::Readable for FastClkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fast_clk_status::W`](W) writer structure"]
impl crate::Writable for FastClkStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAST_CLK_STATUS to value 0"]
impl crate::Resettable for FastClkStatusSpec {
    const RESET_VALUE: u32 = 0;
}
