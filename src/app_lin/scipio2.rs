#[doc = "Register `SCIPIO2` reader"]
pub type R = crate::R<Scipio2Spec>;
#[doc = "Register `SCIPIO2` writer"]
pub type W = crate::W<Scipio2Spec>;
#[doc = "Field `CLKIN` reader - 0:0\\]
Reserved"]
pub type ClkinR = crate::BitReader;
#[doc = "Field `CLKIN` writer - 0:0\\]
Reserved"]
pub type ClkinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIN` reader - 1:1\\]
Receive data in. This bit is effective in LIN or SCI-compatible mode. This bit contains the current value on the LINRX pin."]
pub type RxinR = crate::BitReader;
#[doc = "Field `RXIN` writer - 1:1\\]
Receive data in. This bit is effective in LIN or SCI-compatible mode. This bit contains the current value on the LINRX pin."]
pub type RxinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIN` reader - 2:2\\]
Transmit data in. This bit is effective in LIN or SCI-compatible mode. This bit contains the current value on the LINTX pin."]
pub type TxinR = crate::BitReader;
#[doc = "Field `TXIN` writer - 2:2\\]
Transmit data in. This bit is effective in LIN or SCI-compatible mode. This bit contains the current value on the LINTX pin."]
pub type TxinW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn clkin(&self) -> ClkinR {
        ClkinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive data in. This bit is effective in LIN or SCI-compatible mode. This bit contains the current value on the LINRX pin."]
    #[inline(always)]
    pub fn rxin(&self) -> RxinR {
        RxinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit data in. This bit is effective in LIN or SCI-compatible mode. This bit contains the current value on the LINTX pin."]
    #[inline(always)]
    pub fn txin(&self) -> TxinR {
        TxinR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn clkin(&mut self) -> ClkinW<Scipio2Spec> {
        ClkinW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive data in. This bit is effective in LIN or SCI-compatible mode. This bit contains the current value on the LINRX pin."]
    #[inline(always)]
    #[must_use]
    pub fn rxin(&mut self) -> RxinW<Scipio2Spec> {
        RxinW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit data in. This bit is effective in LIN or SCI-compatible mode. This bit contains the current value on the LINTX pin."]
    #[inline(always)]
    #[must_use]
    pub fn txin(&mut self) -> TxinW<Scipio2Spec> {
        TxinW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio2Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "The SCIPIO2 register indicates the current status of the LINTX and LINRX pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio2Spec;
impl crate::RegisterSpec for Scipio2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio2::R`](R) reader structure"]
impl crate::Readable for Scipio2Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio2::W`](W) writer structure"]
impl crate::Writable for Scipio2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO2 to value 0"]
impl crate::Resettable for Scipio2Spec {
    const RESET_VALUE: u32 = 0;
}
