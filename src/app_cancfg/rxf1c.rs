#[doc = "Register `RXF1C` reader"]
pub type R = crate::R<Rxf1cSpec>;
#[doc = "Register `RXF1C` writer"]
pub type W = crate::W<Rxf1cSpec>;
#[doc = "Field `NU499` reader - 1:0\\]
Reserved"]
pub type Nu499R = crate::FieldReader;
#[doc = "Field `NU499` writer - 1:0\\]
Reserved"]
pub type Nu499W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F1SA` reader - 14:2\\]
Rx FIFO 0 Start Address"]
pub type F1saR = crate::FieldReader<u16>;
#[doc = "Field `F1SA` writer - 14:2\\]
Rx FIFO 0 Start Address"]
pub type F1saW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `NU50` reader - 15:15\\]
Reserved"]
pub type Nu50R = crate::BitReader;
#[doc = "Field `NU50` writer - 15:15\\]
Reserved"]
pub type Nu50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1S` reader - 22:16\\]
Rx FIFO 0 Size"]
pub type F1sR = crate::FieldReader;
#[doc = "Field `F1S` writer - 22:16\\]
Rx FIFO 0 Size"]
pub type F1sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU50_1` reader - 23:23\\]
Reserved"]
pub type Nu50_1R = crate::BitReader;
#[doc = "Field `NU50_1` writer - 23:23\\]
Reserved"]
pub type Nu50_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1WM` reader - 30:24\\]
Rx FIFO 0 Watermark"]
pub type F1wmR = crate::FieldReader;
#[doc = "Field `F1WM` writer - 30:24\\]
Rx FIFO 0 Watermark"]
pub type F1wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1OM` reader - 31:31\\]
Rx FIFO 0 Operation Mode"]
pub type F1omR = crate::BitReader;
#[doc = "Field `F1OM` writer - 31:31\\]
Rx FIFO 0 Operation Mode"]
pub type F1omW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu499(&self) -> Nu499R {
        Nu499R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1saR {
        F1saR::new(((self.bits >> 2) & 0x1fff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    pub fn nu50(&self) -> Nu50R {
        Nu50R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f1s(&self) -> F1sR {
        F1sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved"]
    #[inline(always)]
    pub fn nu50_1(&self) -> Nu50_1R {
        Nu50_1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Rx FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1wmR {
        F1wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Rx FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1omR {
        F1omR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu499(&mut self) -> Nu499W<Rxf1cSpec> {
        Nu499W::new(self, 0)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Rx FIFO 0 Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn f1sa(&mut self) -> F1saW<Rxf1cSpec> {
        F1saW::new(self, 2)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu50(&mut self) -> Nu50W<Rxf1cSpec> {
        Nu50W::new(self, 15)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Rx FIFO 0 Size"]
    #[inline(always)]
    #[must_use]
    pub fn f1s(&mut self) -> F1sW<Rxf1cSpec> {
        F1sW::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu50_1(&mut self) -> Nu50_1W<Rxf1cSpec> {
        Nu50_1W::new(self, 23)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Rx FIFO 0 Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn f1wm(&mut self) -> F1wmW<Rxf1cSpec> {
        F1wmW::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
Rx FIFO 0 Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1omW<Rxf1cSpec> {
        F1omW::new(self, 31)
    }
}
#[doc = "RXF1C\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf1cSpec;
impl crate::RegisterSpec for Rxf1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1c::R`](R) reader structure"]
impl crate::Readable for Rxf1cSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf1c::W`](W) writer structure"]
impl crate::Writable for Rxf1cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF1C to value 0"]
impl crate::Resettable for Rxf1cSpec {
    const RESET_VALUE: u32 = 0;
}
