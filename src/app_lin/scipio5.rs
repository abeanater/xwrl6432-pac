#[doc = "Register `SCIPIO5` reader"]
pub type R = crate::R<Scipio5Spec>;
#[doc = "Register `SCIPIO5` writer"]
pub type W = crate::W<Scipio5Spec>;
#[doc = "Field `CLKCLR` reader - 0:0\\]
Reserved"]
pub type ClkclrR = crate::BitReader;
#[doc = "Field `CLKCLR` writer - 0:0\\]
Reserved"]
pub type ClkclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCLR` reader - 1:1\\]
Receive pin clear. This bit is effective in LIN or SCI mode. This bit clears the logic to be output on pin LINRX."]
pub type RxclrR = crate::BitReader;
#[doc = "Field `RXCLR` writer - 1:1\\]
Receive pin clear. This bit is effective in LIN or SCI mode. This bit clears the logic to be output on pin LINRX."]
pub type RxclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCLR` reader - 2:2\\]
Transmit pin clear. This bit is effective in LIN or SCI mode. This bit clears the logic to be output on pin LINTX."]
pub type TxclrR = crate::BitReader;
#[doc = "Field `TXCLR` writer - 2:2\\]
Transmit pin clear. This bit is effective in LIN or SCI mode. This bit clears the logic to be output on pin LINTX."]
pub type TxclrW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn clkclr(&self) -> ClkclrR {
        ClkclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin clear. This bit is effective in LIN or SCI mode. This bit clears the logic to be output on pin LINRX."]
    #[inline(always)]
    pub fn rxclr(&self) -> RxclrR {
        RxclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin clear. This bit is effective in LIN or SCI mode. This bit clears the logic to be output on pin LINTX."]
    #[inline(always)]
    pub fn txclr(&self) -> TxclrR {
        TxclrR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn clkclr(&mut self) -> ClkclrW<Scipio5Spec> {
        ClkclrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin clear. This bit is effective in LIN or SCI mode. This bit clears the logic to be output on pin LINRX."]
    #[inline(always)]
    #[must_use]
    pub fn rxclr(&mut self) -> RxclrW<Scipio5Spec> {
        RxclrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin clear. This bit is effective in LIN or SCI mode. This bit clears the logic to be output on pin LINTX."]
    #[inline(always)]
    #[must_use]
    pub fn txclr(&mut self) -> TxclrW<Scipio5Spec> {
        TxclrW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio5Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "SCIPIO5\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio5Spec;
impl crate::RegisterSpec for Scipio5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio5::R`](R) reader structure"]
impl crate::Readable for Scipio5Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio5::W`](W) writer structure"]
impl crate::Writable for Scipio5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO5 to value 0"]
impl crate::Resettable for Scipio5Spec {
    const RESET_VALUE: u32 = 0;
}
