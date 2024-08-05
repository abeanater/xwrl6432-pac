#[doc = "Register `ICPSC` reader"]
pub type R = crate::R<IcpscSpec>;
#[doc = "Register `ICPSC` writer"]
pub type W = crate::W<IcpscSpec>;
#[doc = "Field `IPSC7_IPSC0` reader - 7:0\\]
8-bit prescaler to divide the system clock down to 4/8/12Mhz clock and used by the I2C module. This register must be initialized while the I2C is still in reset (IRS_=0). The value takes effect on the rising edge of IRS_."]
pub type Ipsc7Ipsc0R = crate::FieldReader;
#[doc = "Field `IPSC7_IPSC0` writer - 7:0\\]
8-bit prescaler to divide the system clock down to 4/8/12Mhz clock and used by the I2C module. This register must be initialized while the I2C is still in reset (IRS_=0). The value takes effect on the rising edge of IRS_."]
pub type Ipsc7Ipsc0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU` reader - 31:8\\]
Reserved."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:8\\]
Reserved."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit prescaler to divide the system clock down to 4/8/12Mhz clock and used by the I2C module. This register must be initialized while the I2C is still in reset (IRS_=0). The value takes effect on the rising edge of IRS_."]
    #[inline(always)]
    pub fn ipsc7_ipsc0(&self) -> Ipsc7Ipsc0R {
        Ipsc7Ipsc0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit prescaler to divide the system clock down to 4/8/12Mhz clock and used by the I2C module. This register must be initialized while the I2C is still in reset (IRS_=0). The value takes effect on the rising edge of IRS_."]
    #[inline(always)]
    #[must_use]
    pub fn ipsc7_ipsc0(&mut self) -> Ipsc7Ipsc0W<IcpscSpec> {
        Ipsc7Ipsc0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcpscSpec> {
        NuW::new(self, 8)
    }
}
#[doc = "I2C Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcpscSpec;
impl crate::RegisterSpec for IcpscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpsc::R`](R) reader structure"]
impl crate::Readable for IcpscSpec {}
#[doc = "`write(|w| ..)` method takes [`icpsc::W`](W) writer structure"]
impl crate::Writable for IcpscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPSC to value 0"]
impl crate::Resettable for IcpscSpec {
    const RESET_VALUE: u32 = 0;
}
