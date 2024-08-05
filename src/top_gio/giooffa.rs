#[doc = "Register `GIOOFFA` reader"]
pub type R = crate::R<GiooffaSpec>;
#[doc = "Register `GIOOFFA` writer"]
pub type W = crate::W<GiooffaSpec>;
#[doc = "Field `GIOOFFA` reader - 5:0\\]
Index bits for currently pending high-priority interrupt Register A"]
pub type GiooffaR = crate::FieldReader;
#[doc = "Field `GIOOFFA` writer - 5:0\\]
Index bits for currently pending high-priority interrupt Register A"]
pub type GiooffaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU1` reader - 31:6\\]
Reserved"]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - 31:6\\]
Reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Index bits for currently pending high-priority interrupt Register A"]
    #[inline(always)]
    pub fn giooffa(&self) -> GiooffaR {
        GiooffaR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Index bits for currently pending high-priority interrupt Register A"]
    #[inline(always)]
    #[must_use]
    pub fn giooffa(&mut self) -> GiooffaW<GiooffaSpec> {
        GiooffaW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<GiooffaSpec> {
        Nu1W::new(self, 6)
    }
}
#[doc = "Index bits for currently pending high-priority interrupt Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`giooffa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giooffa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiooffaSpec;
impl crate::RegisterSpec for GiooffaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giooffa::R`](R) reader structure"]
impl crate::Readable for GiooffaSpec {}
#[doc = "`write(|w| ..)` method takes [`giooffa::W`](W) writer structure"]
impl crate::Writable for GiooffaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOOFFA to value 0"]
impl crate::Resettable for GiooffaSpec {
    const RESET_VALUE: u32 = 0;
}
