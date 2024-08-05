#[doc = "Register `SCIPIO8` reader"]
pub type R = crate::R<Scipio8Spec>;
#[doc = "Register `SCIPIO8` writer"]
pub type W = crate::W<Scipio8Spec>;
#[doc = "Field `CLKPSL` reader - 0:0\\]
Reserved"]
pub type ClkpslR = crate::BitReader;
#[doc = "Field `CLKPSL` writer - 0:0\\]
Reserved"]
pub type ClkpslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPSL` reader - 1:1\\]
RX pin pull select. This bit is effective in LIN or SCI mode. This bit selects pull type in the input pin LINRX."]
pub type RxpslR = crate::BitReader;
#[doc = "Field `RXPSL` writer - 1:1\\]
RX pin pull select. This bit is effective in LIN or SCI mode. This bit selects pull type in the input pin LINRX."]
pub type RxpslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPSL` reader - 2:2\\]
TX pin pull select. This bit is effective in LIN or SCI mode. This bit selects pull type in the input pin LINTX."]
pub type TxpslR = crate::BitReader;
#[doc = "Field `TXPSL` writer - 2:2\\]
TX pin pull select. This bit is effective in LIN or SCI mode. This bit selects pull type in the input pin LINTX."]
pub type TxpslW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn clkpsl(&self) -> ClkpslR {
        ClkpslR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RX pin pull select. This bit is effective in LIN or SCI mode. This bit selects pull type in the input pin LINRX."]
    #[inline(always)]
    pub fn rxpsl(&self) -> RxpslR {
        RxpslR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TX pin pull select. This bit is effective in LIN or SCI mode. This bit selects pull type in the input pin LINTX."]
    #[inline(always)]
    pub fn txpsl(&self) -> TxpslR {
        TxpslR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn clkpsl(&mut self) -> ClkpslW<Scipio8Spec> {
        ClkpslW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RX pin pull select. This bit is effective in LIN or SCI mode. This bit selects pull type in the input pin LINRX."]
    #[inline(always)]
    #[must_use]
    pub fn rxpsl(&mut self) -> RxpslW<Scipio8Spec> {
        RxpslW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TX pin pull select. This bit is effective in LIN or SCI mode. This bit selects pull type in the input pin LINTX."]
    #[inline(always)]
    #[must_use]
    pub fn txpsl(&mut self) -> TxpslW<Scipio8Spec> {
        TxpslW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio8Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "SCIPIO8\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio8Spec;
impl crate::RegisterSpec for Scipio8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio8::R`](R) reader structure"]
impl crate::Readable for Scipio8Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio8::W`](W) writer structure"]
impl crate::Writable for Scipio8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO8 to value 0"]
impl crate::Resettable for Scipio8Spec {
    const RESET_VALUE: u32 = 0;
}
