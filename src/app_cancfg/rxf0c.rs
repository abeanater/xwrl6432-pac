#[doc = "Register `RXF0C` reader"]
pub type R = crate::R<Rxf0cSpec>;
#[doc = "Register `RXF0C` writer"]
pub type W = crate::W<Rxf0cSpec>;
#[doc = "Field `NU41` reader - 1:0\\]
Reserved"]
pub type Nu41R = crate::FieldReader;
#[doc = "Field `NU41` writer - 1:0\\]
Reserved"]
pub type Nu41W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F0SA` reader - 14:2\\]
Rx FIFO 0 Start Address"]
pub type F0saR = crate::FieldReader<u16>;
#[doc = "Field `F0SA` writer - 14:2\\]
Rx FIFO 0 Start Address"]
pub type F0saW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `NU42` reader - 15:15\\]
Reserved"]
pub type Nu42R = crate::BitReader;
#[doc = "Field `NU42` writer - 15:15\\]
Reserved"]
pub type Nu42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0S` reader - 22:16\\]
Rx FIFO 0 Size"]
pub type F0sR = crate::FieldReader;
#[doc = "Field `F0S` writer - 22:16\\]
Rx FIFO 0 Size"]
pub type F0sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU42_1` reader - 23:23\\]
Reserved"]
pub type Nu42_1R = crate::BitReader;
#[doc = "Field `NU42_1` writer - 23:23\\]
Reserved"]
pub type Nu42_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0WM` reader - 30:24\\]
Rx FIFO 0 Watermark"]
pub type F0wmR = crate::FieldReader;
#[doc = "Field `F0WM` writer - 30:24\\]
Rx FIFO 0 Watermark"]
pub type F0wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0OM` reader - 31:31\\]
Rx FIFO 0 Operation Mode"]
pub type F0omR = crate::BitReader;
#[doc = "Field `F0OM` writer - 31:31\\]
Rx FIFO 0 Operation Mode"]
pub type F0omW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu41(&self) -> Nu41R {
        Nu41R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0saR {
        F0saR::new(((self.bits >> 2) & 0x1fff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    pub fn nu42(&self) -> Nu42R {
        Nu42R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&self) -> F0sR {
        F0sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved"]
    #[inline(always)]
    pub fn nu42_1(&self) -> Nu42_1R {
        Nu42_1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Rx FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0wmR {
        F0wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Rx FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0omR {
        F0omR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu41(&mut self) -> Nu41W<Rxf0cSpec> {
        Nu41W::new(self, 0)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Rx FIFO 0 Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn f0sa(&mut self) -> F0saW<Rxf0cSpec> {
        F0saW::new(self, 2)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu42(&mut self) -> Nu42W<Rxf0cSpec> {
        Nu42W::new(self, 15)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Rx FIFO 0 Size"]
    #[inline(always)]
    #[must_use]
    pub fn f0s(&mut self) -> F0sW<Rxf0cSpec> {
        F0sW::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu42_1(&mut self) -> Nu42_1W<Rxf0cSpec> {
        Nu42_1W::new(self, 23)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Rx FIFO 0 Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn f0wm(&mut self) -> F0wmW<Rxf0cSpec> {
        F0wmW::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
Rx FIFO 0 Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0omW<Rxf0cSpec> {
        F0omW::new(self, 31)
    }
}
#[doc = "RXF0C\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf0cSpec;
impl crate::RegisterSpec for Rxf0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0c::R`](R) reader structure"]
impl crate::Readable for Rxf0cSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf0c::W`](W) writer structure"]
impl crate::Writable for Rxf0cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF0C to value 0"]
impl crate::Resettable for Rxf0cSpec {
    const RESET_VALUE: u32 = 0;
}
