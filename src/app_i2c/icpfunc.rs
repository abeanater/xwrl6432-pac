#[doc = "Register `ICPFUNC` reader"]
pub type R = crate::R<IcpfuncSpec>;
#[doc = "Register `ICPFUNC` writer"]
pub type W = crate::W<IcpfuncSpec>;
#[doc = "Field `PFUNC0` reader - 0:0\\]
Controls the function of the I2C SCL and SDA pins. 0 = Pins function as SCL and SDA 1 = Pins functions as GPIO Note: No hardware protection is required to disable I2C function when the PFUNC\\[0\\]
and IRS_ bits are both set to one. When PFUNC\\[0\\]
is\"1\" (GPIO mode) the sub-module which controls the I2C function receives the value\"1\" for SCL and SDA. IRS_ can be set to\"1\" regardless of PFUNC\\[0\\]
and the I2C function works whenever the IRS_ bit is\"1\". The user is expected to hold I2C in reset via IRS_ bit when changing to/from GPIO mode via the PFUNC\\[0\\]
bit."]
pub type Pfunc0R = crate::BitReader;
#[doc = "Field `PFUNC0` writer - 0:0\\]
Controls the function of the I2C SCL and SDA pins. 0 = Pins function as SCL and SDA 1 = Pins functions as GPIO Note: No hardware protection is required to disable I2C function when the PFUNC\\[0\\]
and IRS_ bits are both set to one. When PFUNC\\[0\\]
is\"1\" (GPIO mode) the sub-module which controls the I2C function receives the value\"1\" for SCL and SDA. IRS_ can be set to\"1\" regardless of PFUNC\\[0\\]
and the I2C function works whenever the IRS_ bit is\"1\". The user is expected to hold I2C in reset via IRS_ bit when changing to/from GPIO mode via the PFUNC\\[0\\]
bit."]
pub type Pfunc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:1\\]
Reserved."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:1\\]
Reserved."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls the function of the I2C SCL and SDA pins. 0 = Pins function as SCL and SDA 1 = Pins functions as GPIO Note: No hardware protection is required to disable I2C function when the PFUNC\\[0\\]
and IRS_ bits are both set to one. When PFUNC\\[0\\]
is\"1\" (GPIO mode) the sub-module which controls the I2C function receives the value\"1\" for SCL and SDA. IRS_ can be set to\"1\" regardless of PFUNC\\[0\\]
and the I2C function works whenever the IRS_ bit is\"1\". The user is expected to hold I2C in reset via IRS_ bit when changing to/from GPIO mode via the PFUNC\\[0\\]
bit."]
    #[inline(always)]
    pub fn pfunc0(&self) -> Pfunc0R {
        Pfunc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls the function of the I2C SCL and SDA pins. 0 = Pins function as SCL and SDA 1 = Pins functions as GPIO Note: No hardware protection is required to disable I2C function when the PFUNC\\[0\\]
and IRS_ bits are both set to one. When PFUNC\\[0\\]
is\"1\" (GPIO mode) the sub-module which controls the I2C function receives the value\"1\" for SCL and SDA. IRS_ can be set to\"1\" regardless of PFUNC\\[0\\]
and the I2C function works whenever the IRS_ bit is\"1\". The user is expected to hold I2C in reset via IRS_ bit when changing to/from GPIO mode via the PFUNC\\[0\\]
bit."]
    #[inline(always)]
    #[must_use]
    pub fn pfunc0(&mut self) -> Pfunc0W<IcpfuncSpec> {
        Pfunc0W::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcpfuncSpec> {
        NuW::new(self, 1)
    }
}
#[doc = "I2C Pin Function register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpfunc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpfunc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcpfuncSpec;
impl crate::RegisterSpec for IcpfuncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpfunc::R`](R) reader structure"]
impl crate::Readable for IcpfuncSpec {}
#[doc = "`write(|w| ..)` method takes [`icpfunc::W`](W) writer structure"]
impl crate::Writable for IcpfuncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPFUNC to value 0"]
impl crate::Resettable for IcpfuncSpec {
    const RESET_VALUE: u32 = 0;
}
