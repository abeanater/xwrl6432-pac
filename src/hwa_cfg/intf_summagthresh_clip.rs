#[doc = "Register `INTF_SUMMAGTHRESH_CLIP` reader"]
pub type R = crate::R<IntfSummagthreshClipSpec>;
#[doc = "Register `INTF_SUMMAGTHRESH_CLIP` writer"]
pub type W = crate::W<IntfSummagthreshClipSpec>;
#[doc = "Field `INTF_SUMMAGTHRESH_CLIP` reader - 0:0\\]
Indicates the clip status of sum of magnitude threshold values"]
pub type IntfSummagthreshClipR = crate::BitReader;
#[doc = "Field `INTF_SUMMAGTHRESH_CLIP` writer - 0:0\\]
Indicates the clip status of sum of magnitude threshold values"]
pub type IntfSummagthreshClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the clip status of sum of magnitude threshold values"]
    #[inline(always)]
    pub fn intf_summagthresh_clip(&self) -> IntfSummagthreshClipR {
        IntfSummagthreshClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the clip status of sum of magnitude threshold values"]
    #[inline(always)]
    #[must_use]
    pub fn intf_summagthresh_clip(&mut self) -> IntfSummagthreshClipW<IntfSummagthreshClipSpec> {
        IntfSummagthreshClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfSummagthreshClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "INTF_SUMMAGTHRESH_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_summagthresh_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_summagthresh_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSummagthreshClipSpec;
impl crate::RegisterSpec for IntfSummagthreshClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_summagthresh_clip::R`](R) reader structure"]
impl crate::Readable for IntfSummagthreshClipSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_summagthresh_clip::W`](W) writer structure"]
impl crate::Writable for IntfSummagthreshClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_SUMMAGTHRESH_CLIP to value 0"]
impl crate::Resettable for IntfSummagthreshClipSpec {
    const RESET_VALUE: u32 = 0;
}
