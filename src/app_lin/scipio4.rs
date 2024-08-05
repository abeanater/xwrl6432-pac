#[doc = "Register `SCIPIO4` reader"]
pub type R = crate::R<Scipio4Spec>;
#[doc = "Register `SCIPIO4` writer"]
pub type W = crate::W<Scipio4Spec>;
#[doc = "Field `CLKSET` reader - 0:0\\]
Reserved"]
pub type ClksetR = crate::BitReader;
#[doc = "Field `CLKSET` writer - 0:0\\]
Reserved"]
pub type ClksetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSET` reader - 1:1\\]
Receive pin set. This bit is effective in LIN or SCI mode. This bit sets the logic to be output on pin LINRX."]
pub type RxsetR = crate::BitReader;
#[doc = "Field `RXSET` writer - 1:1\\]
Receive pin set. This bit is effective in LIN or SCI mode. This bit sets the logic to be output on pin LINRX."]
pub type RxsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSET` reader - 2:2\\]
Transmit pin set. This bit is effective in LIN or SCI mode. This bit sets the logic to be output on pin LINTX."]
pub type TxsetR = crate::BitReader;
#[doc = "Field `TXSET` writer - 2:2\\]
Transmit pin set. This bit is effective in LIN or SCI mode. This bit sets the logic to be output on pin LINTX."]
pub type TxsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 31:16\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:16\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn clkset(&self) -> ClksetR {
        ClksetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin set. This bit is effective in LIN or SCI mode. This bit sets the logic to be output on pin LINRX."]
    #[inline(always)]
    pub fn rxset(&self) -> RxsetR {
        RxsetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin set. This bit is effective in LIN or SCI mode. This bit sets the logic to be output on pin LINTX."]
    #[inline(always)]
    pub fn txset(&self) -> TxsetR {
        TxsetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clkset(&mut self) -> ClksetW<Scipio4Spec> {
        ClksetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin set. This bit is effective in LIN or SCI mode. This bit sets the logic to be output on pin LINRX."]
    #[inline(always)]
    #[must_use]
    pub fn rxset(&mut self) -> RxsetW<Scipio4Spec> {
        RxsetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin set. This bit is effective in LIN or SCI mode. This bit sets the logic to be output on pin LINTX."]
    #[inline(always)]
    #[must_use]
    pub fn txset(&mut self) -> TxsetW<Scipio4Spec> {
        TxsetW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio4Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "SCIPIO4\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio4Spec;
impl crate::RegisterSpec for Scipio4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio4::R`](R) reader structure"]
impl crate::Readable for Scipio4Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio4::W`](W) writer structure"]
impl crate::Writable for Scipio4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO4 to value 0"]
impl crate::Resettable for Scipio4Spec {
    const RESET_VALUE: u32 = 0;
}
