#[doc = "Register `ISUM1MSB` reader"]
pub type R = crate::R<Isum1msbSpec>;
#[doc = "Register `ISUM1MSB` writer"]
pub type W = crate::W<Isum1msbSpec>;
#[doc = "Field `ISUM1MSB` reader - 3:0\\]
Refer ISUM1LSB"]
pub type Isum1msbR = crate::FieldReader;
#[doc = "Field `ISUM1MSB` writer - 3:0\\]
Refer ISUM1LSB"]
pub type Isum1msbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn isum1msb(&self) -> Isum1msbR {
        Isum1msbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    #[must_use]
    pub fn isum1msb(&mut self) -> Isum1msbW<Isum1msbSpec> {
        Isum1msbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Isum1msbSpec> {
        NuW::new(self, 4)
    }
}
#[doc = "ISUM1MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum1msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum1msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isum1msbSpec;
impl crate::RegisterSpec for Isum1msbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isum1msb::R`](R) reader structure"]
impl crate::Readable for Isum1msbSpec {}
#[doc = "`write(|w| ..)` method takes [`isum1msb::W`](W) writer structure"]
impl crate::Writable for Isum1msbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISUM1MSB to value 0"]
impl crate::Resettable for Isum1msbSpec {
    const RESET_VALUE: u32 = 0;
}
