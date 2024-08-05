#[doc = "Register `DCACC4_CLIP` reader"]
pub type R = crate::R<Dcacc4ClipSpec>;
#[doc = "Register `DCACC4_CLIP` writer"]
pub type W = crate::W<Dcacc4ClipSpec>;
#[doc = "Field `DCACC4_CLIP` reader - 0:0\\]
This register contains the clip status of both I/Q of DC accumulators for bcnt =3"]
pub type Dcacc4ClipR = crate::BitReader;
#[doc = "Field `DCACC4_CLIP` writer - 0:0\\]
This register contains the clip status of both I/Q of DC accumulators for bcnt =3"]
pub type Dcacc4ClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This register contains the clip status of both I/Q of DC accumulators for bcnt =3"]
    #[inline(always)]
    pub fn dcacc4_clip(&self) -> Dcacc4ClipR {
        Dcacc4ClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This register contains the clip status of both I/Q of DC accumulators for bcnt =3"]
    #[inline(always)]
    #[must_use]
    pub fn dcacc4_clip(&mut self) -> Dcacc4ClipW<Dcacc4ClipSpec> {
        Dcacc4ClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcacc4ClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "DCACC4_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcacc4_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcacc4_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcacc4ClipSpec;
impl crate::RegisterSpec for Dcacc4ClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcacc4_clip::R`](R) reader structure"]
impl crate::Readable for Dcacc4ClipSpec {}
#[doc = "`write(|w| ..)` method takes [`dcacc4_clip::W`](W) writer structure"]
impl crate::Writable for Dcacc4ClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACC4_CLIP to value 0"]
impl crate::Resettable for Dcacc4ClipSpec {
    const RESET_VALUE: u32 = 0;
}
