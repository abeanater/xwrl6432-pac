#[doc = "Register `QSUM2MSB` reader"]
pub type R = crate::R<Qsum2msbSpec>;
#[doc = "Register `QSUM2MSB` writer"]
pub type W = crate::W<Qsum2msbSpec>;
#[doc = "Field `QSUM2MSB` reader - 3:0\\]
Refer ISUM1LSB"]
pub type Qsum2msbR = crate::FieldReader;
#[doc = "Field `QSUM2MSB` writer - 3:0\\]
Refer ISUM1LSB"]
pub type Qsum2msbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn qsum2msb(&self) -> Qsum2msbR {
        Qsum2msbR::new((self.bits & 0x0f) as u8)
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
    pub fn qsum2msb(&mut self) -> Qsum2msbW<Qsum2msbSpec> {
        Qsum2msbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Qsum2msbSpec> {
        NuW::new(self, 4)
    }
}
#[doc = "QSUM2MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum2msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum2msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qsum2msbSpec;
impl crate::RegisterSpec for Qsum2msbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsum2msb::R`](R) reader structure"]
impl crate::Readable for Qsum2msbSpec {}
#[doc = "`write(|w| ..)` method takes [`qsum2msb::W`](W) writer structure"]
impl crate::Writable for Qsum2msbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSUM2MSB to value 0"]
impl crate::Resettable for Qsum2msbSpec {
    const RESET_VALUE: u32 = 0;
}
