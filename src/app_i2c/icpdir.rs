#[doc = "Register `ICPDIR` reader"]
pub type R = crate::R<IcpdirSpec>;
#[doc = "Register `ICPDIR` writer"]
pub type W = crate::W<IcpdirSpec>;
#[doc = "Field `PDIR0` reader - 0:0\\]
Controls the direction of the I2C SCL pin when configured as GPIO. 0 = SCL pin functions as input 1 = SCL pin functions as output"]
pub type Pdir0R = crate::BitReader;
#[doc = "Field `PDIR0` writer - 0:0\\]
Controls the direction of the I2C SCL pin when configured as GPIO. 0 = SCL pin functions as input 1 = SCL pin functions as output"]
pub type Pdir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIR1` reader - 1:1\\]
Controls the direction of the I2C SDA pin when configured as GPIO. 0 = SDA pin functions as input 1 = SDA pin functions as output"]
pub type Pdir1R = crate::BitReader;
#[doc = "Field `PDIR1` writer - 1:1\\]
Controls the direction of the I2C SDA pin when configured as GPIO. 0 = SDA pin functions as input 1 = SDA pin functions as output"]
pub type Pdir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:2\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:2\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls the direction of the I2C SCL pin when configured as GPIO. 0 = SCL pin functions as input 1 = SCL pin functions as output"]
    #[inline(always)]
    pub fn pdir0(&self) -> Pdir0R {
        Pdir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls the direction of the I2C SDA pin when configured as GPIO. 0 = SDA pin functions as input 1 = SDA pin functions as output"]
    #[inline(always)]
    pub fn pdir1(&self) -> Pdir1R {
        Pdir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls the direction of the I2C SCL pin when configured as GPIO. 0 = SCL pin functions as input 1 = SCL pin functions as output"]
    #[inline(always)]
    #[must_use]
    pub fn pdir0(&mut self) -> Pdir0W<IcpdirSpec> {
        Pdir0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls the direction of the I2C SDA pin when configured as GPIO. 0 = SDA pin functions as input 1 = SDA pin functions as output"]
    #[inline(always)]
    #[must_use]
    pub fn pdir1(&mut self) -> Pdir1W<IcpdirSpec> {
        Pdir1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcpdirSpec> {
        NuW::new(self, 2)
    }
}
#[doc = "I2C Pin Direction register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcpdirSpec;
impl crate::RegisterSpec for IcpdirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpdir::R`](R) reader structure"]
impl crate::Readable for IcpdirSpec {}
#[doc = "`write(|w| ..)` method takes [`icpdir::W`](W) writer structure"]
impl crate::Writable for IcpdirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPDIR to value 0"]
impl crate::Resettable for IcpdirSpec {
    const RESET_VALUE: u32 = 0;
}
