#[doc = "Register `INTF_MAGACC2_CLIP` reader"]
pub type R = crate::R<IntfMagacc2ClipSpec>;
#[doc = "Register `INTF_MAGACC2_CLIP` writer"]
pub type W = crate::W<IntfMagacc2ClipSpec>;
#[doc = "Field `INTF_MAGACC2_CLIP` reader - 0:0\\]
Interference magnitude accumulator clip status"]
pub type IntfMagacc2ClipR = crate::BitReader;
#[doc = "Field `INTF_MAGACC2_CLIP` writer - 0:0\\]
Interference magnitude accumulator clip status"]
pub type IntfMagacc2ClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interference magnitude accumulator clip status"]
    #[inline(always)]
    pub fn intf_magacc2_clip(&self) -> IntfMagacc2ClipR {
        IntfMagacc2ClipR::new((self.bits & 1) != 0)
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
    pub fn intf_magacc2_clip(&mut self) -> IntfMagacc2ClipW<IntfMagacc2ClipSpec> {
        IntfMagacc2ClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagacc2ClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "INTF_MAGACC2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc2_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc2_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagacc2ClipSpec;
impl crate::RegisterSpec for IntfMagacc2ClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magacc2_clip::R`](R) reader structure"]
impl crate::Readable for IntfMagacc2ClipSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magacc2_clip::W`](W) writer structure"]
impl crate::Writable for IntfMagacc2ClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGACC2_CLIP to value 0"]
impl crate::Resettable for IntfMagacc2ClipSpec {
    const RESET_VALUE: u32 = 0;
}
