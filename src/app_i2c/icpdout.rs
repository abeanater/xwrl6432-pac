#[doc = "Register `ICPDOUT` reader"]
pub type R = crate::R<IcpdoutSpec>;
#[doc = "Register `ICPDOUT` writer"]
pub type W = crate::W<IcpdoutSpec>;
#[doc = "Field `PDOUT0` reader - 0:0\\]
Controls the level driven on the SCL pin when configured as GPIO output. Reads: Reads return register values not GPIO pin levels. Writes: 0 = SCL pin driven low 1 = SCL pin driven high Note: If SCL is connected to an open-drain buffer at the chiplevel the I2C cannot drive SCL to high."]
pub type Pdout0R = crate::BitReader;
#[doc = "Field `PDOUT0` writer - 0:0\\]
Controls the level driven on the SCL pin when configured as GPIO output. Reads: Reads return register values not GPIO pin levels. Writes: 0 = SCL pin driven low 1 = SCL pin driven high Note: If SCL is connected to an open-drain buffer at the chiplevel the I2C cannot drive SCL to high."]
pub type Pdout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDOUT1` reader - 1:1\\]
Controls the level driven on the SDA pin when configured as GPIO output. Reads: Reads return register values not GPIO pin levels. Writes: 0 = SDA pin driven low 1 = SDA pin driven high. Note: If SDA is connected to an open-drain buffer at the chiplevel the I2C cannot drive SDA to high."]
pub type Pdout1R = crate::BitReader;
#[doc = "Field `PDOUT1` writer - 1:1\\]
Controls the level driven on the SDA pin when configured as GPIO output. Reads: Reads return register values not GPIO pin levels. Writes: 0 = SDA pin driven low 1 = SDA pin driven high. Note: If SDA is connected to an open-drain buffer at the chiplevel the I2C cannot drive SDA to high."]
pub type Pdout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:2\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:2\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls the level driven on the SCL pin when configured as GPIO output. Reads: Reads return register values not GPIO pin levels. Writes: 0 = SCL pin driven low 1 = SCL pin driven high Note: If SCL is connected to an open-drain buffer at the chiplevel the I2C cannot drive SCL to high."]
    #[inline(always)]
    pub fn pdout0(&self) -> Pdout0R {
        Pdout0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls the level driven on the SDA pin when configured as GPIO output. Reads: Reads return register values not GPIO pin levels. Writes: 0 = SDA pin driven low 1 = SDA pin driven high. Note: If SDA is connected to an open-drain buffer at the chiplevel the I2C cannot drive SDA to high."]
    #[inline(always)]
    pub fn pdout1(&self) -> Pdout1R {
        Pdout1R::new(((self.bits >> 1) & 1) != 0)
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
Controls the level driven on the SCL pin when configured as GPIO output. Reads: Reads return register values not GPIO pin levels. Writes: 0 = SCL pin driven low 1 = SCL pin driven high Note: If SCL is connected to an open-drain buffer at the chiplevel the I2C cannot drive SCL to high."]
    #[inline(always)]
    #[must_use]
    pub fn pdout0(&mut self) -> Pdout0W<IcpdoutSpec> {
        Pdout0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls the level driven on the SDA pin when configured as GPIO output. Reads: Reads return register values not GPIO pin levels. Writes: 0 = SDA pin driven low 1 = SDA pin driven high. Note: If SDA is connected to an open-drain buffer at the chiplevel the I2C cannot drive SDA to high."]
    #[inline(always)]
    #[must_use]
    pub fn pdout1(&mut self) -> Pdout1W<IcpdoutSpec> {
        Pdout1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcpdoutSpec> {
        NuW::new(self, 2)
    }
}
#[doc = "I2C Pin Data Out register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcpdoutSpec;
impl crate::RegisterSpec for IcpdoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpdout::R`](R) reader structure"]
impl crate::Readable for IcpdoutSpec {}
#[doc = "`write(|w| ..)` method takes [`icpdout::W`](W) writer structure"]
impl crate::Writable for IcpdoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPDOUT to value 0"]
impl crate::Resettable for IcpdoutSpec {
    const RESET_VALUE: u32 = 0;
}
