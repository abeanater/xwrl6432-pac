#[doc = "Register `SCIPIO1` reader"]
pub type R = crate::R<Scipio1Spec>;
#[doc = "Register `SCIPIO1` writer"]
pub type W = crate::W<Scipio1Spec>;
#[doc = "Field `CLKDIR` reader - 0:0\\]
Reserved"]
pub type ClkdirR = crate::BitReader;
#[doc = "Field `CLKDIR` writer - 0:0\\]
Reserved"]
pub type ClkdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIR` reader - 1:1\\]
Receive pin direction. This bit is effective in LIN or SCI mode. This bit determines the data direction on the LINRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). 0: general purpose input pin. 1: general-purpose output pin"]
pub type RxdirR = crate::BitReader;
#[doc = "Field `RXDIR` writer - 1:1\\]
Receive pin direction. This bit is effective in LIN or SCI mode. This bit determines the data direction on the LINRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). 0: general purpose input pin. 1: general-purpose output pin"]
pub type RxdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIR` reader - 2:2\\]
Transmit pin direction. This bit is effective in LIN or SCI mode. This bit determines the data direction on the LINTX pin if it is configured with general-purpose I/O functionality (TX FUNC = 0). 0: general purpose input pin. 1: general-purpose output pin"]
pub type TxdirR = crate::BitReader;
#[doc = "Field `TXDIR` writer - 2:2\\]
Transmit pin direction. This bit is effective in LIN or SCI mode. This bit determines the data direction on the LINTX pin if it is configured with general-purpose I/O functionality (TX FUNC = 0). 0: general purpose input pin. 1: general-purpose output pin"]
pub type TxdirW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn clkdir(&self) -> ClkdirR {
        ClkdirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin direction. This bit is effective in LIN or SCI mode. This bit determines the data direction on the LINRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). 0: general purpose input pin. 1: general-purpose output pin"]
    #[inline(always)]
    pub fn rxdir(&self) -> RxdirR {
        RxdirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin direction. This bit is effective in LIN or SCI mode. This bit determines the data direction on the LINTX pin if it is configured with general-purpose I/O functionality (TX FUNC = 0). 0: general purpose input pin. 1: general-purpose output pin"]
    #[inline(always)]
    pub fn txdir(&self) -> TxdirR {
        TxdirR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn clkdir(&mut self) -> ClkdirW<Scipio1Spec> {
        ClkdirW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin direction. This bit is effective in LIN or SCI mode. This bit determines the data direction on the LINRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). 0: general purpose input pin. 1: general-purpose output pin"]
    #[inline(always)]
    #[must_use]
    pub fn rxdir(&mut self) -> RxdirW<Scipio1Spec> {
        RxdirW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin direction. This bit is effective in LIN or SCI mode. This bit determines the data direction on the LINTX pin if it is configured with general-purpose I/O functionality (TX FUNC = 0). 0: general purpose input pin. 1: general-purpose output pin"]
    #[inline(always)]
    #[must_use]
    pub fn txdir(&mut self) -> TxdirW<Scipio1Spec> {
        TxdirW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio1Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "SCIPIO1\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio1Spec;
impl crate::RegisterSpec for Scipio1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio1::R`](R) reader structure"]
impl crate::Readable for Scipio1Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio1::W`](W) writer structure"]
impl crate::Writable for Scipio1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO1 to value 0"]
impl crate::Resettable for Scipio1Spec {
    const RESET_VALUE: u32 = 0;
}
