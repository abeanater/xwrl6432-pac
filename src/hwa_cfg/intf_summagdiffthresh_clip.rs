#[doc = "Register `INTF_SUMMAGDIFFTHRESH_CLIP` reader"]
pub type R = crate::R<IntfSummagdiffthreshClipSpec>;
#[doc = "Register `INTF_SUMMAGDIFFTHRESH_CLIP` writer"]
pub type W = crate::W<IntfSummagdiffthreshClipSpec>;
#[doc = "Field `INTF_SUMMAGDIFFTHRESH_CLIP` reader - 0:0\\]
Indicates the clip status of sum of magnitude difference threshold values"]
pub type IntfSummagdiffthreshClipR = crate::BitReader;
#[doc = "Field `INTF_SUMMAGDIFFTHRESH_CLIP` writer - 0:0\\]
Indicates the clip status of sum of magnitude difference threshold values"]
pub type IntfSummagdiffthreshClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the clip status of sum of magnitude difference threshold values"]
    #[inline(always)]
    pub fn intf_summagdiffthresh_clip(&self) -> IntfSummagdiffthreshClipR {
        IntfSummagdiffthreshClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the clip status of sum of magnitude difference threshold values"]
    #[inline(always)]
    #[must_use]
    pub fn intf_summagdiffthresh_clip(
        &mut self,
    ) -> IntfSummagdiffthreshClipW<IntfSummagdiffthreshClipSpec> {
        IntfSummagdiffthreshClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfSummagdiffthreshClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "INTF_SUMMAGDIFFTHRESH_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_summagdiffthresh_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_summagdiffthresh_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSummagdiffthreshClipSpec;
impl crate::RegisterSpec for IntfSummagdiffthreshClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_summagdiffthresh_clip::R`](R) reader structure"]
impl crate::Readable for IntfSummagdiffthreshClipSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_summagdiffthresh_clip::W`](W) writer structure"]
impl crate::Writable for IntfSummagdiffthreshClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_SUMMAGDIFFTHRESH_CLIP to value 0"]
impl crate::Resettable for IntfSummagdiffthreshClipSpec {
    const RESET_VALUE: u32 = 0;
}
