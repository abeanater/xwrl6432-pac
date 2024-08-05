#[doc = "Register `REG8` reader"]
pub type R = crate::R<Reg8Spec>;
#[doc = "Register `REG8` writer"]
pub type W = crate::W<Reg8Spec>;
#[doc = "Field `GPADC_CLK_DIV` reader - 7:0\\]
TI reserved"]
pub type GpadcClkDivR = crate::FieldReader;
#[doc = "Field `GPADC_CLK_DIV` writer - 7:0\\]
TI reserved"]
pub type GpadcClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPADC_CLK_ENABLE` reader - 8:8\\]
TI reserved"]
pub type GpadcClkEnableR = crate::BitReader;
#[doc = "Field `GPADC_CLK_ENABLE` writer - 8:8\\]
TI reserved"]
pub type GpadcClkEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI reserved"]
    #[inline(always)]
    pub fn gpadc_clk_div(&self) -> GpadcClkDivR {
        GpadcClkDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
TI reserved"]
    #[inline(always)]
    pub fn gpadc_clk_enable(&self) -> GpadcClkEnableR {
        GpadcClkEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_div(&mut self) -> GpadcClkDivW<Reg8Spec> {
        GpadcClkDivW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_enable(&mut self) -> GpadcClkEnableW<Reg8Spec> {
        GpadcClkEnableW::new(self, 8)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Reg8Spec> {
        NuW::new(self, 9)
    }
}
#[doc = "REG8\n\nYou can [`read`](crate::Reg::read) this register and get [`reg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg8Spec;
impl crate::RegisterSpec for Reg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg8::R`](R) reader structure"]
impl crate::Readable for Reg8Spec {}
#[doc = "`write(|w| ..)` method takes [`reg8::W`](W) writer structure"]
impl crate::Writable for Reg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG8 to value 0"]
impl crate::Resettable for Reg8Spec {
    const RESET_VALUE: u32 = 0;
}
