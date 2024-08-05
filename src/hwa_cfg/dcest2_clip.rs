#[doc = "Register `DCEST2_CLIP` reader"]
pub type R = crate::R<Dcest2ClipSpec>;
#[doc = "Register `DCEST2_CLIP` writer"]
pub type W = crate::W<Dcest2ClipSpec>;
#[doc = "Field `DCEST2_CLIP` reader - 0:0\\]
This register contains the clip status of both I/Q DC estimates for bcnt =1"]
pub type Dcest2ClipR = crate::BitReader;
#[doc = "Field `DCEST2_CLIP` writer - 0:0\\]
This register contains the clip status of both I/Q DC estimates for bcnt =1"]
pub type Dcest2ClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This register contains the clip status of both I/Q DC estimates for bcnt =1"]
    #[inline(always)]
    pub fn dcest2_clip(&self) -> Dcest2ClipR {
        Dcest2ClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This register contains the clip status of both I/Q DC estimates for bcnt =1"]
    #[inline(always)]
    #[must_use]
    pub fn dcest2_clip(&mut self) -> Dcest2ClipW<Dcest2ClipSpec> {
        Dcest2ClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest2ClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "DCEST2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest2_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest2_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest2ClipSpec;
impl crate::RegisterSpec for Dcest2ClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest2_clip::R`](R) reader structure"]
impl crate::Readable for Dcest2ClipSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest2_clip::W`](W) writer structure"]
impl crate::Writable for Dcest2ClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST2_CLIP to value 0"]
impl crate::Resettable for Dcest2ClipSpec {
    const RESET_VALUE: u32 = 0;
}
