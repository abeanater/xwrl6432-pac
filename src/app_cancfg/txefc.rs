#[doc = "Register `TXEFC` reader"]
pub type R = crate::R<TxefcSpec>;
#[doc = "Register `TXEFC` writer"]
pub type W = crate::W<TxefcSpec>;
#[doc = "Field `NU66` reader - 1:0\\]
Reserved"]
pub type Nu66R = crate::FieldReader;
#[doc = "Field `NU66` writer - 1:0\\]
Reserved"]
pub type Nu66W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFSA` reader - 15:2\\]
Event FIFO Start Address"]
pub type EfsaR = crate::FieldReader<u16>;
#[doc = "Field `EFSA` writer - 15:2\\]
Event FIFO Start Address"]
pub type EfsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `EFS` reader - 21:16\\]
Event FIFO Size"]
pub type EfsR = crate::FieldReader;
#[doc = "Field `EFS` writer - 21:16\\]
Event FIFO Size"]
pub type EfsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU67` reader - 23:22\\]
Reserved"]
pub type Nu67R = crate::FieldReader;
#[doc = "Field `NU67` writer - 23:22\\]
Reserved"]
pub type Nu67W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFWM` reader - 29:24\\]
Event FIFO Watermark"]
pub type EfwmR = crate::FieldReader;
#[doc = "Field `EFWM` writer - 29:24\\]
Event FIFO Watermark"]
pub type EfwmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU68` reader - 31:30\\]
Reserved"]
pub type Nu68R = crate::FieldReader;
#[doc = "Field `NU68` writer - 31:30\\]
Reserved"]
pub type Nu68W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu66(&self) -> Nu66R {
        Nu66R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&self) -> EfsaR {
        EfsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&self) -> EfsR {
        EfsR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved"]
    #[inline(always)]
    pub fn nu67(&self) -> Nu67R {
        Nu67R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&self) -> EfwmR {
        EfwmR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Reserved"]
    #[inline(always)]
    pub fn nu68(&self) -> Nu68R {
        Nu68R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu66(&mut self) -> Nu66W<TxefcSpec> {
        Nu66W::new(self, 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Event FIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn efsa(&mut self) -> EfsaW<TxefcSpec> {
        EfsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Event FIFO Size"]
    #[inline(always)]
    #[must_use]
    pub fn efs(&mut self) -> EfsW<TxefcSpec> {
        EfsW::new(self, 16)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu67(&mut self) -> Nu67W<TxefcSpec> {
        Nu67W::new(self, 22)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Event FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn efwm(&mut self) -> EfwmW<TxefcSpec> {
        EfwmW::new(self, 24)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu68(&mut self) -> Nu68W<TxefcSpec> {
        Nu68W::new(self, 30)
    }
}
#[doc = "TXEFC\n\nYou can [`read`](crate::Reg::read) this register and get [`txefc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxefcSpec;
impl crate::RegisterSpec for TxefcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefc::R`](R) reader structure"]
impl crate::Readable for TxefcSpec {}
#[doc = "`write(|w| ..)` method takes [`txefc::W`](W) writer structure"]
impl crate::Writable for TxefcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXEFC to value 0"]
impl crate::Resettable for TxefcSpec {
    const RESET_VALUE: u32 = 0;
}
