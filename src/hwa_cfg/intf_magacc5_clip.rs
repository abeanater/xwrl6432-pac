#[doc = "Register `INTF_MAGACC5_CLIP` reader"]
pub type R = crate::R<IntfMagacc5ClipSpec>;
#[doc = "Register `INTF_MAGACC5_CLIP` writer"]
pub type W = crate::W<IntfMagacc5ClipSpec>;
#[doc = "Field `INTF_MAGACC5_CLIP` reader - 0:0\\]
Interference magnitude accumulator clip status"]
pub type IntfMagacc5ClipR = crate::BitReader;
#[doc = "Field `INTF_MAGACC5_CLIP` writer - 0:0\\]
Interference magnitude accumulator clip status"]
pub type IntfMagacc5ClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interference magnitude accumulator clip status"]
    #[inline(always)]
    pub fn intf_magacc5_clip(&self) -> IntfMagacc5ClipR {
        IntfMagacc5ClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interference magnitude accumulator clip status"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magacc5_clip(&mut self) -> IntfMagacc5ClipW<IntfMagacc5ClipSpec> {
        IntfMagacc5ClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagacc5ClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "INTF_MAGACC5_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc5_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc5_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagacc5ClipSpec;
impl crate::RegisterSpec for IntfMagacc5ClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magacc5_clip::R`](R) reader structure"]
impl crate::Readable for IntfMagacc5ClipSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magacc5_clip::W`](W) writer structure"]
impl crate::Writable for IntfMagacc5ClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGACC5_CLIP to value 0"]
impl crate::Resettable for IntfMagacc5ClipSpec {
    const RESET_VALUE: u32 = 0;
}
