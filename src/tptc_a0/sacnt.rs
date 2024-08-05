#[doc = "Register `SACNT` reader"]
pub type R = crate::R<SacntSpec>;
#[doc = "Register `SACNT` writer"]
pub type W = crate::W<SacntSpec>;
#[doc = "Field `ADIMENSION_COUNT__NUMBER` reader - 22:0\\]
A-Dimension count. Number of bytes to be transferred in first dimension."]
pub type AdimensionCount_NumberR = crate::FieldReader<u32>;
#[doc = "Field `ADIMENSION_COUNT__NUMBER` writer - 22:0\\]
A-Dimension count. Number of bytes to be transferred in first dimension."]
pub type AdimensionCount_NumberW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - 22:0\\]
A-Dimension count. Number of bytes to be transferred in first dimension."]
    #[inline(always)]
    pub fn adimension_count__number(&self) -> AdimensionCount_NumberR {
        AdimensionCount_NumberR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - 22:0\\]
A-Dimension count. Number of bytes to be transferred in first dimension."]
    #[inline(always)]
    #[must_use]
    pub fn adimension_count__number(&mut self) -> AdimensionCount_NumberW<SacntSpec> {
        AdimensionCount_NumberW::new(self, 0)
    }
}
#[doc = "Src Actv Set A-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`sacnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sacnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SacntSpec;
impl crate::RegisterSpec for SacntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sacnt::R`](R) reader structure"]
impl crate::Readable for SacntSpec {}
#[doc = "`write(|w| ..)` method takes [`sacnt::W`](W) writer structure"]
impl crate::Writable for SacntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SACNT to value 0"]
impl crate::Resettable for SacntSpec {
    const RESET_VALUE: u32 = 0;
}
