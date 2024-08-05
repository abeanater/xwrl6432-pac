#[doc = "Register `DCSUB3_CLIP` reader"]
pub type R = crate::R<Dcsub3ClipSpec>;
#[doc = "Register `DCSUB3_CLIP` writer"]
pub type W = crate::W<Dcsub3ClipSpec>;
#[doc = "Field `DCSUB3_CLIP` reader - 0:0\\]
Indicates the DC subtraction clip status for bcnt =2"]
pub type Dcsub3ClipR = crate::BitReader;
#[doc = "Field `DCSUB3_CLIP` writer - 0:0\\]
Indicates the DC subtraction clip status for bcnt =2"]
pub type Dcsub3ClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the DC subtraction clip status for bcnt =2"]
    #[inline(always)]
    pub fn dcsub3_clip(&self) -> Dcsub3ClipR {
        Dcsub3ClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the DC subtraction clip status for bcnt =2"]
    #[inline(always)]
    #[must_use]
    pub fn dcsub3_clip(&mut self) -> Dcsub3ClipW<Dcsub3ClipSpec> {
        Dcsub3ClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcsub3ClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "DCSUB3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcsub3_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsub3_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcsub3ClipSpec;
impl crate::RegisterSpec for Dcsub3ClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcsub3_clip::R`](R) reader structure"]
impl crate::Readable for Dcsub3ClipSpec {}
#[doc = "`write(|w| ..)` method takes [`dcsub3_clip::W`](W) writer structure"]
impl crate::Writable for Dcsub3ClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCSUB3_CLIP to value 0"]
impl crate::Resettable for Dcsub3ClipSpec {
    const RESET_VALUE: u32 = 0;
}
