#[doc = "Register `GIOOFFB` reader"]
pub type R = crate::R<GiooffbSpec>;
#[doc = "Register `GIOOFFB` writer"]
pub type W = crate::W<GiooffbSpec>;
#[doc = "Field `GIOOFFB` reader - 5:0\\]
Index bits for currently pending high-priority interrupt Register B"]
pub type GiooffbR = crate::FieldReader;
#[doc = "Field `GIOOFFB` writer - 5:0\\]
Index bits for currently pending high-priority interrupt Register B"]
pub type GiooffbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU2` reader - 31:6\\]
Reserved"]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - 31:6\\]
Reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Index bits for currently pending high-priority interrupt Register B"]
    #[inline(always)]
    pub fn giooffb(&self) -> GiooffbR {
        GiooffbR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Index bits for currently pending high-priority interrupt Register B"]
    #[inline(always)]
    #[must_use]
    pub fn giooffb(&mut self) -> GiooffbW<GiooffbSpec> {
        GiooffbW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<GiooffbSpec> {
        Nu2W::new(self, 6)
    }
}
#[doc = "Index bits for currently pending high-priority interrupt Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`giooffb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giooffb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiooffbSpec;
impl crate::RegisterSpec for GiooffbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giooffb::R`](R) reader structure"]
impl crate::Readable for GiooffbSpec {}
#[doc = "`write(|w| ..)` method takes [`giooffb::W`](W) writer structure"]
impl crate::Writable for GiooffbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOOFFB to value 0"]
impl crate::Resettable for GiooffbSpec {
    const RESET_VALUE: u32 = 0;
}
