#[doc = "Register `GIOEMUB` reader"]
pub type R = crate::R<GioemubSpec>;
#[doc = "Register `GIOEMUB` writer"]
pub type W = crate::W<GioemubSpec>;
#[doc = "Field `GIOEMUB` reader - 5:0\\]
GIO emulation register B"]
pub type GioemubR = crate::FieldReader;
#[doc = "Field `GIOEMUB` writer - 5:0\\]
GIO emulation register B"]
pub type GioemubW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU4` reader - 31:6\\]
Reserved"]
pub type Nu4R = crate::FieldReader<u32>;
#[doc = "Field `NU4` writer - 31:6\\]
Reserved"]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
GIO emulation register B"]
    #[inline(always)]
    pub fn gioemub(&self) -> GioemubR {
        GioemubR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
GIO emulation register B"]
    #[inline(always)]
    #[must_use]
    pub fn gioemub(&mut self) -> GioemubW<GioemubSpec> {
        GioemubW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<GioemubSpec> {
        Nu4W::new(self, 6)
    }
}
#[doc = "GIO emulation register B\n\nYou can [`read`](crate::Reg::read) this register and get [`gioemub::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioemub::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioemubSpec;
impl crate::RegisterSpec for GioemubSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioemub::R`](R) reader structure"]
impl crate::Readable for GioemubSpec {}
#[doc = "`write(|w| ..)` method takes [`gioemub::W`](W) writer structure"]
impl crate::Writable for GioemubSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOEMUB to value 0"]
impl crate::Resettable for GioemubSpec {
    const RESET_VALUE: u32 = 0;
}
