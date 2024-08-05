#[doc = "Register `DCCCLKSSRC1` reader"]
pub type R = crate::R<Dccclkssrc1Spec>;
#[doc = "Register `DCCCLKSSRC1` writer"]
pub type W = crate::W<Dccclkssrc1Spec>;
#[doc = "Field `CLK_SRC1` reader - 3:0\\]
RCOSC Input1_clksrc\\[10\\]
CANFD_GCM Input1_clksrc\\[9\\]
APPSS_GCM Input1_clksrc\\[8\\]
OSC_CLK Input1_clksrc\\[7\\]
LIN_CLK Input1_clksrc\\[6\\]
MDLL_CLK Input1_clksrc\\[5\\]
SYNTH_CLK Input1_clksrc\\[4\\]
RAMPGEN/DFE CLK Input1_clksrc\\[3\\]
GPADC CLK Input1_clksrc\\[2\\]
FECSS_GCM Input1_clksrc\\[1\\]
fast_clk(muxed apll and pll_dig_clk )(root mux) Input1_clksrc\\[0\\]"]
pub type ClkSrc1R = crate::FieldReader;
#[doc = "Field `CLK_SRC1` writer - 3:0\\]
RCOSC Input1_clksrc\\[10\\]
CANFD_GCM Input1_clksrc\\[9\\]
APPSS_GCM Input1_clksrc\\[8\\]
OSC_CLK Input1_clksrc\\[7\\]
LIN_CLK Input1_clksrc\\[6\\]
MDLL_CLK Input1_clksrc\\[5\\]
SYNTH_CLK Input1_clksrc\\[4\\]
RAMPGEN/DFE CLK Input1_clksrc\\[3\\]
GPADC CLK Input1_clksrc\\[2\\]
FECSS_GCM Input1_clksrc\\[1\\]
fast_clk(muxed apll and pll_dig_clk )(root mux) Input1_clksrc\\[0\\]"]
pub type ClkSrc1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU10` reader - 11:4\\]
Reserved"]
pub type Nu10R = crate::FieldReader;
#[doc = "Field `NU10` writer - 11:4\\]
Reserved"]
pub type Nu10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `KEY_B4` reader - 15:12\\]
Key Programing (1010 is the KEY Value)"]
pub type KeyB4R = crate::FieldReader;
#[doc = "Field `KEY_B4` writer - 15:12\\]
Key Programing (1010 is the KEY Value)"]
pub type KeyB4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU11` reader - 31:16\\]
Reserved"]
pub type Nu11R = crate::FieldReader<u16>;
#[doc = "Field `NU11` writer - 31:16\\]
Reserved"]
pub type Nu11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
RCOSC Input1_clksrc\\[10\\]
CANFD_GCM Input1_clksrc\\[9\\]
APPSS_GCM Input1_clksrc\\[8\\]
OSC_CLK Input1_clksrc\\[7\\]
LIN_CLK Input1_clksrc\\[6\\]
MDLL_CLK Input1_clksrc\\[5\\]
SYNTH_CLK Input1_clksrc\\[4\\]
RAMPGEN/DFE CLK Input1_clksrc\\[3\\]
GPADC CLK Input1_clksrc\\[2\\]
FECSS_GCM Input1_clksrc\\[1\\]
fast_clk(muxed apll and pll_dig_clk )(root mux) Input1_clksrc\\[0\\]"]
    #[inline(always)]
    pub fn clk_src1(&self) -> ClkSrc1R {
        ClkSrc1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Reserved"]
    #[inline(always)]
    pub fn nu10(&self) -> Nu10R {
        Nu10R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Key Programing (1010 is the KEY Value)"]
    #[inline(always)]
    pub fn key_b4(&self) -> KeyB4R {
        KeyB4R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu11(&self) -> Nu11R {
        Nu11R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
RCOSC Input1_clksrc\\[10\\]
CANFD_GCM Input1_clksrc\\[9\\]
APPSS_GCM Input1_clksrc\\[8\\]
OSC_CLK Input1_clksrc\\[7\\]
LIN_CLK Input1_clksrc\\[6\\]
MDLL_CLK Input1_clksrc\\[5\\]
SYNTH_CLK Input1_clksrc\\[4\\]
RAMPGEN/DFE CLK Input1_clksrc\\[3\\]
GPADC CLK Input1_clksrc\\[2\\]
FECSS_GCM Input1_clksrc\\[1\\]
fast_clk(muxed apll and pll_dig_clk )(root mux) Input1_clksrc\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src1(&mut self) -> ClkSrc1W<Dccclkssrc1Spec> {
        ClkSrc1W::new(self, 0)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu10(&mut self) -> Nu10W<Dccclkssrc1Spec> {
        Nu10W::new(self, 4)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Key Programing (1010 is the KEY Value)"]
    #[inline(always)]
    #[must_use]
    pub fn key_b4(&mut self) -> KeyB4W<Dccclkssrc1Spec> {
        KeyB4W::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu11(&mut self) -> Nu11W<Dccclkssrc1Spec> {
        Nu11W::new(self, 16)
    }
}
#[doc = "Clock source1 selection control\n\nYou can [`read`](crate::Reg::read) this register and get [`dccclkssrc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccclkssrc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dccclkssrc1Spec;
impl crate::RegisterSpec for Dccclkssrc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccclkssrc1::R`](R) reader structure"]
impl crate::Readable for Dccclkssrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`dccclkssrc1::W`](W) writer structure"]
impl crate::Writable for Dccclkssrc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCCLKSSRC1 to value 0"]
impl crate::Resettable for Dccclkssrc1Spec {
    const RESET_VALUE: u32 = 0;
}
