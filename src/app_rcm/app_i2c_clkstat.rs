#[doc = "Register `APP_I2C_CLKSTAT` reader"]
pub type R = crate::R<AppI2cClkstatSpec>;
#[doc = "Register `APP_I2C_CLKSTAT` writer"]
pub type W = crate::W<AppI2cClkstatSpec>;
#[doc = "Field `currdivr` reader - 3:0\\]
Gives the current divr setting used by the clock divider."]
pub type CurrdivrR = crate::FieldReader;
#[doc = "Field `currdivr` writer - 3:0\\]
Gives the current divr setting used by the clock divider."]
pub type CurrdivrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Gives the current divr setting used by the clock divider."]
    #[inline(always)]
    pub fn currdivr(&self) -> CurrdivrR {
        CurrdivrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Gives the current divr setting used by the clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn currdivr(&mut self) -> CurrdivrW<AppI2cClkstatSpec> {
        CurrdivrW::new(self, 0)
    }
}
#[doc = "APP_I2C_CLKSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_i2c_clkstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_i2c_clkstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppI2cClkstatSpec;
impl crate::RegisterSpec for AppI2cClkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_i2c_clkstat::R`](R) reader structure"]
impl crate::Readable for AppI2cClkstatSpec {}
#[doc = "`write(|w| ..)` method takes [`app_i2c_clkstat::W`](W) writer structure"]
impl crate::Writable for AppI2cClkstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_I2C_CLKSTAT to value 0"]
impl crate::Resettable for AppI2cClkstatSpec {
    const RESET_VALUE: u32 = 0;
}
