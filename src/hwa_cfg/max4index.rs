#[doc = "Register `MAX4INDEX` reader"]
pub type R = crate::R<Max4indexSpec>;
#[doc = "Register `MAX4INDEX` writer"]
pub type W = crate::W<Max4indexSpec>;
#[doc = "Field `MAX4INDEX` reader - 11:0\\]
Refer MAX1VALUE"]
pub type Max4indexR = crate::FieldReader<u16>;
#[doc = "Field `MAX4INDEX` writer - 11:0\\]
Refer MAX1VALUE"]
pub type Max4indexW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Refer MAX1VALUE"]
    #[inline(always)]
    pub fn max4index(&self) -> Max4indexR {
        Max4indexR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Refer MAX1VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn max4index(&mut self) -> Max4indexW<Max4indexSpec> {
        Max4indexW::new(self, 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Max4indexSpec> {
        NuW::new(self, 12)
    }
}
#[doc = "MAX4INDEX\n\nYou can [`read`](crate::Reg::read) this register and get [`max4index::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max4index::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Max4indexSpec;
impl crate::RegisterSpec for Max4indexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max4index::R`](R) reader structure"]
impl crate::Readable for Max4indexSpec {}
#[doc = "`write(|w| ..)` method takes [`max4index::W`](W) writer structure"]
impl crate::Writable for Max4indexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX4INDEX to value 0"]
impl crate::Resettable for Max4indexSpec {
    const RESET_VALUE: u32 = 0;
}
