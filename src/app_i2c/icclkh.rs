#[doc = "Register `ICCLKH` reader"]
pub type R = crate::R<IcclkhSpec>;
#[doc = "Register `ICCLKH` writer"]
pub type W = crate::W<IcclkhSpec>;
#[doc = "Field `ICCH15_ICCLH0` reader - 15:0\\]
High time I 2 C SCL Clock Division Factor. They are used to divide down the master clock to create the SCL high time transition frequency. This register must be configured while the I2C is still in reset (IRS_=0)."]
pub type Icch15Icclh0R = crate::FieldReader<u16>;
#[doc = "Field `ICCH15_ICCLH0` writer - 15:0\\]
High time I 2 C SCL Clock Division Factor. They are used to divide down the master clock to create the SCL high time transition frequency. This register must be configured while the I2C is still in reset (IRS_=0)."]
pub type Icch15Icclh0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - 31:16\\]
Reserved"]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 31:16\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
High time I 2 C SCL Clock Division Factor. They are used to divide down the master clock to create the SCL high time transition frequency. This register must be configured while the I2C is still in reset (IRS_=0)."]
    #[inline(always)]
    pub fn icch15_icclh0(&self) -> Icch15Icclh0R {
        Icch15Icclh0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
High time I 2 C SCL Clock Division Factor. They are used to divide down the master clock to create the SCL high time transition frequency. This register must be configured while the I2C is still in reset (IRS_=0)."]
    #[inline(always)]
    #[must_use]
    pub fn icch15_icclh0(&mut self) -> Icch15Icclh0W<IcclkhSpec> {
        Icch15Icclh0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcclkhSpec> {
        NuW::new(self, 16)
    }
}
#[doc = "I2C Clock Divider High register\n\nYou can [`read`](crate::Reg::read) this register and get [`icclkh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icclkh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcclkhSpec;
impl crate::RegisterSpec for IcclkhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icclkh::R`](R) reader structure"]
impl crate::Readable for IcclkhSpec {}
#[doc = "`write(|w| ..)` method takes [`icclkh::W`](W) writer structure"]
impl crate::Writable for IcclkhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICCLKH to value 0"]
impl crate::Resettable for IcclkhSpec {
    const RESET_VALUE: u32 = 0;
}
