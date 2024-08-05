#[doc = "Register `RXF1S` reader"]
pub type R = crate::R<Rxf1sSpec>;
#[doc = "Register `RXF1S` writer"]
pub type W = crate::W<Rxf1sSpec>;
#[doc = "Field `F1FL` reader - 6:0\\]
Rx FIFO 0 Fill Level"]
pub type F1flR = crate::FieldReader;
#[doc = "Field `F1FL` writer - 6:0\\]
Rx FIFO 0 Fill Level"]
pub type F1flW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU51` reader - 7:7\\]
Reserved"]
pub type Nu51R = crate::BitReader;
#[doc = "Field `NU51` writer - 7:7\\]
Reserved"]
pub type Nu51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1GI` reader - 13:8\\]
Rx FIFO 0 Get Index"]
pub type F1giR = crate::FieldReader;
#[doc = "Field `F1GI` writer - 13:8\\]
Rx FIFO 0 Get Index"]
pub type F1giW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU52` reader - 15:14\\]
Reserved"]
pub type Nu52R = crate::FieldReader;
#[doc = "Field `NU52` writer - 15:14\\]
Reserved"]
pub type Nu52W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F1PI` reader - 21:16\\]
Rx FIFO 0 Put Index"]
pub type F1piR = crate::FieldReader;
#[doc = "Field `F1PI` writer - 21:16\\]
Rx FIFO 0 Put Index"]
pub type F1piW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU53` reader - 23:22\\]
Reserved"]
pub type Nu53R = crate::FieldReader;
#[doc = "Field `NU53` writer - 23:22\\]
Reserved"]
pub type Nu53W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F1F` reader - 24:24\\]
Rx FIFO 0 Full"]
pub type F1fR = crate::BitReader;
#[doc = "Field `F1F` writer - 24:24\\]
Rx FIFO 0 Full"]
pub type F1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1L` reader - 25:25\\]
Rx FIFO 0 Message Lost"]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `RF1L` writer - 25:25\\]
Rx FIFO 0 Message Lost"]
pub type Rf1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU54` reader - 31:26\\]
Reserved"]
pub type Nu54R = crate::FieldReader;
#[doc = "Field `NU54` writer - 31:26\\]
Reserved"]
pub type Nu54W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1flR {
        F1flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    pub fn nu51(&self) -> Nu51R {
        Nu51R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1giR {
        F1giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    pub fn nu52(&self) -> Nu52R {
        Nu52R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1piR {
        F1piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved"]
    #[inline(always)]
    pub fn nu53(&self) -> Nu53R {
        Nu53R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1fR {
        F1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn nu54(&self) -> Nu54R {
        Nu54R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Rx FIFO 0 Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn f1fl(&mut self) -> F1flW<Rxf1sSpec> {
        F1flW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu51(&mut self) -> Nu51W<Rxf1sSpec> {
        Nu51W::new(self, 7)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Rx FIFO 0 Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1gi(&mut self) -> F1giW<Rxf1sSpec> {
        F1giW::new(self, 8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu52(&mut self) -> Nu52W<Rxf1sSpec> {
        Nu52W::new(self, 14)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Rx FIFO 0 Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1pi(&mut self) -> F1piW<Rxf1sSpec> {
        F1piW::new(self, 16)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu53(&mut self) -> Nu53W<Rxf1sSpec> {
        Nu53W::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn f1f(&mut self) -> F1fW<Rxf1sSpec> {
        F1fW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> Rf1lW<Rxf1sSpec> {
        Rf1lW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu54(&mut self) -> Nu54W<Rxf1sSpec> {
        Nu54W::new(self, 26)
    }
}
#[doc = "RXF1S\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf1sSpec;
impl crate::RegisterSpec for Rxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1s::R`](R) reader structure"]
impl crate::Readable for Rxf1sSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf1s::W`](W) writer structure"]
impl crate::Writable for Rxf1sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for Rxf1sSpec {
    const RESET_VALUE: u32 = 0;
}
