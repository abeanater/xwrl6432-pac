#[doc = "Register `APP_RTI_CLKSTAT` reader"]
pub type R = crate::R<AppRtiClkstatSpec>;
#[doc = "Register `APP_RTI_CLKSTAT` writer"]
pub type W = crate::W<AppRtiClkstatSpec>;
#[doc = "Field `currdivr` reader - 3:0\\]
Gives the current divr setting used by the clock divider."]
pub type CurrdivrR = crate::FieldReader;
#[doc = "Field `currdivr` writer - 3:0\\]
Gives the current divr setting used by the clock divider."]
pub type CurrdivrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `currclk` reader - 11:4\\]
Current Clock selected by GCM Clock Mux 0x1 : XTALCLK 0x2 : XTALCLKX2 0x4 : MDLL 0x8 : APLL/DPLL 0x10 : RCCLK"]
pub type CurrclkR = crate::FieldReader;
#[doc = "Field `currclk` writer - 11:4\\]
Current Clock selected by GCM Clock Mux 0x1 : XTALCLK 0x2 : XTALCLKX2 0x4 : MDLL 0x8 : APLL/DPLL 0x10 : RCCLK"]
pub type CurrclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Gives the current divr setting used by the clock divider."]
    #[inline(always)]
    pub fn currdivr(&self) -> CurrdivrR {
        CurrdivrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Current Clock selected by GCM Clock Mux 0x1 : XTALCLK 0x2 : XTALCLKX2 0x4 : MDLL 0x8 : APLL/DPLL 0x10 : RCCLK"]
    #[inline(always)]
    pub fn currclk(&self) -> CurrclkR {
        CurrclkR::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Gives the current divr setting used by the clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn currdivr(&mut self) -> CurrdivrW<AppRtiClkstatSpec> {
        CurrdivrW::new(self, 0)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Current Clock selected by GCM Clock Mux 0x1 : XTALCLK 0x2 : XTALCLKX2 0x4 : MDLL 0x8 : APLL/DPLL 0x10 : RCCLK"]
    #[inline(always)]
    #[must_use]
    pub fn currclk(&mut self) -> CurrclkW<AppRtiClkstatSpec> {
        CurrclkW::new(self, 4)
    }
}
#[doc = "APP_RTI_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_rti_clkstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_rti_clkstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppRtiClkstatSpec;
impl crate::RegisterSpec for AppRtiClkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_rti_clkstat::R`](R) reader structure"]
impl crate::Readable for AppRtiClkstatSpec {}
#[doc = "`write(|w| ..)` method takes [`app_rti_clkstat::W`](W) writer structure"]
impl crate::Writable for AppRtiClkstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_RTI_CLKSTAT to value 0"]
impl crate::Resettable for AppRtiClkstatSpec {
    const RESET_VALUE: u32 = 0;
}
