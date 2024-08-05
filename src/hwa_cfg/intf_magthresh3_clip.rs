#[doc = "Register `INTF_MAGTHRESH3_CLIP` reader"]
pub type R = crate::R<IntfMagthresh3ClipSpec>;
#[doc = "Register `INTF_MAGTHRESH3_CLIP` writer"]
pub type W = crate::W<IntfMagthresh3ClipSpec>;
#[doc = "Field `INTF_MAGTHRESH3_CLIP` reader - 0:0\\]
Interference magnitude threshold clip status"]
pub type IntfMagthresh3ClipR = crate::BitReader;
#[doc = "Field `INTF_MAGTHRESH3_CLIP` writer - 0:0\\]
Interference magnitude threshold clip status"]
pub type IntfMagthresh3ClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interference magnitude threshold clip status"]
    #[inline(always)]
    pub fn intf_magthresh3_clip(&self) -> IntfMagthresh3ClipR {
        IntfMagthresh3ClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interference magnitude threshold clip status"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magthresh3_clip(&mut self) -> IntfMagthresh3ClipW<IntfMagthresh3ClipSpec> {
        IntfMagthresh3ClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagthresh3ClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "INTF_MAGTHRESH3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh3_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh3_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagthresh3ClipSpec;
impl crate::RegisterSpec for IntfMagthresh3ClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magthresh3_clip::R`](R) reader structure"]
impl crate::Readable for IntfMagthresh3ClipSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magthresh3_clip::W`](W) writer structure"]
impl crate::Writable for IntfMagthresh3ClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGTHRESH3_CLIP to value 0"]
impl crate::Resettable for IntfMagthresh3ClipSpec {
    const RESET_VALUE: u32 = 0;
}
