#[doc = "Register `MAX2INDEX` reader"]
pub type R = crate::R<Max2indexSpec>;
#[doc = "Register `MAX2INDEX` writer"]
pub type W = crate::W<Max2indexSpec>;
#[doc = "Field `MAX2INDEX` reader - 11:0\\]
Refer MAX1INDEX"]
pub type Max2indexR = crate::FieldReader<u16>;
#[doc = "Field `MAX2INDEX` writer - 11:0\\]
Refer MAX1INDEX"]
pub type Max2indexW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Refer MAX1INDEX"]
    #[inline(always)]
    pub fn max2index(&self) -> Max2indexR {
        Max2indexR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Refer MAX1INDEX"]
    #[inline(always)]
    #[must_use]
    pub fn max2index(&mut self) -> Max2indexW<Max2indexSpec> {
        Max2indexW::new(self, 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Max2indexSpec> {
        NuW::new(self, 12)
    }
}
#[doc = "MAX2INDEX\n\nYou can [`read`](crate::Reg::read) this register and get [`max2index::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max2index::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Max2indexSpec;
impl crate::RegisterSpec for Max2indexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max2index::R`](R) reader structure"]
impl crate::Readable for Max2indexSpec {}
#[doc = "`write(|w| ..)` method takes [`max2index::W`](W) writer structure"]
impl crate::Writable for Max2indexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX2INDEX to value 0"]
impl crate::Resettable for Max2indexSpec {
    const RESET_VALUE: u32 = 0;
}
