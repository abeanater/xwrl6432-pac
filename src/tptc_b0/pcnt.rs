#[doc = "Register `PCNT` reader"]
pub type R = crate::R<PcntSpec>;
#[doc = "Register `PCNT` writer"]
pub type W = crate::W<PcntSpec>;
#[doc = "Field `ADIMENSION_COUNT__NUMBER` reader - 15:0\\]
A-Dimension count. Number of bytes to be transferred in first dimension."]
pub type AdimensionCount_NumberR = crate::FieldReader<u16>;
#[doc = "Field `ADIMENSION_COUNT__NUMBER` writer - 15:0\\]
A-Dimension count. Number of bytes to be transferred in first dimension."]
pub type AdimensionCount_NumberW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BDIMENSION_COUNT__NUMBER` reader - 31:16\\]
B-Dimension count. Number of arrays to be transferred where each array is ACNT in length."]
pub type BdimensionCount_NumberR = crate::FieldReader<u16>;
#[doc = "Field `BDIMENSION_COUNT__NUMBER` writer - 31:16\\]
B-Dimension count. Number of arrays to be transferred where each array is ACNT in length."]
pub type BdimensionCount_NumberW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
A-Dimension count. Number of bytes to be transferred in first dimension."]
    #[inline(always)]
    pub fn adimension_count__number(&self) -> AdimensionCount_NumberR {
        AdimensionCount_NumberR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
B-Dimension count. Number of arrays to be transferred where each array is ACNT in length."]
    #[inline(always)]
    pub fn bdimension_count__number(&self) -> BdimensionCount_NumberR {
        BdimensionCount_NumberR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
A-Dimension count. Number of bytes to be transferred in first dimension."]
    #[inline(always)]
    #[must_use]
    pub fn adimension_count__number(&mut self) -> AdimensionCount_NumberW<PcntSpec> {
        AdimensionCount_NumberW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
B-Dimension count. Number of arrays to be transferred where each array is ACNT in length."]
    #[inline(always)]
    #[must_use]
    pub fn bdimension_count__number(&mut self) -> BdimensionCount_NumberW<PcntSpec> {
        BdimensionCount_NumberW::new(self, 16)
    }
}
#[doc = "Prog Set Count\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcntSpec;
impl crate::RegisterSpec for PcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnt::R`](R) reader structure"]
impl crate::Readable for PcntSpec {}
#[doc = "`write(|w| ..)` method takes [`pcnt::W`](W) writer structure"]
impl crate::Writable for PcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCNT to value 0"]
impl crate::Resettable for PcntSpec {
    const RESET_VALUE: u32 = 0;
}
