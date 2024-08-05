#[doc = "Register `SCIPIO6` reader"]
pub type R = crate::R<Scipio6Spec>;
#[doc = "Register `SCIPIO6` writer"]
pub type W = crate::W<Scipio6Spec>;
#[doc = "Field `CLKDR` reader - 0:0\\]
Reserved"]
pub type ClkdrR = crate::BitReader;
#[doc = "Field `CLKDR` writer - 0:0\\]
Reserved"]
pub type ClkdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPDR` reader - 1:1\\]
Receive pin open drain enable. This bit is effective in LIN or SCI mode. This bit enables open-drain capability in the output pin LINRX."]
pub type RxpdrR = crate::BitReader;
#[doc = "Field `RXPDR` writer - 1:1\\]
Receive pin open drain enable. This bit is effective in LIN or SCI mode. This bit enables open-drain capability in the output pin LINRX."]
pub type RxpdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPDR` reader - 2:2\\]
Transmit pin open drain enable. This bit is effective in LIN or SCI mode. This bit enables open-drain capability in the output pin LINTX."]
pub type TxpdrR = crate::BitReader;
#[doc = "Field `TXPDR` writer - 2:2\\]
Transmit pin open drain enable. This bit is effective in LIN or SCI mode. This bit enables open-drain capability in the output pin LINTX."]
pub type TxpdrW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn clkdr(&self) -> ClkdrR {
        ClkdrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin open drain enable. This bit is effective in LIN or SCI mode. This bit enables open-drain capability in the output pin LINRX."]
    #[inline(always)]
    pub fn rxpdr(&self) -> RxpdrR {
        RxpdrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin open drain enable. This bit is effective in LIN or SCI mode. This bit enables open-drain capability in the output pin LINTX."]
    #[inline(always)]
    pub fn txpdr(&self) -> TxpdrR {
        TxpdrR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn clkdr(&mut self) -> ClkdrW<Scipio6Spec> {
        ClkdrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin open drain enable. This bit is effective in LIN or SCI mode. This bit enables open-drain capability in the output pin LINRX."]
    #[inline(always)]
    #[must_use]
    pub fn rxpdr(&mut self) -> RxpdrW<Scipio6Spec> {
        RxpdrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin open drain enable. This bit is effective in LIN or SCI mode. This bit enables open-drain capability in the output pin LINTX."]
    #[inline(always)]
    #[must_use]
    pub fn txpdr(&mut self) -> TxpdrW<Scipio6Spec> {
        TxpdrW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio6Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "SCIPIO6\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio6Spec;
impl crate::RegisterSpec for Scipio6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio6::R`](R) reader structure"]
impl crate::Readable for Scipio6Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio6::W`](W) writer structure"]
impl crate::Writable for Scipio6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO6 to value 0"]
impl crate::Resettable for Scipio6Spec {
    const RESET_VALUE: u32 = 0;
}
