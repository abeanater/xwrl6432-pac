#[doc = "Register `MAX3VALUE` reader"]
pub type R = crate::R<Max3valueSpec>;
#[doc = "Register `MAX3VALUE` writer"]
pub type W = crate::W<Max3valueSpec>;
#[doc = "Field `MAX3VALUE` reader - 23:0\\]
Refer MAX1VALUE"]
pub type Max3valueR = crate::FieldReader<u32>;
#[doc = "Field `MAX3VALUE` writer - 23:0\\]
Refer MAX1VALUE"]
pub type Max3valueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Refer MAX1VALUE"]
    #[inline(always)]
    pub fn max3value(&self) -> Max3valueR {
        Max3valueR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Refer MAX1VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn max3value(&mut self) -> Max3valueW<Max3valueSpec> {
        Max3valueW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Max3valueSpec> {
        NuW::new(self, 24)
    }
}
#[doc = "MAX3VALUE\n\nYou can [`read`](crate::Reg::read) this register and get [`max3value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max3value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Max3valueSpec;
impl crate::RegisterSpec for Max3valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max3value::R`](R) reader structure"]
impl crate::Readable for Max3valueSpec {}
#[doc = "`write(|w| ..)` method takes [`max3value::W`](W) writer structure"]
impl crate::Writable for Max3valueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX3VALUE to value 0"]
impl crate::Resettable for Max3valueSpec {
    const RESET_VALUE: u32 = 0;
}
