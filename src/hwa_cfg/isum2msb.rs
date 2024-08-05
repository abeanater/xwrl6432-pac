#[doc = "Register `ISUM2MSB` reader"]
pub type R = crate::R<Isum2msbSpec>;
#[doc = "Register `ISUM2MSB` writer"]
pub type W = crate::W<Isum2msbSpec>;
#[doc = "Field `ISUM2MSB` reader - 3:0\\]
Refer ISUM1LSB"]
pub type Isum2msbR = crate::FieldReader;
#[doc = "Field `ISUM2MSB` writer - 3:0\\]
Refer ISUM1LSB"]
pub type Isum2msbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn isum2msb(&self) -> Isum2msbR {
        Isum2msbR::new((self.bits & 0x0f) as u8)
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
    pub fn isum2msb(&mut self) -> Isum2msbW<Isum2msbSpec> {
        Isum2msbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Isum2msbSpec> {
        NuW::new(self, 4)
    }
}
#[doc = "ISUM2MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum2msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum2msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isum2msbSpec;
impl crate::RegisterSpec for Isum2msbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isum2msb::R`](R) reader structure"]
impl crate::Readable for Isum2msbSpec {}
#[doc = "`write(|w| ..)` method takes [`isum2msb::W`](W) writer structure"]
impl crate::Writable for Isum2msbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISUM2MSB to value 0"]
impl crate::Resettable for Isum2msbSpec {
    const RESET_VALUE: u32 = 0;
}
