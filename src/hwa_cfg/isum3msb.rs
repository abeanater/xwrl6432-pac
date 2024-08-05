#[doc = "Register `ISUM3MSB` reader"]
pub type R = crate::R<Isum3msbSpec>;
#[doc = "Register `ISUM3MSB` writer"]
pub type W = crate::W<Isum3msbSpec>;
#[doc = "Field `ISUM3MSB` reader - 3:0\\]
Refer ISUM1LSB"]
pub type Isum3msbR = crate::FieldReader;
#[doc = "Field `ISUM3MSB` writer - 3:0\\]
Refer ISUM1LSB"]
pub type Isum3msbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn isum3msb(&self) -> Isum3msbR {
        Isum3msbR::new((self.bits & 0x0f) as u8)
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
    pub fn isum3msb(&mut self) -> Isum3msbW<Isum3msbSpec> {
        Isum3msbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Isum3msbSpec> {
        NuW::new(self, 4)
    }
}
#[doc = "ISUM3MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum3msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum3msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isum3msbSpec;
impl crate::RegisterSpec for Isum3msbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isum3msb::R`](R) reader structure"]
impl crate::Readable for Isum3msbSpec {}
#[doc = "`write(|w| ..)` method takes [`isum3msb::W`](W) writer structure"]
impl crate::Writable for Isum3msbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISUM3MSB to value 0"]
impl crate::Resettable for Isum3msbSpec {
    const RESET_VALUE: u32 = 0;
}
