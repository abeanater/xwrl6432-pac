#[doc = "Register `I2C_RESERVED1` reader"]
pub type R = crate::R<I2cReserved1Spec>;
#[doc = "Register `I2C_RESERVED1` writer"]
pub type W = crate::W<I2cReserved1Spec>;
#[doc = "Field `NU` reader - 31:0\\]
Reserved."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:0\\]
Reserved."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<I2cReserved1Spec> {
        NuW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_reserved1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_reserved1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cReserved1Spec;
impl crate::RegisterSpec for I2cReserved1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_reserved1::R`](R) reader structure"]
impl crate::Readable for I2cReserved1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_reserved1::W`](W) writer structure"]
impl crate::Writable for I2cReserved1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_RESERVED1 to value 0"]
impl crate::Resettable for I2cReserved1Spec {
    const RESET_VALUE: u32 = 0;
}
