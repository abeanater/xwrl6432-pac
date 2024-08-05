#[doc = "Register `DCSUB6_CLIP` reader"]
pub type R = crate::R<Dcsub6ClipSpec>;
#[doc = "Register `DCSUB6_CLIP` writer"]
pub type W = crate::W<Dcsub6ClipSpec>;
#[doc = "Field `DCSUB6_CLIP` reader - 0:0\\]
Indicates the DC subtraction clip status for bcnt =5"]
pub type Dcsub6ClipR = crate::BitReader;
#[doc = "Field `DCSUB6_CLIP` writer - 0:0\\]
Indicates the DC subtraction clip status for bcnt =5"]
pub type Dcsub6ClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the DC subtraction clip status for bcnt =5"]
    #[inline(always)]
    pub fn dcsub6_clip(&self) -> Dcsub6ClipR {
        Dcsub6ClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the DC subtraction clip status for bcnt =5"]
    #[inline(always)]
    #[must_use]
    pub fn dcsub6_clip(&mut self) -> Dcsub6ClipW<Dcsub6ClipSpec> {
        Dcsub6ClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcsub6ClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "DCSUB6_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcsub6_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsub6_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcsub6ClipSpec;
impl crate::RegisterSpec for Dcsub6ClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcsub6_clip::R`](R) reader structure"]
impl crate::Readable for Dcsub6ClipSpec {}
#[doc = "`write(|w| ..)` method takes [`dcsub6_clip::W`](W) writer structure"]
impl crate::Writable for Dcsub6ClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCSUB6_CLIP to value 0"]
impl crate::Resettable for Dcsub6ClipSpec {
    const RESET_VALUE: u32 = 0;
}
