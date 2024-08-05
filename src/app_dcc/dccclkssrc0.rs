#[doc = "Register `DCCCLKSSRC0` reader"]
pub type R = crate::R<Dccclkssrc0Spec>;
#[doc = "Register `DCCCLKSSRC0` writer"]
pub type W = crate::W<Dccclkssrc0Spec>;
#[doc = "Field `CLK_SRC0` reader - 3:0\\]
APLL clock 400MHz Input0_clksrc\\[2\\]
PLL_DIG clock 400MHz Input0_clksrc\\[1\\]
OSC_CLK Input0_clksrc\\[0\\]"]
pub type ClkSrc0R = crate::FieldReader;
#[doc = "Field `CLK_SRC0` writer - 3:0\\]
APLL clock 400MHz Input0_clksrc\\[2\\]
PLL_DIG clock 400MHz Input0_clksrc\\[1\\]
OSC_CLK Input0_clksrc\\[0\\]"]
pub type ClkSrc0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU12` reader - 11:4\\]
Reserved"]
pub type Nu12R = crate::FieldReader;
#[doc = "Field `NU12` writer - 11:4\\]
Reserved"]
pub type Nu12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `KEY_B4` reader - 15:12\\]
Key Programing (1010 is the KEY Value)"]
pub type KeyB4R = crate::FieldReader;
#[doc = "Field `KEY_B4` writer - 15:12\\]
Key Programing (1010 is the KEY Value)"]
pub type KeyB4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU13` reader - 31:16\\]
Reserved"]
pub type Nu13R = crate::FieldReader<u16>;
#[doc = "Field `NU13` writer - 31:16\\]
Reserved"]
pub type Nu13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
APLL clock 400MHz Input0_clksrc\\[2\\]
PLL_DIG clock 400MHz Input0_clksrc\\[1\\]
OSC_CLK Input0_clksrc\\[0\\]"]
    #[inline(always)]
    pub fn clk_src0(&self) -> ClkSrc0R {
        ClkSrc0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Reserved"]
    #[inline(always)]
    pub fn nu12(&self) -> Nu12R {
        Nu12R::new(((self.bits >> 4) & 0xff) as u8)
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
    pub fn nu13(&self) -> Nu13R {
        Nu13R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
APLL clock 400MHz Input0_clksrc\\[2\\]
PLL_DIG clock 400MHz Input0_clksrc\\[1\\]
OSC_CLK Input0_clksrc\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src0(&mut self) -> ClkSrc0W<Dccclkssrc0Spec> {
        ClkSrc0W::new(self, 0)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu12(&mut self) -> Nu12W<Dccclkssrc0Spec> {
        Nu12W::new(self, 4)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Key Programing (1010 is the KEY Value)"]
    #[inline(always)]
    #[must_use]
    pub fn key_b4(&mut self) -> KeyB4W<Dccclkssrc0Spec> {
        KeyB4W::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu13(&mut self) -> Nu13W<Dccclkssrc0Spec> {
        Nu13W::new(self, 16)
    }
}
#[doc = "Clock source0 selection control\n\nYou can [`read`](crate::Reg::read) this register and get [`dccclkssrc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccclkssrc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dccclkssrc0Spec;
impl crate::RegisterSpec for Dccclkssrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccclkssrc0::R`](R) reader structure"]
impl crate::Readable for Dccclkssrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`dccclkssrc0::W`](W) writer structure"]
impl crate::Writable for Dccclkssrc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCCLKSSRC0 to value 0"]
impl crate::Resettable for Dccclkssrc0Spec {
    const RESET_VALUE: u32 = 0;
}
