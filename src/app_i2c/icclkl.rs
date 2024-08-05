#[doc = "Register `ICCLKL` reader"]
pub type R = crate::R<IcclklSpec>;
#[doc = "Register `ICCLKL` writer"]
pub type W = crate::W<IcclklSpec>;
#[doc = "Field `ICCL15_ICCL0` reader - 15:0\\]
Low time I 2 C SCL Clock Division Factor. They are used to divide down the master clock to create the SCL low time transition frequency. This register must be configured while the I2C is still in reset (IRS_=0)."]
pub type Iccl15Iccl0R = crate::FieldReader<u16>;
#[doc = "Field `ICCL15_ICCL0` writer - 15:0\\]
Low time I 2 C SCL Clock Division Factor. They are used to divide down the master clock to create the SCL low time transition frequency. This register must be configured while the I2C is still in reset (IRS_=0)."]
pub type Iccl15Iccl0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - 31:16\\]
Reserved"]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 31:16\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Low time I 2 C SCL Clock Division Factor. They are used to divide down the master clock to create the SCL low time transition frequency. This register must be configured while the I2C is still in reset (IRS_=0)."]
    #[inline(always)]
    pub fn iccl15_iccl0(&self) -> Iccl15Iccl0R {
        Iccl15Iccl0R::new((self.bits & 0xffff) as u16)
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
Low time I 2 C SCL Clock Division Factor. They are used to divide down the master clock to create the SCL low time transition frequency. This register must be configured while the I2C is still in reset (IRS_=0)."]
    #[inline(always)]
    #[must_use]
    pub fn iccl15_iccl0(&mut self) -> Iccl15Iccl0W<IcclklSpec> {
        Iccl15Iccl0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcclklSpec> {
        NuW::new(self, 16)
    }
}
#[doc = "I2C Clock Divider Low register\n\nYou can [`read`](crate::Reg::read) this register and get [`icclkl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icclkl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcclklSpec;
impl crate::RegisterSpec for IcclklSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icclkl::R`](R) reader structure"]
impl crate::Readable for IcclklSpec {}
#[doc = "`write(|w| ..)` method takes [`icclkl::W`](W) writer structure"]
impl crate::Writable for IcclklSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICCLKL to value 0"]
impl crate::Resettable for IcclklSpec {
    const RESET_VALUE: u32 = 0;
}
