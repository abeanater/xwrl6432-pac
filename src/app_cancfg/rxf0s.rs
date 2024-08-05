#[doc = "Register `RXF0S` reader"]
pub type R = crate::R<Rxf0sSpec>;
#[doc = "Register `RXF0S` writer"]
pub type W = crate::W<Rxf0sSpec>;
#[doc = "Field `F0FL` reader - 6:0\\]
Rx FIFO 0 Fill Level"]
pub type F0flR = crate::FieldReader;
#[doc = "Field `F0FL` writer - 6:0\\]
Rx FIFO 0 Fill Level"]
pub type F0flW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU43` reader - 7:7\\]
Reserved"]
pub type Nu43R = crate::BitReader;
#[doc = "Field `NU43` writer - 7:7\\]
Reserved"]
pub type Nu43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0GI` reader - 13:8\\]
Rx FIFO 0 Get Index"]
pub type F0giR = crate::FieldReader;
#[doc = "Field `F0GI` writer - 13:8\\]
Rx FIFO 0 Get Index"]
pub type F0giW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU44` reader - 15:14\\]
Reserved"]
pub type Nu44R = crate::FieldReader;
#[doc = "Field `NU44` writer - 15:14\\]
Reserved"]
pub type Nu44W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F0PI` reader - 21:16\\]
Rx FIFO 0 Put Index"]
pub type F0piR = crate::FieldReader;
#[doc = "Field `F0PI` writer - 21:16\\]
Rx FIFO 0 Put Index"]
pub type F0piW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU45` reader - 23:22\\]
Reserved"]
pub type Nu45R = crate::FieldReader;
#[doc = "Field `NU45` writer - 23:22\\]
Reserved"]
pub type Nu45W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F0F` reader - 24:24\\]
Rx FIFO 0 Full"]
pub type F0fR = crate::BitReader;
#[doc = "Field `F0F` writer - 24:24\\]
Rx FIFO 0 Full"]
pub type F0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - 25:25\\]
Rx FIFO 0 Message Lost"]
pub type Rf0lR = crate::BitReader;
#[doc = "Field `RF0L` writer - 25:25\\]
Rx FIFO 0 Message Lost"]
pub type Rf0lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU46` reader - 31:26\\]
Reserved"]
pub type Nu46R = crate::FieldReader;
#[doc = "Field `NU46` writer - 31:26\\]
Reserved"]
pub type Nu46W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0flR {
        F0flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    pub fn nu43(&self) -> Nu43R {
        Nu43R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0giR {
        F0giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    pub fn nu44(&self) -> Nu44R {
        Nu44R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0piR {
        F0piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved"]
    #[inline(always)]
    pub fn nu45(&self) -> Nu45R {
        Nu45R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0fR {
        F0fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn nu46(&self) -> Nu46R {
        Nu46R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Rx FIFO 0 Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn f0fl(&mut self) -> F0flW<Rxf0sSpec> {
        F0flW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu43(&mut self) -> Nu43W<Rxf0sSpec> {
        Nu43W::new(self, 7)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Rx FIFO 0 Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0gi(&mut self) -> F0giW<Rxf0sSpec> {
        F0giW::new(self, 8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu44(&mut self) -> Nu44W<Rxf0sSpec> {
        Nu44W::new(self, 14)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Rx FIFO 0 Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0pi(&mut self) -> F0piW<Rxf0sSpec> {
        F0piW::new(self, 16)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu45(&mut self) -> Nu45W<Rxf0sSpec> {
        Nu45W::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn f0f(&mut self) -> F0fW<Rxf0sSpec> {
        F0fW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> Rf0lW<Rxf0sSpec> {
        Rf0lW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu46(&mut self) -> Nu46W<Rxf0sSpec> {
        Nu46W::new(self, 26)
    }
}
#[doc = "RXF0S\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf0sSpec;
impl crate::RegisterSpec for Rxf0sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0s::R`](R) reader structure"]
impl crate::Readable for Rxf0sSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf0s::W`](W) writer structure"]
impl crate::Writable for Rxf0sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for Rxf0sSpec {
    const RESET_VALUE: u32 = 0;
}
