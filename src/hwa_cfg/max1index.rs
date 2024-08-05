#[doc = "Register `MAX1INDEX` reader"]
pub type R = crate::R<Max1indexSpec>;
#[doc = "Register `MAX1INDEX` writer"]
pub type W = crate::W<Max1indexSpec>;
#[doc = "Field `MAX1INDEX` reader - 11:0\\]
Max index: These registers contain the max index on a per-iteration basis, corresponding to each max value in the MAXn_VALUE registers."]
pub type Max1indexR = crate::FieldReader<u16>;
#[doc = "Field `MAX1INDEX` writer - 11:0\\]
Max index: These registers contain the max index on a per-iteration basis, corresponding to each max value in the MAXn_VALUE registers."]
pub type Max1indexW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Max index: These registers contain the max index on a per-iteration basis, corresponding to each max value in the MAXn_VALUE registers."]
    #[inline(always)]
    pub fn max1index(&self) -> Max1indexR {
        Max1indexR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Max index: These registers contain the max index on a per-iteration basis, corresponding to each max value in the MAXn_VALUE registers."]
    #[inline(always)]
    #[must_use]
    pub fn max1index(&mut self) -> Max1indexW<Max1indexSpec> {
        Max1indexW::new(self, 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Max1indexSpec> {
        NuW::new(self, 12)
    }
}
#[doc = "MAX1INDEX\n\nYou can [`read`](crate::Reg::read) this register and get [`max1index::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max1index::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Max1indexSpec;
impl crate::RegisterSpec for Max1indexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max1index::R`](R) reader structure"]
impl crate::Readable for Max1indexSpec {}
#[doc = "`write(|w| ..)` method takes [`max1index::W`](W) writer structure"]
impl crate::Writable for Max1indexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAX1INDEX to value 0"]
impl crate::Resettable for Max1indexSpec {
    const RESET_VALUE: u32 = 0;
}
