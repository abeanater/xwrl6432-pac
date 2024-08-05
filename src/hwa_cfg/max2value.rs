#[doc = "Register `MAX2VALUE` reader"]
pub type R = crate::R<Max2valueSpec>;
#[doc = "Register `MAX2VALUE` writer"]
pub type W = crate::W<Max2valueSpec>;
#[doc = "Field `MAX2VALUE` reader - 23:0\\]
Refer MAX1VALUE"]
pub type Max2valueR = crate::FieldReader<u32>;
#[doc = "Field `MAX2VALUE` writer - 23:0\\]
Refer MAX1VALUE"]
pub type Max2valueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Refer MAX1VALUE"]
    #[inline(always)]
    pub fn max2value(&self) -> Max2valueR {
        Max2valueR::new(self.bits & 0x00ff_ffff)
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
    pub fn max2value(&mut self) -> Max2valueW<Max2valueSpec> {
        Max2valueW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Max2valueSpec> {
        NuW::new(self, 24)
    }
}
#[doc = "MAX2VALUE\n\nYou can [`read`](crate::Reg::read) this register and get [`max2value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max2value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Max2valueSpec;
impl crate::RegisterSpec for Max2valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max2value::R`](R) reader structure"]
impl crate::Readable for Max2valueSpec {}
#[doc = "`write(|w| ..)` method takes [`max2value::W`](W) writer structure"]
impl crate::Writable for Max2valueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX2VALUE to value 0"]
impl crate::Resettable for Max2valueSpec {
    const RESET_VALUE: u32 = 0;
}
