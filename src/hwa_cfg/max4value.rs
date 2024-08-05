#[doc = "Register `MAX4VALUE` reader"]
pub type R = crate::R<Max4valueSpec>;
#[doc = "Register `MAX4VALUE` writer"]
pub type W = crate::W<Max4valueSpec>;
#[doc = "Field `MAX4VALUE` reader - 23:0\\]
Refer MAX1INDEX"]
pub type Max4valueR = crate::FieldReader<u32>;
#[doc = "Field `MAX4VALUE` writer - 23:0\\]
Refer MAX1INDEX"]
pub type Max4valueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Refer MAX1INDEX"]
    #[inline(always)]
    pub fn max4value(&self) -> Max4valueR {
        Max4valueR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Refer MAX1INDEX"]
    #[inline(always)]
    #[must_use]
    pub fn max4value(&mut self) -> Max4valueW<Max4valueSpec> {
        Max4valueW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Max4valueSpec> {
        NuW::new(self, 24)
    }
}
#[doc = "MAX4VALUE\n\nYou can [`read`](crate::Reg::read) this register and get [`max4value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max4value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Max4valueSpec;
impl crate::RegisterSpec for Max4valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max4value::R`](R) reader structure"]
impl crate::Readable for Max4valueSpec {}
#[doc = "`write(|w| ..)` method takes [`max4value::W`](W) writer structure"]
impl crate::Writable for Max4valueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX4VALUE to value 0"]
impl crate::Resettable for Max4valueSpec {
    const RESET_VALUE: u32 = 0;
}
