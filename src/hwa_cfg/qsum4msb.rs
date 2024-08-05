#[doc = "Register `QSUM4MSB` reader"]
pub type R = crate::R<Qsum4msbSpec>;
#[doc = "Register `QSUM4MSB` writer"]
pub type W = crate::W<Qsum4msbSpec>;
#[doc = "Field `QSUM4MSB` reader - 3:0\\]
Refer ISUM1LSB"]
pub type Qsum4msbR = crate::FieldReader;
#[doc = "Field `QSUM4MSB` writer - 3:0\\]
Refer ISUM1LSB"]
pub type Qsum4msbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn qsum4msb(&self) -> Qsum4msbR {
        Qsum4msbR::new((self.bits & 0x0f) as u8)
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
    pub fn qsum4msb(&mut self) -> Qsum4msbW<Qsum4msbSpec> {
        Qsum4msbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Qsum4msbSpec> {
        NuW::new(self, 4)
    }
}
#[doc = "QSUM4MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum4msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum4msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qsum4msbSpec;
impl crate::RegisterSpec for Qsum4msbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsum4msb::R`](R) reader structure"]
impl crate::Readable for Qsum4msbSpec {}
#[doc = "`write(|w| ..)` method takes [`qsum4msb::W`](W) writer structure"]
impl crate::Writable for Qsum4msbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSUM4MSB to value 0"]
impl crate::Resettable for Qsum4msbSpec {
    const RESET_VALUE: u32 = 0;
}
