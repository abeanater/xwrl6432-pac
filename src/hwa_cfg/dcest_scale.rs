#[doc = "Register `DCEST_SCALE` reader"]
pub type R = crate::R<DcestScaleSpec>;
#[doc = "Register `DCEST_SCALE` writer"]
pub type W = crate::W<DcestScaleSpec>;
#[doc = "Field `DCEST_SCALE` reader - 8:0\\]
9-bit scale applied to all 6 accumulators. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation.Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
pub type DcestScaleR = crate::FieldReader<u16>;
#[doc = "Field `DCEST_SCALE` writer - 8:0\\]
9-bit scale applied to all 6 accumulators. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation.Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
pub type DcestScaleW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
9-bit scale applied to all 6 accumulators. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation.Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
    #[inline(always)]
    pub fn dcest_scale(&self) -> DcestScaleR {
        DcestScaleR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
9-bit scale applied to all 6 accumulators. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation.Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
    #[inline(always)]
    #[must_use]
    pub fn dcest_scale(&mut self) -> DcestScaleW<DcestScaleSpec> {
        DcestScaleW::new(self, 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<DcestScaleSpec> {
        Nu1W::new(self, 9)
    }
}
#[doc = "DCEST_SCALE\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest_scale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest_scale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcestScaleSpec;
impl crate::RegisterSpec for DcestScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest_scale::R`](R) reader structure"]
impl crate::Readable for DcestScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest_scale::W`](W) writer structure"]
impl crate::Writable for DcestScaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST_SCALE to value 0"]
impl crate::Resettable for DcestScaleSpec {
    const RESET_VALUE: u32 = 0;
}
