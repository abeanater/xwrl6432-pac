#[doc = "Register `QSUM1MSB` reader"]
pub type R = crate::R<Qsum1msbSpec>;
#[doc = "Register `QSUM1MSB` writer"]
pub type W = crate::W<Qsum1msbSpec>;
#[doc = "Field `QSUM1MSB` reader - 3:0\\]
Refer ISUM1LSB"]
pub type Qsum1msbR = crate::FieldReader;
#[doc = "Field `QSUM1MSB` writer - 3:0\\]
Refer ISUM1LSB"]
pub type Qsum1msbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn qsum1msb(&self) -> Qsum1msbR {
        Qsum1msbR::new((self.bits & 0x0f) as u8)
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
    pub fn qsum1msb(&mut self) -> Qsum1msbW<Qsum1msbSpec> {
        Qsum1msbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Qsum1msbSpec> {
        NuW::new(self, 4)
    }
}
#[doc = "QSUM1MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum1msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum1msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qsum1msbSpec;
impl crate::RegisterSpec for Qsum1msbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsum1msb::R`](R) reader structure"]
impl crate::Readable for Qsum1msbSpec {}
#[doc = "`write(|w| ..)` method takes [`qsum1msb::W`](W) writer structure"]
impl crate::Writable for Qsum1msbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSUM1MSB to value 0"]
impl crate::Resettable for Qsum1msbSpec {
    const RESET_VALUE: u32 = 0;
}
