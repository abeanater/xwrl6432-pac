#[doc = "Register `APP_I2C_CLKCTL` reader"]
pub type R = crate::R<AppI2cClkctlSpec>;
#[doc = "Register `APP_I2C_CLKCTL` writer"]
pub type W = crate::W<AppI2cClkctlSpec>;
#[doc = "Field `gate` reader - 3:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type GateR = crate::FieldReader;
#[doc = "Field `gate` writer - 3:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type GateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `divr` reader - 27:16\\]
Divide value 0x0 : div1 0x1 : div2 0x2 : div3 . . 0xF = div16 Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register."]
pub type DivrR = crate::FieldReader<u16>;
#[doc = "Field `divr` writer - 27:16\\]
Divide value 0x0 : div1 0x1 : div2 0x2 : div3 . . 0xF = div16 Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register."]
pub type DivrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn gate(&self) -> GateR {
        GateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Divide value 0x0 : div1 0x1 : div2 0x2 : div3 . . 0xF = div16 Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register."]
    #[inline(always)]
    pub fn divr(&self) -> DivrR {
        DivrR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn gate(&mut self) -> GateW<AppI2cClkctlSpec> {
        GateW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Divide value 0x0 : div1 0x1 : div2 0x2 : div3 . . 0xF = div16 Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register."]
    #[inline(always)]
    #[must_use]
    pub fn divr(&mut self) -> DivrW<AppI2cClkctlSpec> {
        DivrW::new(self, 16)
    }
}
#[doc = "APP_I2C_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_i2c_clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_i2c_clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppI2cClkctlSpec;
impl crate::RegisterSpec for AppI2cClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_i2c_clkctl::R`](R) reader structure"]
impl crate::Readable for AppI2cClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`app_i2c_clkctl::W`](W) writer structure"]
impl crate::Writable for AppI2cClkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_I2C_CLKCTL to value 0"]
impl crate::Resettable for AppI2cClkctlSpec {
    const RESET_VALUE: u32 = 0;
}
