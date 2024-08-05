#[doc = "Register `GFC` reader"]
pub type R = crate::R<GfcSpec>;
#[doc = "Register `GFC` writer"]
pub type W = crate::W<GfcSpec>;
#[doc = "Field `RRFE` reader - 0:0\\]
reject Remote Frames Extended"]
pub type RrfeR = crate::BitReader;
#[doc = "Field `RRFE` writer - 0:0\\]
reject Remote Frames Extended"]
pub type RrfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - 1:1\\]
reject Remote Frames Standard"]
pub type RrfsR = crate::BitReader;
#[doc = "Field `RRFS` writer - 1:1\\]
reject Remote Frames Standard"]
pub type RrfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` reader - 3:2\\]
Accept Non-matching Frames Extended"]
pub type AnfeR = crate::FieldReader;
#[doc = "Field `ANFE` writer - 3:2\\]
Accept Non-matching Frames Extended"]
pub type AnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` reader - 5:4\\]
Accept Non-matching Frames Standard"]
pub type AnfsR = crate::FieldReader;
#[doc = "Field `ANFS` writer - 5:4\\]
Accept Non-matching Frames Standard"]
pub type AnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU34` reader - 31:6\\]
Reserved"]
pub type Nu34R = crate::FieldReader<u32>;
#[doc = "Field `NU34` writer - 31:6\\]
Reserved"]
pub type Nu34W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RrfeR {
        RrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RrfsR {
        RrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> AnfeR {
        AnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> AnfsR {
        AnfsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    pub fn nu34(&self) -> Nu34R {
        Nu34R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
reject Remote Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RrfeW<GfcSpec> {
        RrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
reject Remote Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RrfsW<GfcSpec> {
        RrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Accept Non-matching Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> AnfeW<GfcSpec> {
        AnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Accept Non-matching Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> AnfsW<GfcSpec> {
        AnfsW::new(self, 4)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu34(&mut self) -> Nu34W<GfcSpec> {
        Nu34W::new(self, 6)
    }
}
#[doc = "GFC\n\nYou can [`read`](crate::Reg::read) this register and get [`gfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfcSpec;
impl crate::RegisterSpec for GfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfc::R`](R) reader structure"]
impl crate::Readable for GfcSpec {}
#[doc = "`write(|w| ..)` method takes [`gfc::W`](W) writer structure"]
impl crate::Writable for GfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GfcSpec {
    const RESET_VALUE: u32 = 0;
}
